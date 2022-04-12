use actix_web::web;

use crate::views::{
    pages,
    auth,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages::pages_routes)
    .configure(auth::auth_routes)
    ;
}
