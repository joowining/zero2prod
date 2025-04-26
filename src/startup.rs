// startp.rs
use actix_web::{web, App, HttpServer };
use actix_web::dev::Server;
use std::net::TcpListener;

use crate::routes::*;
// `run`을 pub로 마크해주고 
// `run`은 바이너리 엔트리 포인트가 아니기에 proc-macro 없이 
// 단순 async로 마크
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(||{
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
