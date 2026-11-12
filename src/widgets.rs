// The widgets (Windows App SDK widget provider): "breaking", "channel" (a channel or a feed the user picks)
// and "top" (one big headline that turns every 15 seconds). Cards are Adaptive Cards, right to left.
// The page in the board does the fetching (it has every source adapter and keeps running while the board
// is closed): it asks /winfeed/widgets/need what to load and pushes the items to /winfeed/widgets/push.
// The last push is kept on disk, so the widgets show something after a restart too.
use crate::bindings::*;
use crate::log;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use windows_core::{implement, Ref, HSTRING};

pub struct W {
    kind: String,
    size: WidgetSize,
    pick: String,
    active: bool,
    customizing: bool,
}
static WIDGETS: Mutex<BTreeMap<String, W>> = Mutex::new(BTreeMap::new());
static DATA: Mutex<Option<Value>> = Mutex::new(None);
static REFRESH: AtomicBool = AtomicBool::new(false);
static TURN: AtomicUsize = AtomicUsize::new(0);
static FRAME: AtomicUsize = AtomicUsize::new(0);
static GOTO: Mutex<String> = Mutex::new(String::new());
// Pictures the page prepared (data URLs: the board draws no WEBP or SVG, and some sites refuse it). Items name them "img:<key>". Only the ones the current lists use are kept.
static IMGS: Mutex<BTreeMap<String, String>> = Mutex::new(BTreeMap::new());
pub fn put_image(key: &str, data: &str) {
    if data.starts_with("data:image/") && data.len() < 400_000 { IMGS.lock().unwrap().insert(key.to_string(), data.to_string()); }
}
fn imgs_path() -> std::path::PathBuf { data_path().with_file_name("widget-images.json") }
fn pic(v: &Value) -> String {
    let u = v.as_str().unwrap_or("");
    let Some(k) = u.strip_prefix("img:") else { return u.to_string() };
    let mut m = IMGS.lock().unwrap();
    if m.is_empty() {
        if let Some(saved) = std::fs::read(imgs_path()).ok().and_then(|b| serde_json::from_slice::<BTreeMap<String, String>>(&b).ok()) { *m = saved; }
    }
    m.get(k).cloned().unwrap_or_default()
}

fn data_path() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var("LOCALAPPDATA").unwrap_or_default()).join("HagizraFeed").join("widgets.json")
}
fn data() -> Value {
    let mut g = DATA.lock().unwrap();
    g.get_or_insert_with(|| std::fs::read(data_path()).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or(json!({})))
        .clone()
}

/// What the page should load for the widgets: {"lists":[...],"refresh":bool}. Only for widgets on the board now.
pub fn need() -> String {
    let w = WIDGETS.lock().unwrap();
    let mut lists: Vec<String> = w.values().filter(|x| x.active).map(list_of).map(|l| if l == "search:" { "popular".into() } else { l }).collect();
    lists.sort();
    lists.dedup();
    let goto = std::mem::take(&mut *GOTO.lock().unwrap());
    json!({ "lists": lists, "refresh": REFRESH.swap(false, Ordering::Relaxed), "goto": goto }).to_string()
}

/// The page's items: {"choices":[{id,name}],"lists":{id:[item]}}; item = {n,logo,t,title,img,url,brk}.
pub fn push(body: &str) {
    let Ok(new) = serde_json::from_str::<Value>(body) else { return log("widgets: bad push") };
    let mut all = data();
    if let Some(c) = new.get("choices") { all["choices"] = c.clone(); }
    if let Some(c) = new.get("choicesTop") { all["choicesTop"] = c.clone(); }
    if all.get("lists").and_then(Value::as_object).is_none() { all["lists"] = json!({}); }
    if let Some(l) = new.get("lists").and_then(Value::as_object) {
        for (k, v) in l { all["lists"][k] = v.clone(); }
    }
    // The lists the page is still looking deeper into (a search: first results, then more).
    if let Some(b) = new.get("busy") { all["busy"] = b.clone(); }
    let used = all.to_string();
    {
        let mut m = IMGS.lock().unwrap();
        m.retain(|k, _| used.contains(&format!("img:{k}")));
        let _ = std::fs::write(imgs_path(), serde_json::to_string(&*m).unwrap_or_default());
    }
    let _ = std::fs::write(data_path(), all.to_string());
    *DATA.lock().unwrap() = Some(all);
    update_all(false);
}

