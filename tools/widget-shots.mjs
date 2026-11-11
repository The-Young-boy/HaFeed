// Dev aid: each widget card of tools/widget-preview as its own PNG, at 2x, light and dark.
// Needs the dev server and %TEMP%\hagizra-cards.json (hagizra-widget.exe --cards ...). node tools/widget-shots.mjs <out dir> <kinds>
import { spawn } from "node:child_process";
import { mkdtempSync, writeFileSync, mkdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const [outDir, kinds = "popular,cats,deal,search"] = process.argv.slice(2);
mkdirSync(outDir, { recursive: true });
const PORT = 9339;
const edge = spawn("C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe", [
  "--headless=new", `--remote-debugging-port=${PORT}`, `--user-data-dir=${mkdtempSync(join(tmpdir(), "hz-wshots-"))}`, "--no-first-run", "--hide-scrollbars", "about:blank",
]);
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
let targets;
for (let i = 0; i < 40 && !targets; i++) { await sleep(250); targets = await fetch(`http://127.0.0.1:${PORT}/json`).then((r) => r.json()).catch(() => null); }
const ws = new WebSocket(targets.find((t) => t.type === "page").webSocketDebuggerUrl);
await new Promise((r) => (ws.onopen = r));
let id = 0; const waiting = new Map();
ws.onmessage = (m) => { const d = JSON.parse(m.data); if (waiting.has(d.id)) { waiting.get(d.id)(d); waiting.delete(d.id); } };
const cdp = (method, params = {}) => new Promise((r) => { waiting.set(++id, r); ws.send(JSON.stringify({ id, method, params })); });
await cdp("Page.enable");
await cdp("Emulation.setDeviceMetricsOverride", { width: 1000, height: 3000, deviceScaleFactor: 2, mobile: false });
for (const theme of ["light", "dark"]) {
  // A change in the hash alone would not load the page again.
  await cdp("Page.navigate", { url: "about:blank" }); await sleep(300);
  await cdp("Page.navigate", { url: `http://localhost:5178/widgets?${kinds},raw${theme === "dark" ? "#dark" : ""}` });
  await sleep(5000);
  const boxes = JSON.parse((await cdp("Runtime.evaluate", { expression: `JSON.stringify([...document.querySelectorAll(".row")].flatMap((row) => [...row.querySelectorAll(".w")].map((w) => { const r = w.getBoundingClientRect(); return { kind: row.id.slice(2), size: w.classList[1], x: r.x, y: r.y, w: r.width, h: r.height }; })))`, returnByValue: true })).result.result.value);
  for (const b of boxes) {
    const png = await cdp("Page.captureScreenshot", { format: "png", clip: { x: b.x, y: b.y, width: b.w, height: b.h, scale: 1 }, captureBeyondViewport: true });
    writeFileSync(join(outDir, `wcard-${b.kind}-${b.size}-${theme}.png`), Buffer.from(png.result.data, "base64"));
  }
  console.log(theme, boxes.length);
}
ws.close(); edge.kill();
