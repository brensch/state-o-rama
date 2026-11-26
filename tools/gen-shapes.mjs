// Builds the state outlines. US states: the Census Bureau's 2017 cartographic
// boundaries (clipped at the shoreline, so Michigan has no lake in it) via the
// us-atlas npm package. Australian states: Natural Earth's 1:10m admin-1
// boundaries (public domain, https://www.naturalearthdata.com/). Each state gets its own
// azimuthal equal-area projection centred on itself, so nothing is stretched
// the way a whole-country Mercator would stretch it. Output:
//   ../shapes/<id>.svg      one standalone SVG per state (committed, reusable)
//   ../web/shapes.json      { id: { name, country, d } } embedded in the server
//   ../web/maps.json        each country with every state on it, for the reveal
//
// Run: npm i && node gen-shapes.mjs   (expects data/ne_admin1.geojson; see README)
import fs from "node:fs";
import { feature } from "topojson-client";
import { geoAlbersUsa, geoArea, geoAzimuthalEqualArea, geoCentroid, geoConicEqualArea } from "d3-geo";

const SIZE = 500, PAD = 18, EPS = 0.35; // viewBox, padding, simplification tolerance (px)

const AU = { NS: "New South Wales", VI: "Victoria", QL: "Queensland", SA: "South Australia",
  WA: "Western Australia", TS: "Tasmania", NT: "Northern Territory", CT: "Australian Capital Territory" };
const AU_ID = { NS: "NSW", VI: "VIC", QL: "QLD", SA: "SA", WA: "WA", TS: "TAS", NT: "NT", CT: "ACT" };
// Islands smaller than this fraction of a state's biggest piece are dropped
// (keeps Hawaii's main islands and Alaska's Aleutians, loses specks).
const KEEP = { AK: 0.0008, HI: 0.004 };

const ne = JSON.parse(fs.readFileSync(new URL("data/ne_admin1.geojson", import.meta.url)));
const usa = JSON.parse(fs.readFileSync(new URL("node_modules/us-atlas/states-10m.json", import.meta.url)));
// us-atlas only carries names, so borrow the postal codes from Natural Earth
const postal = Object.fromEntries(ne.features.filter((f) => f.properties.iso_a2 === "US" && f.properties.type_en === "State")
  .map((f) => [f.properties.name, f.properties.postal]));
const features = [
  ...feature(usa, usa.objects.states).features.filter((f) => postal[f.properties.name])
    .map((f) => ({ ...f, properties: { iso_a2: "US", postal: postal[f.properties.name], name: f.properties.name } })),
  ...ne.features.filter((f) => f.properties.iso_a2 === "AU"),
];
const out = {}, geoms = {};
fs.mkdirSync(new URL("../shapes/", import.meta.url), { recursive: true });

for (const f of features) {
  const p = f.properties;
  let id, name, country;
  if (p.iso_a2 === "US") { id = "US-" + p.postal; name = p.name; country = "US"; }
  else if (p.iso_a2 === "AU" && AU[p.postal] && p.name === AU[p.postal]) { id = "AU-" + AU_ID[p.postal]; name = p.name; country = "AU"; }
  else continue;

  const polys = f.geometry.type === "Polygon" ? [f.geometry.coordinates] : f.geometry.coordinates;
  const areas = polys.map((c) => geoArea({ type: "Polygon", coordinates: c }));
  const max = Math.max(...areas), keep = KEEP[p.postal] ?? 0.002;
  const geom = { type: "MultiPolygon", coordinates: polys.filter((_, i) => areas[i] >= max * keep) };

  geoms[id] = geom;
  const [lon, lat] = geoCentroid(geom);
  const proj = geoAzimuthalEqualArea().rotate([-lon, -lat]).precision(0.1)
    .fitExtent([[PAD, PAD], [SIZE - PAD, SIZE - PAD]], geom);

  const d = pathOf(geom, proj, EPS);
  out[id] = { name, country, d };
  fs.writeFileSync(new URL(`../shapes/${id}.svg`, import.meta.url),
    `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${SIZE} ${SIZE}"><title>${name}</title>` +
    `<path d="${d}" fill="#222" fill-rule="evenodd"/></svg>\n`);
}

const ids = Object.keys(out).sort();
fs.writeFileSync(new URL("../web/shapes.json", import.meta.url),
  JSON.stringify(Object.fromEntries(ids.map((k) => [k, out[k]]))));
console.log(`${ids.length} shapes`, ids.filter((i) => i.startsWith("AU")).join(" "));
for (const i of ids) if (out[i].d.length > 40000) console.log("big:", i, out[i].d.length);

// Country maps: Albers USA (Alaska and Hawaii inset) and a conic for Australia.
const maps = {};
for (const [c, proj, w, h] of [["US", geoAlbersUsa(), 600, 380], ["AU", geoConicEqualArea().parallels([-18, -36]).rotate([-134, 0]), 600, 520]]) {
  const mine = ids.filter((i) => out[i].country === c);
  proj.fitExtent([[6, 6], [w - 6, h - 6]], { type: "GeometryCollection", geometries: mine.map((i) => geoms[i]) });
  maps[c] = { w, h, states: Object.fromEntries(mine.map((i) => [i, pathOf(geoms[i], proj, 0.6, 0)])) };
}
fs.writeFileSync(new URL("../web/maps.json", import.meta.url), JSON.stringify(maps));

function pathOf(geom, proj, eps, dp = 1) {
  let d = "";
  for (const poly of geom.coordinates)
    for (const ring of poly) {
      const pts = rdp(ring.map((c) => proj(c)).filter(Boolean), eps);
      if (pts.length < 4) continue;
      d += "M" + pts.slice(0, -1).map(([x, y]) => `${x.toFixed(dp)} ${y.toFixed(dp)}`).join("L") + "Z";
    }
  return d;
}

// Ramer–Douglas–Peucker on a closed ring (first point == last point).
function rdp(pts, eps) {
  if (pts.length < 5) return pts;
  const keep = new Uint8Array(pts.length); keep[0] = keep[pts.length - 1] = 1;
  // split the closed ring at its farthest point from the start so both halves are open paths
  let far = 0, fd = -1;
  for (let i = 1; i < pts.length - 1; i++) { const dd = (pts[i][0] - pts[0][0]) ** 2 + (pts[i][1] - pts[0][1]) ** 2; if (dd > fd) { fd = dd; far = i; } }
  keep[far] = 1;
  const stack = [[0, far], [far, pts.length - 1]];
  while (stack.length) {
    const [a, b] = stack.pop(); let md = 0, mi = -1;
    const [ax, ay] = pts[a], [bx, by] = pts[b], dx = bx - ax, dy = by - ay, L = Math.hypot(dx, dy) || 1;
    for (let i = a + 1; i < b; i++) { const dist = Math.abs(dy * pts[i][0] - dx * pts[i][1] + bx * ay - by * ax) / L; if (dist > md) { md = dist; mi = i; } }
    if (md > eps) { keep[mi] = 1; stack.push([a, mi], [mi, b]); }
  }
  return pts.filter((_, i) => keep[i]);
}
