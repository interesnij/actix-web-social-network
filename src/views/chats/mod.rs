pub mod chat_pages;
use actix_web::web::ServiceConfig;

pub use self::{
    pages::*,
};

pub fn chat_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(pages_urls)
    ;
}
