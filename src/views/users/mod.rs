pub mod profile;
pub mod templ;
pub mod settings;
pub mod pages;
pub mod load;
pub mod progs;

use actix_web::web::ServiceConfig;

pub use self::{
    profile::*,
    templ::*,
    settings::*,
    pages::*,
    load::*,
    progs::*,
};

pub fn user_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(profile_urls)
    .configure(pages_urls)
    .configure(load_urls)
    .configure(settings_urls)
    .configure(progs_urls)
    ;
}
