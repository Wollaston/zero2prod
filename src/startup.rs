use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap connection in a smart pointer
    let db_pool = web::Data::new(connection_pool);

    // Capture 'connection' from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .route("health-check", web::get().to(crate::routes::health_check))
            .route("/subscriptions", web::post().to(crate::routes::subscribe))
            // Get a pointer copy and attach it to the app state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
