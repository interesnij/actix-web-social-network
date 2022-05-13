#[macro_use]
extern crate diesel;
#[macro_use(concat_string)]
extern crate concat_string;

pub mod schema;
pub mod models;
pub mod routes;
mod errors;
mod vars;

#[macro_use]
mod utils;
#[macro_use]
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //use actix_cors::Cors;
    use actix_files::Files;
    use crate::routes::routes;
    use actix_redis::RedisSession;
    use actix_web::{App, HttpServer};
    use actix_session::CookieSession;

    HttpServer::new(|| {
        let static_files = Files::new("/static", "static/").show_files_listing();
        let media_files = Files::new("/media", "media/").show_files_listing();
        let private_key = actix_web::cookie::Key::generate();
        //let cors = Cors::default()
        //    .allowed_origin("151.248.120.138:9015")
        //    .allowed_methods(vec!["GET", "POST"])
        //    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        //    .allowed_header(header::CONTENT_TYPE)
        //    .max_age(3600);

        App::new()
            //.wrap(RedisSession::new("127.0.0.1:6379", private_key.master()))
            //.wrap(cors)
            .wrap (
                CookieSession::signed(&[0; 32])
                    .domain("http://134.0.112.253:9000")
                    .name("auth")
                    .path("/")
                    .secure(false)
            )
            .service(static_files)
            .service(media_files)
            .configure(routes)
    })
    .bind("134.0.112.253:9000")?
    .run()
    .await
}
