use claim::assert_some_eq;
use crate::utils::{
    app::TestApp,
    http_client::Path,
};

#[tokio::test]
async fn test_health_check() {
    let app = TestApp::spawn().await;
    
    let body: Option<&'static str> = None;
    let response = app.client.request(
        Path::GET("/health-check"),
        &[],
        body,
    ).await;
    
    assert!(response.status().is_success());
    assert_some_eq!(response.content_length(), 0);
}