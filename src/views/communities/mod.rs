pub mod community_pages;
//pub mod community_settings;
use actix_web::web::ServiceConfig;

pub use self::{
    community_pages::*,
    //community_settings::*,
};

pub fn community_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(community_urls)
    //.configure(community_settings_urls)
    ;
}