/// The list a widget shows: the breaking news, the headline's pictures ("top:<pick>"), or the pick itself.
fn list_of(w: &W) -> String {
    match w.kind.as_str() {
        "breaking" | "popular" | "cats" | "deal" => w.kind.clone(),
        "top" => format!("top:{}", w.pick),
        // search: the query is the list ("search:<words>"); none yet = nothing to load
        _ => w.pick.clone(),
    }
}
fn ago(t: i64) -> String {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0);
    let m = (now - t).max(0) / 60000;
    match m {
        0 => "עכשיו".into(),
        1..=59 => format!("{m} דק'"),
        60..=1439 => format!("{} שע'", m / 60),
        _ => format!("{} ימים", m / 1440),
    }
}

fn items(list: &str, n: usize) -> Vec<Value> {
    let d = data();
    d["lists"][list].as_array().map(|a| a.iter().take(n).map(|it| {
        let mut it = it.clone();
        it["ago"] = json!(ago(it["t"].as_i64().unwrap_or(0)));
        it
    }).collect()).unwrap_or_default()
}
fn choice_name(pick: &str) -> String {
    data()["choices"].as_array().and_then(|a| a.iter().find(|c| c["id"] == pick)).and_then(|c| c["name"].as_str()).unwrap_or("הפיד").to_string()
}

fn card(body: Value, actions: Value, header: Value) -> String {
    let mut c = json!({ "type": "AdaptiveCard", "version": "1.5", "body": body, "actions": actions,
            "$schema": "http://adaptivecards.io/schemas/adaptive-card.json" });
    if !header.is_null() { c["header"] = header; }
    c.to_string()
}
fn empty(text: &str) -> Value {
    json!([{ "type": "TextBlock", "text": text, "wrap": true, "isSubtle": true, "horizontalAlignment": "Center", "spacing": "Large" }])
}
const WAIT: &str = "פתח פעם אחת את הפיד בלוח, וכאן יופיעו הידיעות. אחר כך הן מתעדכנות לבד.";

