use reqwest::{
    IntoUrl, RequestBuilder,
    header::{HeaderMap, HeaderName, HeaderValue},
};

pub enum Path<U>
where U: IntoUrl {
    GET(U),
    POST(U),
    PUT(U),
    PATCH(U),
    DELETE(U),
}

impl<U> std::fmt::Debug for Path<U>
where U: IntoUrl {
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
    U: IntoUrl
{
    pub fn builder(&self, client: &Client) -> RequestBuilder {
        match self {
            Path::GET(url) => client.client.get(format!("{}{}", client.base_url, url.as_str())),
            Path::POST(url) => client.client.post(format!("{}{}", client.base_url, url.as_str())),
            Path::PUT(url) => client.client.put(format!("{}{}", client.base_url, url.as_str())),
            Path::PATCH(url) => client.client.patch(format!("{}{}", client.base_url, url.as_str())),
            Path::DELETE(url) => client.client.delete(format!("{}{}", client.base_url, url.as_str())),
        }
    }
}

pub enum Header {
    Authorization(&'static str),
    ContentType(ContentType),
}

impl Into<(&str, &str)> for Header {
    fn into(self) -> (&'static str, &'static str) {
        match self {
            Header::Authorization(token) => ("Authorization", token),
            Header::ContentType(content_type) => {
                let name = "Content-Type";
                match content_type {
                    ContentType::FormURLEncoded => (name, "application/x-www-form-urlencoded"),
                    ContentType::JSON => (name, "application/json"),
                }
            }
        }
    }
}

pub enum ContentType {
    FormURLEncoded,
    JSON,
}

pub struct Client{
    client: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::new();
        Client{ client, base_url }
    }

    pub async fn request<B, U>(
        &self,
        path: Path<U>,
        headers: &[(&'static str, &'static str)],
        body: Option<B>
    ) -> reqwest::Response
    where 
        B: Into<reqwest::Body>,
        U: IntoUrl
    {
        let mut request = path.builder(self)
            .headers(Self::parse_headers(headers));
        if let Some(body) = body {
            request = request.body(body);
        };

        request.send().await.expect(&format!("Failed to execute request {:?}", path))
    }

    fn parse_headers(headers: &[(&'static str, &'static str)]) -> HeaderMap {
        let mut map = HeaderMap::new();
        for (name, value) in headers {
            let header_value = HeaderValue::from_static(value);
            map.insert(*name, header_value);
        };
        map
    }
}