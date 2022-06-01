pub mod pages;
pub mod progs;
pub mod auth;
pub mod users;
pub mod communities;
pub mod posts;
pub mod chats;
pub mod docs;
pub mod goods;
pub mod music;
pub mod photos;
pub mod survey;
pub mod video;
pub mod admin;
pub mod managers;

pub use self::{
    pages::*,
    auth::*,
    users::*,
    communities::*,
    posts::*,
    chats::*,
    docs::*,
    goods::*,
    music::*,
    photos::*,
    survey::*,
    video::*,
    progs::*,
    admin::*,
    managers::*,
};
