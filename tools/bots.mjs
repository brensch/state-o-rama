// Plays a full game against a running server with fake players, to smoke-test it.
// node bots.mjs [base-url] [code] [rounds]   (one bot drops out and comes back mid-game)
const base = process.argv[2] || "http://127.0.0.1:8050";
const code = process.argv[3] || "BOTTY";
const rounds = +(process.argv[4] || 5);
const url = base.replace(/^http/, "ws") + "/ws/" + code;

function bot(name, token, { host = false, wrong = false, slow = 0 } = {}) {
  return new Promise((done) => {
    const ws = new WebSocket(url);
    let me, answered = new Set(), started = false;
    ws.onopen = () => ws.send(JSON.stringify({ t: "hello", token, name }));
    ws.onmessage = (e) => {
      const m = JSON.parse(e.data);
      if (m.t === "welcome") me = m.you;
      if (m.t !== "state") return;
      if (host && m.phase === "lobby" && m.players.filter((p) => p.online).length >= 3 && !started) {
        started = true; ws.send(JSON.stringify({ t: "start", rounds }));
      }
      if (m.phase === "question" && !answered.has(m.round)) {
        answered.add(m.round);
        const opts = m.q.options;
        const pick = opts[Math.floor(Math.random() * opts.length)].id;
        setTimeout(() => ws.send(JSON.stringify({ t: "answer", round: m.round, choice: pick })), slow);
      }
      if (m.phase === "reveal" && host && !answered.has("r" + m.round)) {
        answered.add("r" + m.round);
        console.log(`R${m.round} ${m.q.title.padEnd(40)} ans=${m.reveal.name.padEnd(28)} picks=${m.players.map((p) => p.name + ":" + (p.pick || "-")).join(" ")}`);
        for (const q of m.reveal.quips) console.log("     >", q);
      }
      if (m.phase === "over") {
        if (host) console.log("OVER:", m.over.winner, "|", m.over.loser, m.players.map((p) => `${p.name}=${p.score}`).join(" "));
        ws.close(); done(m);
      }
    };
  });
}
const t = Date.now().toString(36);
const a = bot("Hosty", "host" + t, { host: true });
const b = bot("Bluey", "blue" + t, { slow: 300 });
// Flaky joins, drops after 2.5 s (phone locked), comes back 6 s later with the same token
const flakyToken = "flak" + t;
const ws = new WebSocket(url);
ws.onopen = () => ws.send(JSON.stringify({ t: "hello", token: flakyToken, name: "Flaky" }));
setTimeout(() => ws.close(), 2500);
await new Promise((r) => setTimeout(r, 8500));
const c = bot("Flaky", flakyToken, { slow: 1000 });
await Promise.all([a, b, c]);
process.exit(0);
