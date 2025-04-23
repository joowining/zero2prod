use actix_web::{web, App, HttpResponse, HttpServer };

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// `run`을 pub로 마크해주고 
// `run`은 바이너리 엔트리 포인트가 아니기에 proc-macro 없이 
// 단순 async로 마크
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}