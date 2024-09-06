use actix_web::web::Data;
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::routes::{health, subscribe};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let db_pool = Data::new(connection_pool);

    // Here we choose to bind explicitly to localhost, 127.0.0.1, for security
    // reasons. This binding may cause issues in some environments. For example,
    // it causes connectivity issues running in WSL2, where you cannot reach the
    // server when it is bound to WSL2's localhost interface. As a workaround,
    // you can choose to bind to all interfaces, 0.0.0.0, instead, but be aware
    // of the security implications when you expose the server on all interfaces.
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .service(health)
            .service(subscribe)
    })
    .listen(listener)?
    .run()
    .await?;

    Ok(())
}
