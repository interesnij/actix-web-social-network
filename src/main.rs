#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod routes;

#[macro_use]
mod utils;
#[macro_use]
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::middleware::cors::Cors;
    use actix_files::Files;
    use crate::routes::routes;
    use actix_redis::RedisSession;
    use actix_session::Session;
    use actix_web::{middleware, web, App, HttpServer, http::header};

    HttpServer::new(|| {
        let static_files = Files::new("/static", "static/").show_files_listing();
        let media_files = Files::new("/media", "media/").show_files_listing();
        //let cors = Cors::default();
        let private_key = actix_web::cookie::Key::generate();

        App::new()
            .configure(|app| Cors::for_app(app) // <- Construct CORS middleware builder
                .allowed_origin("151.248.120.138:9015")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600)
                .register()
            )
            .wrap(RedisSession::new("127.0.0.1:6379", private_key.master()))
            .service(static_files)
            .service(media_files)
            .configure(routes)
    })
    .bind("151.248.120.138:9015")?
    .run()
    .await
}
