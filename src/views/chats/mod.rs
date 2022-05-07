pub mod chats;

pub use self::{
    chats::*,
};

pub fn chat_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(chats_pages)
    ;
}
