use actix_web::web;

use crate::views::{
    pages,
    global,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages::pages_routes)
    //.configure(global::global_routes)
    ;
}
