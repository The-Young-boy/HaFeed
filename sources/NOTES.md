# Sources - RE notes (2026-11-11)

Machine-readable data: `sources.json`. Raw samples: `samples/<id>.json`.
All requests were plain GETs, the same ones a visitor's browser makes. No logins. No challenge bypass.

## 1. hagizra.news breaking strip ("מבזקים")

Found in `main-*.js` (component `app-breaking-ticker`, behind feature flag `breaking_news` from `/api/features`).

- **Separate endpoint:** `GET /api/breaking`
  ```json
  {"color":"","items":[{"id":1224,"messageId":2313633,"title":"סיכום חדשות הלילה","text":"...",
    "imageUrl":"","redirectId":"","createdAt":1794388976,"createdBy":"","messageTimestamp":1794374006}],
   "type":"breaking_update"}
  ```
  `createdAt` / `messageTimestamp` are unix seconds. `color` is an optional accent color.
- **Flag on the message:** `breakingId` (int). `0` = not breaking. `>0` = the id of the breaking item.
  Example: message 2313633 has `breakingId: 1224`, and breaking item 1224 has `messageId: 2313633`.
- **Live updates:** SSE `GET /api/events`. Event `breaking_update` sends the same payload as `/api/breaking`.
  The client then sets `breakingId` on loaded messages from `items[].messageId -> items[].id` and clears it on the rest.
  Other events: `reactions` (`{type, items:[{id, reactions}]}`), `heartbeat`.
- **Click (`navigateToMessage`):**
  - if `redirectId` is set: `window.open('/api/r/<redirectId>', '_blank')` (external redirect link).
  - else if `messageId > 0`: `chatService.jumpToMessage(messageId)` and `router.navigate(['/'])`.
    The chat view scrolls to that message. If it is not loaded, it loads it with
    `GET /api/v3/messages?around=<id>&limit=N` (centred window).
- Display: 1-5 items visible by width (`>=1280:5, >=900:4, >=600:3, >=400:2`), auto-scrolls when more. Time shown from `messageTimestamp` (HH:MM today, date otherwise).
- Admin-only routes seen (not used): `/api/admin/breaking/add`, `/api/admin/breaking/list`, `DELETE /api/admin/breaking/<id>`.

### hagizra v3 messages API (what the site itself uses now)
`GET /api/v3/messages?offset=0&limit=N&direction=desc` -> `{messages:[...], reactions:{"<id>":{emoji:n}}, my_reactions:{}}`.
`offset=<id>&direction=desc` = ids below `<id>` (exclusive). `direction=asc` = newer.
The old `GET /api/messages?limit=N&offset=<id>` (bare array, reactions inside each message) still works.

## 2. Shared "chat" platform: two builds

The same product (Angular SPA) runs in two builds. The API differs:

| | "thechats" build (hagizra, aliexpress.thechats, hakavhacharedi, zirat, h-news, ysiva) | "chatfree" build (*.chatfree.app, musiconline, myshemesh, yedid-nefesh) |
|---|---|---|
| list | `/api/messages?limit=N` | `/api/messages?limit=N` |
| container | bare array | `{messages:[], metadata:{}}` |
| older | `offset=<oldestId>` (exclusive) | `offset=<oldestId-1>&direction=desc` (offset is **inclusive**) |
| ad flag | `is_ads` (some also `is_advertisement`) | `isAds` |
| breaking | `breakingId` + `/api/breaking` (hagizra build only) | none |
| extra | `limit=N` may return N+1 items | `approvalStatus`, `topicIds`, `isThread`, `authorPictureUrl`; side banner ads at `/api/ads/active` |

Common: `id`, `text` (markdown), `timestamp` (ISO UTC, precision varies up to ns), `last_edit` (`0001-01-01T00:00:00Z` = never), `deleted`, `views`, `reactions`, `author`, `file{url,filename,filetype}` (empty in every sample).

Media embeds inside `text`:
- `[image-embedded#](URL#WxH)`; also `![image-embedded#](URL)` and `#800x533:` (trailing colon). The `#WxH` part is optional.
- `[video-embedded#](URL[#WxH])`, `[audio-embedded#](URL)`.
- `[quote-embedded#](<quotedId>@<quoted text>)` - the quoted text can contain `%29` for `)`.
- URLs can be relative (`/api/files/<sha1>`): prefix with the channel origin.

