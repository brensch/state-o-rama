//! The quiz itself: every state, how hard it is, which ones look alike, the
//! roasts, and the question generator. All the variety lives here: each game
//! draws a fresh plan of difficulties, Aussie rounds and modifiers, decoys come
//! from overlapping look-alike groups, and titles, roasts and quips are pooled.

use rand::seq::{IndexedRandom, SliceRandom};
use rand::RngExt;
use std::collections::HashSet;

pub use crate::content::{GROUPS, STATES};
use std::collections::HashMap;
use std::sync::LazyLock;

/// Measured shape similarity (tools/similarity.mjs): for each state, every
/// other state with a 0..1 overlap score, most similar first.
static SIMILAR: LazyLock<HashMap<String, Vec<(String, f64)>>> =
    LazyLock::new(|| serde_json::from_str(include_str!("../web/similar.json")).expect("similar.json"));

/// Other states, most similar-looking first.
pub fn similar(id: &str) -> Vec<(&'static State, f64)> {
    SIMILAR[id].iter().map(|(o, s)| (by_id(o), *s)).collect()
}
use crate::content as c;

pub struct State {
    pub id: &'static str,
    pub name: &'static str,
    /// 1 easy (everyone gets it), 2 medium, 3 hard (rectangles and friends)
    pub tier: u8,
    pub roasts: &'static [&'static str],
}

impl State {
    pub fn au(&self) -> bool {
        self.id.starts_with("AU-")
    }
}

/// Look-alike groups live in content.rs; a state's decoys come from every group it's in.
pub struct Group {
    pub members: &'static [&'static str],
    pub titles: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Modifier {
    None,
    /// shown rotated 180°, as nature intended
    DownUnder,
    /// shown rotated by a random angle
    Tilt,
    /// double points
    Double,
    /// final round: double points, double shame
    Final,
    /// three Aussie options and one American; the answer could be either
    AussieInvasion,
    /// spins slowly the whole round
    Spin,
    /// starts blurred and sharpens over the round
    Fog,
    /// outline only, no fill
    Outline,
    /// wobbles like a jelly
    Wobble,
    /// an Aussie answer with only Aussie options
    AllAussie,
}

pub struct Question {
    pub answer: &'static State,
    pub options: Vec<&'static State>,
    pub title: String,
    pub modifier: Modifier,
    /// degrees to rotate the shape on screen
    pub rotate: i32,
    pub tier: u8,
    pub multiplier: u32,
}

pub fn by_id(id: &str) -> &'static State {
    STATES.iter().find(|s| s.id == id).unwrap_or_else(|| panic!("unknown state {id}"))
}

/// Hand-picked look-alikes (every group the state is in) plus the shapes that
/// measure as very close (overlap 0.72+, top 3).
fn lookalikes(id: &str) -> Vec<&'static State> {
    let mut seen = HashSet::new();
    let curated = GROUPS
        .iter()
        .filter(|g| g.members.contains(&id))
        .flat_map(|g| g.members.iter())
        .filter(|m| **m != id)
        .map(|m| by_id(m));
    let measured = similar(id).into_iter().take(3).filter(|(_, s)| *s >= 0.72).map(|(st, _)| st);
    curated.chain(measured).filter(|s| seen.insert(s.id)).collect()
}

struct Slot {
    tier: u8,
    au: bool,
    modifier: Modifier,
}

