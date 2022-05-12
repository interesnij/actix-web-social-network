pub mod pages;
pub mod photo_progs;

use actix_web::web::ServiceConfig;

pub use self::{
    pages::*,
    photo_progs::*,
};

pub fn photos_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(photos_urls)
    .configure(photo_progs)
    ;
}
