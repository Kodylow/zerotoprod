use std::net::TcpListener;

// If we're testing we don't need to propogate errors
// if we fail to setup and execute properly we can panic and crash to find the error
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve OS assigned port
    let port = listener.local_addr().unwrap().port();

    let server = zerotoprod::run(listener).expect("Failed address binding");
    // Launch server as background task
    // tokio::spawn returns a handle to the spawned feature
    // don't need it though so drop with _
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    // Bring in 'reqwest'
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // Act
    let res = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_addr = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=kody%20test&email=kody%40gmail.com";
    let res = client
        .post(&format!("{}/subscriptions", &app_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, res.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_addr = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=kody%20test", "missing the email"),
        ("email=kody%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let res = client
            .post(&format!("{}/subscriptions", &app_addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            res.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
