#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await.expect("Failed to spawn our app.");
    // Bring in 'reqwest'
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // Act
    let res = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

// Launch app in background
async fn spawn_app() -> std::io::Result<()> {
    todo!()
}
