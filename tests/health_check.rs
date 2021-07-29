#[actix_rt::#[test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8080/health_check")
        .send()
        .await
        .expect("Failed to get health check.");

    // Assert
    assert!(response.status_code().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> std::io::Result<()> {
    api::run().await
}

fn spawn_app() {
    let server = api::run().await.expect("Failed to bind address.");

    let _ = tokio::spawn(server);
}