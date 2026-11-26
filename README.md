# state-o-rama

**Guess the state from its shape. With friends. Against the clock. Mostly
American, suspiciously Australian.** 🦘

### ▶ Play: [states.brensch.com](https://states.brensch.com)

Hit *Start a lobby*, pick a name, send the link round. Everyone sees the same
shape at the same time and picks from four options. A round ends when everyone
has answered or after 30 seconds. Faster right answers score more.

## The game

- **58 shapes**: all 50 US states plus Australia's 6 states and 2 mainland
  territories.
- **It gets harder.** The first ~30% of a game is easy shapes with decoys
  that look nothing like them. The middle mixes in a look-alike or two. The
  last third is the hard shapes against their look-alikes: Colorado vs Wyoming
  vs the Dakotas, Vermont vs New Hampshire, Alabama vs Mississippi, Kentucky
  vs… Victoria.
- **Decoys are chosen on evidence.** 29 hand-picked look-alike groups, plus a
  measured similarity score for every pair of shapes (`tools/similarity.mjs`
  overlays them, also mirrored and upside down, and scores the overlap). Easy
  rounds draw decoys from the least similar half, medium from the close-ish
  ones, hard from the groups and the closest matches. In 20,000 sampled games
  the decoys average 0.36 similarity in easy rounds, 0.57 in medium and 0.64
  in hard.
- **There is always an Aussie option**, usually the Aussie state that looks
  most like the answer, so it's a trap rather than a freebie. When the answer
  *is* Australian, at least three of the four options are.
- **No two games are alike.** Every one of 200,000 sampled 15-round games had
  a different sequence of answers. About a third of rounds get a twist:
  🙃 upside down, 📐 tilted, 🌀 spinning, 🌫️ fog (blurred, clearing as the
  clock runs: wait or guess?), ✏️ outline only, 🍮 wobbling, ✨ double points,
  🦘 *Aussie Invasion* (three Aussie options and one American, either could be
  right) and the *All-Aussie* round. The last round is always double points.
- **Cheek, in bulk.** 290 roasts (5 per state), 75 round titles, 116 look-alike
  group titles, 55 twist titles and 114 reveal quips: who said Tasmania for
  Texas, who fell for which trap, who was the only one right, buzzer-beaters,
  streaks, broken streaks, lead changes.
- **Titles never give it away.** They show before anyone answers, so a test
  fails the build if any title names a state, a nickname, a region or a
  country.
- **Lockout-proof.** Lobbies live at their URL (`/l/CODE`). Your browser keeps
  a token, so a locked phone or dropped wifi reconnects you as the same player
  with the same score; just reopen the link.

## How it's built

| | |
|---|---|
| Server | Rust: [axum](https://github.com/tokio-rs/axum) 0.8 + WebSockets on tokio. One actor task per lobby owns all its state; players get JSON snapshots over a `watch` channel. Lobbies live in memory and go away after 30 idle minutes. |
| Frontend | Plain HTML/CSS/JS, embedded in the binary. Font: [Fredoka](https://fonts.google.com/specimen/Fredoka) (SIL OFL), self-hosted. |
| Shapes | Precomputed SVG paths (`shapes/*.svg`, `web/shapes.json`). |
| Hosting | The brensch.com box behind Caddy, run by systemd; one static (musl) binary. |

### Where the shapes come from

`tools/gen-shapes.mjs` builds every outline with [d3-geo](https://d3js.org/d3-geo):

- **US states**: the Census Bureau's 2017 cartographic boundaries (shoreline
  clipped, so Michigan has no lake in it), via [us-atlas](https://github.com/topojson/us-atlas).
- **Australian states**: [Natural Earth](https://www.naturalearthdata.com/)
  1:10m admin-1 boundaries (public domain).

Each state is drawn in its own azimuthal equal-area projection centred on
itself, so shapes aren't stretched the way a whole-country map stretches them,
then simplified (Ramer–Douglas–Peucker, 0.35 px on a 500 px canvas). Specks
are dropped but Hawaii keeps its main islands and Alaska its Aleutians. The
same script makes the country maps shown on the reveal (Albers USA,
conic equal-area for Australia).

`shapes/` has every state as a standalone SVG if you want them for something
else.

## Running it

```sh
cargo run                 # http://127.0.0.1:8050 (BIND to change)
cargo test                # includes 1,500 random games checked against the rules
cargo test sample -- --ignored --nocapture   # print a random game plan
cargo test --release variety -- --ignored --nocapture        # count the variety
cargo test --release decoy_quality -- --ignored --nocapture  # how hard the decoys are
node tools/bots.mjs http://127.0.0.1:8050 BOTS 5   # bots play a whole game
make shapes               # regenerate the shapes (downloads Natural Earth once)
```

Deploying: `make setup-vm` once, then `make deploy`.

All the writing (roasts, titles, look-alike groups, quips) is in
[`src/content.rs`](src/content.rs); the question generator is
[`src/quiz.rs`](src/quiz.rs).
