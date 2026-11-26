//! One actor task per lobby owns all of its state. Connections send it
//! commands and watch its latest snapshot (one JSON string everyone shares).
//! Players are keyed by a secret token the browser keeps, so a locked phone or
//! a dropped connection comes back as the same player with the same score.

use crate::quiz::{self, Question, RoundFacts};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, oneshot, watch};
use tokio::time::Instant;

pub const QUESTION_SECS: u64 = 30;
const REVEAL_SECS: u64 = 10;
const IDLE_EXIT: Duration = Duration::from_secs(30 * 60);
const MAX_PLAYERS: usize = 40;

static SHAPES: LazyLock<HashMap<String, Value>> =
    LazyLock::new(|| serde_json::from_str(include_str!("../web/shapes.json")).expect("shapes.json"));

pub type Registry = Arc<Mutex<HashMap<String, mpsc::Sender<Cmd>>>>;

pub enum Cmd {
    Join { token: String, name: String, reply: oneshot::Sender<Result<Joined, String>> },
    Leave { token: String },
    Start { token: String, rounds: usize },
    Answer { token: String, round: usize, choice: String },
    Again { token: String },
    Rename { token: String, name: String },
}

pub struct Joined {
    pub you: u32,
    pub snapshots: watch::Receiver<Arc<str>>,
}

/// The lobby's command sender, starting the lobby if it isn't running.
pub fn get_or_start(reg: &Registry, code: &str) -> mpsc::Sender<Cmd> {
    let mut map = reg.lock().unwrap();
    if let Some(tx) = map.get(code) {
        if !tx.is_closed() {
            return tx.clone();
        }
    }
    let (tx, rx) = mpsc::channel(256);
    map.insert(code.to_string(), tx.clone());
    tokio::spawn(Lobby::new(code.to_string()).run(rx, reg.clone()));
    tx
}

struct Player {
    id: u32,
    name: String,
    score: u32,
    conns: u32,
    /// this round's pick and how many seconds it took
    pick: Option<(&'static quiz::State, f64)>,
    gained: u32,
    streak: u32,
    best_streak: u32,
    correct: u32,
}

enum Phase {
    Lobby,
    Question { started: Instant, deadline: Instant },
    Reveal { until: Instant, quips: Vec<String> },
    Over { winner: String, loser: Option<String> },
}

struct Lobby {
    code: String,
    players: Vec<(String, Player)>, // (token, player), in join order
    next_id: u32,
    host: Option<u32>,
    phase: Phase,
    questions: Vec<Question>,
    round: usize, // index into questions
    rounds_setting: usize,
    games_played: u32,
    snap: watch::Sender<Arc<str>>,
    idle_since: Option<Instant>,
}

fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}

/// Instant → wall-clock ms for the clients
fn wall(at: Instant) -> u64 {
    let now = Instant::now();
    if at >= now { now_ms() + (at - now).as_millis() as u64 } else { now_ms().saturating_sub((now - at).as_millis() as u64) }
}

pub fn clean_name(name: &str) -> String {
    let n: String = name.chars().filter(|c| !c.is_control()).collect::<String>().trim().chars().take(20).collect();
    if n.is_empty() { "Mystery Guest".into() } else { n }
}

impl Lobby {
    fn new(code: String) -> Self {
        Lobby {
            code,
            players: vec![],
            next_id: 1,
            host: None,
            phase: Phase::Lobby,
            questions: vec![],
            round: 0,
            rounds_setting: 15,
            games_played: 0,
            snap: watch::channel(Arc::from("")).0,
            idle_since: Some(Instant::now()),
        }
    }

    async fn run(mut self, mut rx: mpsc::Receiver<Cmd>, reg: Registry) {
        self.publish();
        loop {
            let wake = match &self.phase {
                Phase::Question { deadline, .. } => Some(*deadline),
                Phase::Reveal { until, .. } => Some(*until),
                _ => None,
            };
            let idle_check = Instant::now() + Duration::from_secs(60);
            tokio::select! {
                cmd = rx.recv() => match cmd {
                    Some(cmd) => self.handle(cmd),
                    None => return,
                },
                _ = tokio::time::sleep_until(wake.unwrap_or(idle_check)) => {
                    if wake.is_some_and(|w| w <= Instant::now()) {
                        self.advance();
                    }
                }
            }
            if self.idle_since.is_some_and(|t| t.elapsed() > IDLE_EXIT) {
                let mut map = reg.lock().unwrap();
                if rx.is_empty() {
                    map.remove(&self.code);
                    return; // joins already in flight see a closed channel and start a fresh lobby
                }
            }
            self.publish();
        }
    }

    fn player(&mut self, token: &str) -> Option<&mut Player> {
        self.players.iter_mut().find(|(t, _)| t == token).map(|(_, p)| p)
    }