Channel info: `GET /api/channel/info` (thechats build; public on zirat and h-news even though messages need login) or `GET /api/channel/info-public` (chatfree build; public even on login-gated channels).

### "User not authenticated" channels - no guest flow exists
Checked in the bundles (`chunk-D6YG5SKO.js` on chatfree, chunks on hagizra build): the only auth routes are
`GET /auth/google` (returns Google OAuth client config), `POST /auth/login {code}`, `POST /auth/logout`.
The channel admin setting `require_auth` ("require login to view the channel") makes all `/api/*` return 401.
There is no anonymous/guest login and no cookie from the HTML page that unlocks it.
The `channel_guest` cookie that open chatfree channels set on `/api/messages` is only for counting guest views.

So: music-express, ysiva, zirat, avodot-eichut, yedid-nefesh, mizrach, israelshapira, h-news need a real Google sign-in.
The only way for the reader: a user signs in once in a browser and the proxy reuses that session cookie (cookie name not seen, since no login was done).
Alternative for two of them: authenti mirrors **זירת החדשות** and **המוקד** as open sources (see below).

## 3. Other platforms

- **hateder.net** (pargod / pargodmusic): own server-rendered site. `/pargod/api/messages` is 404.
  Real API: `GET /api/messages?source=<slug>&md=<MD_V>`. First page = 200 newest items **ascending** + state.
  Older: `&before=<id>` -> `{messages:[30], more}`. `md` is the page constant `var MD_V = "..."` (68a154a47d now); without it you get plain text only (no `rich`/`lead`).
  Ads are separate HTML slots (easy-click.online), not in the JSON.
- **authenti.newsupdates.click**: PHP aggregator of ~52 news sources. Native: `GET /api/get_messages_optimized.php?source=ALL_CHANNELS&last_id=0` (50 items ascending; `before_id=` older; `last_id=` newer; `source=<Hebrew name>` per source; source list at `/api/get_channels_optimized.php`). A chat-compatible `/api/messages` also exists (bare array, `Access-Control-Allow-Origin: *`) with a different id space and raw HTML (`<audio>`) inside `text`.
- **emess.co.il/online**: Nuxt. `GET https://www.emess.co.il/Online/Feed/0[?page=n]` (newest first, ~50/page). Picture ids -> `https://kcm.fm/upload/pictures/<floor(id/1000)>/<id>.jpg`. Video = HLS `media.kcm.fm/.../master.m3u8`. Page 0 can start with an older pinned item.
- **hm-news.co.il**: WordPress custom post type `news-flash`. `GET /wp-json/wp/v2/news-flash?per_page=20&acf_format=standard`. Text is in `title.rendered` (content is empty). `acf_format=standard` turns `acf.image`/`acf.video` ids into objects with `url`. Sponsored posts: `acf.change_author_name == "המחדש חדשות"`.

## 4. Blocked
- **news.moreshet-maran.com**: Cloudflare managed challenge (`403`, `cf-mitigated: challenge`) on every path including `/api/*`. Not bypassed.
- **black-cat.thechats.click**: the development machine's local resolver answers `0.0.0.0` (a filter on that network, not the site). Public DNS gives 144.91.100.123 (same server as ysiva). Not bypassed, so its real content is unknown.

## 5. Ad filtering summary
Flags: `is_ads`, `is_advertisement`, `isAds` (chat), `acf.change_author_name == "המחדש חדשות"` (hm-news).
Weak text patterns (use only as a hint, they gave false positives on news text): `<brand> מגישים:` at the start (musiconline), lines wrapped in `*...*` with 👈/✅ bullets and a CTA (hm-news), affiliate hosts `s.click.aliexpress.com`, `s.flylinking.com` (the deals channels are fully commercial anyway).
Note: on hagizra `is_ads` is also set on paid "news-like" posts (fundraising trips, rabbi events).
Not in the feeds: chatfree banner column (`/api/ads/active`), hateder easy-click slots, emess clicka slots, authenti overlay ads.

## 6. CORS
Only `authenti /api/messages` (`*`) and `hm-news /wp-json` (reflects Origin) send `Access-Control-Allow-Origin`. All others: none. Use the local proxy.

