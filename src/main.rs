// Hagizra feed for the Windows widgets board: a Windows App SDK feed provider (out-of-proc COM server).
// The board loads https://hagizra.news/winfeed/ ; WebRequestFilter routes that path to OnResourceRequested,
// which answers with the embedded page. The page then reads hagizra.news/api/* same-origin.
#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

#[allow(warnings)]
mod bindings;
mod http;
mod login;
mod clip;
mod notify;
mod widgets;
mod update;

use bindings::*;
use core::ffi::c_void;
use std::collections::BTreeMap;
use windows_core::{implement, interface, IUnknown, Interface, Ref, BOOL, GUID, HRESULT, HSTRING};

// Must match AppxManifest.xml (com:Class Id and CreateInstance ClassId). The distribution build sets its own, so it can sit next to the dev package.
const CLSID: &str = match option_env!("HZ_CLSID") { Some(s) => s, None => "5D2F8A3E-7C41-4B9E-9A63-2E8F0C7D1B54" };
// The widgets' class (widgets.rs), registered by the same process.
const WCLSID: &str = match option_env!("HZ_WCLSID") { Some(s) => s, None => "53219DDD-D127-43D1-8F2A-E0137971AFAF" };
const PAGE: &str = include_str!("feed.html");

#[link(name = "ole32")]
unsafe extern "system" {
    fn CoInitializeEx(reserved: *const c_void, coinit: u32) -> HRESULT;
    fn CoRegisterClassObject(clsid: *const GUID, unk: *mut c_void, ctx: u32, flags: u32, cookie: *mut u32) -> HRESULT;
    fn CoWaitForMultipleObjects(flags: u32, timeout: u32, n: u32, handles: *const *mut c_void, index: *mut u32) -> HRESULT;
}
#[link(name = "kernel32")]
unsafe extern "system" {
    fn CreateEventW(attr: *const c_void, manual: BOOL, initial: BOOL, name: *const u16) -> *mut c_void;
}
#[link(name = "shell32")]
unsafe extern "system" {
    fn ShellExecuteW(hwnd: *mut c_void, op: *const u16, file: *const u16, params: *const u16, dir: *const u16, show: i32) -> *mut c_void;
}
#[link(name = "shlwapi")]
unsafe extern "system" {
    fn SHCreateMemStream(data: *const u8, len: u32) -> *mut c_void;
}
#[link(name = "shcore")]
unsafe extern "system" {
    fn CreateRandomAccessStreamOverStream(stream: *mut c_void, opts: u32, iid: *const GUID, out: *mut *mut c_void) -> HRESULT;
}

fn response(body: &[u8], mime: &str) -> windows_core::Result<FeedResourceResponse> {
    respond(body, mime, 200)
}

fn respond(body: &[u8], mime: &str, status: i32) -> windows_core::Result<FeedResourceResponse> {
    unsafe {
        let istream = SHCreateMemStream(body.as_ptr(), body.len() as u32);
        if istream.is_null() {
            return Err(windows_core::Error::from_hresult(HRESULT(0x8007000Eu32 as i32))); // E_OUTOFMEMORY
        }
        let istream = IUnknown::from_raw(istream); // owns the IStream ref
        let mut ras = core::ptr::null_mut();
        CreateRandomAccessStreamOverStream(istream.as_raw(), 0, &IRandomAccessStream::IID, &mut ras).ok()?;
        let ras = IRandomAccessStream::from_raw(ras);
        let r = FeedResourceResponse::CreateInstance(&RandomAccessStreamReference::CreateFromStream(&ras)?, &HSTRING::from(if status < 400 { "OK" } else { "Error" }), status)?;
        let mut h = BTreeMap::new();
        h.insert(HSTRING::from("Content-Type"), HSTRING::from(mime));
        h.insert(HSTRING::from("Cache-Control"), HSTRING::from("no-store"));
        r.SetHeaders(&windows_collections::IMapView::<HSTRING, HSTRING>::from(h).cast::<windows_collections::IIterable<windows_collections::IKeyValuePair<HSTRING, HSTRING>>>()?)?;
        Ok(r)
    }
}

