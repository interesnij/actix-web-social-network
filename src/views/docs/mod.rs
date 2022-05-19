pub mod pages;

use actix_web::web::ServiceConfig;

pub use self::{
    pages::*,
};

pub fn docs_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(pages_urls)
    ;
}