## 7. curl examples (one per variant)

```bash
# chat, thechats build (bare array), newest 20, then older than id 2313700
curl.exe -s "https://hagizra.news/api/messages?limit=20"
curl.exe -s "https://hagizra.news/api/messages?limit=20&offset=2313700"
# hagizra v3 + breaking strip + live events
curl.exe -s "https://hagizra.news/api/v3/messages?offset=0&limit=20&direction=desc"
curl.exe -s "https://hagizra.news/api/breaking"
curl.exe -s -N "https://hagizra.news/api/events"

# chat, chatfree build ({messages,metadata}); older: offset is inclusive
curl.exe -s "https://n.musiconline.co.il/api/messages?limit=20"
curl.exe -s "https://n.musiconline.co.il/api/messages?limit=20&offset=8889&direction=desc"
curl.exe -s "https://music-express.chatfree.app/api/channel/info-public"   # works even on login-gated channels

# hateder
curl.exe -s "https://hateder.net/api/messages?source=pargod&md=68a154a47d"
curl.exe -s "https://hateder.net/api/messages?source=pargod&md=68a154a47d&before=7300"

# authenti (native)
curl.exe -s "https://authenti.newsupdates.click/api/get_messages_optimized.php?source=ALL_CHANNELS&last_id=0"
curl.exe -s "https://authenti.newsupdates.click/api/get_messages_optimized.php?source=ALL_CHANNELS&before_id=1108300"
curl.exe -s "https://authenti.newsupdates.click/api/get_messages_optimized.php?source=%D7%94%D7%9E%D7%95%D7%A7%D7%93&last_id=0"  # source=המוקד

# emess
curl.exe -s "https://www.emess.co.il/Online/Feed/0"
curl.exe -s "https://www.emess.co.il/Online/Feed/0?page=1"

# hm-news
curl.exe -s "https://hm-news.co.il/wp-json/wp/v2/news-flash?per_page=20&acf_format=standard&_fields=id,date,date_gmt,link,title,acf"
curl.exe -s "https://hm-news.co.il/wp-json/wp/v2/news-flash?per_page=20&before=2026-11-11T10:00:00&acf_format=standard"
```

## 8. Haredi news sites (RSS and breaking)
Checked 11.11.26. NetFree status comes from the Raspberry Pi (on NetFree). The laptop is NOT on NetFree, so laptop results are marked "laptop".
Samples (first 2 items, trimmed) are in `samples/haredi-news/`.
(Asked for as section 7. Numbered 8 because section 7 already holds the curl examples.)

### NetFree status (Pi)
- Open (200): jdn.co.il, hm-news.co.il, emess.co.il, kol-barama.co.il (radio), kore.co.il (כל רגע), hidabroot.org, bizzness.net, chabad.info, col.org.il, theyeshivaworld.com, hamodia.com, mishpacha.com.
- Blocked (`418 Blocked by NetFree`, text/html, iframe to `netfree.link/block` with `"block":"deny"`): bhol.co.il (every path), kikar.co.il (also `/feed`), ch10.co.il, actualic.co.il, kooker.co.il, kolhazman.co.il, news08.net, kipa, ynet, kan, inn.
- bhol.co.il is also behind a Cloudflare managed challenge (laptop: `403`, `cf-mitigated`, "Just a moment"). Not bypassed.
- kikar.co.il works from the laptop (`/feed` is in the home page), but NetFree blocks it. Do not add.
- No DNS record: bkehila.co.il, haredim10.co.il/.com, hamevaser.co.il, bhaderech.co.il, yated-neeman.co.il. hamodia.co.il resolves to 255.255.255.255. yated.co.il and mishpacha.co.il are unrelated businesses.
- kolbarama.co.il is an events/lifestyle blog ("רמה", last post Aug 2026). The radio "קול ברמה" is **kol-barama.co.il**.

### NetFree image review (important)
A new image returns `200 image/*` from NetFree, but the body is a pixelated placeholder with the text "כעת התמונה נשלחה לנטפרי לבדיקה". Same size in pixels, much smaller in bytes.
Seen on hm-news (23 KB vs 491 KB) and kol-barama (44 KB vs 512 KB). The emess image too (39 KB vs 828 KB).
The placeholder adds `Cache-Control: no-cache, no-store, must-revalidate`. Do not cache images with that header. Fetch again later. Older images (JDN, kore, kcm.fm) came back real.