// ---------- the cards ----------
// Microsoft's widget rules (learn.microsoft.com, widgets design fundamentals and interaction design):
// the board draws the title row itself (so no title inside the card), at most 1 / 3 / 4 click targets for
// small / medium / large, round avatars of 24 px, the Windows type ramp, spacing in multiples of 4.
fn open(w_kind: &str, it: &Value) -> Value {
    // A feed item in the headline opens in the feed (the page jumps to it); everything else opens on its site.
    // A site's article is not in the feed: it opens on its site.
    if w_kind == "top" && !s(&it["key"]).is_empty() { json!({ "type": "Action.Execute", "verb": "open", "data": { "key": it["key"] } }) }
    else { json!({ "type": "Action.OpenUrl", "url": it["url"] }) }
}
fn s(v: &Value) -> &str { v.as_str().unwrap_or("") }
/// One line: a red "● מבזק" or the channel name, then the time in the quiet colour.
fn meta(it: &Value, breaking_widget: bool, name: bool) -> Value {
    let mut runs = vec![];
    if it["brk"] == true || breaking_widget {
        runs.push(json!({ "type": "TextRun", "text": "● ", "color": "Attention", "size": "Small" }));
    }
    if name {
        runs.push(json!({ "type": "TextRun", "text": s(&it["n"]), "size": "Small", "weight": "Bolder" }));
        runs.push(json!({ "type": "TextRun", "text": " · ", "size": "Small", "isSubtle": true }));
    }
    runs.push(json!({ "type": "TextRun", "text": s(&it["ago"]), "size": "Small", "isSubtle": true }));
    json!({ "type": "RichTextBlock", "inlines": runs, "spacing": "None" })
}
/// The avatar beside that line.
fn caption(it: &Value, breaking_widget: bool) -> Value {
    let mut cols = vec![];
    if !pic(&it["logo"]).is_empty() {
        cols.push(json!({ "type": "Column", "width": "auto", "verticalContentAlignment": "Center",
            "items": [{ "type": "Container", "rtl": false, "items": [{ "type": "Image", "url": pic(&it["logo"]), "style": "Person", "width": "24px", "height": "24px" }] }] }));
    }
    cols.push(json!({ "type": "Column", "width": "stretch", "verticalContentAlignment": "Center", "items": [meta(it, breaking_widget, true)] }));
    json!({ "type": "ColumnSet", "columns": cols, "spacing": "None" })
}
/// A picture cropped to fill a box (a background image never stretches out of shape).
fn hero(img: &str, h: u32) -> Value {
    // "rtl": false: in the board's right-to-left layout a picture would be drawn mirrored.
    json!({ "type": "Container", "rtl": false, "minHeight": format!("{h}px"),
            "backgroundImage": { "url": img, "fillMode": "Cover", "horizontalAlignment": "Center", "verticalAlignment": "Center" },
            "items": [{ "type": "TextBlock", "text": " ", "size": "Small" }] })
}
/// The main item: caption, then the headline in a strong style.
fn lead(kind: &str, it: &Value, size: &str, lines: u32, img_h: u32, sub: u32) -> Value {
    let mut items = vec![];
    if img_h > 0 && !pic(&it["img"]).is_empty() { items.push(hero(&pic(&it["img"]), img_h)); }
    items.push(caption(it, kind == "breaking"));
    items.push(json!({ "type": "TextBlock", "text": it["title"], "wrap": true, "maxLines": lines, "weight": "Bolder", "size": size, "spacing": "Small" }));
    if sub > 0 && !s(&it["sub"]).is_empty() {
        items.push(json!({ "type": "TextBlock", "text": it["sub"], "wrap": true, "maxLines": sub, "isSubtle": true, "spacing": "Small" }));
    }
    json!({ "type": "Container", "selectAction": open(kind, it), "items": items })
}
/// The lead item with its picture beside the text (medium size).
fn lead_side(kind: &str, it: &Value, lines: u32) -> Value {
    let mut cols = vec![json!({ "type": "Column", "width": "stretch", "items": [caption(it, kind == "breaking"),
        { "type": "TextBlock", "text": it["title"], "wrap": true, "maxLines": lines, "weight": "Bolder", "size": "Medium", "spacing": "Small" },
        { "type": "TextBlock", "text": it["sub"], "wrap": true, "maxLines": 2, "isSubtle": true, "spacing": "Small", "isVisible": !s(&it["sub"]).is_empty() }] })];
    if !pic(&it["img"]).is_empty() {
        cols.push(json!({ "type": "Column", "width": "88px", "items": [hero(&pic(&it["img"]), 88)] }));
    }
    json!({ "type": "ColumnSet", "selectAction": open(kind, it), "columns": cols })
}
/// A row under the main item: the small line (channel and time), then the headline.
fn row(kind: &str, it: &Value, lines: u32) -> Value {
    json!({ "type": "Container", "spacing": "Medium", "selectAction": open(kind, it), "items": [
        meta(it, kind == "breaking", kind != "breaking"),
        { "type": "TextBlock", "text": if s(&it["line"]).is_empty() { it["title"].clone() } else { it["line"].clone() }, "wrap": true, "maxLines": lines, "spacing": "None" }
    ] })
}
fn dots(n: usize, at: usize) -> Value {
    let d: Vec<&str> = (0..n).map(|i| if i == at { "●" } else { "○" }).collect();
    json!({ "type": "TextBlock", "text": d.join(" "), "size": "Small", "isSubtle": true, "horizontalAlignment": "Center", "spacing": "Small" })
}

