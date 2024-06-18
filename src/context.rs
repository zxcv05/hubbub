use anyhow::Result;
use http::{HeaderMap, HeaderValue, Method, StatusCode};
use reqwest::Url;
use serde_json::Value as JSON;

use crate::types::{guild::CachedGuild, user::{BotUser, User}};

static BASE_URL: &str = "https://discord.com/";


#[derive(Debug)]
pub struct ResumeInfo {
    pub url: String,
    pub id: String,
}


#[derive(Default, Debug)]
pub struct Cache {
    pub users: Vec<User>,
    pub guilds: Vec<CachedGuild>,
}


#[derive(Debug)]
pub struct Context {
    pub user: Option<BotUser>,
    pub resume_info: Option<ResumeInfo>,
    pub cache: Cache,
    client: reqwest::Client,
    auth: Option<String>,
}


impl Default for Context {
    fn default() -> Self {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(8);
        headers.insert("Referrer", HeaderValue::from_static("https://discord.com"));
        headers.insert("Sec-Ch-Ua", HeaderValue::from_static(r#""Not(A:Brand";v="24", "Chromium";v="122""#));
        headers.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
        headers.insert("Sec-Ch-Ua-Platform", HeaderValue::from_static("\"Linux\""));

        Self {
            user: None,
            resume_info: None,
            cache: Cache::default(),
            client: reqwest::Client::builder()
                .default_headers(headers)
                .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
                .build().expect("Couldn't build client"),
            auth: None,
        }
    }
}

#[derive(Debug)]
pub struct Response {
    pub status: StatusCode,
    pub content: JSON
}

impl Context {
    pub fn set_auth(&mut self, token: String) {
        self.auth = Some(token);
    }

    pub async fn request(&mut self, method: Method, endpoint: &str, body: Option<JSON>) -> Result<Response> {
        log::debug!(">> {} {}", method, endpoint);

        let builder = self.client.request(method, Url::parse(BASE_URL)?.join(format!("/api/{}", endpoint).as_str())?);

        let builder = match &self.auth {
            Some(a) => builder.header("Authorization", HeaderValue::from_str(a.as_str())?),
            None => builder,
        };

        let builder = match body {
            Some(b) => builder.body(serde_json::to_vec(&b)?).header("Content-Type", HeaderValue::from_str("application/json")?),
            None => builder,
        };

        let res = self.client.execute(builder.build()?).await?;

        let status = res.status();
        let text = res.text().await?;
        log::debug!("<< {}", status);
        log::trace!("{text}");

        Ok(Response {
            status,
            content: serde_json::from_str(text.as_str()).unwrap_or(JSON::Null)
        })
    }
}

