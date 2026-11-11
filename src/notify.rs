// Notifications for the page: a Windows toast and/or an announcement on the taskbar widgets button.
// The page decides what to show (its rules, its source adapters); this file only delivers.
use crate::bindings::*;
use windows_core::HSTRING;

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// A toast. Clicking it (or "פתיחה") opens `url` in the browser; `img` is a hero picture, `icon` the channel logo.
pub fn toast(title: &str, body: &str, url: &str, img: &str, icon: &str) -> windows_core::Result<()> {
    let https = |u: &str| u.starts_with("https://");
    let mut xml = format!(r#"<toast launch="{u}" activationType="protocol"><visual><binding template="ToastGeneric"><text hint-maxLines="2">{t}</text><text>{b}</text>"#,
        u = esc(url), t = esc(title), b = esc(body));
    if https(icon) { xml += &format!(r#"<image placement="appLogoOverride" hint-crop="circle" src="{}"/>"#, esc(icon)); }
    if https(img) { xml += &format!(r#"<image placement="hero" src="{}"/>"#, esc(img)); }
    xml += r#"</binding></visual>"#;
    if https(url) { xml += &format!(r#"<actions><action content="פתיחה" activationType="protocol" arguments="{}"/></actions>"#, esc(url)); }
    xml += "</toast>";
    let doc = XmlDocument::new()?;
    doc.LoadXml(&HSTRING::from(xml))?;
    ToastNotificationManager::CreateToastNotifier()?.Show(&ToastNotification::CreateToastNotification(&doc)?)
}

/// The announcement on the widgets button (Windows shows it while the board is closed).
pub fn announce(id: &str, title: &str, body: &str) -> windows_core::Result<()> {
    let icon = Uri::CreateUri(&HSTRING::from("ms-appx:///Images/logo.png"))?;
    let a = FeedAnnouncement::CreateInstance(&HSTRING::from(id), &HSTRING::from(title), &HSTRING::from(body), &icon, &icon)?;
    a.SetShowBadgeIfUserNotEngaged(true)?;
    a.SetPrimaryTextColor(AnnouncementTextColor::Accent)?;
    FeedManager::GetDefault()?.TryShowAnnouncement(&HSTRING::from("HagizraFeed"), &HSTRING::from("hagizra"), &a)
}
