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
        let _files = Files::new("/static", "static/").show_files_listing();
        let _files2 = Files::new("/media", "media/").show_files_listing();
        App::new()
            .service(_files)
            .service(_files2)
            .configure(routes)
    })
    .bind("151.248.120.138:9015")?
    .run()
    .await
}