#[implement(IFeedProvider, IFeedResourceProvider)]
struct Provider;

/// Appends one line to %TEMP%\hagizra-feed.log (diagnostics while the board integration is unproven).
fn log(msg: &str) {
    use std::io::Write;
    let path = std::env::temp_dir().join("hagizra-feed.log");
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let _ = writeln!(f, "{t} {msg}");
    }
}

/// FNV-1a of the embedded page: changes exactly when the page changes.
fn page_hash() -> String {
    let h = PAGE.bytes().fold(0xcbf29ce484222325u64, |h, b| (h ^ b as u64).wrapping_mul(0x100000001b3));
    format!("{h:016x}")
}

/// The docs have the provider hand the board its query string once enabled; ours is empty.
fn set_query(provider_id: windows_core::Result<HSTRING>) -> windows_core::Result<()> {
    let id = provider_id?;
    log(&format!("set query params for {}", id.to_string_lossy()));
    FeedManager::GetDefault()?.SetCustomQueryParameters(&CustomQueryParametersUpdateOptions::CreateInstance(&id, &HSTRING::new())?)
}

impl IFeedProvider_Impl for Provider_Impl {
    fn OnFeedProviderEnabled(&self, a: Ref<FeedProviderEnabledArgs>) -> windows_core::Result<()> {
        log("OnFeedProviderEnabled");
        set_query(a.ok()?.FeedProviderDefinitionId())
    }
    fn OnFeedProviderDisabled(&self, _a: Ref<FeedProviderDisabledArgs>) -> windows_core::Result<()> { log("OnFeedProviderDisabled"); Ok(()) }
    fn OnFeedEnabled(&self, _a: Ref<FeedEnabledArgs>) -> windows_core::Result<()> { log("OnFeedEnabled"); Ok(()) }
    fn OnFeedDisabled(&self, _a: Ref<FeedDisabledArgs>) -> windows_core::Result<()> { log("OnFeedDisabled"); Ok(()) }
    fn OnCustomQueryParametersRequested(&self, a: Ref<CustomQueryParametersRequestedArgs>) -> windows_core::Result<()> {
        log("OnCustomQueryParametersRequested");
        set_query(a.ok()?.FeedProviderDefinitionId())
    }
}

