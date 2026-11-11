// Minimal blocking HTTPS client over WinHTTP, for the page's /winfeed/proxy.
// WinHTTP uses the Windows certificate store (so a filtering proxy's own CA just works), decompresses gzip,
// and keeps cookies per session: one process-wide session = one cookie jar for guest logins.
use core::ffi::c_void;
use std::sync::OnceLock;
use windows_core::BOOL;

type H = *mut c_void;
#[link(name = "winhttp")]
unsafe extern "system" {
    fn WinHttpOpen(agent: *const u16, access: u32, proxy: *const u16, bypass: *const u16, flags: u32) -> H;
    fn WinHttpConnect(s: H, host: *const u16, port: u16, reserved: u32) -> H;
    fn WinHttpOpenRequest(c: H, verb: *const u16, path: *const u16, version: *const u16, referrer: *const u16, accept: *const *const u16, flags: u32) -> H;
    fn WinHttpSetOption(h: H, option: u32, buf: *const c_void, len: u32) -> BOOL;
    fn WinHttpSetTimeouts(h: H, resolve: i32, connect: i32, send: i32, receive: i32) -> BOOL;
    fn WinHttpSendRequest(r: H, headers: *const u16, hlen: u32, body: *const c_void, blen: u32, total: u32, ctx: usize) -> BOOL;
    fn WinHttpReceiveResponse(r: H, reserved: *mut c_void) -> BOOL;
    fn WinHttpQueryHeaders(r: H, level: u32, name: *const u16, buf: *mut c_void, len: *mut u32, index: *mut u32) -> BOOL;
    fn WinHttpReadData(r: H, buf: *mut c_void, n: u32, read: *mut u32) -> BOOL;
    fn WinHttpCloseHandle(h: H) -> BOOL;
    fn WinHttpWebSocketCompleteUpgrade(r: H, ctx: usize) -> H;
    fn WinHttpWebSocketSend(ws: H, kind: u32, buf: *const c_void, len: u32) -> u32;
    fn WinHttpWebSocketReceive(ws: H, buf: *mut c_void, len: u32, read: *mut u32, kind: *mut u32) -> u32;
    fn WinHttpWebSocketClose(ws: H, status: u16, reason: *const c_void, len: u32) -> u32;
}

struct Handle(H);
impl Drop for Handle {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let _ = unsafe { WinHttpCloseHandle(self.0) };
        }
    }
}
struct Session(H);
unsafe impl Send for Session {}
unsafe impl Sync for Session {}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

fn session() -> H {
    static S: OnceLock<Session> = OnceLock::new();
    S.get_or_init(|| unsafe {
        // 4 = WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY
        let h = WinHttpOpen(wide("Mozilla/5.0 (Windows NT 10.0; Win64; x64) HagizraFeed").as_ptr(), 4, core::ptr::null(), core::ptr::null(), 0);
        let all: u32 = 3; // WINHTTP_DECOMPRESSION_FLAG_ALL
        let _ = WinHttpSetOption(h, 118, &all as *const u32 as _, 4); // WINHTTP_OPTION_DECOMPRESSION
        let _ = WinHttpSetTimeouts(h, 10_000, 10_000, 15_000, 20_000);
        Session(h)
    })
    .0
}

pub struct Reply {
    pub status: u32,
    pub content_type: String,
    pub body: Vec<u8>,
    pub set_cookies: Vec<String>,
}