pub fn render(w: &W) -> String {
    if w.customizing {
        let key = if w.kind == "top" { "choicesTop" } else { "choices" };
        let choices: Vec<Value> = data()[key].as_array().cloned().unwrap_or_default().iter()
            .map(|c| json!({ "title": c["name"], "value": c["id"] })).collect();
        let body = json!([
            { "type": "TextBlock", "text": if w.kind == "top" { "מאיזה אתר להציג כתבות?" } else { "מה להציג בווידג'ט?" }, "weight": "Bolder", "wrap": true },
            { "type": "Input.ChoiceSet", "id": "pick", "value": w.pick, "style": "compact", "choices": choices },
        ]);
        let actions = json!([{ "type": "Action.Execute", "title": "שמירה", "verb": "save" }, { "type": "Action.Execute", "title": "ביטול", "verb": "cancel" }]);
        return card(body, actions, Value::Null);
    }
    let list_id = list_of(w);
    let list = items(&list_id, 8);
    if w.kind == "search" { return search_card(w, &list, data()["lists"].get(&list_id).is_some(), searching(w)); }
    // The channel widget is titled with what it shows (the board draws the title row).
    let head = if w.kind == "channel" { json!(choice_name(&w.pick)) } else { Value::Null };
    if list.is_empty() {
        let known = data()["lists"].get(&list_id).is_some();
        let msg = if !known { WAIT } else if w.kind == "breaking" { "אין מבזקים חדשים כרגע." } else { "אין כאן ידיעות כרגע." };
        return card(empty(msg), json!([]), head);
    }
    let body = if w.kind == "deal" {
        // The five newest deals, turning every 15 seconds: picture, product, price line.
        let n = list.len().min(5);
        let at = TURN.load(Ordering::Relaxed) % n;
        let it = &list[at];
        match w.size {
            WidgetSize::Small => json!([lead("deal", it, "Default", 3, 0, 0)]),
            WidgetSize::Medium => json!([lead_side("deal", it, 3), dots(n, at)]),
            _ => json!([lead("deal", it, "Medium", 3, 220, 3), dots(n, at)]),
        }
    } else if w.kind == "cats" {
        // One line per category: its newest item.
        let take = match w.size { WidgetSize::Small => 1, WidgetSize::Medium => 3, _ => 6 };
        Value::Array(list.iter().take(take).map(|it| row("cats", it, if w.size == WidgetSize::Small { 3 } else { 2 })).collect())
    } else if w.kind == "top" {
        let n = list.len().min(10);
        let at = TURN.load(Ordering::Relaxed) % n;
        let it = &list[at];
        match w.size {
            // Small: a square picture beside the headline.
            WidgetSize::Small => {
                let mut cols = vec![json!({ "type": "Column", "width": "stretch", "items": [meta(it, false, true),
                    { "type": "TextBlock", "text": it["title"], "wrap": true, "maxLines": 3, "weight": "Bolder", "spacing": "Small" }] })];
                if !pic(&it["img"]).is_empty() { cols.push(json!({ "type": "Column", "width": "72px", "items": [hero(&pic(&it["img"]), 72)] })); }
                json!([{ "type": "ColumnSet", "selectAction": open("top", it), "columns": cols }])
            }
            // Medium: the picture across, then the headline.
            WidgetSize::Medium => json!([lead("top", it, "Medium", 2, 116, 0), dots(n, at)]),
            _ => json!([lead("top", it, "Large", 3, 212, 3), dots(n, at)]),
        }
    } else {
        let kind = w.kind.as_str();
        match w.size {
            WidgetSize::Small => json!([lead(kind, &list[0], "Default", 2, 0, 0)]),
            WidgetSize::Medium => {
                let mut b = vec![lead_side(kind, &list[0], 2)];
                b.extend(list.iter().skip(1).take(2).map(|it| row(kind, it, 2)));
                Value::Array(b)
            }
            _ => {
                // No room may stay empty: without a picture, more rows; with few items, longer rows.
                let rows = if pic(&list[0]["img"]).is_empty() { 5 } else { 3 };
                let lines = if list.len() <= 4 { 4 } else { 2 };
                let mut b = vec![lead(kind, &list[0], "Medium", 3, 168, if lines == 4 { 4 } else { 2 })];
                b.extend(list.iter().skip(1).take(rows).map(|it| row(kind, it, lines)));
                Value::Array(b)
            }
        }
    };
    card(body, json!([]), head)
}

