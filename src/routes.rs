use actix_web::web;

use crate::views::{
    pages,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages::pages_routes)
    ;
}
