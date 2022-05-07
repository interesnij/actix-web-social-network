pub mod chat_pages;
use actix_web::web::ServiceConfig;

pub use self::{
    chat_pages::*,
};

pub fn chat_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(chats_pages)
    ;
}