/// The search widget: a box with a search button, then what was found (the board draws no keyboard
/// focus tricks; Enter in the box also searches).
/// A search the page has not finished: nothing came back yet, or the first results and more on the way.
fn searching(w: &W) -> bool {
    let (id, d) = (list_of(w), data());
    w.pick != "search:" && (d["lists"].get(&id).is_none() || d["busy"].as_array().is_some_and(|b| b.iter().any(|x| x == &json!(id))))
}
/// A line of 12 parts with a 3-part piece that runs back and forth (the card is redrawn every 300 ms while searching).
fn runner() -> Value {
    let f = FRAME.load(Ordering::Relaxed) % 18;
    let at = if f > 9 { 18 - f } else { f };
    // Text, not coloured boxes: the board draws no empty box.
    let runs: Vec<Value> = (0..12).map(|i| if (at..at + 3).contains(&i) { json!({ "type": "TextRun", "text": "━", "color": "Accent", "weight": "Bolder" }) }
        else { json!({ "type": "TextRun", "text": "━", "isSubtle": true }) }).collect();
    json!({ "type": "RichTextBlock", "inlines": runs, "horizontalAlignment": "Center", "spacing": "Medium" })
}
fn search_card(w: &W, list: &[Value], loaded: bool, busy: bool) -> String {
    let q = w.pick.strip_prefix("search:").unwrap_or("");
    let mut body = vec![json!({ "type": "Input.Text", "id": "q", "value": q, "placeholder": "מילה לחיפוש בכל הערוצים",
        "inlineAction": { "type": "Action.Execute", "verb": "search", "title": "חיפוש" } })];
    let take = match w.size { WidgetSize::Small => 0, WidgetSize::Medium => 2, _ => 4 };
    if busy {
        body.push(runner());
        if list.is_empty() {
            body.push(json!({ "type": "TextBlock", "text": "מחפש בכל הערוצים הדלוקים ובאתרים...", "wrap": true, "isSubtle": true, "size": "Small", "horizontalAlignment": "Center", "spacing": "Small" }));
        }
    }
    if q.is_empty() {
        // Nothing asked yet: what is hot now, so the widget is not empty.
        let hot = items("popular", take);
        if take > 0 && !hot.is_empty() {
            body.push(json!({ "type": "TextBlock", "text": "חם עכשיו", "size": "Small", "weight": "Bolder", "isSubtle": true, "spacing": "Medium" }));
            body.extend(hot.iter().map(|it| row("search", it, 2)));
        } else if w.size != WidgetSize::Small { body.extend(empty("חיפוש בידיעות האחרונות של הערוצים הדלוקים.").as_array().cloned().unwrap_or_default()); }
    } else if list.is_empty() {
        if !busy { body.extend(empty(if loaded { "לא נמצאו ידיעות." } else { "מחפש..." }).as_array().cloned().unwrap_or_default()); }
    } else if take == 0 {
        body.push(json!({ "type": "TextBlock", "text": format!("נמצאו {} ידיעות. הגדילו את הווידג'ט כדי לראות אותן.", list.len()), "wrap": true, "isSubtle": true, "spacing": "Small" }));
    } else {
        body.extend(list.iter().take(take).map(|it| row("search", it, 2)));
    }
    card(Value::Array(body), json!([]), Value::Null)
}

/// Redraws a searching widget every 300 ms (the moving bar) until the page has finished, at most two minutes.
fn spin(id: String) {
    std::thread::spawn(move || {
        let _ = unsafe { crate::CoInitializeEx(core::ptr::null(), 0) };
        for _ in 0..400 {
            std::thread::sleep(std::time::Duration::from_millis(300));
            FRAME.fetch_add(1, Ordering::Relaxed);
            let w = WIDGETS.lock().unwrap();
            let Some(x) = w.get(&id) else { return };
            send(&id, x);
            if !searching(x) || !x.active { return; }
        }
    });
}

