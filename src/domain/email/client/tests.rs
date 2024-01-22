#[cfg(test)]
mod tests {
    use crate::domain::{email::Client, user};
    use fake::{
        faker::{
            internet::en::SafeEmail,
            lorem::en::{Paragraph, Sentence},
        },
        Fake,
    };
    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };
    #[tokio::test]
    async fn send_email_sends_request_to_base_url() {
        let mock_server = MockServer::start().await;
        let sender = user::Email::try_from(SafeEmail().fake::<String>()).unwrap();
        let email_client = Client::new(mock_server.uri(), sender);

        Mock::given(header("Content-Type", "application/json"))
            .and(path("/send"))
            .and(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let email = user::Email::try_from(SafeEmail().fake::<String>()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let body: String = Paragraph(1..10).fake();

        let _ = email_client
            .send_email(email, subject, String::new(), body)
            .await;
    }
}
