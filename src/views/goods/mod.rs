pub mod pages;

use actix_web::web::ServiceConfig;

pub use self::{
    pages::*,
};

pub fn goods_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(goods_urls)
    ;
}
