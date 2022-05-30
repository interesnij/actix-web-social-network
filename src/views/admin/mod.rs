pub mod chat_pages;
use actix_web::web::ServiceConfig;

pub use self::{
    create_pages::*,
    create_progs::*,
};

pub fn chat_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(create_pages_urls)
    .configure(create_progs_urls)
    ;
}
