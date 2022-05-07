pub mod post_pages;
pub mod post_progs;
use actix_web::web::ServiceConfig;

pub use self::{
    post_pages::*,
    post_progs::*,
};

pub fn user_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(post_pages)
    .configure(post_progs)
    ;
}
