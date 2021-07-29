use std::net::TcpListener;

fn start_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let server = api::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    // return application address to caller!
    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn test_health_check() {
    let address = start_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request. ");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}