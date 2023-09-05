//! provide blocking version of [`get_ticket`](super::get_ticket)
//!
//! Using this module requires enabling `blocking` feature.

use super::*;
use reqwest::blocking;

/// log into USTC CAS System and get ticket value. blocking version of
/// [`get_ticket`](super::get_ticket).
///
/// # Panics
///
/// The function will panic if `validate-code` feature is disabled but validate code recognition
/// is needed.
///
/// The function may panic if the CAS interface changed. This kind of panic is considered
/// a bug and needs to be fixed.
///
/// # Example
/// ```rust
///  let result = ustc_cas::blocking::get_ticket(
///     "PB00000000",
///     "12345678",
///     "https://jw.ustc.edu.cn/ucas-sso/login",
///  );
///
///  match result {
///     Ok(s) => {
///         println!("ticket: {s}");
///     },
///     Err(e) => {
///         println!("Error: {e}");
///     }
///  }
/// ```
pub fn get_ticket<U, P, S>(username: U, password: P, service_url: S) -> Result<String, CasError>
where
    U: AsRef<str>,
    P: AsRef<str>,
    S: AsRef<str>,
{
    static CLIENT: Lazy<blocking::Client> = Lazy::new(|| {
        blocking::Client::builder()
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
        .send()?
        .error_for_status()
        .unwrap();

    let text = rsps.text().unwrap();
    let cas_lt = get_cas_lt(&text)?.into();
    let mut form = get_form(text)?;
    form.insert("username".into(), username.into());
    form.insert("password".into(), password.into());
    form.insert("CAS_LT".into(), cas_lt);

    #[cfg(feature = "validate-code")]
    if form["showCode"] == "1" {
        let rsps = CLIENT.get(IMAGE_URL).send()?.error_for_status().unwrap();
        let code = validate_code::get_validatecode(rsps.bytes().unwrap());
        form.insert("LT".into(), code);
    }

    #[cfg(not(feature = "validate-code"))]
    if form["showCode"] == "1" {
        panic!("validate code needed but validate-code feature not enabled");
    }

    form.insert("button".into(), "".into());

    let rsps = CLIENT
        .post(URL)
        .form(&form)
        .send()?
        .error_for_status()
        .unwrap();

    match_ticket(rsps.headers())
}
