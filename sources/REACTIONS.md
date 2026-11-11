# Reactions - RE notes (2026-11-11)

Scope: how a visitor adds a reaction or a like to a post on each source.
Method: read the JS bundles and the inline scripts, and send GET requests only.
No reaction was sent. No POST, PUT or DELETE was sent to any site. No login.
Machine: all findings come from the **laptop** (curl.exe). Nothing was blocked, so the Pi was not needed.

Summary:

| Site | Guest allowed? | Endpoint | Allowed reactions | Shows "mine"? |
|---|---|---|---|---|
| Chat, thechats build (hagizra, hakavhacharedi, aliexpress.thechats) | No (Google login) | `POST /api/reactions/set-reactions` | per channel, `GET /api/emojis/list` | Yes, `my_reactions` in `/api/v3/messages` (logged in) |
| Chat, chatfree build (myshemesh, musiconline, yeshiva-zucher) | No (Google login) | `POST /api/reactions/set-reactions` | per channel, `/api/emojis/list` + `/api/emojis/ads-list` for ad posts | No field seen |
| hateder.net | Yes (`vid` cookie), with a wait for new visitors | `POST /api/react` | 7 fixed keys, minus per-item `react_off` | In the POST response only. Feed `mine` is always null (cached) |
| emess.co.il/online | Probably yes (no login in the flow). Identity not confirmed | `POST /Online/Like` | rank 1-4 (👍 👎 😄 😢), 0 = remove | Feed field `likes` (map id->rank), but the CDN copy is `{}` |
| authenti.newsupdates.click | No (Google login) | `POST /api/add_reaction.php` | like, dislike, laugh, vomit | Yes, `user_reaction` per item (logged in) |
| hm-news.co.il news-flash | n/a | none | none (share only) | n/a |

---

## 1. Chat platform - "thechats" build

Sites: hagizra.news, channel.hakavhacharedi.com, aliexpress.thechats.click.
Source: `main-66WPV2OD.js` (message component) and `chunk-NTNJ6P4M.js` (chat service) on hagizra.

- **Endpoint:** `POST /api/reactions/set-reactions`
- **Body:** `{"messageId": <int>, "emoji": "<emoji>"}` (JSON). Angular HttpClient with `withCredentials`.
- **Auth:** login is required. The client returns early when `_authService.userInfo` is empty.
  The emoji picker also opens only for a logged-in user. Login = Google only (`/auth/google`, `POST /auth/login {code}`).
  The session is a cookie. Its name is unknown (no login was done).
- **Response:** `{reactions: {emoji: count}}` = the new counts for that message. `reactions: null` = no change.
- **Toggle semantics:** one reaction per user per message.
  - Send the same emoji that is already "mine" -> the server removes it (the client sends the same body; no separate remove call).
  - Send a different emoji -> it replaces the old one (the client decrements the old count, increments the new one).
- **Client throttle:** 4 seconds between two new reactions ("אנא המתן N שניות"). Removing is not throttled.
  The client also ignores SSE `reactions` updates for a message for 3.5 s after a click (`REACTION_GUARD_MS`).
- **Allowed emojis:** `GET /api/emojis/list` (public, per channel). Values on 2026-11-11:
  - hagizra.news: 👍 👎 💥 🔥 😍 ❤️ 💯 👌 💪 🥳 🤣 😶 😔 😒 😳 😓 😭 💔 😰 😡 🤮 😲 🥲 🫂 👀 📖
  - channel.hakavhacharedi.com: 👍 👎
  - aliexpress.thechats.click: 👍 🫡 🤑 😍 🤩 🫰 🙏 🫶
  - `/api/emojis/ads-list` does not exist in this build (returns the SPA HTML).
  - Ad posts (`is_ads`): `canAddEmoji` is false for normal users, so no new emoji on ads.
- **Mine:** `GET /api/v3/messages?...` returns `my_reactions: {"<id>": "<emoji>"}`. For a guest it is `{}`.
- **Live counts:** SSE `/api/events`, event `reactions` = `{type, items:[{id, reactions}]}`.
- Admin-only (not for the reader): `/api/admin/set-message-reactions`.

```bash
# DO NOT RUN - changes data, and needs a logged-in session cookie
curl.exe -s -X POST "https://hagizra.news/api/reactions/set-reactions" \
  -H "Content-Type: application/json" -H "Cookie: <session cookie after Google login>" \
  -d "{\"messageId\":2313743,\"emoji\":\"👍\"}"
# safe (read-only): allowed set
curl.exe -s "https://hagizra.news/api/emojis/list"
```

## 2. Chat platform - "chatfree" build

Sites: myshemesh.co.il, n.musiconline.co.il, yeshiva-zucher.chatfree.app.
Source: `chunk-GPURFDCR.js` (chat service) and `main-U4TIDC5O.js` on myshemesh.