impl IFeedResourceProvider_Impl for Provider_Impl {
    fn OnResourceRequested(&self, args: Ref<FeedResourceRequestedArgs>) -> windows_core::Result<()> {
        let args = args.ok()?;
        let uri = args.Request()?.Uri()?.to_string_lossy();
        // The board keeps the page alive across openings; the page polls this to reload itself after a deploy.
        if uri.contains("/winfeed/version") {
            return args.SetResponse(&response(page_hash().as_bytes(), "text/plain")?);
        }
        // Other sites refuse cross-origin reads, so the page asks us to fetch them:
        // /winfeed/proxy?u=<url>[&m=POST&b=<body>&t=<body type>][&h=<Name: value>]...
        // Google sign-in for login-only channels (see login.rs). site = the channel host.
        if uri.contains("/winfeed/auth") {
            if let Some(q) = uri.split_once("/winfeed/auth?").map(|(_, q)| q) {
                let host = http::query(q, "site").next().unwrap_or_default();
                let host = host.trim_start_matches("https://").trim_end_matches('/').to_string();
                if !host.is_empty() && host.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '-') {
                    match http::query(q, "do").next().as_deref() {
                        Some("login") => login::start(host),
                        Some("logout") => login::forget(&host),
                        _ => {}
                    }
                }
            }
            return args.SetResponse(&respond(login::status_json().as_bytes(), "application/json", 200)?);
        }
        // The page's own diagnostics (script errors, reaction attempts) into this log: the board has no console.
        if let Some(q) = uri.split_once("/winfeed/log?").map(|(_, q)| q) {
            let m: String = http::query(q, "m").next().unwrap_or_default().chars().take(300).collect();
            log(&format!("page: {m}"));
            return args.SetResponse(&respond(b"ok", "text/plain", 200)?);
        }
        // The widgets: what to load, and the loaded items (see widgets.rs).
        if uri.contains("/winfeed/widgets/need") {
            return args.SetResponse(&respond(widgets::need().as_bytes(), "application/json", 200)?);
        }
        if let Some(q) = uri.split_once("/winfeed/widgets/img?").map(|(_, q)| q) {
            widgets::put_image(&http::query(q, "k").next().unwrap_or_default(), &http::query(q, "d").next().unwrap_or_default());
            return args.SetResponse(&respond(b"ok", "text/plain", 200)?);
        }
        if let Some(q) = uri.split_once("/winfeed/widgets/push?").map(|(_, q)| q) {
            widgets::push(&http::query(q, "d").next().unwrap_or_default());
            return args.SetResponse(&respond(b"ok", "text/plain", 200)?);
        }
        // /winfeed/update?do=check|install[&auto=0|1]: the About tab (see update.rs).
        if let Some(q) = uri.split_once("/winfeed/update").map(|(_, q)| q) {
            let q = q.trim_start_matches('?');
            if let Some(a) = http::query(q, "auto").next() { update::set_auto(a == "1"); }
            match http::query(q, "do").next().as_deref() {
                Some("check") => update::check(false),
                Some("install") => { std::thread::spawn(|| update::check(true)); }
                _ => {}
            }
            return args.SetResponse(&respond(update::status().as_bytes(), "application/json", 200)?);
        }
        // /winfeed/open?u=<https url>: the default browser (the board's own web view would keep it inside).
        if let Some(q) = uri.split_once("/winfeed/open?").map(|(_, q)| q) {
            let url = http::query(q, "u").next().unwrap_or_default();
            if !url.starts_with("https://") {
                return args.SetResponse(&respond(b"https only", "text/plain", 400)?);
            }
            let w = |s: &str| s.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
            // 1 = SW_SHOWNORMAL; above 32 = success
            let r = unsafe { ShellExecuteW(core::ptr::null_mut(), w("open").as_ptr(), w(&url).as_ptr(), core::ptr::null(), core::ptr::null(), 1) };
            return args.SetResponse(&respond(b"", "text/plain", if r as usize > 32 { 200 } else { 500 })?);
        }
        // /winfeed/notify?via=toast|board|both&id=&title=&body=&url=&img=&icon=
        if let Some(q) = uri.split_once("/winfeed/notify?").map(|(_, q)| q) {
            let p = |k: &str| http::query(q, k).next().unwrap_or_default();
            let via = p("via");
            let mut errs = Vec::new();
            if via != "board" {
                if let Err(e) = notify::toast(&p("title"), &p("body"), &p("url"), &p("img"), &p("icon")) { errs.push(format!("toast {e}")); }
            }
            if via == "board" || via == "both" {
                if let Err(e) = notify::announce(&p("id"), &p("title"), &p("body")) { errs.push(format!("board {e}")); }
            }
            if !errs.is_empty() { log(&format!("notify: {}", errs.join("; "))); }
            return args.SetResponse(&respond(errs.join("; ").as_bytes(), "text/plain; charset=utf-8", if errs.is_empty() { 200 } else { 500 })?);
        }
        if let Some(q) = uri.split_once("/winfeed/copyfile?").map(|(_, q)| q) {
            let url = http::query(q, "u").next().unwrap_or_default();
            return match clip::copy_video(&url) {
                Ok(()) => args.SetResponse(&respond(b"ok", "text/plain", 200)?),
                Err(e) => args.SetResponse(&respond(e.as_bytes(), "text/plain; charset=utf-8", 500)?),
            };
        }
        if let Some(q) = uri.split_once("/winfeed/proxy?").map(|(_, q)| q) {
            let url = http::query(q, "u").next().unwrap_or_default();
            let method = http::query(q, "m").next().unwrap_or_else(|| "GET".into());
            let body = http::query(q, "b").next().unwrap_or_default();
            let mut headers: String = http::query(q, "h").map(|h| h + "\r\n").collect();
            let host = url.trim_start_matches("https://").split('/').next().unwrap_or("");
            if let Some(c) = login::cookie_for(host).or_else(|| login::guest_cookie(host)) {
                headers += &format!("Cookie: {c}\r\n");
            }
            if let Some(t) = http::query(q, "t").next() {
                headers += &format!("Content-Type: {t}\r\n");
            }
            if !matches!(method.as_str(), "GET" | "POST") {
                return args.SetResponse(&respond(b"method", "text/plain", 405)?);
            }
            let reply = http::fetch(&method, &url, &headers, body.as_bytes());
            if let (Some(r), None) = (&reply, login::cookie_for(host)) {
                login::keep_guest(host, &r.set_cookies);
            }
            if method == "POST" {
                // Reactions and the like: log where it went and what came back (not the cookies, not the body sent).
                let back = reply.as_ref().map(|r| format!("{} {}", r.status, String::from_utf8_lossy(&r.body[..r.body.len().min(200)]))).unwrap_or_else(|| "no reply".into());
                log(&format!("POST {} -> {back}", url.split('?').next().unwrap_or("")));
            }
            return match reply {
                Some(r) => args.SetResponse(&respond(&r.body, &r.content_type, r.status as i32)?),
                None => args.SetResponse(&respond(b"fetch failed", "text/plain", 502)?),
            };
        }
        log(&format!("resource {uri}"));
        // ponytail: one page for every other path under /winfeed/; split assets out only if the page grows past one file.
        args.SetResponse(&response(PAGE.as_bytes(), "text/html; charset=utf-8")?)
    }
}

