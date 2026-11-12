// Updates from the GitHub releases of The-Young-boy/HaFeed. Two minutes after start and then every 6 hours:
// read the latest release; when it is newer and updates are on, download HaFeed.msix and install it.
// The install runs from a one-time Task Scheduler task: a package cannot replace itself from its own
// processes (installing shuts them down). Windows accepts the new package only if it is signed with the
// certificate the installer trusted, so a foreign file cannot get in this way.
// Only the distribution build knows its version (HZ_VERSION, set by tools/build-dist.ps1); a dev build never updates.
use crate::{http, log};
use serde_json::{json, Value};
use std::sync::Mutex;

const VERSION: Option<&str> = option_env!("HZ_VERSION");
const API: &str = "https://api.github.com/repos/The-Young-boy/HaFeed/releases/latest";
const TASK: &str = "HaFeedUpdate";

struct State {
    latest: String,
    state: &'static str, // idle | checking | downloading | installing | failed | current
    note: String,
}
static ST: Mutex<State> = Mutex::new(State { latest: String::new(), state: "idle", note: String::new() });

fn dir() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var("LOCALAPPDATA").unwrap_or_default()).join("HagizraFeed")
}
/// Automatic updates: on unless the user switched them off (a marker file).
pub fn auto() -> bool { !dir().join("no-auto-update").exists() }
pub fn set_auto(on: bool) {
    let p = dir().join("no-auto-update");
    if on { let _ = std::fs::remove_file(p); } else { let _ = std::fs::create_dir_all(dir()); let _ = std::fs::write(p, b"1"); }
}

fn parse(v: &str) -> Vec<u32> { v.trim_start_matches('v').split('.').map(|x| x.parse().unwrap_or(0)).collect() }
fn newer(a: &str, b: &str) -> bool {
    let (mut x, mut y) = (parse(a), parse(b));
    let n = x.len().max(y.len());
    x.resize(n, 0); y.resize(n, 0);
    x > y
}

/// A dev build has no version built in: the installed package says which one it is.
fn installed() -> String {
    let m = std::env::current_exe().ok().and_then(|e| std::fs::read_to_string(e.with_file_name("AppxManifest.xml")).ok()).unwrap_or_default();
    m.split("Version=\"").nth(1).and_then(|v| v.split('"').next()).map(|v| v.strip_suffix(".0").unwrap_or(v).to_string()).unwrap_or_default()
}
pub fn status() -> String {
    let s = ST.lock().unwrap();
    json!({ "current": VERSION.map(str::to_string).unwrap_or_else(installed), "latest": s.latest, "state": s.state, "note": s.note, "auto": auto(), "dev": VERSION.is_none() }).to_string()
}
fn set(state: &'static str, note: &str) {
    let mut s = ST.lock().unwrap();
    s.state = state; s.note = note.to_string();
}

