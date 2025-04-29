//! tests/health_check.rs
//! test/health_check.rs

// tokio::test는 test용 tokio::main이 된다.
// #[test]를 지정할 수고를 덜어준다. 

// cargo expand --test health_check을 통해 
// 위의 매크로가 생성하는 코드를 확인할 수 있다. 
use reqwest::{self, Client};
use std::net::TcpListener;
use sqlx::{PgConnection, PgPool, Connection};
use zero2prod::configuration::get_configuration;

pub struct TestApp {
    pub address: String,
    pub connection: PgPool
}

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
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
            .expect("Failed to bind random port");
    
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}",port);

    let configuration = get_configuration.expect("Failed to read configuration");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    )
        .await
        .expect("Failed to connect to Postgres.");

    let server = zero2prod::startup::run(listener, connection_pool.clone()).expect("Faild to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        connection: connection_pool,
    }


    // 서버를 백그라운드로 구동
    // tokio::spawn은 생성된 퓨처에 대한 핸들을 반환한다.
    // 하지만 이 퓨처를 다루지 않으므로 일단 무시한다.
    
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arange
    let app = spawn_app().await;
    let configuration = get_configuration().expect("Failed to read configuration");
    // Connection 트레이트는 반드시 스코프 안에 있어야 PgConnection::connect를 호출할 수 있다. 
    // 구조체의 상속 메서드가 아니다. 

    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
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
