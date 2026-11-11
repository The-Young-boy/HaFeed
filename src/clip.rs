// "Copy the video": download it to a temp folder and put the file on the clipboard, the way File Explorer
// copies a file (paste works in WhatsApp desktop, mail, folders). Old copies are removed after a day.
use crate::{http, login};
use std::os::windows::process::CommandExt;
use std::time::{Duration, SystemTime};

pub fn copy_video(url: &str) -> Result<(), String> {
    if !url.starts_with("https://") {
        return Err("כתובת לא תקינה".into());
    }
    let dir = std::env::temp_dir().join("HagizraFeed-clips");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    if let Ok(entries) = std::fs::read_dir(&dir) {
        let day = SystemTime::now() - Duration::from_secs(86_400);
        for e in entries.flatten() {
            if e.metadata().and_then(|m| m.modified()).is_ok_and(|t| t < day) {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
    // File name: the last path segment, made safe; default .mp4
    let last = url.split(['?', '#']).next().unwrap_or("").rsplit('/').next().unwrap_or("video");
    let mut name: String = last.chars().filter(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_')).take(60).collect();
    if name.is_empty() {
        name = "video".into();
    }
    if !name.contains('.') {
        name.push_str(".mp4");
    }
    let path = dir.join(&name);
    let host = url.trim_start_matches("https://").split('/').next().unwrap_or("");
    let headers = login::cookie_for(host).map(|c| format!("Cookie: {c}\r\n")).unwrap_or_default();
    match http::download(url, &headers, &path) {
        Some(200) => {}
        Some(s) => return Err(format!("השרת החזיר {s}")),
        None => return Err("ההורדה נכשלה".into()),
    }
    // ponytail: PowerShell's Set-Clipboard -Path builds the file-drop clipboard data; a native CF_HDROP needs an owner window.
    let lit = path.display().to_string().replace('\'', "''");
    let status = std::process::Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", &format!("Set-Clipboard -LiteralPath '{lit}'")])
        .creation_flags(0x0800_0000) // CREATE_NO_WINDOW
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() { Ok(()) } else { Err("ההעתקה ללוח נכשלה".into()) }
}
