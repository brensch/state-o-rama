// State-o-rama client. The server owns the game; this just draws its latest
// snapshot and sends taps back. Reconnects on its own (phones lock, wifi drops).
"use strict";

const $app = document.getElementById("app");
const $conn = document.getElementById("conn");
const store = {
  get: (k) => { try { return localStorage.getItem(k); } catch { return null; } },
  set: (k, v) => { try { localStorage.setItem(k, v); } catch {} },
};
const esc = (s) => String(s ?? "").replace(/[&<>"']/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[c]);
const flag = (id) => (id.startsWith("AU-") ? "🇦🇺" : "🇺🇸");

let token = store.get("sor-token");
if (!token) {
  token = (crypto.randomUUID ? crypto.randomUUID() : Math.random().toString(36).slice(2) + Date.now().toString(36)).replace(/-/g, "");
  store.set("sor-token", token);
}

const G = { code: null, me: null, snap: null, offset: 0, ws: null, key: null, retry: 0, picks: {}, maps: null, fatal: null };
fetch("/maps.json").then((r) => r.json()).then((m) => { G.maps = m; if (G.snap?.phase === "reveal") draw(true); });

const path = location.pathname.match(/^\/l\/([A-Za-z0-9]{3,8})$/);
if (path) {
  G.code = path[1].toUpperCase();
  if (store.get("sor-name")) connect(); else nameScreen();
} else home();

// ---------- screens outside a game ----------

function home() {
  $app.innerHTML = `
    <div class="center">
      <h1 class="logo">State&#8209;o&#8209;rama</h1>
      <p class="tagline">Guess the state from its shape. With friends. Against the clock.
      Mostly American, suspiciously Australian. 🦘</p>
      <a class="btn primary" href="/new" style="width:100%">Start a lobby</a>
      <div class="card">
        <form id="join" class="row">
          <input id="code" placeholder="Lobby code" maxlength="8" autocapitalize="characters" autocomplete="off">
          <button style="flex:0 0 auto">Join</button>
        </form>
      </div>
      <p class="small-print">50 US states, 8 Aussie ones, 30 seconds each, zero mercy.</p>
    </div>`;
  document.getElementById("join").onsubmit = (e) => {
    e.preventDefault();
    const c = document.getElementById("code").value.trim().toUpperCase().replace(/[^A-Z0-9]/g, "");
    if (c.length >= 3) location.href = "/l/" + c;
  };
}

function nameScreen(renaming) {
  const quips = ["What do we call you?", "Name, please. Real or otherwise.", "Who's about to lose to Wyoming?", "Pick a name you won't regret."];
  $app.innerHTML = `
    <div class="center">
      <h1 class="logo">State&#8209;o&#8209;rama</h1>
      <div class="card center">
        <h2>${quips[Math.floor(Math.random() * quips.length)]}</h2>
        <p class="muted">Joining lobby <b>${esc(G.code)}</b></p>
        <form id="nm" style="width:100%" class="center">
          <input id="name" maxlength="20" placeholder="Your name" autocomplete="nickname" value="${esc(store.get("sor-name") || "")}">
          <button class="primary" style="width:100%">${renaming ? "Save" : "Let me in"}</button>
        </form>
      </div>
    </div>`;
  const $n = document.getElementById("name");
  $n.focus();
  document.getElementById("nm").onsubmit = (e) => {
    e.preventDefault();
    const n = $n.value.trim();
    if (!n) return $n.focus();
    store.set("sor-name", n);
    G.key = null;
    if (renaming && G.ws?.readyState === 1) { send({ t: "rename", name: n }); if (G.snap) draw(true); }
    else connect();
  };
}

// ---------- connection ----------

function connect() {
  if (G.ws && G.ws.readyState <= 1) return;
  const ws = new WebSocket(`${location.protocol === "https:" ? "wss" : "ws"}://${location.host}/ws/${G.code}`);
  G.ws = ws;
  ws.onopen = () => ws.send(JSON.stringify({ t: "hello", token, name: store.get("sor-name") || "" }));
  ws.onmessage = (e) => {
    const m = JSON.parse(e.data);
    if (m.t === "welcome") { G.me = m.you; G.retry = 0; $conn.hidden = true; }
    else if (m.t === "error") { G.fatal = m.msg; $app.innerHTML = `<div class="center"><h1 class="logo">Uh oh</h1><p class="tagline">${esc(m.msg)}</p><a class="btn primary" href="/new">Start a lobby</a></div>`; }
    else if (m.t === "state") { G.offset = m.now - Date.now(); G.snap = m; draw(); }
  };
  ws.onclose = () => {
    if (G.ws !== ws || G.fatal) return;
    G.ws = null;
    $conn.hidden = false;
    setTimeout(connect, Math.min(5000, 400 * 2 ** G.retry++));
  };
}
const send = (m) => G.ws?.readyState === 1 && G.ws.send(JSON.stringify(m));
document.addEventListener("visibilitychange", () => { if (!document.hidden && G.code && store.get("sor-name") && !G.fatal) connect(); });
const now = () => Date.now() + G.offset;

// ---------- drawing ----------

function draw(force) {
  const s = G.snap;
  const key = `${s.phase}:${s.games}:${s.round}`;
  if (key !== G.key || force) {
    G.key = key;
    ({ lobby: drawLobby, question: drawQuestion, reveal: drawReveal, over: drawOver })[s.phase](s);
  } else if (s.phase === "question") updateQuestion(s);
  else if (s.phase === "lobby") drawLobby(s);
  else if (s.phase === "over") drawOver(s);
}

const meP = (s) => s.players.find((p) => p.id === G.me);
const isHost = (s) => s.host === G.me;
const hostName = (s) => s.players.find((p) => p.id === s.host)?.name || "the host";
const chips = (s, cls = () => "") => s.players.map((p) =>
  `<span class="chip ${p.id === G.me ? "me" : ""} ${p.online ? "" : "off"} ${cls(p)}">${p.id === s.host ? '<span class="crown">👑</span>' : ""}${esc(p.name)}</span>`).join("");

function drawLobby(s) {
  const link = `${location.origin}/l/${s.code}`;
  const n = s.players.filter((p) => p.online).length;
  const rounds = G.rounds || s.rounds_setting || 15;
  $app.innerHTML = `
    <div class="center">
      <h1 class="logo">State&#8209;o&#8209;rama</h1>
      <div class="card center">
        <div class="muted">Lobby code</div>
        <div class="code">${esc(s.code)}</div>
        <div class="row">
          <button class="ghost small" id="share">📤 Share</button>
          <button class="ghost small" id="rename">Change name</button>
        </div>
      </div>
      <div class="card center">
        <h3>${n === 1 ? "Just you so far. Send the link!" : `${n} players ready to embarrass themselves`}</h3>
        <div class="players">${chips(s)}</div>
      </div>
      ${isHost(s) ? `
      <div class="card center">
        <div class="muted">How many rounds?</div>
        <div class="seg" style="width:100%">${[10, 15, 20, 30].map((r) => `<button class="small ${r === rounds ? "on" : ""}" data-r="${r}">${r}</button>`).join("")}</div>
        <button class="primary" id="start" style="width:100%">Start the game</button>
      </div>` : `<p class="muted">Waiting for <b>${esc(hostName(s))}</b> to start…</p>`}
      <p class="small-print">Pick the state from its shape. Faster answers score more. Easy ones first, then the look-alikes.
      There's always an Aussie option. Don't trust it. Or do. 🦘</p>
    </div>`;
  document.getElementById("share").onclick = () => share(s.code, link);
  document.getElementById("rename").onclick = () => nameScreen(true);
  document.querySelectorAll("[data-r]").forEach((b) => (b.onclick = () => { G.rounds = +b.dataset.r; drawLobby(s); }));
  const $start = document.getElementById("start");
  if ($start) $start.onclick = () => send({ t: "start", rounds });
}

// The phone's own share sheet where there is one; otherwise ours (QR, copy, apps).
// Ours lives outside #app so lobby updates don't close it.
async function share(code, link) {
  const text = "Guess the states. Lose to me. 🦘";
  if (navigator.share) {
    try { await navigator.share({ title: "State-o-rama", text, url: link }); return; }
    catch (e) { if (e.name === "AbortError") return; }
  }
  document.getElementById("sheet")?.remove();
  const enc = encodeURIComponent, msg = enc(`${text} ${link}`);
  const el = document.createElement("div");
  el.id = "sheet";
  el.innerHTML = `
    <div class="sheet card center toast">
      <h3>Get them in here</h3>
      <img class="qr" src="/qr/${enc(code)}" alt="QR code for the lobby link" width="220" height="220">
      <div class="small-print">Point a phone camera at it</div>
      <input id="link" readonly value="${esc(link)}">
      <button class="primary" id="copy" style="width:100%">Copy link</button>
      <div class="row">
        <a class="btn small" target="_blank" rel="noopener" href="https://wa.me/?text=${msg}">WhatsApp</a>
        <a class="btn small" href="sms:?&body=${msg}">SMS</a>
        <a class="btn small" href="mailto:?subject=${enc("State-o-rama?")}&body=${msg}">Email</a>
      </div>
      <button class="ghost small" id="close">Close</button>
    </div>`;
  document.body.append(el);
  const close = () => el.remove();
  el.onclick = (e) => { if (e.target === el) close(); };
  el.querySelector("#close").onclick = close;
  el.querySelector("#link").onfocus = (e) => e.target.select();
  el.querySelector("#copy").onclick = async (e) => {
    try { await navigator.clipboard.writeText(link); }
    catch { const i = el.querySelector("#link"); i.focus(); i.select(); document.execCommand("copy"); }
    e.target.textContent = "Copied! 📋";
    setTimeout(() => (e.target.textContent = "Copy link"), 1800);
  };
}

function shapeSvg(q, rotate) {
  return `<svg viewBox="0 0 500 500"><g class="spin" style="transform:rotate(${rotate}deg)"><path d="${q.shape}" fill-rule="evenodd"/></g></svg>`;
}

function header(s) {
  const q = s.q;
  return `
    <div class="top">
      <span class="round">Round ${s.round} / ${s.rounds}</span>
      <span class="round">${s.players.length > 1 ? `${meP(s)?.score ?? 0} pts` : ""}</span>
    </div>
    <div class="title">${esc(q.title)}${q.multiplier > 1 ? `<span class="mult">×${q.multiplier}</span>` : ""}</div>`;
}

const pickKey = (s) => `${s.code}:${s.games}:${s.round}`;

function drawQuestion(s) {
  const q = s.q;
  $app.innerHTML = `
    ${header(s)}
    <div class="timer" id="timer"><div></div></div>
    <div class="shape" id="shape">${shapeSvg(q, 0)}</div>
    <div class="options">${q.options.map((o) => `<button data-id="${o.id}"><span class="flag">${flag(o.id)}</span>${esc(o.name)}</button>`).join("")}</div>
    <div class="status" id="status"></div>
    <div class="players" id="chips" style="margin-top:10px"></div>`;
  // spin into place for the wonky rounds
  if (q.rotate) requestAnimationFrame(() => requestAnimationFrame(() => {
    const g = document.querySelector(".spin"); if (g) g.style.transform = `rotate(${q.rotate}deg)`;
  }));
  document.querySelectorAll(".options button").forEach((b) => (b.onclick = () => {
    if (G.picks[pickKey(s)] || meP(G.snap)?.answered) return;
    G.picks[pickKey(s)] = b.dataset.id;
    send({ t: "answer", round: s.round, choice: b.dataset.id });
    updateQuestion(G.snap);
  }));
  updateQuestion(s);
}

function updateQuestion(s) {
  const mine = G.picks[pickKey(s)];
  const locked = mine || meP(s)?.answered;
  document.querySelectorAll(".options button").forEach((b) => {
    b.classList.toggle("mine", b.dataset.id === mine);
    b.classList.toggle("dim", !!locked && b.dataset.id !== mine);
    b.disabled = !!locked;
  });
  const online = s.players.filter((p) => p.online);
  const done = online.filter((p) => p.answered).length;
  const waiting = online.filter((p) => !p.answered).map((p) => p.name);
  const $st = document.getElementById("status");
  if ($st) $st.textContent = !locked ? "" :
    waiting.length === 0 ? "Everyone's in!" :
    waiting.length <= 2 ? `Locked in. Waiting on ${waiting.join(" and ")}…` : `Locked in. ${done}/${online.length} answered.`;
  const $c = document.getElementById("chips");
  if ($c && s.players.length > 1) $c.innerHTML = chips(s, (p) => (p.answered ? "done" : ""));
}

function drawReveal(s) {
  const q = s.q, r = s.reveal, me = meP(s);
  const mine = me?.pick ?? G.picks[pickKey(s)];
  const right = mine === r.answer;
  const pickers = (id) => s.players.filter((p) => p.pick === id).map((p) => esc(p.name)).join(", ");
  const ranked = [...s.players].sort((a, b) => b.score - a.score);
  const map = G.maps?.[r.answer.slice(0, 2)];
  const heading = !mine ? ["Too slow!", "Asleep?", "No answer, no glory."][s.round % 3]
    : right ? ["Nailed it!", "Correct!", "Big brain!", "Yes!!"][s.round % 4]
    : ["Nope!", "Oof.", "Not quite.", "Wrong, but confidently."][s.round % 4];
  $app.innerHTML = `
    ${header(s)}
    <div class="title" style="font-size:24px;color:${right ? "var(--green)" : "var(--red)"}">${heading}${right && me?.gained ? ` +${me.gained}` : ""}</div>
    <div class="reveal-top">
      <div class="shape right mini" id="shape">${shapeSvg(q, q.rotate)}</div>
      ${map ? `<svg class="map" viewBox="0 0 ${map.w} ${map.h}">${
        Object.entries(map.states).map(([id, d]) => `<path d="${d}" class="${id === r.answer ? "hit" : ""}"/>`).join("")}</svg>` : ""}
    </div>
    <div class="reveal-name">${flag(r.answer)} ${esc(r.name)}</div>
    <div class="roast">${esc(r.roast)}</div>
    <div class="options compact">${q.options.map((o) => {
      const cls = o.id === r.answer ? "correct" : o.id === mine ? "wrong" : "dim";
      const who = s.players.length > 1 ? pickers(o.id) : "";
      return `<button disabled class="${cls}">${flag(o.id)} ${esc(o.name)}${who ? `<span class="who">${who}</span>` : ""}</button>`;
    }).join("")}</div>
    ${r.quips.length ? `<ul class="quips">${r.quips.map((x) => `<li class="toast">${esc(x)}</li>`).join("")}</ul>` : ""}
    ${s.players.length > 1 ? `<h3 style="margin:14px 0 8px">Leaderboard</h3><ol class="board">${ranked.map((p, i) => `
      <li class="${p.id === G.me ? "me" : ""}"><span class="pos">${i + 1}</span><span class="nm">${esc(p.name)}${p.streak >= 3 ? " 🔥" : ""}</span>
      <span class="gain ${p.gained ? "" : "zero"}">${p.gained ? "+" + p.gained : "+0"}</span><span class="pts">${p.score}</span></li>`).join("")}</ol>` : ""}
    <div class="next" id="next"></div>`;
  // turn it back the right way up so everyone sees the real shape
  if (q.rotate) setTimeout(() => { const g = document.querySelector(".spin"); if (g) g.style.transform = "rotate(0deg)"; }, 700);
}

function drawOver(s) {
  const ranked = [...s.players].sort((a, b) => b.score - a.score);
  const medals = ["🥇", "🥈", "🥉"];
  $app.innerHTML = `
    <div class="center">
      <div class="podium wobble">🏆</div>
      <div class="winner">${esc(s.over.winner)}</div>
      ${s.over.loser ? `<p class="muted">${esc(s.over.loser)}</p>` : ""}
      <ol class="board" style="width:100%">${ranked.map((p, i) => `
        <li class="${p.id === G.me ? "me" : ""}"><span class="pos">${medals[i] || i + 1}</span>
        <span class="nm">${esc(p.name)}<br><span class="stats">${p.correct}/${s.rounds} right · best streak ${p.best_streak}</span></span>
        <span class="pts">${p.score}</span></li>`).join("")}</ol>
      ${isHost(s) ? `<div class="row"><button class="primary" id="again">Play again</button><button class="ghost" id="lobby">Back to lobby</button></div>`
        : `<p class="muted">Waiting for <b>${esc(hostName(s))}</b> to go again…</p>`}
    </div>`;
  const $a = document.getElementById("again");
  if ($a) $a.onclick = () => send({ t: "start", rounds: s.rounds_setting });
  const $l = document.getElementById("lobby");
  if ($l) $l.onclick = () => send({ t: "again" });
}

// ---------- clocks ----------

(function tick() {
  const s = G.snap;
  if (s?.phase === "question") {
    const left = Math.max(0, s.q.deadline - now());
    const $t = document.getElementById("timer");
    if ($t) {
      $t.firstElementChild.style.transform = `scaleX(${left / 30000})`;
      $t.classList.toggle("hurry", left < 8000);
    }
  } else if (s?.phase === "reveal") {
    const $n = document.getElementById("next");
    if ($n) $n.textContent = `${s.reveal.last ? "Results" : "Next round"} in ${Math.max(0, Math.ceil((s.reveal.until - now()) / 1000))}s`;
  }
  requestAnimationFrame(tick);
})();
