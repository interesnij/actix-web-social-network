pub mod create_pages;
pub mod create_progs;
use actix_web::web::ServiceConfig;

pub use self::{
    create_pages::*,
    create_progs::*,
};

pub fn admin_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(create_pages_urls)
    .configure(create_progs_urls)
    ;
}
