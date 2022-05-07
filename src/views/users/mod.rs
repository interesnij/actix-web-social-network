pub mod profile;
pub mod templ;
pub mod user_settings;
use actix_web::web::ServiceConfig;

pub use self::{
    profile::*,
    templ::*,
    user_settings::*,
};

pub fn user_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(profile_routes)
    .configure(profile_settings_routes)
    ;
}
