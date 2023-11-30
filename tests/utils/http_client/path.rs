use reqwest::{IntoUrl, RequestBuilder};

use super::Client;

pub enum Path<U>
where
    U: IntoUrl,
{
    GET(U),
    POST(U),
    PUT(U),
    PATCH(U),
    DELETE(U),
}

impl<U> std::fmt::Debug for Path<U>
where
    U: IntoUrl,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GET(url) => write!(f, "GET: {}", url.as_str()),
            Self::POST(url) => write!(f, "POST: {}", url.as_str()),
            Self::PUT(url) => write!(f, "PUT: {}", url.as_str()),
            Self::PATCH(url) => write!(f, "PATCH: {}", url.as_str()),
            Self::DELETE(url) => write!(f, "DELETE: {}", url.as_str()),
        }
    }
}

impl<U> Path<U>
where
    U: IntoUrl,
{
    pub fn builder(&self, client: &Client) -> RequestBuilder {
        match self {
            Path::GET(url) => client
                .client
                .get(format!("{}{}", client.base_url, url.as_str())),
            Path::POST(url) => client
                .client
                .post(format!("{}{}", client.base_url, url.as_str())),
            Path::PUT(url) => client
                .client
                .put(format!("{}{}", client.base_url, url.as_str())),
            Path::PATCH(url) => client
                .client
                .patch(format!("{}{}", client.base_url, url.as_str())),
            Path::DELETE(url) => {
                client
                    .client
                    .delete(format!("{}{}", client.base_url, url.as_str()))
            }
        }
    }
}
