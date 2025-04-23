//! tests/health_check.rs
//! test/health_check.rs

// tokio::test는 test용 tokio::main이 된다.
// #[test]를 지정할 수고를 덜어준다. 

// cargo expand --test health_check을 통해 
// 위의 매크로가 생성하는 코드를 확인할 수 있다. 
use reqwest;

#[tokio::test]
async fn health_check_works(){
	// 준비
	spawn_app().await.expect("Failed to spawn our app.");
	// reqwest 를 통해서 클라이언트를 생성하고 어플리케이션에 대한 HTTP요청 시도
	let client = reqwest::Client::new();
	
	let response = client
		.get("http://127.0.0.1:8000/health_check")
		.send()
		.await
		.expect("Failed to execute request.");
		
	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}

// 백그라운드에서 어플리케이션을 실행
async fn spawn_app() -> std::io::Result<()>{
	todo!()
}