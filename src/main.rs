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
    use actix_cors::Cors;
    use actix_files::Files;
    use crate::routes::routes;
    use actix_session::CookieSession;
    use actix_web::{middleware, web, App, HttpServer, http::header};

    HttpServer::new(|| {
        let static_files = Files::new("/static", "static/").show_files_listing();
        let media_files = Files::new("/media", "media/").show_files_listing();
        App::new()
            //.wrap(
            //    CookieSession::signed(&[0; 32])
            //        .domain("151.248.120.138:9015")
            //        .name("actix_session")
            //        .path("/")
            //        .secure(true)
            //)
            .wrap(Cors::default())
            .service(static_files)
            .service(media_files)
            .configure(routes)
    })
    .bind("151.248.120.138:9015")?
    .run()
    .await
}
