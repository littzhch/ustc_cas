mod error;
mod validate_code;

use std::collections::HashMap;
pub use error::*;

use once_cell::sync::OnceCell;
use regex::Regex;
use reqwest::redirect::Policy;
use reqwest::Client;

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

    static TICKET_RE: OnceCell<Regex> = OnceCell::new();
    static CLIENT: OnceCell<Client> = OnceCell::new();

    let ticket_re = TICKET_RE.get_or_init(|| Regex::new(r#"ticket=(\S*)"#).unwrap());
    let client = CLIENT.get_or_init(|| {
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

    let rsps = client
        .get(format!("{}?service={}", URL, service_url))
        .send()
        .await?
        .error_for_status()
        .unwrap();

    let text = rsps.text().await.unwrap();



    todo!()
}


