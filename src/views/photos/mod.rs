pub mod pages;

use actix_web::web::ServiceConfig;

pub use self::{
    pages::*,
};

pub fn photos_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(photos_urls)
    ;
}
