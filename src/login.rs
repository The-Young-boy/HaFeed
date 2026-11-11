// Google sign-in for channels that require it. The channels only accept a Google code on their own
// /login page, so the session cookie always ends up in a browser. We open Microsoft Edge (present on
// every Windows) with a profile of our own and the DevTools port on, let the user sign in, read the
// channel's cookies over the DevTools protocol, check that the channel opens with them, and close Edge.
// Cookies are kept per host, encrypted with DPAPI (only this Windows user can read them).
use crate::http;
use core::ffi::c_void;
use std::collections::BTreeMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use windows_core::BOOL;

#[derive(Clone, PartialEq)]
pub enum State {
    Pending,
    Failed(String),
}

static JAR: Mutex<Option<BTreeMap<String, String>>> = Mutex::new(None); // host -> "a=1; b=2"
static RUNS: Mutex<BTreeMap<String, State>> = Mutex::new(BTreeMap::new());

fn dir() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var("LOCALAPPDATA").unwrap_or_default()).join("HagizraFeed")
}

// ---------- DPAPI ----------
#[repr(C)]
struct Blob {
    len: u32,
    data: *mut u8,
}
#[link(name = "crypt32")]
unsafe extern "system" {
    fn CryptProtectData(input: *const Blob, desc: *const u16, entropy: *const Blob, reserved: *const c_void, prompt: *const c_void, flags: u32, out: *mut Blob) -> BOOL;
    fn CryptUnprotectData(input: *const Blob, desc: *mut *mut u16, entropy: *const Blob, reserved: *const c_void, prompt: *const c_void, flags: u32, out: *mut Blob) -> BOOL;
}
#[link(name = "kernel32")]
unsafe extern "system" {
    fn LocalFree(p: *mut c_void) -> *mut c_void;
}
fn dpapi(data: &[u8], protect: bool) -> Option<Vec<u8>> {
    unsafe {
        let input = Blob { len: data.len() as u32, data: data.as_ptr() as *mut u8 };
        let mut out = Blob { len: 0, data: core::ptr::null_mut() };
        let ok = if protect {
            CryptProtectData(&input, core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), 1, &mut out)
        } else {
            CryptUnprotectData(&input, core::ptr::null_mut(), core::ptr::null(), core::ptr::null(), core::ptr::null(), 1, &mut out)
        };
        if !ok.as_bool() {
            return None;
        }
        let v = std::slice::from_raw_parts(out.data, out.len as usize).to_vec();
        LocalFree(out.data as _);
        Some(v)
    }
}

fn with_jar<R>(f: impl FnOnce(&mut BTreeMap<String, String>) -> R) -> R {
    let mut g = JAR.lock().unwrap();
    let jar = g.get_or_insert_with(|| {
        let raw = std::fs::read(dir().join("cookies.dat")).ok().and_then(|b| dpapi(&b, false)).unwrap_or_default();
        String::from_utf8_lossy(&raw).lines().filter_map(|l| l.split_once('\t')).map(|(h, c)| (h.to_string(), c.to_string())).collect()
    });
    f(jar)
}
fn save(jar: &BTreeMap<String, String>) {
    let text: String = jar.iter().map(|(h, c)| format!("{h}\t{c}\n")).collect();
    if let Some(enc) = dpapi(text.as_bytes(), true) {
        let _ = std::fs::create_dir_all(dir());
        let _ = std::fs::write(dir().join("cookies.dat"), enc);
    }
}