/// A whole game's worth of questions, drawn fresh every time.
pub fn game(rounds: usize) -> Vec<Question> {
    let mut rng = rand::rng();
    let n = rounds.max(3);

    // difficulty ramp: ~30% easy, ~35% medium, the rest hard
    let easy = (n * 3).div_ceil(10);
    let medium = (n * 35) / 100;
    let mut slots: Vec<Slot> = (0..n)
        .map(|i| Slot {
            tier: if i < easy { 1 } else if i < easy + medium { 2 } else { 3 },
            au: false,
            modifier: Modifier::None,
        })
        .collect();

    // Aussie answers: a quarter of the game, give or take, never round 1,
    // and at least one in the hard stretch when there's room.
    let aus = (n / 4 + rng.random_range(0..=1)).clamp(2, 8);
    let mut idx: Vec<usize> = (1..n).collect();
    idx.shuffle(&mut rng);
    for &i in idx.iter().take(aus) {
        slots[i].au = true;
    }
    if !slots.iter().any(|s| s.tier == 3 && s.au) {
        if let Some(i) = (easy + medium..n).collect::<Vec<_>>().choose(&mut rng) {
            slots[*i].au = true;
        }
    }

    // modifiers: none in the first two rounds or back to back, the last round is the final
    let mut last_mod = true;
    for (i, s) in slots.iter_mut().enumerate() {
        if i == n - 1 {
            s.modifier = Modifier::Final;
            continue;
        }
        if i < 2 || last_mod {
            last_mod = false;
            continue;
        }
        // about two in five rounds get a twist; the visual ones only once it's not too early
        let mut pool: Vec<(Modifier, u32)> = vec![(Modifier::None, 50), (Modifier::Double, 7)];
        if i >= 3 {
            pool.extend([(Modifier::DownUnder, 6), (Modifier::Tilt, 6), (Modifier::Spin, 5), (Modifier::Fog, 6), (Modifier::Outline, 6), (Modifier::Wobble, 5)]);
        }
        if s.tier <= 2 {
            pool.push((Modifier::AussieInvasion, 6));
        }
        if s.au {
            pool.push((Modifier::AllAussie, 8));
        }
        let total: u32 = pool.iter().map(|p| p.1).sum();
        let mut roll = rng.random_range(0..total);
        s.modifier = pool.iter().find(|(_, w)| { let hit = roll < *w; roll = roll.saturating_sub(*w); hit }).unwrap().0;
        last_mod = s.modifier != Modifier::None;
    }

    let mut used = HashSet::new();
    slots.iter().map(|s| question(s, &mut used, &mut rng)).collect()
}

fn question(slot: &Slot, used: &mut HashSet<&'static str>, rng: &mut impl rand::Rng) -> Question {
    let unused = |f: &dyn Fn(&State) -> bool| -> Vec<&'static State> {
        STATES.iter().filter(|s| !used.contains(s.id) && f(s)).collect()
    };
    // answer: right country and tier if possible, then loosen
    let pool = [
        unused(&|s| s.au() == slot.au && s.tier == slot.tier),
        unused(&|s| s.au() == slot.au && s.tier.abs_diff(slot.tier) == 1),
        unused(&|s| s.au() == slot.au),
        unused(&|_| true),
    ];
    let answer = *pool.iter().find(|p| !p.is_empty()).unwrap().choose(rng).unwrap();
    used.insert(answer.id);

    let looks = lookalikes(answer.id);
    let sim = similar(answer.id); // everyone else, most alike first
    let mut options: Vec<&'static State> = vec![answer];
    // add random picks from `from` until there are `upto` options
    let take = |options: &mut Vec<&'static State>, from: &[&'static State], upto: usize, rng: &mut dyn rand::Rng| {
        let mut from: Vec<_> = from.iter().filter(|s| !options.iter().any(|o| o.id == s.id)).copied().collect();
        from.shuffle(rng);
        let k = upto.saturating_sub(options.len());
        options.extend(from.into_iter().take(k));
    };
    let ranked = |f: &dyn Fn(&State) -> bool, from: usize, to: usize| -> Vec<&'static State> {
        sim.iter().map(|(s, _)| *s).filter(|s| f(s)).skip(from).take(to - from).collect()
    };
    let not_look = |s: &State| !looks.iter().any(|l| l.id == s.id);
    // the closest-looking Aussies, for when an Aussie has to be there anyway
    let close_aussies = ranked(&|s| s.au(), 0, 4);

    match slot.modifier {
        Modifier::AussieInvasion if !answer.au() => take(&mut options, &close_aussies, 4, rng),
        Modifier::AussieInvasion => {
            // an Aussie answer with two Aussie decoys and the most confusable American
            take(&mut options, &close_aussies, 3, rng);
            take(&mut options, &ranked(&|s| !s.au(), 0, 3), 4, rng);
        }
        Modifier::AllAussie if answer.au() => take(&mut options, &close_aussies, 4, rng),
        _ => match answer.tier.max(slot.tier) {
            // easy: decoys that look nothing like it
            1 => {
                let n = sim.len();
                take(&mut options, &ranked(&not_look, n / 2, n), 4, rng);
            }
            // medium: one or two real traps, the rest at least vaguely similar
            2 => {
                let k: usize = rng.random_range(1..=2);
                take(&mut options, &looks, 1 + k, rng);
                take(&mut options, &ranked(&not_look, 2, 18), 4, rng);
            }
            // hard: lead with one hand-picked group so the trap is coherent,
            // then the measured closest shapes
            _ => {
                let gs: Vec<_> = GROUPS.iter().filter(|g| g.members.contains(&answer.id)).collect();
                if let Some(g) = gs.choose(rng) {
                    let members: Vec<_> = g.members.iter().map(|m| by_id(m)).collect();
                    take(&mut options, &members, 3, rng);
                }
                take(&mut options, &looks, 4, rng);
                take(&mut options, &ranked(&|_| true, 0, 8), 4, rng);
            }
        },
    }

    // The Aussie rule: there is always an Aussie option, and an Aussie answer
    // always has Aussie company (at least two more). The Aussie brought in is
    // usually one of the closest-looking, so it's a trap, not a freebie.
    let want_au = if answer.au() { 3 } else { 1 };
    while options.iter().filter(|s| s.au()).count() < want_au {
        // replace the least convincing non-Aussie decoy
        let score = |id: &str| sim.iter().find(|(s, _)| s.id == id).map_or(0.0, |x| x.1);
        let pos = (0..options.len())
            .filter(|&i| !options[i].au() && options[i].id != answer.id)
            .min_by(|&a, &b| score(options[a].id).total_cmp(&score(options[b].id)))
            .unwrap();
        let mut free: Vec<_> = ranked(&|s| s.au() && !options.iter().any(|o| o.id == s.id), 0, 8);
        if answer.tier.max(slot.tier) == 1 {
            free.reverse(); // easy rounds want the least alike
        }
        let top = &free[..free.len().min(2)];
        options[pos] = if rng.random_bool(0.75) { top.choose(rng) } else { free.choose(rng) }.copied().unwrap();
    }
    options.shuffle(rng);

    let title = title(slot, answer, &options, rng);
    let (rotate, multiplier) = match slot.modifier {
        Modifier::DownUnder => (180, 1),
        Modifier::Tilt => (rng.random_range(25..=65) * if rng.random_bool(0.5) { 1 } else { -1 }, 1),
        Modifier::Double | Modifier::Final => (0, 2),
        _ => (0, 1),
    };
    Question { answer, options, title, modifier: slot.modifier, rotate, tier: slot.tier, multiplier }
}