/// One request. `headers` is "Name: value\r\n..." (may be empty). Only https.
pub fn fetch(method: &str, url: &str, headers: &str, body: &[u8]) -> Option<Reply> {
    let rest = url.strip_prefix("https://")?;
    let (hostport, path) = rest.split_at(rest.find('/').unwrap_or(rest.len()));
    let (host, port) = match hostport.split_once(':') {
        Some((h, p)) => (h, p.parse().ok()?),
        None => (hostport, 443u16),
    };
    let path = if path.is_empty() { "/" } else { path };
    unsafe {
        let c = Handle(WinHttpConnect(session(), wide(host).as_ptr(), port, 0));
        if c.0.is_null() {
            return None;
        }
        // 0x00800000 = WINHTTP_FLAG_SECURE
        let r = Handle(WinHttpOpenRequest(c.0, wide(method).as_ptr(), wide(path).as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(), 0x0080_0000));
        if r.0.is_null() {
            return None;
        }
        let h = wide(headers);
        let (hp, hl) = if headers.is_empty() { (core::ptr::null(), 0) } else { (h.as_ptr(), u32::MAX) }; // u32::MAX = null-terminated
        if !WinHttpSendRequest(r.0, hp, hl, body.as_ptr() as _, body.len() as u32, body.len() as u32, 0).as_bool()
            || !WinHttpReceiveResponse(r.0, core::ptr::null_mut()).as_bool()
        {
            return None;
        }
        let mut status = 0u32;
        let mut len = 4u32;
        // 19 = WINHTTP_QUERY_STATUS_CODE, 0x20000000 = WINHTTP_QUERY_FLAG_NUMBER
        let _ = WinHttpQueryHeaders(r.0, 19 | 0x2000_0000, core::ptr::null(), &mut status as *mut u32 as _, &mut len, core::ptr::null_mut());
        let mut ct = [0u16; 256];
        let mut len = (ct.len() * 2) as u32;
        let content_type = if WinHttpQueryHeaders(r.0, 1, core::ptr::null(), ct.as_mut_ptr() as _, &mut len, core::ptr::null_mut()).as_bool() {
            String::from_utf16_lossy(&ct[..len as usize / 2])
        } else {
            "application/octet-stream".into()
        };
        // 43 = WINHTTP_QUERY_SET_COOKIE; the index walks the repeated header.
        let mut set_cookies = Vec::new();
        let mut idx = 0u32;
        loop {
            let mut sc = [0u16; 1024];
            let mut len = (sc.len() * 2) as u32;
            let before = idx;
            if !WinHttpQueryHeaders(r.0, 43, core::ptr::null(), sc.as_mut_ptr() as _, &mut len, &mut idx).as_bool() || idx == before {
                break;
            }
            set_cookies.push(String::from_utf16_lossy(&sc[..len as usize / 2]));
        }
        let mut out = Vec::new();
        let mut buf = vec![0u8; 64 * 1024];
        loop {
            let mut n = 0u32;
            if !WinHttpReadData(r.0, buf.as_mut_ptr() as _, buf.len() as u32, &mut n).as_bool() || n == 0 {
                break;
            }
            out.extend_from_slice(&buf[..n as usize]);
            if out.len() > 8 * 1024 * 1024 {
                break; // ponytail: the proxy is for feed JSON/HTML, not media; media loads directly in the page.
            }
        }
        Some(Reply { status, content_type, body: out, set_cookies })
    }
}

/// Streams a GET to a file (no size cap, unlike fetch). Returns the status code.
pub fn download(url: &str, headers: &str, path: &std::path::Path) -> Option<u32> {
    use std::io::Write;
    let rest = url.strip_prefix("https://")?;
    let (host, p) = rest.split_at(rest.find('/').unwrap_or(rest.len()));
    let p = if p.is_empty() { "/" } else { p };
    unsafe {
        let c = Handle(WinHttpConnect(session(), wide(host).as_ptr(), 443, 0));
        if c.0.is_null() {
            return None;
        }
        let r = Handle(WinHttpOpenRequest(c.0, wide("GET").as_ptr(), wide(p).as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(), 0x0080_0000));
        let h = wide(headers);
        let (hp, hl) = if headers.is_empty() { (core::ptr::null(), 0) } else { (h.as_ptr(), u32::MAX) };
        if r.0.is_null() || !WinHttpSendRequest(r.0, hp, hl, core::ptr::null(), 0, 0, 0).as_bool() || !WinHttpReceiveResponse(r.0, core::ptr::null_mut()).as_bool() {
            return None;
        }
        let mut status = 0u32;
        let mut len = 4u32;
        let _ = WinHttpQueryHeaders(r.0, 19 | 0x2000_0000, core::ptr::null(), &mut status as *mut u32 as _, &mut len, core::ptr::null_mut());
        if status != 200 {
            return Some(status);
        }
        let mut file = std::fs::File::create(path).ok()?;
        let mut buf = vec![0u8; 256 * 1024];
        loop {
            let mut n = 0u32;
            if !WinHttpReadData(r.0, buf.as_mut_ptr() as _, buf.len() as u32, &mut n).as_bool() {
                return None;
            }
            if n == 0 {
                break;
            }
            file.write_all(&buf[..n as usize]).ok()?;
        }
        Some(status)
    }
}

