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