/// The Cookie header for a host we signed in to.
pub fn cookie_for(host: &str) -> Option<String> {
    with_jar(|j| j.get(host).cloned())
}
// Guest cookies, per host, kept like a browser keeps them: the sites know a guest by a cookie (hateder's
// `vid` must be a minute old before a reaction counts), and the board restarts us often, so they must
// outlive the process. Signed-in hosts use the DPAPI jar above instead. Not secrets: a plain file.
static GUEST: Mutex<Option<BTreeMap<String, BTreeMap<String, String>>>> = Mutex::new(None); // host -> name -> value
fn with_guest<R>(f: impl FnOnce(&mut BTreeMap<String, BTreeMap<String, String>>) -> R) -> R {
    let mut g = GUEST.lock().unwrap();
    let jar = g.get_or_insert_with(|| {
        let all = std::fs::read_to_string(dir().join("guest.txt")).unwrap_or_default();
        all.lines().filter_map(|l| l.split_once('\t')).map(|(h, c)| (h.to_string(), parse_pairs(c))).collect()
    });
    f(jar)
}
fn parse_pairs(c: &str) -> BTreeMap<String, String> {
    c.split("; ").filter_map(|p| p.split_once('=')).map(|(n, v)| (n.trim().to_string(), v.to_string())).collect()
}
pub fn guest_cookie(host: &str) -> Option<String> {
    with_guest(|j| j.get(host).filter(|m| !m.is_empty()).map(|m| m.iter().map(|(n, v)| format!("{n}={v}")).collect::<Vec<_>>().join("; ")))
}
/// Merge a reply's Set-Cookie headers into the host's guest cookies. A cookie set to expire is dropped.
pub fn keep_guest(host: &str, set_cookies: &[String]) {
    if set_cookies.is_empty() || host.is_empty() { return; }
    with_guest(|j| {
        let m = j.entry(host.to_string()).or_default();
        let before = m.clone();
        for sc in set_cookies {
            let Some((n, v)) = sc.split(';').next().and_then(|p| p.split_once('=')) else { continue };
            let gone = v.is_empty() || sc.to_ascii_lowercase().contains("max-age=0");
            if gone { m.remove(n.trim()); } else { m.insert(n.trim().to_string(), v.to_string()); }
        }
        if *m == before { return; }
        let text: String = j.iter().filter(|(_, m)| !m.is_empty()).map(|(h, m)| format!("{h}\t{}\n", m.iter().map(|(n, v)| format!("{n}={v}")).collect::<Vec<_>>().join("; "))).collect();
        let _ = std::fs::create_dir_all(dir());
        let _ = std::fs::write(dir().join("guest.txt"), text);
    });
}
pub fn forget(host: &str) {
    with_jar(|j| {
        j.remove(host);
        save(j);
    });
}

/// {"signed":[hosts],"pending":[hosts],"failed":{host:reason}}
pub fn status_json() -> String {
    let q = |s: &str| format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""));
    let signed: Vec<String> = with_jar(|j| j.keys().map(|h| q(h)).collect());
    let runs = RUNS.lock().unwrap();
    let pending: Vec<String> = runs.iter().filter(|(_, s)| **s == State::Pending).map(|(h, _)| q(h)).collect();
    let failed: Vec<String> = runs.iter().filter_map(|(h, s)| if let State::Failed(r) = s { Some(format!("{}:{}", q(h), q(r))) } else { None }).collect();
    format!("{{\"signed\":[{}],\"pending\":[{}],\"failed\":{{{}}}}}", signed.join(","), pending.join(","), failed.join(","))
}

/// Starts a sign-in for https://<host> in the background. The page polls status_json().
pub fn start(host: String) {
    {
        let mut runs = RUNS.lock().unwrap();
        if runs.get(&host) == Some(&State::Pending) {
            return;
        }
        runs.insert(host.clone(), State::Pending);
    }
    std::thread::spawn(move || {
        let result = run(&host);
        let mut runs = RUNS.lock().unwrap();
        match result {
            Ok(cookie) => {
                runs.remove(&host);
                drop(runs);
                with_jar(|j| {
                    j.insert(host.clone(), cookie);
                    save(j);
                });
                crate::log(&format!("signed in to {host}"));
            }
            Err(e) => {
                crate::log(&format!("sign-in to {host} failed: {e}"));
                runs.insert(host, State::Failed(e));
            }
        }
    });
}

fn edge_path() -> Option<std::path::PathBuf> {
    ["ProgramFiles(x86)", "ProgramFiles", "LOCALAPPDATA"]
        .iter()
        .filter_map(|v| std::env::var(v).ok())
        .map(|b| std::path::PathBuf::from(b).join(r"Microsoft\Edge\Application\msedge.exe"))
        .find(|p| p.exists())
}

fn run(host: &str) -> Result<String, String> {
    let edge = edge_path().ok_or("Microsoft Edge לא נמצא במחשב")?;
    let profile = dir().join("signin");
    // Step 1: a plain Edge window, no DevTools port. Google refuses to sign in when the port is open
    // ("this browser may not be secure"). The user signs in and closes the window.
    let mut win = std::process::Command::new(&edge)
        .arg(format!("--user-data-dir={}", profile.display()))
        .args(["--no-first-run", "--no-default-browser-check"])
        .arg(format!("--app=https://{host}/"))
        .spawn()
        .map_err(|e| format!("Edge לא נפתח: {e}"))?;
    let deadline = Instant::now() + Duration::from_secs(900);
    loop {
        if let Ok(Some(_)) = win.try_wait() {
            break;
        }
        if Instant::now() > deadline {
            let _ = win.kill();
            return Err("עבר הזמן להתחברות".into());
        }
        std::thread::sleep(Duration::from_millis(700));
    }
    // ponytail: wait for the profile's other Edge processes to let go of the cookie store.
    std::thread::sleep(Duration::from_millis(1500));
    read_cookies(&edge, &profile, host)
}

