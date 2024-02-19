use crate::utils::{app::TestApp, http_client::Path};
use claim::assert_some_eq;
use muttr_server::handlers::health_check::HEALTH_CHECK_PATH;

#[actix::test]
async fn test_health_check() {
    let app = TestApp::spawn().await;

    let body: Option<&'static str> = None;
    let response = app
        .client
        .request(Path::GET(HEALTH_CHECK_PATH), &[], body)
        .await;

    assert!(response.status().is_success());
    assert_some_eq!(response.content_length(), 0);
}