/// Sends one text message over ws://127.0.0.1:<port><path> and returns the first text message back.
/// Used for the local Edge DevTools endpoint only.
pub fn ws_roundtrip(port: u16, path: &str, msg: &str) -> Option<String> {
    unsafe {
        let c = Handle(WinHttpConnect(session(), wide("127.0.0.1").as_ptr(), port, 0));
        if c.0.is_null() {
            return None;
        }
        let r = Handle(WinHttpOpenRequest(c.0, wide("GET").as_ptr(), wide(path).as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(), 0));
        if r.0.is_null() || !WinHttpSetOption(r.0, 114, core::ptr::null(), 0).as_bool() // WINHTTP_OPTION_UPGRADE_TO_WEB_SOCKET
            || !WinHttpSendRequest(r.0, core::ptr::null(), 0, core::ptr::null(), 0, 0, 0).as_bool()
            || !WinHttpReceiveResponse(r.0, core::ptr::null_mut()).as_bool()
        {
            return None;
        }
        let ws = Handle(WinHttpWebSocketCompleteUpgrade(r.0, 0));
        if ws.0.is_null() || WinHttpWebSocketSend(ws.0, 2, msg.as_ptr() as _, msg.len() as u32) != 0 {
            return None; // 2 = UTF8 message
        }
        let mut out = Vec::new();
        let mut buf = vec![0u8; 64 * 1024];
        loop {
            let (mut n, mut kind) = (0u32, 0u32);
            if WinHttpWebSocketReceive(ws.0, buf.as_mut_ptr() as _, buf.len() as u32, &mut n, &mut kind) != 0 {
                return None;
            }
            out.extend_from_slice(&buf[..n as usize]);
            match kind {
                3 => continue, // UTF8 fragment
                2 => break,    // UTF8 message complete
                _ => return None,
            }
        }
        let _ = WinHttpWebSocketClose(ws.0, 1000, core::ptr::null(), 0);
        Some(String::from_utf8_lossy(&out).into_owned())
    }
}

/// Percent-decoding for query values ('+' = space).
pub fn unescape(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'+' => out.push(b' '),
            b'%' if i + 2 < b.len() => match std::str::from_utf8(&b[i + 1..i + 3]).ok().and_then(|h| u8::from_str_radix(h, 16).ok()) {
                Some(v) => {
                    out.push(v);
                    i += 2;
                }
                None => out.push(b'%'),
            },
            c => out.push(c),
        }
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// All values of `key` in a query string, decoded.
pub fn query<'a>(q: &'a str, key: &'a str) -> impl Iterator<Item = String> + 'a {
    q.split('&').filter_map(move |kv| {
        let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
        (k == key).then(|| unescape(v))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decode() {
        assert_eq!(unescape("a%20b+c%2Fd%"), "a b c/d%");
        assert_eq!(unescape("%D7%94"), "ה");
        let q = "u=https%3A%2F%2Fx.y%2Fa%3Fb%3D1&h=A%3A%201&h=B%3A%202";
        assert_eq!(query(q, "u").next().unwrap(), "https://x.y/a?b=1");
        assert_eq!(query(q, "h").collect::<Vec<_>>(), ["A: 1", "B: 2"]);
    }
}