#[interface("00000001-0000-0000-C000-000000000046")]
unsafe trait IClassFactory: IUnknown {
    fn CreateInstance(&self, outer: *mut c_void, iid: *const GUID, out: *mut *mut c_void) -> HRESULT;
    fn LockServer(&self, lock: BOOL) -> HRESULT;
}

#[implement(IClassFactory)]
struct Factory {
    widgets: bool,
}

impl IClassFactory_Impl for Factory_Impl {
    unsafe fn CreateInstance(&self, outer: *mut c_void, iid: *const GUID, out: *mut *mut c_void) -> HRESULT {
        unsafe {
            *out = core::ptr::null_mut();
            if !outer.is_null() {
                return HRESULT(0x80040110u32 as i32); // CLASS_E_NOAGGREGATION
            }
            log(if self.widgets { "CreateInstance widgets" } else { "CreateInstance" });
            if self.widgets {
                let p: IWidgetProvider = widgets::Provider::new().into();
                return p.query(iid, out);
            }
            let p: IFeedProvider = Provider.into();
            p.query(iid, out)
        }
    }
    unsafe fn LockServer(&self, _lock: BOOL) -> HRESULT {
        HRESULT(0)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--cards") {
        return widgets::dump(args.get(i + 1).map(String::as_str).unwrap_or("cards.json"));
    }
    unsafe {
        CoInitializeEx(core::ptr::null(), 0).ok().ok(); // COINIT_MULTITHREADED
        log("process start");
        // 4 = CLSCTX_LOCAL_SERVER, 1 = REGCLS_MULTIPLEUSE. One process serves the feed and the widgets.
        let guid = |s: &str| GUID::from_u128(u128::from_str_radix(&s.replace('-', ""), 16).unwrap_or(0));
        let feed: IUnknown = Factory { widgets: false }.into();
        let wid: IUnknown = Factory { widgets: true }.into();
        let (mut c1, mut c2) = (0u32, 0u32);
        let ok1 = CoRegisterClassObject(&guid(CLSID), feed.as_raw(), 4, 1, &mut c1).is_ok();
        let ok2 = CoRegisterClassObject(&guid(WCLSID), wid.as_raw(), 4, 1, &mut c2).is_ok();
        if !ok1 && !ok2 {
            return;
        }
        update::start();
        // ponytail: never signalled; the board starts us on demand and Windows ends us with the session.
        let ev = CreateEventW(core::ptr::null(), BOOL(1), BOOL(0), core::ptr::null());
        let mut idx = 0u32;
        let _ = CoWaitForMultipleObjects(0, u32::MAX, 1, &ev, &mut idx);
    }
}