fn send(id: &str, w: &W) {
    let template = render(w);
    let r = (|| -> windows_core::Result<()> {
        let o = WidgetUpdateRequestOptions::CreateInstance(&HSTRING::from(id))?;
        o.SetTemplate(&HSTRING::from(template))?;
        o.SetData(&HSTRING::from("{}"))?;
        o.SetCustomState(&HSTRING::from(json!({ "pick": w.pick }).to_string()))?;
        WidgetManager::GetDefault()?.UpdateWidget(&o)
    })();
    if let Err(e) = r { log(&format!("widget update {id}: {e}")); }
}
fn update_all(only_top: bool) {
    let w = WIDGETS.lock().unwrap();
    for (id, x) in w.iter().filter(|(_, x)| x.active && (!only_top || x.kind == "top" || x.kind == "deal")) { send(id, x); }
}

fn from_ctx(ctx: &WidgetContext, custom: &str) -> windows_core::Result<(String, W)> {
    let kind = ctx.DefinitionId()?.to_string_lossy();
    let pick = serde_json::from_str::<Value>(custom).ok().and_then(|v| v["pick"].as_str().map(str::to_string))
        .unwrap_or_default();
    let pick = match (kind.as_str(), pick.as_str()) {
        ("top", p) if !p.starts_with("articles:") => "articles:all".into(),
        ("search", p) if !p.starts_with("search:") => "search:".into(),
        (_, "") => "tab:all".into(),
        _ => pick,
    };
    Ok((ctx.Id()?.to_string_lossy(), W { kind, size: ctx.Size()?, pick, active: ctx.IsActive()?, customizing: false }))
}

#[implement(IWidgetProvider, IWidgetProvider2)]
pub struct Provider;

impl Provider {
    /// The board may have restarted us: take back the widgets it still shows, and start the clock.
    pub fn new() -> Self {
        static STARTED: AtomicBool = AtomicBool::new(false);
        if !STARTED.swap(true, Ordering::Relaxed) {
            if let Ok(infos) = WidgetManager::GetDefault().and_then(|m| m.GetWidgetInfos()) {
                let mut w = WIDGETS.lock().unwrap();
                for info in infos.iter().flatten() {
                    if let (Ok(ctx), Ok(cs)) = (info.WidgetContext(), info.CustomState()) {
                        if let Ok((id, x)) = from_ctx(&ctx, &cs.to_string_lossy()) { w.insert(id, x); }
                    }
                }
            }
            std::thread::spawn(|| {
                let _ = unsafe { crate::CoInitializeEx(core::ptr::null(), 0) };
                // ponytail: one clock. Every 15 s the headline turns; every minute every card gets fresh "minutes ago".
                for tick in 1usize.. {
                    std::thread::sleep(std::time::Duration::from_secs(15));
                    TURN.fetch_add(1, Ordering::Relaxed);
                    update_all(tick % 4 != 0);
                }
            });
        }
        Provider
    }
}

