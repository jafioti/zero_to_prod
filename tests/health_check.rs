#[tokio::test]
async fn health_check_works() {
    spawn_app();

    // Send health check request
    let response = reqwest::Client::new()
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Spawn the server as a background task
fn spawn_app() -> tokio::task::JoinHandle<Result<(), hyper::Error>> {
    tokio::spawn(zero_to_prod::run())
}
