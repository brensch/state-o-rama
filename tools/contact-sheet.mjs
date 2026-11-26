// Writes an HTML page showing every shape (for eyeballing the generator's output).
// node contact-sheet.mjs <out.html>
import fs from "node:fs";
const s = JSON.parse(fs.readFileSync(new URL("../web/shapes.json", import.meta.url)));
let h = '<html><body style="display:flex;flex-wrap:wrap;background:#fff">';
for (const [k, v] of Object.entries(s))
  h += `<div style="width:160px;font:12px sans-serif;text-align:center"><svg viewBox="0 0 500 500" width=150 height=150 style="border:1px solid #ccc"><path d="${v.d}" fill-rule=evenodd /></svg><br>${k}</div>`;
fs.writeFileSync(process.argv[2], h);
