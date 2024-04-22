use crate::utils::{
    app::TestApp,
    http_client::{ContentType, Header, Path},
};
use claim::assert_some;
use muttr_server::{
    domain::user::{Email, Handle, Password},
    handlers::user::{PatchUserRequestBody, BASE_PATH},
};
use secrecy::Secret;
use serde_json::to_string;

#[actix::test]
async fn test_patch_user_success() {
    let mut app = TestApp::spawn().await;

    let mut user = app
        .database
        .insert_user("testuser@youwish.com", "test.user", true)
        .await;

    let test_cases = [
        (
            PatchUserRequestBody {
                email: None,
                handle: None,
                password: None,
                name: Some(String::from("George")),
                profile_photo: None,
                bio: None,
            },
            "name is updated",
        ),
        (
            PatchUserRequestBody {
                email: Some(Email::try_from("guestemail@test.com").unwrap()),
                handle: None,
                password: None,
                name: None,
                profile_photo: None,
                bio: None,
            },
            "email is updated",
        ),
        (
            PatchUserRequestBody {
                email: None,
                handle: Some(Handle::try_from("new.handle").unwrap()),
                password: None,
                name: None,
                profile_photo: None,
                bio: None,
            },
            "handle is updated",
        ),
        (
            PatchUserRequestBody {
                email: None,
                handle: None,
                password: Some(
                    Password::try_from(Secret::new("Cr@zyn3wpassword!".into())).unwrap(),
                ),
                name: None,
                profile_photo: None,
                bio: None,
            },
            "password is updated",
        ),
        (
            PatchUserRequestBody {
                email: Some(Email::try_from("itsmyemail@test.com").unwrap()),
                handle: Some(Handle::try_from("newer.handle").unwrap()),
                password: Some(
                    Password::try_from(Secret::new("Cr@zyn3wpassword!222".into())).unwrap(),
                ),
                name: Some(String::from("Gus")),
                profile_photo: Some(String::from("base64")),
                bio: Some(String::from("long story")),
            },
            "all fields updated",
        ),
    ];

    for (body, error_case) in test_cases {
        let response = app
            .client
            .request(
                Path::PATCH(format!("{}/{}", BASE_PATH, user.id())),
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

        user = match app.database.get_user_by_id(user.id()).await {
            Ok(user) => user,
            Err(e) => panic!("failed to retrieve user {} from database: {}", user.id(), e),
        };

        check_field(body.email, user.email(), error_case);
        check_field(body.handle, user.handle(), error_case);
        check_field(body.password, user.password(), error_case);
        check_optional_field(body.name, user.name(), error_case);
        check_optional_field(body.profile_photo, user.profile_photo(), error_case);
        check_optional_field(body.bio, user.bio(), error_case);
    }
}

fn check_field<T>(left: Option<T>, right: T, error_case: &str)
where
    T: PartialEq + std::fmt::Debug,
{
    if let Some(left) = left {
        assert_eq!(
            left, right,
            "The user was not properly updated in the database when {}",
            error_case
        );
    }
}

fn check_optional_field<T>(left: Option<T>, right: Option<T>, error_case: &str)
where
    T: PartialEq + std::fmt::Debug,
{
    if left.is_some() {
        let right = assert_some!(
            right,
            "The user was not properly updated in the database when {}",
            error_case
        );
        check_field(left, right, error_case);
    }
}