fn title(slot: &Slot, answer: &State, options: &[&State], rng: &mut impl rand::Rng) -> String {
    let pick = |xs: &[&str], rng: &mut dyn rand::Rng| xs.choose(rng).unwrap().to_string();
    let pool = match slot.modifier {
        Modifier::DownUnder => c::DOWN_UNDER_TITLES,
        Modifier::Tilt => c::TILT_TITLES,
        Modifier::Spin => c::SPIN_TITLES,
        Modifier::Fog => c::FOG_TITLES,
        Modifier::Outline => c::OUTLINE_TITLES,
        Modifier::Wobble => c::WOBBLE_TITLES,
        Modifier::Double => c::DOUBLE_TITLES,
        Modifier::Final => c::FINAL_TITLES,
        Modifier::AussieInvasion => c::INVASION_TITLES,
        Modifier::AllAussie => c::ALL_AUSSIE_TITLES,
        Modifier::None => &[],
    };
    if !pool.is_empty() {
        return pick(pool, rng);
    }
    // a hard round whose options mostly share a group gets one of that group's titles
    if slot.tier == 3 {
        let best = GROUPS
            .iter()
            .filter(|g| g.members.contains(&answer.id))
            .max_by_key(|g| options.iter().filter(|o| g.members.contains(&o.id)).count());
        if let Some(g) = best {
            if options.iter().filter(|o| g.members.contains(&o.id)).count() >= 3 && rng.random_bool(0.7) {
                return pick(g.titles, rng);
            }
        }
    }
    pick(match slot.tier { 1 => c::EASY_TITLES, 2 => c::MEDIUM_TITLES, _ => c::HARD_TITLES }, rng)
}

/// Lines for the reveal: who fell for what, who was fast, streaks, lead changes.
pub struct RoundFacts<'a> {
    pub state: &'a State,
    pub picks: Vec<(String, Option<&'a State>, f64)>, // name, pick, seconds taken
    pub everyone_right: bool,
    pub nobody_right: bool,
    pub streaks: Vec<(String, u32)>,
    pub broken: Vec<(String, u32)>,
    pub new_leader: Option<String>,
}

