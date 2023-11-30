use muttr_server::consts::headers::{APP_JSON, AUTHORIZATION, CONTENT_TYPE, FORM_URL_ENCODED};

#[derive(Clone)]
pub enum Header {
    Authorization(String),
    ContentType(ContentType),
}

impl Into<(&'static str, String)> for Header {
    fn into(self) -> (&'static str, String) {
        match self {
            Header::Authorization(token) => (AUTHORIZATION, token),
            Header::ContentType(content_type) => (CONTENT_TYPE, content_type.to_string()),
        }
    }
}

#[derive(Clone)]
pub enum ContentType {
    FormURLEncoded,
    Json,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::FormURLEncoded => FORM_URL_ENCODED,
            ContentType::Json => APP_JSON,
        }
        .into()
    }
}
