pub mod profile;
pub mod templ;
pub mod user_settings;
pub mod pages;

use actix_web::web::ServiceConfig;

pub use self::{
    profile::*,
    templ::*,
    user_settings::*,
    pages::*,
};

pub fn user_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(profile_urls)
    .configure(user_pages_urls)
    .configure(profile_settings_urls)
    ;
}