pub fn quips(f: &RoundFacts) -> Vec<String> {
    let mut rng = rand::rng();
    let mut used: HashSet<&str> = HashSet::new();
    let mut lines: Vec<(u8, String)> = vec![]; // (priority, line); lower shows first
    // a random line from a pool, never one already used in this reveal, filled in
    let mut say = |prio: u8, pool: &'static [&'static str], name: &str, pick: &str, num: &str| {
        let fresh: Vec<&&str> = pool.iter().filter(|x| !used.contains(**x)).collect();
        let line: &str = fresh.choose(&mut rng).map_or(pool[0], |l| **l);
        used.insert(line);
        let text = line.replace("{state}", f.state.name).replace("{name}", name).replace("{pick}", pick)
            .replace("{s}", num).replace("{n}", num);
        // "A and B all said..." reads badly; two people are "both"
        let text = if name.contains(" and ") && !name.contains(", ") { text.replacen(" all ", " both ", 1) } else { text };
        lines.push((prio, text));
    };

    let right: Vec<&(String, Option<&State>, f64)> =
        f.picks.iter().filter(|(_, p, _)| p.is_some_and(|p| p.id == f.state.id)).collect();
    if f.nobody_right && !f.picks.is_empty() {
        say(0, c::NOBODY, "", "", "");
    }
    if f.everyone_right && f.picks.len() > 1 {
        say(0, c::EVERYONE, "", "", "");
    }
    if right.len() == 1 && f.picks.len() > 2 {
        say(0, c::SOLO_RIGHT, &right[0].0, "", "");
    }

    // one line per wrong answer, naming everyone who gave it
    let mut wrong: Vec<(Option<&State>, Vec<&str>)> = vec![];
    for (name, pick, _) in &f.picks {
        if pick.is_some_and(|p| p.id == f.state.id) {
            continue;
        }
        match wrong.iter_mut().find(|(p, _)| p.map(|p| p.id) == pick.map(|p| p.id)) {
            Some((_, names)) => names.push(name),
            None => wrong.push((*pick, vec![name])),
        }
    }
    let looks = lookalikes(f.state.id);
    for (pick, names) in wrong {
        let many = names.len() > 1;
        let (prio, pool) = match pick {
            None => (3, if many { c::NO_ANSWER_MANY } else { c::NO_ANSWER }),
            Some(p) if p.au() && !f.state.au() => (1, if many { c::AU_FOR_US_MANY } else { c::AU_FOR_US }),
            Some(p) if !p.au() && f.state.au() => (1, if many { c::US_FOR_AU_MANY } else { c::US_FOR_AU }),
            Some(p) if looks.iter().any(|l| l.id == p.id) => (2, if many { c::TRAP_MANY } else { c::TRAP }),
            Some(_) => (2, if many { c::WILD_MANY } else { c::WILD }),
        };
        say(prio, pool, &join_names(&names), pick.map_or("", |p| p.name), "");
    }

    if right.len() > 1 {
        let fast = right.iter().min_by(|a, b| a.2.total_cmp(&b.2)).unwrap();
        say(2, c::FASTEST, &fast.0, "", &format!("{:.1}", fast.2));
        let slow = right.iter().max_by(|a, b| a.2.total_cmp(&b.2)).unwrap();
        let left = crate::lobby::QUESTION_SECS as f64 - slow.2;
        if left < 3.0 {
            say(2, c::BUZZER, &slow.0, "", &format!("{left:.1}"));
        }
    }
    for (name, n) in &f.streaks {
        say(1, c::STREAK, name, "", &n.to_string());
    }
    for (name, n) in &f.broken {
        say(1, c::BROKEN, name, "", &n.to_string());
    }
    if let Some(name) = &f.new_leader {
        say(0, c::LEADER, name, "", "");
    }
    lines.shuffle(&mut rand::rng());
    lines.sort_by_key(|l| l.0);
    lines.into_iter().take(4).map(|l| l.1).collect()
}