impl IWidgetProvider_Impl for Provider_Impl {
    fn CreateWidget(&self, ctx: Ref<WidgetContext>) -> windows_core::Result<()> {
        let (id, x) = from_ctx(ctx.ok()?, "")?;
        send(&id, &x);
        WIDGETS.lock().unwrap().insert(id, x);
        REFRESH.store(true, Ordering::Relaxed);
        Ok(())
    }
    fn DeleteWidget(&self, id: &HSTRING, _custom: &HSTRING) -> windows_core::Result<()> {
        WIDGETS.lock().unwrap().remove(&id.to_string_lossy());
        Ok(())
    }
    fn OnActionInvoked(&self, a: Ref<WidgetActionInvokedArgs>) -> windows_core::Result<()> {
        let a = a.ok()?;
        let id = a.WidgetContext()?.Id()?.to_string_lossy();
        let verb = a.Verb()?.to_string_lossy();
        let input: Value = serde_json::from_str(&a.Data()?.to_string_lossy()).unwrap_or(json!({}));
        let mut w = WIDGETS.lock().unwrap();
        let Some(x) = w.get_mut(&id) else { return Ok(()) };
        match verb.as_str() {
            "save" => { if let Some(p) = input["pick"].as_str() { x.pick = p.to_string(); } x.customizing = false; REFRESH.store(true, Ordering::Relaxed); }
            "cancel" => x.customizing = false,
            "refresh" => REFRESH.store(true, Ordering::Relaxed),
            "search" => {
                let q: String = input["q"].as_str().unwrap_or("").trim().chars().take(60).collect();
                // Only this list: a refresh of every list would make the search wait for all the others.
                x.pick = format!("search:{q}");
                if !q.is_empty() { spin(id.clone()); }
            }
            // The feed is another page of the board; the page jumps to the item when it shows (see feed.html).
            "open" => { if let Some(k) = input["key"].as_str() { *GOTO.lock().unwrap() = k.to_string(); } return Ok(()); }
            _ => {}
        }
        send(&id, x);
        Ok(())
    }
    fn OnWidgetContextChanged(&self, a: Ref<WidgetContextChangedArgs>) -> windows_core::Result<()> {
        let ctx = a.ok()?.WidgetContext()?;
        let id = ctx.Id()?.to_string_lossy();
        let mut w = WIDGETS.lock().unwrap();
        if let Some(x) = w.get_mut(&id) { x.size = ctx.Size()?; send(&id, x); }
        Ok(())
    }
    fn Activate(&self, ctx: Ref<WidgetContext>) -> windows_core::Result<()> {
        let id = ctx.ok()?.Id()?.to_string_lossy();
        let mut w = WIDGETS.lock().unwrap();
        if let Some(x) = w.get_mut(&id) { x.active = true; send(&id, x); }
        REFRESH.store(true, Ordering::Relaxed);
        Ok(())
    }
    fn Deactivate(&self, id: &HSTRING) -> windows_core::Result<()> {
        if let Some(x) = WIDGETS.lock().unwrap().get_mut(&id.to_string_lossy()) { x.active = false; }
        Ok(())
    }
}
impl IWidgetProvider2_Impl for Provider_Impl {
    fn OnCustomizationRequested(&self, a: Ref<WidgetCustomizationRequestedArgs>) -> windows_core::Result<()> {
        let id = a.ok()?.WidgetContext()?.Id()?.to_string_lossy();
        let mut w = WIDGETS.lock().unwrap();
        if let Some(x) = w.get_mut(&id) { x.customizing = true; send(&id, x); }
        Ok(())
    }
}

/// Dev aid (hagizra-widget.exe --cards <file>): every widget in every size, from the last pushed data, for
/// tools/widget-preview.html. The board itself never calls this.
pub fn dump(path: &str, data_file: Option<&str>) {
    // A data file from the dev page (tools/dev-server.mjs, POST /widgets/data) instead of the last push.
    if let Some(v) = data_file.and_then(|f| std::fs::read(f).ok()).and_then(|b| serde_json::from_slice::<Value>(&b).ok()) {
        *DATA.lock().unwrap() = Some(json!({ "lists": v }));
    }
    let mut out = vec![];
    for (kind, pick) in [("breaking", "breaking"), ("channel", "tab:all"), ("top", "articles:all"), ("popular", ""), ("cats", ""), ("deal", ""), ("search", "search:בחירות")] {
        for (name, size) in [("small", WidgetSize::Small), ("medium", WidgetSize::Medium), ("large", WidgetSize::Large)] {
            let w = W { kind: kind.into(), size, pick: pick.into(), active: true, customizing: false };
            out.push(json!({ "kind": kind, "size": name, "card": serde_json::from_str::<Value>(&render(&w)).unwrap_or(Value::Null) }));
        }
    }
    let _ = std::fs::write(path, Value::Array(out).to_string());
}

