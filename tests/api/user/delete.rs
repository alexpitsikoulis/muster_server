use crate::utils::{
    app::TestApp,
    http_client::{ContentType, Header, Path},
};
use chrono::{DateTime, Days, Utc};
use claim::{assert_err, assert_none, assert_ok, assert_some};
use fake::{faker::internet::en::SafeEmail, Fake};
use muttr_server::{domain::user, handlers::user::BASE_PATH};
use uuid::Uuid;

#[actix::test]
async fn test_soft_delete_success() {
    let mut app = TestApp::spawn().await;

    let test_cases = [
        ("test.user", true, "the user's email is confirmed"),
        ("test.user2", false, "the user's email is not confirmed"),
    ];

    for (handle, is_confirmed, error_case) in test_cases {
        let email = assert_ok!(
            user::Email::try_from(SafeEmail().fake::<String>()),
            "failed to generate valid user email"
        );
        let mut user = app
            .database
            .insert_user(email.as_ref(), handle, is_confirmed)
            .await;

        assert_none!(
            user.deleted_at(),
            "User not initialized with None value for deleted_at"
        );

        let now = Utc::now();
        let response = app
            .client
            .request(
                Path::DELETE(format!("{}/{}", BASE_PATH, user.id())),
                &[Header::ContentType(ContentType::Json)],
                None::<String>,
            )
            .await;

        assert_eq!(
            200,
            response.status(),
            "The API did not return 200 on valid user soft delete when {}: {}",
            error_case,
            response.text().await.unwrap_or_default(),
        );

        user = match app.database.get_user_by_id(user.id()).await {
            Ok(user) => user,
            Err(e) => panic!("failed to retrieve user {} from database: {}", user.id(), e),
        };
        let deleted_at = assert_some!(user.deleted_at(), "User deleted_at is None");
        let day = Days::new(1);
        assert!(
            deleted_at > now.checked_sub_days(day).unwrap()
                && deleted_at < now.checked_add_days(day).unwrap(),
            "User's deleted_at time was incorrect",
        )
    }
}

#[actix::test]
async fn test_soft_delete_failure() {
    let mut app = TestApp::spawn().await;

    let id = Uuid::new_v4();
    let mut response = app
        .client
        .request(
            Path::DELETE(format!("{}/{}", BASE_PATH, id)),
            &[Header::ContentType(ContentType::Json)],
            None::<String>,
        )
        .await;

    assert_eq!(
        404,
        response.status(),
        "The API did not return 404 when trying to soft delete a non-existant user",
    );

    let user = app
        .database
        .insert_user("testuser@email.com", "test.user", true)
        .await;
    response = app
        .client
        .request(
            Path::DELETE(format!("{}/{}", BASE_PATH, user.id())),
            &[Header::ContentType(ContentType::Json)],
            None::<String>,
        )
        .await;
    assert_eq!(
        200,
        response.status(),
        "Unexpectedly failed to soft delete test user: {}",
        response.text().await.unwrap_or_default(),
    );
    response = app
        .client
        .request(
            Path::DELETE(format!("{}/{}", BASE_PATH, user.id())),
            &[Header::ContentType(ContentType::Json)],
            None::<String>,
        )
        .await;

    assert_eq!(
        400,
        response.status(),
        "The API did not return 400 when trying to soft delete a user that has already been soft deleted",
    );
    assert_eq!(
        format!("user {} has already been soft deleted", user.id()),
        response.text().await.unwrap_or_default(),
        "The response body did not match the expected text",
    );
}

#[actix::test]
async fn test_hard_delete_success() {
    let mut app = TestApp::spawn().await;

    let test_cases = [
        ("test.user", true, "the user's email is confirmed"),
        ("test.user2", false, "the user's email is not confirmed"),
    ];

    for (handle, is_confirmed, error_case) in test_cases {
        let email = assert_ok!(
            user::Email::try_from(SafeEmail().fake::<String>()),
            "failed to generate valid user email"
        );
        let user = app
            .database
            .insert_user(email.as_ref(), handle, is_confirmed)
            .await;

        let response = app
            .client
            .request(
                Path::DELETE(format!("{}/{}/hard", BASE_PATH, user.id())),
                &[Header::ContentType(ContentType::Json)],
                None::<String>,
            )
            .await;

        assert_eq!(
            200,
            response.status(),
            "The API did not return 200 on valid user hard delete when {}: {}",
            error_case,
            response.text().await.unwrap_or_default(),
        );

        assert_err!(
            app.database.get_user_by_id(user.id()).await,
            "User {} still exists in the database",
            user.id(),
        );
    }
}
