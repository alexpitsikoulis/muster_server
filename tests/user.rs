mod utils;
use secrecy::Secret;
use utils::{spawn_app, clear_database};
use muttr_server::utils::compare_password_hash;
use muttr_server::storage::USERS_TABLE_NAME;


#[tokio::test]
async fn test_signup_success() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "handle=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness";
    let response = client
        .post(&format!("{}/signup", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    match sqlx::query!(
        "SELECT handle, email, password FROM users WHERE email = 'alex.pitsikoulis@youwish.com'",
    )
    .fetch_one(&app.db_pool)
    .await {
        Ok(user) => {
            assert_eq!("alex.pitsikoulis", user.handle);
            assert_eq!("alex.pitsikoulis@youwish.com", user.email);
            assert!(compare_password_hash(Secret::new("N0neofyourbus!ness".into()), user.password));
        },
        Err(e) => {
            panic!("DB query failed: {}", e);
        }
    };
}

#[tokio::test]
async fn test_signup_failed_400() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
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
        let response = client
            .post(&format!("{}/signup", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message,
        );
        clear_database(&app.db_pool, USERS_TABLE_NAME).await;
    }
}

#[tokio::test]
async fn test_login_success() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let mut body = "handle=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness";
    client
        .post(&format!("{}/signup", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    body = "email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness";
    let response = client
        .post(&format!("{}/login", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status());
    assert_eq!("true", response.headers().get("X-Login-Successful").expect("X-Login-Success header not present"));
}

#[tokio::test]
async fn test_login_failure() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let mut body = "handle=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness";
    client
        .post(&format!("{}/signup", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    body = "email=alex.pitsikoulis%40youwish.com&password=someotherpassword";
    let response = client
        .post(&format!("{}/login", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status());
    assert_eq!("false", response.headers().get("X-Login-Successful").expect("X-Login-Success header not present"));
}