//! blocking version of `get_ticket`
use reqwest::blocking;
use super::*;


/// blocking version of `get_ticket`
///
/// # example
/// ```rust
/// use std::error::Error;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///
///     let result = ustc_cas::blocking::get_ticket(
///         "PB00000000",
///         "12345678",
///         "https://jw.ustc.edu.cn/ucas-sso/login",
///     );
///
///     match result {
///         Ok(s) => {
///             println!("ticket: {s}");
///         },
///         Err(e) => {
///             println!("Error: {e}");
///         }
///     }
///
///     Ok(())
/// }
/// ```
pub fn get_ticket<U, P, S>(
    username: U,
    password: P,
    service_url: S,
) -> Result<String, CasError>
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
    let mut form = get_form(text)?;
    form.insert("username".into(), username.into());
    form.insert("password".into(), password.into());

    #[cfg(feature = "validate-code")]
    if form["showCode"] == "1" {
        let rsps = CLIENT
            .get(IMAGE_URL)
            .send()?
            .error_for_status()
            .unwrap();
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