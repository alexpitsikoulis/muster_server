use crate::utils::{
    app::TestApp,
    http_client::{ContentType, Header, Path},
};
use muttr_server::handlers::user::CONFIRM_PATH;

#[actix::test]
pub async fn test_confirm_success() {
    let mut app = TestApp::spawn().await;

    let user = app
        .database
        .insert_user("testuser@youwish.com", "test.user", false)
        .await;
    let confirmation_token = app.database.insert_confirmation_token(user.id()).await;

    let body: Option<String> = None;
    let response = app
        .client
        .request(
            Path::POST(format!("{}/{}", CONFIRM_PATH, confirmation_token.expose())),
            &[Header::ContentType(ContentType::Json)],
            body,
        )
        .await;

    assert_eq!(
        200,
        response.status().as_u16(),
        "The API did not return 200 when confirming valid subscription token: {}",
        response.text().await.unwrap(),
    );
}
