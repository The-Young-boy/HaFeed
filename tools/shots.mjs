// Dev aid: screenshots of the feed page for the post and the README, at 2x, light and dark.
// Needs the dev server (node tools/dev-server.mjs). Drives a hidden Edge over the DevTools protocol.
// node tools/shots.mjs <settings.json> <out dir>
import { spawn } from "node:child_process";
import { mkdtempSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const [seedFile, outDir] = process.argv.slice(2);
const seed = JSON.parse(readFileSync(seedFile, "utf8"));
mkdirSync(outDir, { recursive: true });
const PORT = 9338, URL = "http://localhost:5178/winfeed/";
const edge = spawn("C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe", [
  "--headless=new", `--remote-debugging-port=${PORT}`, `--user-data-dir=${mkdtempSync(join(tmpdir(), "hz-shots-"))}`,
  "--no-first-run", "--hide-scrollbars", "--lang=he", "about:blank",
]);
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
let targets;
for (let i = 0; i < 40 && !targets; i++) { await sleep(250); targets = await fetch(`http://127.0.0.1:${PORT}/json`).then((r) => r.json()).catch(() => null); }
const ws = new WebSocket(targets.find((t) => t.type === "page").webSocketDebuggerUrl);
await new Promise((r) => (ws.onopen = r));
let id = 0; const waiting = new Map();
ws.onmessage = (m) => { const d = JSON.parse(m.data); if (waiting.has(d.id)) { waiting.get(d.id)(d); waiting.delete(d.id); } };
const cdp = (method, params = {}) => new Promise((r) => { waiting.set(++id, r); ws.send(JSON.stringify({ id, method, params })); });
const js = async (expr) => (await cdp("Runtime.evaluate", { expression: expr, awaitPromise: true, returnByValue: true })).result?.result?.value;

async function shot(name, { dark = false, w = 720, h = 1100, prep = "" } = {}) {
  if (process.env.ONLY && !process.env.ONLY.split(",").includes(name)) return;
  await cdp("Emulation.setDeviceMetricsOverride", { width: w, height: h, deviceScaleFactor: 2, mobile: false });
  await cdp("Emulation.setEmulatedMedia", { features: [{ name: "prefers-color-scheme", value: dark ? "dark" : "light" }] });
  await cdp("Page.navigate", { url: URL });
  await sleep(7000);
  if (prep) { await js(`(async () => { ${prep} })()`); await sleep(1500); }
  const png = await cdp("Page.captureScreenshot", { format: "png" });
  writeFileSync(join(outDir, name + ".png"), Buffer.from(png.result.data, "base64"));
  console.log(name);
}

await cdp("Page.enable");
await cdp("Page.navigate", { url: URL });
await sleep(1500);
await js(`(() => { localStorage.clear(); ${Object.entries(seed).map(([k, v]) => `localStorage.setItem(${JSON.stringify(k)}, ${JSON.stringify(v)});`).join("")} })()`);

const tab = (t) => `document.querySelector('[aria-label="הגדרות"]').click(); await new Promise(r => setTimeout(r, 400)); document.querySelector('#setTabs [data-t="${t}"]').click();`;
await shot("feed");
await shot("feed-dark", { dark: true });
await shot("feed-news", { prep: `document.querySelector('#tabs [data-f="news"]')?.click(); await new Promise(r => setTimeout(r, 4000));` });
for (const t of ["display", "notify", "feeds", "sources", "accounts", "about"]) await shot("set-" + t, { prep: tab(t) });
await shot("set-notify-dark", { dark: true, prep: tab("notify") });
await shot("set-sources-dark", { dark: true, prep: tab("sources") });
const arts = `S.on = [...new Set([...S.on, "art-jdn", "art-kore", "art-hmnews", "art-emess"])]; saveSettings(); buildTabs();`;
await shot("feed-articles", { prep: arts + ` document.querySelector('#tabs [data-f="articles"]').click(); await new Promise(r => setTimeout(r, 5000));` });
await shot("feed-compact", { prep: `S.compact = true; applyLook(); await new Promise(r => setTimeout(r, 1500));` });
await shot("feed-menu", { prep: `const p = [...document.querySelectorAll('#feed .post')][1]; p.scrollIntoView({ block: "center" }); p.querySelector('.share[aria-label="עוד פעולות"]').click();` });
await shot("set-advanced-dark", { dark: true, prep: tab("about") + ` await new Promise(r => setTimeout(r, 300)); document.querySelector('#advOpen').click();` });
await shot("set-all", { prep: tab("feeds") + ` await new Promise(r => setTimeout(r, 300)); document.querySelector('.frow[data-id="all"] .chip').click();` });
await shot("set-advanced", { prep: tab("about") + ` await new Promise(r => setTimeout(r, 300)); document.querySelector('#advOpen').click();` });
await shot("wizard", { prep: `openWizard(0);` });
await shot("wizard-sources", { prep: `openWizard(1);` });
await shot("wizard-look", { dark: true, prep: `openWizard(3);` });
ws.close(); edge.kill();
