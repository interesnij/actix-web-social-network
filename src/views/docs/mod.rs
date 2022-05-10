pub mod pages;

use actix_web::web::ServiceConfig;

pub use self::{
    pages::*,
};

pub fn user_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(pages)
    ;
}
