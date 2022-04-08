#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod routes;

use actix_web::{
    HttpServer,
    App
};
use actix_files::Files;
use crate::routes::routes;

#[macro_use]
mod utils;
#[macro_use]
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let static_files = Files::new("/static", "static/").show_files_listing();
        let media_files = Files::new("/media", "media/").show_files_listing();
        App::new()
            .service(static_files)
            .service(media_files)
            .configure(routes)
    })
    .bind("151.248.120.138:9015")?
    .run()
    .await
}
