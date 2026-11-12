// Dev only: serves src/feed.html at /winfeed/, proxies /api/* to hagizra.news, and mirrors the provider's
// /winfeed/proxy (same query format), the same origin layout the widgets board gives the page. node tools/dev-server.mjs
import { createServer } from "node:http";
import { readFile, writeFile } from "node:fs/promises";
import { execFile } from "node:child_process";
import { tmpdir } from "node:os";
import { join } from "node:path";

const PORT = 5178;
const page = new URL("../src/feed.html", import.meta.url);
const jar = join(tmpdir(), "hagizra-dev-cookies.txt"); // like the provider's single WinHTTP session

// ponytail: curl instead of fetch, node's fetch times out through the local proxy here.
const curl = (args) =>
  new Promise((ok, fail) =>
    execFile("curl.exe", ["-s", "-m", "20", "-b", jar, "-c", jar, "--compressed", "-w", "\n%{http_code} %{content_type}", ...args],
      { encoding: "buffer", maxBuffer: 32 << 20 }, (e, out) => {
        if (e) return fail(e);
        const cut = out.lastIndexOf(10);
        const [code, type] = out.subarray(cut + 1).toString().split(" ");
        ok({ body: out.subarray(0, cut), status: +code || 502, type: type || "application/octet-stream" });
      }),
  );

createServer(async (req, res) => {
  try {
    const u = new URL(req.url, "http://x");
    if (u.pathname.startsWith("/api/")) {
      const r = await curl(["https://hagizra.news" + req.url]);
      res.writeHead(r.status, { "content-type": r.type }).end(r.body);
    } else if (u.pathname === "/winfeed/proxy") {
      const q = u.searchParams, target = q.get("u") || "";
      if (!target.startsWith("https://")) return res.writeHead(400).end("https only");
      const args = ["-X", q.get("m") || "GET", ...q.getAll("h").flatMap((h) => ["-H", h])];
      if (q.get("t")) args.push("-H", "Content-Type: " + q.get("t"));
      if (q.has("b")) args.push("--data-raw", q.get("b"));
      const r = await curl([...args, target]);
      res.writeHead(r.status, { "content-type": r.type }).end(r.body);
    } else if (u.pathname === "/winfeed/copyfile") {
      res.writeHead(501, { "content-type": "text/plain; charset=utf-8" }).end("רק בלוח"); // the provider does this in the board
    } else if (u.pathname === "/winfeed/notify" || u.pathname === "/winfeed/log") {
      console.log(u.pathname, Object.fromEntries(u.searchParams)); // notifications and page logs: the provider's job in the board
      res.writeHead(200, { "content-type": "text/plain" }).end("");
    } else if (u.pathname === "/winfeed/accent") {
      res.writeHead(200, { "content-type": "text/plain" }).end("#0078D4"); // the Windows accent: the provider reads it in the board
    } else if (u.pathname === "/winfeed/update") {
      res.writeHead(200, { "content-type": "application/json" }).end(`{"current":"","latest":"","state":"idle","note":"","auto":true,"dev":true}`); // updates are the provider's
    } else if (u.pathname === "/winfeed/auth") {
      res.writeHead(200, { "content-type": "application/json" }).end(`{"signed":[],"pending":[],"failed":{}}`); // sign-in only works in the board
    } else if (u.pathname === "/widgets/data" && req.method === "POST") {
      // The lists a dev page computed (wLoad + wItem), for hagizra-widget.exe --cards <out> <this file>.
      const chunks = []; for await (const ch of req) chunks.push(ch);
      await writeFile(join(tmpdir(), "hagizra-widgetdata.json"), Buffer.concat(chunks));
      res.writeHead(200).end("ok");
    } else if (u.pathname === "/widgets/cards.json") {
      // The widget cards in every size: hagizra-widget.exe --cards %TEMP%\hagizra-cards.json, shown by tools/widget-preview.
      res.writeHead(200, { "content-type": "application/json", "cache-control": "no-store" }).end(await readFile(join(tmpdir(), "hagizra-cards.json")));
    } else if (u.pathname === "/widgets" || /^\/widgets\/[\w-]+\.json$/.test(u.pathname)) {
      const file = u.pathname === "/widgets" ? "index.html" : u.pathname.slice(9);
      res.writeHead(200, { "content-type": file.endsWith(".json") ? "application/json" : "text/html; charset=utf-8", "cache-control": "no-store" })
        .end(await readFile(new URL("./widget-preview/" + file, import.meta.url)));
    } else if (u.pathname === "/winfeed/version") {
      res.writeHead(200, { "content-type": "text/plain" }).end("dev");
    } else if (u.pathname === "/" || u.pathname.startsWith("/winfeed")) {
      res.writeHead(200, { "content-type": "text/html; charset=utf-8", "cache-control": "no-store" }).end(await readFile(page));
    } else res.writeHead(404).end();
  } catch (e) {
    res.writeHead(502).end(String(e));
  }
}).listen(PORT, () => console.log(`http://localhost:${PORT}/winfeed/`));
