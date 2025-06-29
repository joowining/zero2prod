use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let configuration = get_configuration().expect("Failed to read configuration.");
    
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let connection = PgPool::connect(
        &configuration.database.connection_string()
    )
        .await
        .expect("Failed to connect to DB");
    
    let listener = TcpListener::bind(address)?;

    let server = run(listener, connection);
    server?.await
}
