use crate::utils::{
    app::TestApp,
    http_client::{ContentType, Header, Path},
};
use muttr_server::{
    domain::user::{Email, Handle, Password, User},
    handlers::user::BASE_PATH,
};
use secrecy::Secret;
use serde_json::to_string;

#[actix::test]
async fn test_update_user_success() {
    let mut app = TestApp::spawn().await;

    let mut user = app
        .database
        .insert_user("testuser@youwish.com", "test.user", true)
        .await;

    let test_cases = [
        (
            User::new(
                user.id(),
                user.email(),
                user.handle(),
                user.password(),
                user.name(),
                user.profile_photo(),
                user.bio(),
                user.failed_attempts(),
                user.email_confirmed(),
                user.created_at(),
                user.updated_at(),
                user.deleted_at(),
            ),
            "nothing was changed",
        ),
        (
            User::new(
                user.id(),
                user.email(),
                user.handle(),
                user.password(),
                Some(String::from("New Name")),
                user.profile_photo(),
                user.bio(),
                user.failed_attempts(),
                user.email_confirmed(),
                user.created_at(),
                user.updated_at(),
                user.deleted_at(),
            ),
            "name was changed",
        ),
        (
            User::new(
                user.id(),
                user.email(),
                user.handle(),
                user.password(),
                user.name(),
                Some(String::from("base64")),
                Some(String::from("new bio")),
                user.failed_attempts(),
                user.email_confirmed(),
                user.created_at(),
                user.updated_at(),
                user.deleted_at(),
            ),
            "multiple fields were changed",
        ),
        (
            User::new(
                user.id(),
                Email::try_from("new_email@test.com").unwrap(),
                Handle::try_from("cool_new_handle").unwrap(),
                Password::try_from(Secret::new(String::from("Newp@ssword1"))).unwrap(),
                user.name(),
                user.profile_photo(),
                user.bio(),
                user.failed_attempts(),
                user.email_confirmed(),
                user.created_at(),
                user.updated_at(),
                user.deleted_at(),
            ),
            "all validated fields were changed",
        ),
    ];

    for (body, error_case) in test_cases {
        let response = app
            .client
            .request(
                Path::PUT(format!("{}/{}", BASE_PATH, user.id())),
                &[Header::ContentType(ContentType::Json)],
                Some(to_string(&body).unwrap()),
            )
            .await;

        assert_eq!(
            200,
            response.status(),
            "The API did not return 200 on valid user update when {}: {}",
            error_case,
            response.text().await.unwrap_or_default(),
        );

        user = app.database.get_user_by_id(user.id()).await;

        assert_eq!(
            body, user,
            "The user was not properly updated in the database when {}",
            error_case,
        )
    }
}
