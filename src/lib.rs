//! a simple library for logging into USTC CAS System.
//!
//! # Usage
//! All you should do is call `ustc_cas::get_ticket`. The function param `service_url` can
//! be found from browser's address bar when logging into CAS by hand. The returned ticket value
//! can be used for further authentication specific to websites.
//!
//! # example
//! ```rust
//! use std::error::Error;
//! use tokio::runtime::Builder;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let runtime = Builder::new_current_thread()
//!         .enable_io()
//!         .enable_time()
//!         .build()?;
//!
//!     let result = runtime.block_on(ustc_cas::get_ticket(
//!         "PB00000000",
//!         "12345678",
//!         "https://jw.ustc.edu.cn/ucas-sso/login",
//!     ));
//!
//!     match result {
//!         Ok(s) => {
//!             println!("ticket: {s}");
//!         },
//!         Err(e) => {
//!             println!("Error: {e}");
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # features
//! - `native-tls`: Use system tls library. Enabled by default.
//! - `rustls-tls`: Use rustls instead of system tls library. Disable `default` feature before
//! enabling this feature.
//!

mod error;
mod validate_code;

pub use error::*;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::{redirect::Policy, Client};

///
/// log into USTC CAS System and get ticket value.
///
/// # Panic
///
/// The function may panic if the CAS interface changed. Panic is considered
/// a bug and needs to be fixed.
///
pub async fn get_ticket<U, P, S>(
    username: U,
    password: P,
    service_url: S,
) -> Result<String, CasError>
where
    U: AsRef<str>,
    P: AsRef<str>,
    S: AsRef<str>,
{
    const URL: &str = "https://passport.ustc.edu.cn/login";
    const IMAGE_URL: &str = "https://passport.ustc.edu.cn/validatecode.jsp?type=login";
    const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
            (KHTML, like Gecko) Chrome/103.0.5060.134 Safari/537.36 Edg/103.0.1264.77";

    static TICKET_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"ticket=(\S*)"#).unwrap());
    static CLIENT: Lazy<Client> = Lazy::new(|| {
        Client::builder()
            .user_agent(USER_AGENT)
            .cookie_store(true)
            .redirect(Policy::none())
            .build()
            .unwrap()
    });

    let username = username.as_ref();
    let password = password.as_ref();
    let service_url = service_url.as_ref();

    let rsps = CLIENT
        .get(format!("{URL}?service={service_url}"))
        .send()
        .await?
        .error_for_status()
        .unwrap();

    let text = rsps.text().await.unwrap();
    let mut form = get_form(text)?;
    form.insert("username".into(), username.into());
    form.insert("password".into(), password.into());
    if form["showCode"] == "1" {
        let rsps = CLIENT
            .get(IMAGE_URL)
            .send()
            .await?
            .error_for_status()
            .unwrap();
        let code = validate_code::get_validatecode(rsps.bytes().await.unwrap());
        form.insert("LT".into(), code);
    }
    form.insert("button".into(), "".into());

    let rsps = CLIENT
        .post(URL)
        .form(&form)
        .send()
        .await?
        .error_for_status()
        .unwrap();
    let ticket = &TICKET_RE
        .captures_iter(
            rsps.headers()
                .get("location")
                .ok_or(CasError::new(ErrorKind::UserInfoIncorrect))?
                .to_str()
                .unwrap(),
        )
        .next()
        .ok_or(CasError::new(ErrorKind::ServiceUrlIncorrect))?[1];
    Ok(ticket.into())
}

fn get_form(data: String) -> Result<HashMap<String, String>, CasError> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"<input type="hidden"[\s\S]*?name="(\S*?)" value="(\S*?)""#).unwrap()
    });

    let mut map = HashMap::new();
    for cap in RE.captures_iter(&data) {
        map.insert(cap[1].to_string(), cap[2].to_string());
    }
    if map.is_empty() {
        Err(CasError::new(ErrorKind::ServiceUrlIncorrect))
    } else {
        Ok(map)
    }
}
