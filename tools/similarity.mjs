// How alike every pair of shapes looks, so decoys can be chosen on evidence.
// Each shape (already fitted into the 500px box by gen-shapes.mjs) is
// rasterised to a grid; two shapes score their intersection-over-union, taking
// the best of as-shown, mirrored, flipped and upside down (players confuse
// mirror twins like Vermont and New Hampshire too). Writes ../web/similar.json:
//   { id: [[otherId, score], ...] }  every other state, most similar first
//
// Run after gen-shapes.mjs: node similarity.mjs
import fs from "node:fs";

const N = 72; // grid size
const shapes = JSON.parse(fs.readFileSync(new URL("../web/shapes.json", import.meta.url)));
const ids = Object.keys(shapes);

function rings(d) {
  return d.split("Z").filter((r) => r.trim()).map((r) =>
    r.replace("M", "").split("L").map((p) => p.trim().split(/\s+/).map(Number)));
}

// even-odd fill, sampled at cell centres; the shape is scaled so its bounding
// box fills the grid, keeping aspect ratio, so position in the box doesn't matter
function mask(d) {
  const rs = rings(d);
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const r of rs) for (const [x, y] of r) { x0 = Math.min(x0, x); y0 = Math.min(y0, y); x1 = Math.max(x1, x); y1 = Math.max(y1, y); }
  const scale = Math.max(x1 - x0, y1 - y0) / N;
  const ox = x0 - (scale * N - (x1 - x0)) / 2, oy = y0 - (scale * N - (y1 - y0)) / 2;
  const m = new Uint8Array(N * N);
  for (let j = 0; j < N; j++) {
    const py = oy + (j + 0.5) * scale;
    const xs = [];
    for (const r of rs)
      for (let i = 0, k = r.length - 1; i < r.length; k = i++) {
        const [ax, ay] = r[i], [bx, by] = r[k];
        if ((ay > py) !== (by > py)) xs.push(ax + ((py - ay) / (by - ay)) * (bx - ax));
      }
    xs.sort((a, b) => a - b);
    for (let t = 0; t + 1 < xs.length; t += 2) {
      const a = Math.ceil((xs[t] - ox) / scale - 0.5), b = Math.floor((xs[t + 1] - ox) / scale - 0.5);
      for (let i = Math.max(0, a); i <= Math.min(N - 1, b); i++) m[j * N + i] = 1;
    }
  }
  return m;
}

const transforms = [
  (i, j) => [i, j], // as shown
  (i, j) => [N - 1 - i, j], // mirrored
  (i, j) => [i, N - 1 - j], // flipped
  (i, j) => [N - 1 - i, N - 1 - j], // upside down
];
function iou(a, b, t) {
  let inter = 0, union = 0;
  for (let j = 0; j < N; j++)
    for (let i = 0; i < N; i++) {
      const [ti, tj] = t(i, j);
      const x = a[j * N + i], y = b[tj * N + ti];
      inter += x & y; union += x | y;
    }
  return inter / union;
}

const masks = Object.fromEntries(ids.map((id) => [id, mask(shapes[id].d)]));
const out = {};
for (const a of ids) {
  out[a] = ids.filter((b) => b !== a)
    .map((b) => [b, +Math.max(...transforms.map((t) => iou(masks[a], masks[b], t))).toFixed(3)])
    .sort((x, y) => y[1] - x[1]);
}
fs.writeFileSync(new URL("../web/similar.json", import.meta.url), JSON.stringify(out));
for (const a of ["US-CO", "US-VT", "US-AL", "US-KY", "US-NY", "US-KS", "AU-NT", "AU-VIC", "US-FL"])
  console.log(a.padEnd(7), out[a].slice(0, 5).map(([b, s]) => `${b} ${s}`).join("  "));