- **Endpoint and body:** the same as the thechats build: `POST /api/reactions/set-reactions {messageId, emoji}`.
- **Auth:** login is required. The client does not check before the call. The server error maps to the toast
  "יש להתחבר לחשבון בכדי להוסיף אימוג'ים". 403 `{error:"reactions_blocked"}` = the admin blocked this user from reacting.
  The `channel_guest` cookie is only for guest view counts. It does not give reaction rights.
- **Response:** the client does not use it. New counts arrive through SSE.
- **Allowed emojis:** `GET /api/emojis/list` for normal posts, `GET /api/emojis/ads-list` for posts with `isAds`. Values on 2026-11-11:
  - myshemesh.co.il: 👍 👎 😀 ♥️ (ads-list empty)
  - n.musiconline.co.il: 42 emojis (👍 😀 😍 🎊 🎉 🎇 ✨ 🧨 😱 😭 👑 ⭐ 👏 ❤️ 💥 🎈 🎹 🎤 🎸 🎷 🎛️ 👎 🔴 🟢 🟡 ⚫ 😂 ✅ 🎥 🎵 😘 😪 🫣 🎯 🔇 📖 🥘 🍿 🍫 🤗 🥕 🍓); ads-list 18
  - yeshiva-zucher.chatfree.app: 👍 😂 🙏 💪 😯 ❤️ 👌 🔥 💥 😭 😥 💔 🥺 📖 👑 🎯 🚀 🤨 (ads-list empty)
- **Mine:** no "mine" field in `/api/messages` (keys: id, type, text, author, authorId, timestamp, last_edit, file, deleted, views, reactions, approvalStatus, topicIds, isThread, isAds). The client has no "my reaction" state.
- **One or several per user:** not confirmed. Hint: the admin removal API is keyed by `(messageId, userId, emoji)` (`/api/admin/reactions/remove`), which suggests a user can hold several emojis on one message.

```bash
# DO NOT RUN - changes data, and needs a logged-in session cookie
curl.exe -s -X POST "https://n.musiconline.co.il/api/reactions/set-reactions" \
  -H "Content-Type: application/json" -H "Cookie: <session cookie after Google login>" \
  -d "{\"messageId\":8910,\"emoji\":\"👍\"}"
# safe (read-only)
curl.exe -s "https://n.musiconline.co.il/api/emojis/list"
curl.exe -s "https://n.musiconline.co.il/api/emojis/ads-list"
```

## 3. hateder.net (/pargod, /pargodmusic)

Source: inline script in `https://hateder.net/pargod` (functions `reactPost`, `postJSON`, `paintReacts`, `applyReacts`).

- **Endpoint:** `POST https://hateder.net/api/react`
- **Body:** `{"id": <int>, "emoji": "<key>" | null}`. `null` = remove my reaction.
  Note: `emoji` is the **key**, not the emoji character.
- **Allowed keys (fixed in the page, order as in `REACTIONS`):**
  `sad` 😢, `scared` 😱, `happy` 😂, `fire` 🔥, `heart` ❤️, `dislike` 👎, `like` 👍.
  (`angry` 😡 was removed on 6.10.2026; old votes stay in the DB.)
  Per item, the owner can switch off keys: `react_off: [keys]` on the message. A switched-off key returns error `code:"off"`.
- **Identity (guest):** cookie `vid` (HttpOnly, about 400 days). The server sets it on `POST /api/hit`
  (`{path, ref}`, sent once per page load) and also on other responses (an OPTIONS on `/api/react` returned `Set-Cookie: vid=...`).
  No cookie -> error `code:"novid"`, and the response itself carries a new `vid`. The client then retries once.
- **One per user:** yes. Clicking a different key replaces the old one. Clicking the same key sends `null`.
- **Response:** `{counts:{key:n}, mine:"<key>"|null, counted:true|false, wait:<sec>, admin:bool}`.
  - `counted:false` + `wait:N` = a new visitor must wait N seconds before the vote counts
    ("👋 חדשים באתר? נא להמתין N שניות"). The client resends after `wait` (up to 2 tries). This is the anti-abuse gate.
  - `admin:true` = the browser is logged in to the admin; the vote is not counted.
- **Mine:** only in the POST response. In `GET /api/messages`, `reacts.mine` is always `null`, because the feed is cached for all readers.
  The site keeps "mine" in `localStorage` (key `mine`, `MINE.r[id]`). A reader must store "mine" locally too.
- **Live counts:** the poll response has a top-level `reacts` map `{id: {counts}}`.
- Hosting: Cloudflare in front of Render. A cold Render service can return a non-JSON error page.

```bash
# DO NOT RUN - changes data. A proxy must keep the vid cookie per reader.
curl.exe -s -c jar.txt -b jar.txt -X POST "https://hateder.net/api/hit" -H "Content-Type: application/json" -d "{\"path\":\"/pargod\",\"ref\":\"\"}"
curl.exe -s -c jar.txt -b jar.txt -X POST "https://hateder.net/api/react" -H "Content-Type: application/json" -d "{\"id\":7374,\"emoji\":\"like\"}"
curl.exe -s -c jar.txt -b jar.txt -X POST "https://hateder.net/api/react" -H "Content-Type: application/json" -d "{\"id\":7374,\"emoji\":null}"   # remove
```