/// Step 2: the same profile, headless, with the DevTools port: read the channel's cookies and check them.
fn read_cookies(edge: &std::path::Path, profile: &std::path::Path, host: &str) -> Result<String, String> {
    let port_file = profile.join("DevToolsActivePort");
    let _ = std::fs::remove_file(&port_file);
    let mut child = std::process::Command::new(edge)
        .arg(format!("--user-data-dir={}", profile.display()))
        .args(["--headless=new", "--remote-debugging-port=0", "--no-first-run", "about:blank"])
        .spawn()
        .map_err(|e| format!("Edge לא נפתח: {e}"))?;
    let mut result = Err("לא נמצאה התחברות לערוץ. התחבר בחלון, חכה שהערוץ ייפתח, ורק אז סגור אותו.".to_string());
    let deadline = Instant::now() + Duration::from_secs(20);
    while Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(500));
        let Ok(s) = std::fs::read_to_string(&port_file) else { continue };
        let mut l = s.lines();
        let (Some(p), Some(path)) = (l.next(), l.next()) else { continue };
        let port = p.trim().parse::<u16>().unwrap_or(0);
        let Some(json) = cdp_call(port, path.trim(), r#"{"id":1,"method":"Storage.getCookies"}"#) else { continue };
        let cookie = cookies_for_host(&json, host);
        let ok = !cookie.is_empty() && signed_in(host, &cookie);
        if ok {
            result = Ok(cookie);
        }
        let _ = cdp_call(port, path.trim(), r#"{"id":2,"method":"Browser.close"}"#);
        break;
    }
    std::thread::sleep(Duration::from_millis(500));
    let _ = child.kill();
    result
}
/// Is this cookie a signed-in user? The channels' own "who am I" call: 404 for a guest on the chat
/// platform, authenticated:false on Authenti. (Reading /api/messages is no test: open channels answer guests.)
fn signed_in(host: &str, cookie: &str) -> bool {
    let h = format!("Cookie: {cookie}\r\n");
    if host.ends_with("newsupdates.click") {
        return http::fetch("GET", &format!("https://{host}/api/check_auth.php"), &h, b"")
            .is_some_and(|r| String::from_utf8_lossy(&r.body).contains("\"authenticated\":true"));
    }
    http::fetch("GET", &format!("https://{host}/api/user-info"), &h, b"").is_some_and(|r| r.status == 200)
}

/// Cookies from a CDP Storage.getCookies reply that apply to host, as a Cookie header value.
fn cookies_for_host(json: &str, host: &str) -> String {
    let mut out = Vec::new();
    for obj in json.split("},{") {
        let (Some(name), Some(value), Some(domain)) = (field(obj, "name"), field(obj, "value"), field(obj, "domain")) else { continue };
        let d = domain.trim_start_matches('.');
        if host == d || host.ends_with(&format!(".{d}")) {
            out.push(format!("{name}={value}"));
        }
    }
    out.join("; ")
}
fn field<'a>(obj: &'a str, key: &str) -> Option<&'a str> {
    let at = obj.find(&format!("\"{key}\":\""))? + key.len() + 4;
    let rest = &obj[at..];
    let mut end = 0;
    let b = rest.as_bytes();
    while end < b.len() && !(b[end] == b'"' && (end == 0 || b[end - 1] != b'\\')) {
        end += 1;
    }
    Some(&rest[..end])
}

/// One DevTools call over a local WebSocket; returns the first text message that answers it.
fn cdp_call(port: u16, path: &str, msg: &str) -> Option<String> {
    http::ws_roundtrip(port, path, msg)
}

#[cfg(test)]
mod tests {
    // Opens Edge for real: cargo test -- --ignored edge_flow
    #[test]
    #[ignore]
    fn edge_flow() {
        // Opens the window: close it without signing in. A guest must NOT count as signed in.
        let r = super::run("myshemesh.co.il");
        assert!(r.is_err(), "{r:?}");
    }
    #[test]
    fn picks_host_cookies() {
        let j = r#"{"id":1,"result":{"cookies":[{"name":"sid","value":"a\"b","domain":"ysiva.thechats.click","path":"/"},{"name":"g","value":"x","domain":".google.com"},{"name":"t","value":"1","domain":".thechats.click"}]}}"#;
        assert_eq!(super::cookies_for_host(j, "ysiva.thechats.click"), r#"sid=a\"b; t=1"#);
        assert_eq!(super::cookies_for_host(j, "other.com"), "");
    }
}
