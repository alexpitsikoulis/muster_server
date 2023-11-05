use claim::assert_some_eq;
use crate::utils::TestApp;

#[tokio::test]
async fn test_health_check() {
    let app = TestApp::spawn().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get(&format!("{}/health-check", app.address))
        .send()
        .await
        .expect("Failed to execute request");
    
    assert!(response.status().is_success());
    assert_some_eq!(response.content_length(), 0);
}