## 4. emess.co.il/online

Source: Nuxt chunks `CF4J4RML.js` (feed + like button component) and `D4Fb3192.js` (fetch helper).

- **Endpoint:** `POST https://www.emess.co.il/Online/Like`
  (the same component posts to `/Replies/Like` for comments).
- **Headers:** `Accept: application/json`, `Content-Type: application/json; charset=utf-8;`. `credentials: same-origin` (cookies sent if any). No token header.
- **Body:** `{"id": <feed item id>, "rank": <0..4>}`.
- **Ranks** (from the image list `,like,dislike,happy,sad`): `1` 👍 like, `2` 👎 dislike, `3` 😄 happy, `4` 😢 sad. `0` = remove.
  (The button `alt` text array is off by one in the site code; the image order is the true mapping.)
- **One per user:** yes. Clicking the active rank sends `0`; clicking another sends the new rank.
- **Response:** JSON with `value` = the new rank counts `{"1":n,"2":n,...}`.
- **Mine:** `GET /Online/Feed/0` returns `likes: {"<id>": rank}` (the client sets `isRank` from it).
  But the feed is CDN-cached (`cf-cache-status: HIT`, `CDN-Cache-Control: max-age=1000`), and the cached copy has `likes: {}`.
  So in practice a reader must store "mine" locally.
- **Identity:** not confirmed. The page and the feed set no cookie. No login in the flow (the only login hint is a `wp_user_logged` cookie check for a different feature).
  Most likely IP-based, or a cookie set by the Like response. Confirming it needs one real POST.

```bash
# DO NOT RUN - changes data
curl.exe -s -X POST "https://www.emess.co.il/Online/Like" -H "Accept: application/json" \
  -H "Content-Type: application/json; charset=utf-8;" -d "{\"id\":81442,\"rank\":1}"
# remove: same with "rank":0
```

## 5. authenti.newsupdates.click

Source: `/assets/js/app.js?v=20260928b` (`toggleReaction`), `/assets/js/auth-manager.js`.

- **Endpoint:** `POST https://authenti.newsupdates.click/api/add_reaction.php`
- **Body:** `{"news_id": <int>, "reaction_type": "like"|"dislike"|"laugh"|"vomit"}`.
  Emojis: like 👍, dislike 👎, laugh 😂, vomit 🤮.
- **Auth:** login is required. The client checks `APP_CONFIG.isAuthenticated` (false for a guest) and the API returns
  `{error:"authentication_required"}`. Login = Google only (`login.php?return_to=...`). Session cookies: `PHPSESSID`, `user_sess` (set on the first page load even for a guest).
- **Response:** `{success:true, user_reaction:"<type>"|null, counts:{likes,dislikes,laughs,vomits}}`.
- **One per user:** yes (`user_reaction` is a single value; the function is named toggle, so the same type again most likely removes it).
- **Mine:** each item in `get_messages_optimized.php` has `user_reaction` (null for a guest) and the four counts.
- **Live counts:** SSE (`sse-realtime.js`, `handleReactionUpdate`).

```bash
# DO NOT RUN - changes data, and needs a Google-logged-in PHPSESSID/user_sess
curl.exe -s -X POST "https://authenti.newsupdates.click/api/add_reaction.php" \
  -H "Content-Type: application/json" -H "Cookie: PHPSESSID=<...>; user_sess=<...>" \
  -d "{\"news_id\":1108658,\"reaction_type\":\"like\"}"
```

## 6. hm-news.co.il news-flash

- No reaction or like on flashes. The flash card has time, title and a share button only.
- REST item fields: no counts. ACF fields: add_video, change_author_name, hide_image, image, video, video-slider.
- Custom REST namespaces checked: `news_flash/v1` (`/add` POST = submit a flash), `hm/v1` (music-chart vote), `hamechadesh/v1` (`/live-update`), `app/v1` (`/votes` POST, no args, not used by the web page; probably the mobile app's polls). None is a flash reaction.

```bash
# safe (read-only)
curl.exe -s "https://hm-news.co.il/wp-json/app/v1"
```

## Blockers and risks

- **Login wall:** both chat builds and authenti need a Google login to react. A reader can offer reactions there only if the user signs in once in a browser and the proxy reuses that session cookie. Not a guest feature.
- **hateder:** guest reactions work, but a new `vid` must wait (`counted:false`, `wait:N`) before votes count. The proxy must keep one `vid` per reader (not one shared cookie for everyone, or all readers would share one vote).
- **emess:** guest identity is not confirmed (probably IP). Through a proxy, all readers may share one vote. Needs one real test POST to confirm.
- **"Mine" display:** only hagizra v3 and authenti return "mine" from a GET (and only when logged in). hateder and emess serve cached feeds, so the reader must keep "mine" in local storage, as the sites do.
- No captcha was seen in any reaction flow. Rate limits seen: thechats client 4 s; hateder new-visitor wait.
- A guest probe (POST with a non-existent message id, to read the 401 without changing data) was blocked by the local permission guard and was not sent.
