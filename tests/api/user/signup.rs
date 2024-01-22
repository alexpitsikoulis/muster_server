use crate::utils::{
    app::TestApp,
    http_client::{ContentType, Header, Path},
};
use muttr_server::{
    domain::user::Password, handlers::user::SIGNUP_PATH, storage::USERS_TABLE_NAME,
};
use secrecy::Secret;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

#[tokio::test]
async fn test_signup_success() {
    let app = TestApp::spawn().await;

    Mock::given(path("/send"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let body =
        "handle=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness";
    let response = app
        .client
        .request(
            Path::POST(SIGNUP_PATH),
            &[Header::ContentType(ContentType::FormURLEncoded)],
            Some(body),
        )
        .await;

    assert_eq!(
        200,
        response.status().as_u16(),
        "The API did not return 200 when signing up with valid login details: {:?}",
        response.text().await.unwrap(),
    );

    match sqlx::query!(
        "SELECT handle, email, password FROM users WHERE email = 'alex.pitsikoulis@youwish.com'",
    )
    .fetch_one(&app.database.db_pool)
    .await
    {
        Ok(user) => {
            assert_eq!(
                "alex.pitsikoulis", user.handle,
                "Inserted handle does not match the one provided in the request",
            );
            assert_eq!(
                "alex.pitsikoulis@youwish.com", user.email,
                "Inserted email does not match the one provided in the request",
            );
            assert!(
                Password::compare(Secret::new("N0neofyourbus!ness".into()), user.password),
                "Inserted password does not match the one provided in the request",
            );
        }
        Err(e) => {
            panic!("DB query failed: {}", e);
        }
    };
}

#[tokio::test]
async fn test_signup_failed_400() {
    let mut app = TestApp::spawn().await;
    let test_cases = vec![
        ("handle=alex.pitsikoulis0&password=N0neofyourbus!ness", "missing the email"),
        ("email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "the name"),
        ("email=alex.pitsikoulis%40youwish.com&handle=alex.pitsikoulis", "missing the password"),
        ("handle=alex.pitsikoulis", "missing the email and password"),
        ("email=alex.pitsikoulis%40youwish.com", "missing the name and password"),
        ("password=N0neofyourbus!ness", "missing the name and email"),
        ("", "missing the name, email, and password"),
        ("handle=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=c0!", "password too short"),
        ("handle=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!", "password too long"),
        ("handle=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=passw0rd!", "password missing uppercase letter"),
        ("handle=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=Password!", "password missing number"),
        ("handle=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=Passw0rd", "password missing special character"),
        ("handle=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=PASSW0RD!", "password missing lowercase letter"),
        ("handle=&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "handle is empty"),
        ("handle=   &email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "handle is whitespace"),
        ("handle=alex.pitsikoulis.alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "handle is too long"),
        ("handle=alex.pitsikoulis/&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "handle contains forbidden character '/'"),
        ("handle=alex.pitsikoulis(&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "handle contains forbidden character '('"),
        ("handle=alex.pitsikoulis)&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "handle contains forbidden character ')'"),
        ("handle=alex.pitsikoulis\"&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "handle contains forbidden character '\"'"),
        ("handle=alex.pitsikoulis<&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "handle contains forbidden character '<'"),
        ("handle=alex.pitsikoulis>&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "handle contains forbidden character '>'"),
        ("handle=alex.pitsikoulis\\&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "handle contains forbidden character '\\'"),
        ("handle=alex.pitsikoulis{&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "handle contains forbidden character '{'"),
        ("handle=alex.pitsikoulis}&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "handle contains forbidden character '}'"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app
            .client
            .request(
                Path::POST(SIGNUP_PATH),
                &[Header::ContentType(ContentType::FormURLEncoded)],
                Some(invalid_body),
            )
            .await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message,
        );
        app.database.clear(USERS_TABLE_NAME).await;
    }
}
