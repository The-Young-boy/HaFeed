// "Copy the video": download it to a temp folder and put the file on the clipboard, the way File Explorer
// copies a file (paste works in WhatsApp desktop, mail, folders). Old copies are removed after a day.
use crate::{http, login};
use std::os::windows::process::CommandExt;
use std::time::{Duration, SystemTime};

pub fn copy_video(url: &str) -> Result<(), String> {
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
    let path = fetch(url, &dir, "video", "mp4", true)?;
    // ponytail: PowerShell's Set-Clipboard -Path builds the file-drop clipboard data; a native CF_HDROP needs an owner window.
    let lit = path.display().to_string().replace('\'', "''");
    let status = std::process::Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", &format!("Set-Clipboard -LiteralPath '{lit}'")])
        .creation_flags(0x0800_0000) // CREATE_NO_WINDOW
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() { Ok(()) } else { Err("ההעתקה ללוח נכשלה".into()) }
}

/// "Download": a picture, video or audio file into the user's Downloads folder; a taken name gets " (2)", " (3)"...
/// Returns the file name.
pub fn download(url: &str, kind: &str) -> Result<String, String> {
    // ponytail: %USERPROFILE%\Downloads; a Downloads folder moved elsewhere needs SHGetKnownFolderPath.
    let dir = std::path::PathBuf::from(std::env::var("USERPROFILE").map_err(|e| e.to_string())?).join("Downloads");
    let ext = match kind { "image" => "jpg", "audio" => "mp3", _ => "mp4" };
    let path = fetch(url, &dir, kind, ext, false)?;
    Ok(path.file_name().unwrap_or_default().to_string_lossy().into_owned())
}

/// Download url into dir. File name: the last path segment, made safe; default stem and extension.
/// overwrite = false: a free name is found instead.
fn fetch(url: &str, dir: &std::path::Path, stem: &str, ext: &str, overwrite: bool) -> Result<std::path::PathBuf, String> {
    if !url.starts_with("https://") {
        return Err("כתובת לא תקינה".into());
    }
    let last = url.split(['?', '#']).next().unwrap_or("").rsplit('/').next().unwrap_or("");
    let mut name: String = last.chars().filter(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_')).take(60).collect();
    if name.trim_matches('.').is_empty() {
        name = stem.into();
    }
    if !name.contains('.') {
        name = format!("{name}.{ext}");
    }
    let mut path = dir.join(&name);
    if !overwrite {
        let (base, dot) = name.rsplit_once('.').map(|(b, e)| (b.to_string(), format!(".{e}"))).unwrap_or((name.clone(), String::new()));
        let mut n = 2;
        while path.exists() {
            path = dir.join(format!("{base} ({n}){dot}"));
            n += 1;
        }
    }
    let host = url.trim_start_matches("https://").split('/').next().unwrap_or("");
    let headers = login::cookie_for(host).map(|c| format!("Cookie: {c}\r\n")).unwrap_or_default();
    match http::download(url, &headers, &path) {
        Some(200) => Ok(path),
        Some(s) => { let _ = std::fs::remove_file(&path); Err(format!("השרת החזיר {s}")) }
        None => { let _ = std::fs::remove_file(&path); Err("ההורדה נכשלה".into()) }
    }
}
