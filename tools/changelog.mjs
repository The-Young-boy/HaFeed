// CHANGELOG.md from the GitHub releases, the way About shows them ("מה חדש"): a heading per version with its
// date, newest first, without the lines every release repeats. Markdown, ready to paste in the forum.
// Runs with each release: node tools/changelog.mjs <out file>
import { writeFileSync } from "node:fs";

const out = process.argv[2] || "CHANGELOG.md";
const same = /^(\*\*(מה השתנה|מה חדש|התקנה|דרישות):?\*\*|מי שמותקנת)/;   // the same filter as the page
const list = [];
for (let page = 1; ; page++) {
  const r = await fetch(`https://api.github.com/repos/The-Young-boy/HaFeed/releases?per_page=100&page=${page}`, { headers: { "User-Agent": "HaFeed" } });
  if (!r.ok) throw new Error(`GitHub ${r.status}`);
  const rel = await r.json();
  list.push(...rel);
  if (rel.length < 100) break;
}
const md = ["# מה חדש בהפיד", ""];
for (const x of list) {
  const date = new Date(x.published_at).toLocaleDateString("he-IL", { timeZone: "Asia/Jerusalem" });
  md.push(`### גרסה ${x.tag_name.replace(/^v/, "")} · ${date}`, "");
  const lines = (x.body || "").split(/\r?\n/).filter((l) => !same.test(l.trim()));
  // ponytail: blank-line runs folded to one; the body is the release notes as written.
  md.push(lines.join("\n").replace(/\n{3,}/g, "\n\n").trim(), "");
}
writeFileSync(out, md.join("\n").replace(/\n{3,}/g, "\n\n"));
console.log(`${list.length} releases -> ${out}`);
