use claim::assert_ok;
use fake::{faker::internet::en::SafeEmail, Fake};
use muttr_server::{
    domain::user::{self, GetUserResponse},
    handlers::user::BASE_PATH,
};
use uuid::Uuid;

use crate::utils::{
    app::TestApp,
    http_client::{ContentType, Header, Path},
};

#[actix::test]
async fn test_get_by_id_success() {
    let mut app = TestApp::spawn().await;

    for x in 1..6 {
        let email = assert_ok!(
            user::Email::try_from(SafeEmail().fake::<String>()),
            "failed to generated valid user email"
        );
        let handle = format!("test.user{}", x);
        let user = app
            .database
            .insert_user(email.as_ref(), &handle, true)
            .await;

        let response = app
            .client
            .request(
                Path::GET(format!("{}/{}", BASE_PATH, user.id())),
                &[Header::ContentType(ContentType::Json)],
                None::<String>,
            )
            .await;

        assert_eq!(
            200,
            response.status(),
            "The API did not return 200 on valid get user by id request: {}",
            response.text().await.unwrap_or_default(),
        );

        let user_res = match response.json::<GetUserResponse>().await {
            Ok(json) => json,
            Err(e) => panic!(
                "failed to unmarshal json from api response into GetUserResponse struct: {:?}",
                e
            ),
        };

        assert_eq!(user.id(), user_res.id(), "id does not match",);
        assert_eq!(user.email(), user_res.email(), "email does not match",);
        assert_eq!(user.handle(), user_res.handle(), "handle does not match",);
        assert_eq!(user.name(), user_res.name(), "name does not match",);
        assert_eq!(
            user.profile_photo(),
            user_res.profile_photo(),
            "profile_photo does not match",
        );
        assert_eq!(user.bio(), user_res.bio(), "bio does not match",);
    }
}

#[actix::test]
async fn test_get_by_id_failure() {
    let app = TestApp::spawn().await;

    let id = Uuid::new_v4();

    let user_res = app
        .client
        .request(
            Path::GET(format!("{}/{}", BASE_PATH, id)),
            &[Header::ContentType(ContentType::Json)],
            None::<String>,
        )
        .await;

    assert_eq!(
        404,
        user_res.status(),
        "The API did not return 404 when trying to GET a non-existant user id",
    );
}