fn join_names(names: &[&str]) -> String {
    match names {
        [] => String::new(),
        [one] => one.to_string(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

pub fn winner_line(name: &str) -> String {
    c::WINNER.choose(&mut rand::rng()).unwrap().replace("{name}", name)
}

pub fn loser_line(name: &str) -> String {
    c::LOSER.choose(&mut rand::rng()).unwrap().replace("{name}", name)
}

pub fn roast(state: &State) -> &'static str {
    state.roasts.choose(&mut rand::rng()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_group_member_exists() {
        for g in GROUPS {
            for m in g.members {
                by_id(m);
            }
        }
        assert_eq!(STATES.len(), 58);
    }

    #[test]
    fn games_follow_the_rules() {
        for rounds in [5, 10, 15, 20, 30] {
            for _ in 0..300 {
                let qs = game(rounds);
                assert_eq!(qs.len(), rounds);
                let mut seen = HashSet::new();
                for q in &qs {
                    assert!(seen.insert(q.answer.id), "repeat answer");
                    assert_eq!(q.options.len(), 4);
                    let ids: HashSet<_> = q.options.iter().map(|o| o.id).collect();
                    assert_eq!(ids.len(), 4, "duplicate option");
                    assert!(ids.contains(q.answer.id));
                    let au = q.options.iter().filter(|o| o.au()).count();
                    assert!(au >= 1, "no aussie option");
                    if q.answer.au() {
                        assert!(au >= 3, "aussie answer without aussie company");
                    }
                }
                assert!(qs.iter().filter(|q| q.answer.au()).count() >= 2);
                assert_eq!(qs.last().unwrap().modifier, Modifier::Final);
            }
        }
    }

    /// Titles show before anyone answers, so no place names, nicknames,
    /// regions or country hints (Aussie words only on the Aussie modifiers,
    /// whose options already say it).
    #[test]
    fn titles_never_give_it_away() {
        let mut banned: Vec<String> = STATES
            .iter()
            .flat_map(|s| s.name.split(' '))
            .map(|w| w.to_lowercase())
            .collect();
        banned.extend(
            [
                "america", "usa", "yank", "states", "united", "empire", "bayou", "peach", "lone star", "sunshine", "golden",
                "aloha", "garden", "keystone", "granite", "green mountain", "pine", "prairie", "corn", "desert", "lake",
                "gulf", "pacific", "atlantic", "coast", "seaboard", "midwest", "south", "north", "east", "west", "appalachian",
                "outback", "bight", "cowboy", "canyon", "river", "rocky", "mitten", "boot", "heel", "dixie", "y'all",
                "corridor", "belt", "plains", "four corners", "great", "big sky", "cape", "sound", "bay", "aussie",
                "australia", "down under", "kangaroo", "roo", "🦘", "mate", "strewth", "g'day", "dinkum", "crikey",
                "koala", "emu", "wombat", "vegemite", "outback", "tropic", "island", "peninsula", "volcano", "wine",
            ]
            .map(String::from),
        );
        let aussie_ok = ["aussie", "australia", "kangaroo", "roo", "🦘", "mate", "strewth", "g'day", "dinkum", "crikey", "yank"];
        let bad = std::cell::RefCell::new(vec![]);
        let check = |title: &str, allow_aussie: bool| {
            let t = format!(" {} ", title.to_lowercase().replace(|c: char| !c.is_alphanumeric() && c != '\'' && c != '🦘', " "));
            for b in &banned {
                if allow_aussie && aussie_ok.contains(&b.as_str()) {
                    continue;
                }
                let hit = if b.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '\'') { t.contains(&format!(" {b} ")) } else { t.contains(b.as_str()) };
                if hit {
                    bad.borrow_mut().push(format!("{title:?} gives away {b:?}"));
                }
            }
        };
        for g in GROUPS {
            g.titles.iter().for_each(|t| check(t, false));
        }
        for pool in [c::EASY_TITLES, c::MEDIUM_TITLES, c::HARD_TITLES, c::DOWN_UNDER_TITLES, c::TILT_TITLES, c::SPIN_TITLES,
            c::FOG_TITLES, c::OUTLINE_TITLES, c::WOBBLE_TITLES, c::DOUBLE_TITLES, c::FINAL_TITLES] {
            pool.iter().for_each(|t| check(t, false));
        }
        for pool in [c::INVASION_TITLES, c::ALL_AUSSIE_TITLES] {
            pool.iter().for_each(|t| check(t, true));
        }
        let bad = bad.into_inner();
        assert!(bad.is_empty(), "titles that give it away:\n{}", bad.join("\n"));
    }

    #[test]
    fn plenty_of_roasts() {
        for s in STATES {
            assert!(s.roasts.len() >= 5, "{} has {} roasts", s.id, s.roasts.len());
        }
    }

    #[test]
    fn shapes_cover_every_state() {
        let shapes: serde_json::Value = serde_json::from_str(include_str!("../web/shapes.json")).unwrap();
        for s in STATES {
            assert_eq!(shapes[s.id]["name"], s.name, "{}", s.id);
        }
    }
}

#[cfg(test)]
mod sample {
    use std::collections::{HashMap, HashSet};

    /// `cargo test variety -- --ignored --nocapture`: how much there is to see
    /// How hard the decoys are: their average measured similarity to the answer, by tier.
    #[test]
    #[ignore]
    fn decoy_quality() {
        let mut sum = [0.0f64; 4];
        let mut n = [0usize; 4];
        let mut au_sum = 0.0;
        let mut au_n = 0;
        for _ in 0..20_000 {
            for q in super::game(15) {
                let sim = super::similar(q.answer.id);
                for o in q.options.iter().filter(|o| o.id != q.answer.id) {
                    let s = sim.iter().find(|(x, _)| x.id == o.id).unwrap().1;
                    sum[q.tier as usize] += s;
                    n[q.tier as usize] += 1;
                    if o.au() && !q.answer.au() {
                        au_sum += s;
                        au_n += 1;
                    }
                }
            }
        }
        println!("decoy similarity: easy {:.3} medium {:.3} hard {:.3} | aussie decoys on US answers {:.3}",
            sum[1] / n[1] as f64, sum[2] / n[2] as f64, sum[3] / n[3] as f64, au_sum / au_n as f64);
    }

    #[test]
    #[ignore]
    fn variety() {
        use super::c;
        let roasts: usize = super::STATES.iter().map(|s| s.roasts.len()).sum();
        let group_titles: usize = super::GROUPS.iter().map(|g| g.titles.len()).sum();
        let mod_titles = [c::DOWN_UNDER_TITLES, c::TILT_TITLES, c::SPIN_TITLES, c::FOG_TITLES, c::OUTLINE_TITLES,
            c::WOBBLE_TITLES, c::DOUBLE_TITLES, c::FINAL_TITLES, c::INVASION_TITLES, c::ALL_AUSSIE_TITLES]
            .iter().map(|p| p.len()).sum::<usize>();
        let quips = [c::NOBODY, c::EVERYONE, c::SOLO_RIGHT, c::NO_ANSWER, c::NO_ANSWER_MANY, c::AU_FOR_US, c::AU_FOR_US_MANY,
            c::US_FOR_AU, c::US_FOR_AU_MANY, c::TRAP, c::TRAP_MANY, c::WILD, c::WILD_MANY, c::FASTEST, c::BUZZER,
            c::STREAK, c::BROKEN, c::LEADER, c::WINNER, c::LOSER].iter().map(|p| p.len()).sum::<usize>();
        println!("roasts {roasts}, group titles {group_titles}, tier titles {}, modifier titles {mod_titles}, quip templates {quips}",
            c::EASY_TITLES.len() + c::MEDIUM_TITLES.len() + c::HARD_TITLES.len());

        // distinct questions (answer + set of options + modifier) actually generated
        let games = 200_000;
        let mut questions = HashSet::new();
        let mut option_sets: HashMap<&str, HashSet<Vec<&str>>> = HashMap::new();
        let mut answer_seqs = HashSet::new();
        let mut mods: HashMap<String, usize> = HashMap::new();
        for _ in 0..games {
            let g = super::game(15);
            answer_seqs.insert(g.iter().map(|q| q.answer.id).collect::<Vec<_>>());
            for q in &g {
                let mut o: Vec<&str> = q.options.iter().map(|o| o.id).collect();
                o.sort();
                questions.insert((q.answer.id, o.clone(), format!("{:?}", q.modifier)));
                option_sets.entry(q.answer.id).or_default().insert(o);
                *mods.entry(format!("{:?}", q.modifier)).or_default() += 1;
            }
        }
        let per_state: Vec<usize> = option_sets.values().map(|v| v.len()).collect();
        println!("{games} games of 15: {} distinct answer orders, {} distinct questions seen",
            answer_seqs.len(), questions.len());
        println!("option sets per state: min {} median {} max {}",
            per_state.iter().min().unwrap(), { let mut v = per_state.clone(); v.sort(); v[v.len() / 2] }, per_state.iter().max().unwrap());
        let mut m: Vec<_> = mods.into_iter().collect();
        m.sort_by(|a, b| b.1.cmp(&a.1));
        println!("modifiers: {}", m.iter().map(|(k, v)| format!("{k} {:.1}%", 100.0 * *v as f64 / (games * 15) as f64)).collect::<Vec<_>>().join(", "));
    }

    /// `cargo test sample -- --ignored --nocapture` prints a random game
    #[test]
    #[ignore]
    fn sample() {
        for (i, q) in super::game(15).iter().enumerate() {
            let opts: Vec<_> = q.options.iter().map(|o| if o.id == q.answer.id { format!("[{}]", o.id) } else { o.id.to_string() }).collect();
            println!("{:>2} t{} {:<15?} {:<45} {}", i + 1, q.tier, q.modifier, q.title, opts.join(" "));
        }
    }
}
