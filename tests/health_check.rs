//! tests/health_check.rs
//! test/health_check.rs

// tokio::test는 test용 tokio::main이 된다.
// #[test]를 지정할 수고를 덜어준다. 

// cargo expand --test health_check을 통해 
// 위의 매크로가 생성하는 코드를 확인할 수 있다. 
use reqwest::{self, Client};
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works(){
	// 준비
    let address = spawn_app();
	// reqwest 를 통해서 클라이언트를 생성하고 어플리케이션에 대한 HTTP요청 시도
	let client = reqwest::Client::new();
	
	let response = client
        .get(&format!("{}/health_check",&address))
		.send()
		.await
		.expect("Failed to execute request.");
		
	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}

// 백그라운드에서 어플리케이션을 실행
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
            .expect("Failed to bind random port");
    
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Faild to bind address");
    let _ = tokio::spawn(server);


    format!("http://127.0.0.1:{}",port)
    // 서버를 백그라운드로 구동
    // tokio::spawn은 생성된 퓨처에 대한 핸들을 반환한다.
    // 하지만 이 퓨처를 다루지 않으므로 일단 무시한다.
    
}

#[tokio::test]
async fn subscribe_returns_a_200_for_vald_form_data() {
    // Arange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}


#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing(){
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin","missing the email"),
        ("email=ursula_le_guin%40gmail.com","missing the name"),
        ("","missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions",&app_address))
            .header("Content-Type","application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}