### Per site
- **JDN** (WordPress). `https://www.jdn.co.il/feed/` = 8 items, `media:content url=... medium="image"` on every item, `content:encoded` is the full text. Date RFC 822 `+0000`.
  Category feeds work: `/category/<slug>/feed/` (news id 146, flashes id 3). The "flashes" category is short community news (~1 an hour), not real breaking news. Category `mivzak` (1113) is dead since March 2026.
  API: `/wp-json/wp/v2/posts?per_page=20&_embed=wp:featuredmedia` -> `_embedded["wp:featuredmedia"][0].source_url`. `categories=3` / `categories_exclude=3` splits flashes from articles. `date` is Israel local time, `date_gmt` is UTC.
  `www.jdn.co.il/news-flash/` and `/breaking/` redirect to archive.jdn.co.il and give 502.
- **HaMechadesh** (WordPress). Breaking = CPT `news-flash` (see section 3): only a few a day (11.11: 01:18, 10:04, 11:31, 11:42, 11:46, 16:58).
  `https://hm-news.co.il/feed/` = 10 articles, full text, but no image tag (1 of 10 has `<img>` in content). `/news-flash/feed/` and `?post_type=news-flash` return the same article feed.
  For pictures use `/wp-json/wp/v2/posts?per_page=10&_embed=wp:featuredmedia`. Sizes include `hm-hero` (1335x750) and `hm-card-lg`.
- **Emess** (Nuxt front, WordPress back). Breaking = `/Online/Feed/0` (section 3). New items every 1-3 minutes.
  `https://www.emess.co.il/feed` = 50 items, starts with a newline before `<?xml` (strict parsers fail: trim first). Only 8/50 have an `<img>`. `/rss` and `/rss.xml` are the Nuxt HTML page.
  `/wp-json/wp/v2/posts?per_page=10` is a custom shape: `featured_image` (direct URL), `final_slug` (e.g. `/radio/1960666`), `cat_info`, `excerpt`, no `link`. Category `flash` (id 2) is stale since 2024. `/category/*/feed` is 404.
- **Kol Barama radio** (WordPress). Posts are dead. Real content = CPT `item` (interview segments), several an hour.
  `https://kol-barama.co.il/feed/?post_type=item` = 15 items, `<enclosure type="image/jpeg">`, date `+0000`. The XML repeats `xmlns:media` on `<rss>`, so strict parsers fail. Use a loose parser or regex.
  API: `/wp-json/wp/v2/item?per_page=10&_embed=wp:featuredmedia`. No breaking section. Cloudflare only loads its passive `jsd` script, no challenge.
- **Kore (כל רגע)** (own CMS). `https://www.kore.co.il/rss` = 84 items with a custom `<picture>` URL (media.kore.co.il), summary only, date `Wed, 11 Nov 2026 20:33:00 IST`. `/feed/` = same items, image in `content:encoded`, date with 2-digit year `Wed, 11 Nov 26 ... +0200`.
  Breaking: `/flashNews` (= `/newsflash`) HTML only, ~560 items `<div class="item" id="...">` + `<h3>` + `<div class="date">HH:MM</div>`, a few a minute. No JSON found (`/alerts_history.json` is home-front alerts, empty).
- **Hidabroot**: `https://www.hidabroot.org/rss` = 42 items, same format as kore (`<picture>`, IST). Kiruv/news site, Haredi-friendly.
- **bizzness.net** (Haredi business news): `/feed/` 10 items, full text, no images. **chabad.info** `/feed/` = 500. **col.org.il** `/feed/` returns the HTML page.

### Recommendation
- Breaking sources: emess `/Online/Feed/0` (already in), kore `/flashNews` (HTML scrape), hm-news `news-flash` (already in, low volume). JDN has no real breaking feed.
- Headline widget (article + picture): JDN `/feed/` (media:content), kore `/rss` (`<picture>`), hm-news and emess through wp-json (featured image), kol-barama `item` feed (enclosure). Handle the NetFree placeholder image.
