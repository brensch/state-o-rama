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
- **It gets harder.** The first ~30% of a game is easy shapes with obviously
  different decoys (Florida, Texas, Michigan, Tasmania…). The middle mixes in a
  look-alike or two. The last third is the hard shapes against their
  look-alikes: Colorado vs Wyoming vs the Dakotas, Vermont vs New Hampshire,
  Alabama vs Mississippi, Kentucky vs… Victoria.
- **There is always an Aussie option.** When the answer *is* Australian, at
  least three of the four options are.
- **No two games are alike.** Each game draws a fresh plan of difficulties,
  Aussie rounds and modifiers, and decoys come from 29 overlapping look-alike
  groups, so the same state gets different traps each time. Modifiers include
  🙃 *Down Under* (shown upside down), 📐 *Tilted*, ✨ *Double points*,
  🦘 *Aussie Invasion* (three Aussie decoys) and the *All-Aussie* round; the
  last round is always double points.
- **Cheek.** Every state has roasts, every round has a title, and the reveal
  calls out who said Tasmania for Texas, who fell for which trap, streaks,
  broken streaks, lead changes and the fastest finger.
- **Lockout-proof.** Lobbies live at their URL (`/l/CODE`). Your browser keeps
  a token, so a locked phone or dropped wifi reconnects you as the same player
  with the same score; just reopen the link.

## How it's built

| | |
|---|---|
| Server | Rust: [axum](https://github.com/tokio-rs/axum) 0.8 + WebSockets on tokio. One actor task per lobby owns all its state; players get JSON snapshots over a `watch` channel. Lobbies live in memory and go away after 30 idle minutes. |
| Frontend | Plain HTML/CSS/JS, embedded in the binary. |
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
node tools/bots.mjs http://127.0.0.1:8050 BOTS 5   # bots play a whole game
make shapes               # regenerate the shapes (downloads Natural Earth once)
```

Deploying: `make setup-vm` once, then `make deploy`.

The content (roasts, titles, look-alike groups, quips) is all in
[`src/quiz.rs`](src/quiz.rs).
