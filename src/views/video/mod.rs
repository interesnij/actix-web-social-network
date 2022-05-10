pub mod pages;

use actix_web::web::ServiceConfig;

pub use self::{
    pages::*,
};

pub fn video_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(videos_urls)
    ;
}