/// One round: look, and install when asked (or when updates are automatic).
/// `ui`: the user pressed "update" in About, so a small window shows the install; a background update is silent.
pub fn check(install: bool, ui: bool) {
    let Some(current) = VERSION else { return set("idle", "") };
    set("checking", "");
    let Some(r) = http::fetch("GET", API, "User-Agent: HaFeed\r\nAccept: application/vnd.github+json\r\n", b"") else { return set("failed", "אין חיבור ל-GitHub") };
    let Ok(rel) = serde_json::from_slice::<Value>(&r.body) else { return set("failed", "תשובה לא צפויה מ-GitHub") };
    let tag = rel["tag_name"].as_str().unwrap_or("").trim_start_matches('v').to_string();
    ST.lock().unwrap().latest = tag.clone();
    if tag.is_empty() || !newer(&tag, current) { return set("current", "") }
    let url = rel["assets"].as_array().into_iter().flatten()
        .find(|a| a["name"] == "HaFeed.msix").and_then(|a| a["browser_download_url"].as_str()).unwrap_or("").to_string();
    if url.is_empty() { return set("failed", "בגרסה החדשה אין חבילה") }
    if !install { return set("idle", "") }
    set("downloading", "");
    let _ = std::fs::create_dir_all(dir());
    let msix = dir().join("HaFeed-update.msix");
    if http::download(&url, "User-Agent: HaFeed\r\n", &msix) != Some(200) { return set("failed", "ההורדה נכשלה") }
    set("installing", "");
    let err_file = dir().join("update-error.txt");
    let _ = std::fs::remove_file(&err_file);
    // The task removes itself after the install; the board starts the new provider on its own.
    let ps1 = dir().join("update.ps1");
    let script = format!(
        "$ProgressPreference = 'SilentlyContinue'\r\ntry {{ Add-AppxPackage -Path '{m}' -ForceApplicationShutdown -ErrorAction Stop }} catch {{ $_ | Out-File '{log}' }}\r\nRemove-Item '{m}' -Force -ErrorAction SilentlyContinue\r\nschtasks.exe /delete /tn {TASK} /f | Out-Null\r\n",
        m = msix.display(), log = dir().join("update-error.txt").display());
    let script = if !ui { script } else {
        // The logo beside the script: the package folder is replaced during the install.
        let logo = dir().join("logo.png");
        if let Ok(exe) = std::env::current_exe() { let _ = std::fs::copy(exe.with_file_name("Images").join("logo.png"), &logo); }
        include_str!("update-window.ps1").replace("{MSIX}", &msix.display().to_string()).replace("{LOG}", &dir().join("update-error.txt").display().to_string())
            .replace("{LOGO}", &logo.display().to_string()).replace("{VER}", &tag).replace("{TASK}", TASK)
    };
    // With a BOM: Windows PowerShell reads a file without one as ANSI, and the window is in Hebrew.
    if std::fs::write(&ps1, format!("\u{FEFF}{script}")).is_err() { return set("failed", "לא ניתן לכתוב את קובץ העדכון") }
    // conhost --headless: no console window at all (-WindowStyle Hidden still shows one for a moment).
    let tr = format!("conhost.exe --headless powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File \"{}\"", ps1.display());
    let run = |args: &[&str]| {
        use std::os::windows::process::CommandExt;
        std::process::Command::new("schtasks.exe").args(args).creation_flags(0x0800_0000).status().map(|s| s.success()).unwrap_or(false)
    };
    // ponytail: /st 00:00 only satisfies schtasks' syntax; /run starts it now.
    if !run(&["/create", "/tn", TASK, "/sc", "once", "/st", "00:00", "/f", "/tr", &tr]) || !run(&["/run", "/tn", TASK]) {
        return set("failed", "לא ניתן להפעיל את ההתקנה");
    }
    log(&format!("update: installing {tag}"));
    // A good install shuts this process down. Still here = the install failed (its error is in the file) or never
    // started (an antivirus can stop the task): say so, so About does not stay on "updating" for ever.
    std::thread::spawn(move || {
        for _ in 0..90 {
            std::thread::sleep(std::time::Duration::from_secs(2));
            if let Ok(b) = std::fs::read(&err_file) {
                // Windows PowerShell's Out-File writes UTF-16.
                let e = if b.starts_with(&[0xFF, 0xFE]) { String::from_utf16_lossy(&b[2..].chunks(2).map(|c| u16::from_le_bytes([c[0], *c.get(1).unwrap_or(&0)])).collect::<Vec<_>>()) } else { String::from_utf8_lossy(&b).into_owned() };
                let first: String = e.trim_start_matches('\u{FEFF}').lines().find(|l| !l.trim().is_empty()).unwrap_or("").chars().take(200).collect();
                log(&format!("update: failed: {first}"));
                return set("failed", &format!("ההתקנה נכשלה: {first}"));
            }
        }
        log("update: the install did not finish in 3 minutes");
        set("failed", "ההתקנה לא הסתיימה. אפשר להוריד את המתקין מ-GitHub ולהריץ אותו.");
    });
}

/// The background clock.
pub fn start() {
    if VERSION.is_none() { return }
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(120));
        loop {
            check(auto(), false);
            std::thread::sleep(std::time::Duration::from_secs(6 * 3600));
        }
    });
}

#[cfg(test)]
mod tests {
    /// Against GitHub itself: the latest release reads, and its assets are where the updater looks.
    #[test]
    #[ignore]
    fn github() {
        let r = crate::http::fetch("GET", super::API, "User-Agent: HaFeed\r\nAccept: application/vnd.github+json\r\n", b"").unwrap();
        let rel: serde_json::Value = serde_json::from_slice(&r.body).unwrap();
        let tag = rel["tag_name"].as_str().unwrap();
        println!("latest {tag}, assets {:?}", rel["assets"].as_array().unwrap().iter().map(|a| a["name"].as_str().unwrap()).collect::<Vec<_>>());
        let url = rel["assets"].as_array().unwrap().iter().find(|a| a["name"] == "HaFeed-Setup.cmd").unwrap()["browser_download_url"].as_str().unwrap();
        let p = std::env::temp_dir().join("hafeed-dl-test");
        assert_eq!(crate::http::download(url, "User-Agent: HaFeed\r\n", &p), Some(200));
        assert!(std::fs::metadata(&p).unwrap().len() > 500_000);
        let _ = std::fs::remove_file(p);
    }
    #[test]
    fn versions() {
        assert!(super::newer("v0.3.70", "0.3.69"));
        assert!(super::newer("0.4", "0.3.99"));
        assert!(!super::newer("v0.3.69", "0.3.69"));
        assert!(!super::newer("0.3.9", "0.3.10"));
    }
}