    fn online(&self) -> impl Iterator<Item = &Player> {
        self.players.iter().map(|(_, p)| p).filter(|p| p.conns > 0)
    }

    fn is_host(&self, token: &str) -> bool {
        self.players.iter().any(|(t, p)| t == token && Some(p.id) == self.host)
    }

    fn fix_host(&mut self) {
        let host_online = self.online().any(|p| Some(p.id) == self.host);
        if !host_online {
            let first = self.online().next().map(|p| p.id);
            if first.is_some() {
                self.host = first;
            }
        }
        self.idle_since = if self.online().next().is_none() { Some(Instant::now()) } else { None };
    }

    fn handle(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::Join { token, name, reply } => {
                let name = clean_name(&name);
                let id = if let Some(p) = self.player(&token) {
                    p.conns += 1;
                    p.name = name;
                    p.id
                } else if self.players.len() >= MAX_PLAYERS {
                    let _ = reply.send(Err("This lobby is full. Start your own!".into()));
                    return;
                } else {
                    let id = self.next_id;
                    self.next_id += 1;
                    self.players.push((token, Player { id, name, score: 0, conns: 1, pick: None, gained: 0, streak: 0, best_streak: 0, correct: 0 }));
                    id
                };
                self.fix_host();
                let _ = reply.send(Ok(Joined { you: id, snapshots: self.snap.subscribe() }));
            }
            Cmd::Leave { token } => {
                if let Some(p) = self.player(&token) {
                    p.conns = p.conns.saturating_sub(1);
                }
                // a lobby nobody has started yet forgets players who leave
                if matches!(self.phase, Phase::Lobby) {
                    self.players.retain(|(_, p)| p.conns > 0 || p.score > 0);
                }
                self.fix_host();
                self.maybe_end_question();
            }
            Cmd::Rename { token, name } => {
                if let Some(p) = self.player(&token) {
                    p.name = clean_name(&name);
                }
            }
            Cmd::Start { token, rounds } => {
                if self.is_host(&token) && matches!(self.phase, Phase::Lobby | Phase::Over { .. }) {
                    self.rounds_setting = rounds.clamp(5, 30);
                    self.questions = quiz::game(self.rounds_setting);
                    for (_, p) in &mut self.players {
                        p.score = 0;
                        p.streak = 0;
                        p.best_streak = 0;
                        p.correct = 0;
                    }
                    self.players.retain(|(_, p)| p.conns > 0);
                    self.round = 0;
                    self.games_played += 1;
                    self.begin_question();
                }
            }
            Cmd::Again { token } => {
                if self.is_host(&token) && matches!(self.phase, Phase::Over { .. }) {
                    self.players.retain(|(_, p)| p.conns > 0);
                    for (_, p) in &mut self.players {
                        p.score = 0;
                        p.pick = None;
                        p.gained = 0;
                    }
                    self.phase = Phase::Lobby;
                }
            }
            Cmd::Answer { token, round, choice } => {
                let Phase::Question { started, .. } = self.phase else { return };
                if round != self.round {
                    return;
                }
                let Some(q) = self.questions.get(self.round) else { return };
                let Some(pick) = q.options.iter().find(|o| o.id == choice).copied() else { return };
                if let Some(p) = self.player(&token) {
                    if p.pick.is_none() {
                        p.pick = Some((pick, started.elapsed().as_secs_f64()));
                    }
                }
                self.maybe_end_question();
            }
        }
    }

    fn begin_question(&mut self) {
        for (_, p) in &mut self.players {
            p.pick = None;
            p.gained = 0;
        }
        let now = Instant::now();
        self.phase = Phase::Question { started: now, deadline: now + Duration::from_secs(QUESTION_SECS) };
    }

    /// everyone who's here has answered: no point waiting out the clock
    fn maybe_end_question(&mut self) {
        if matches!(self.phase, Phase::Question { .. }) && self.online().next().is_some() && self.online().all(|p| p.pick.is_some()) {
            self.advance();
        }
    }

    fn advance(&mut self) {
        match self.phase {
            Phase::Question { .. } => self.reveal(),
            Phase::Reveal { .. } => {
                self.round += 1;
                if self.round < self.questions.len() {
                    self.begin_question();
                } else {
                    self.finish();
                }
            }
            _ => {}
        }
    }

    fn reveal(&mut self) {
        let q = &self.questions[self.round];
        let leader_before = self.leader();
        let mut facts = RoundFacts {
            state: q.answer,
            picks: vec![],
            everyone_right: true,
            nobody_right: true,
            streaks: vec![],
            broken: vec![],
            new_leader: None,
        };
        for (_, p) in &mut self.players {
            let took_part = p.conns > 0 || p.pick.is_some();
            let right = p.pick.is_some_and(|(s, _)| s.id == q.answer.id);
            if right {
                let (_, secs) = p.pick.unwrap();
                let speed = (1.0 - secs / QUESTION_SECS as f64).clamp(0.0, 1.0);
                p.gained = ((500.0 + 500.0 * speed) / 10.0).round() as u32 * 10 * q.multiplier;
                p.score += p.gained;
                p.streak += 1;
                p.best_streak = p.best_streak.max(p.streak);
                p.correct += 1;
                if p.streak >= 3 {
                    facts.streaks.push((p.name.clone(), p.streak));
                }
            } else {
                if p.streak >= 3 && took_part {
                    facts.broken.push((p.name.clone(), p.streak));
                }
                p.streak = 0;
            }
            if took_part {
                facts.everyone_right &= right;
                facts.nobody_right &= !right;
                facts.picks.push((p.name.clone(), p.pick.map(|(s, _)| s), p.pick.map_or(0.0, |(_, t)| t)));
            }
        }
        let leader_after = self.leader();
        if leader_after.is_some() && leader_after != leader_before && self.players.len() > 1 {
            facts.new_leader = self.players.iter().find(|(_, p)| Some(p.id) == leader_after).map(|(_, p)| p.name.clone());
        }
        let quips = quiz::quips(&facts);
        self.phase = Phase::Reveal { until: Instant::now() + Duration::from_secs(REVEAL_SECS), quips };
    }

    /// sole leader with points, if there is one
    fn leader(&self) -> Option<u32> {
        let top = self.players.iter().map(|(_, p)| p.score).max().filter(|s| *s > 0)?;
        let mut at_top = self.players.iter().filter(|(_, p)| p.score == top);
        let first = at_top.next()?;
        at_top.next().is_none().then_some(first.1.id)
    }

    fn finish(&mut self) {
        let mut ranked: Vec<&Player> = self.players.iter().map(|(_, p)| p).collect();
        ranked.sort_by(|a, b| b.score.cmp(&a.score));
        let winner = ranked.first().map(|p| quiz::winner_line(&p.name)).unwrap_or_default();
        let loser = (ranked.len() > 2).then(|| quiz::loser_line(&ranked.last().unwrap().name));
        self.phase = Phase::Over { winner, loser };
    }

    fn publish(&self) {
        let reveal_picks = matches!(self.phase, Phase::Reveal { .. } | Phase::Over { .. });
        let players: Vec<Value> = self
            .players
            .iter()
            .map(|(_, p)| {
                let mut v = json!({
                    "id": p.id, "name": p.name, "score": p.score, "online": p.conns > 0,
                    "answered": p.pick.is_some(), "streak": p.streak,
                    "correct": p.correct, "best_streak": p.best_streak,
                });
                if reveal_picks {
                    v["pick"] = json!(p.pick.map(|(s, _)| s.id));
                    v["secs"] = json!(p.pick.map(|(_, t)| (t * 10.0).round() / 10.0));
                    v["gained"] = json!(p.gained);
                }
                v
            })
            .collect();
        let mut snap = json!({
            "t": "state", "code": self.code, "now": now_ms(), "host": self.host,
            "players": players, "round": self.round + 1, "rounds": self.questions.len(),
            "rounds_setting": self.rounds_setting, "games": self.games_played,
        });
        let question = |q: &Question| {
            json!({
                "title": q.title, "shape": SHAPES[q.answer.id]["d"], "rotate": q.rotate,
                "modifier": q.modifier, "multiplier": q.multiplier, "tier": q.tier,
                "options": q.options.iter().map(|o| json!({"id": o.id, "name": o.name, "au": o.au()})).collect::<Vec<_>>(),
            })
        };
        match &self.phase {
            Phase::Lobby => snap["phase"] = json!("lobby"),
            Phase::Question { deadline, .. } => {
                snap["phase"] = json!("question");
                let mut q = question(&self.questions[self.round]);
                q["deadline"] = json!(wall(*deadline));
                snap["q"] = q;
            }
            Phase::Reveal { until, quips } => {
                let q = &self.questions[self.round];
                snap["phase"] = json!("reveal");
                snap["q"] = question(q);
                let roasts = q.answer.roasts;
                snap["reveal"] = json!({
                    "answer": q.answer.id, "name": q.answer.name,
                    // stable per round so every screen shows the same roast
                    "roast": roasts[(self.round + self.games_played as usize) % roasts.len()],
                    "quips": quips, "until": wall(*until),
                    "last": self.round + 1 == self.questions.len(),
                });
            }
            Phase::Over { winner, loser } => {
                snap["phase"] = json!("over");
                snap["over"] = json!({ "winner": winner, "loser": loser });
            }
        }
        self.snap.send_replace(Arc::from(snap.to_string()));
    }
}
