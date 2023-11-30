mod header;
mod path;

pub use header::*;
pub use path::*;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    IntoUrl,
};

pub struct Client {
    client: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::new();
        Client { client, base_url }
    }

    pub async fn request<B, U>(
        &self,
        path: Path<U>,
        headers: &[Header],
        body: Option<B>,
    ) -> reqwest::Response
    where
        B: Into<reqwest::Body>,
        U: IntoUrl,
    {
        let mut request = path.builder(self).headers(Self::parse_headers(headers));
        if let Some(body) = body {
            request = request.body(body);
        };

        request
            .send()
            .await
            .expect(&format!("Failed to execute request {:?}", path))
    }

    fn parse_headers(headers: &[Header]) -> HeaderMap {
        let mut map = HeaderMap::new();
        for header in headers {
            let (name, value): (&str, String) = header.clone().into();
            map.insert(name, HeaderValue::from_str(&value).unwrap());
        }
        map
    }
}
