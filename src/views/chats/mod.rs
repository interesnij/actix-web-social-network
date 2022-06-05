pub mod c_pages;
use actix_web::web::ServiceConfig;

pub use self::{
    c_pages::*,
};

pub fn chat_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(c_pages_urls)
    ;
}
