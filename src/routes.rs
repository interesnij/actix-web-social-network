use actix_web::web;

use crate::views::{
    pages,
    auth,
    profile, user_settings,
    post_pages, post_progs,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages::pages_routes)
    .configure(profile::user_routes)
    .configure(user_settings::user_settings_routes)

    .configure(post_pages::post_routes)
    .configure(post_progs::post_progs)

    .configure(auth::auth_routes)
    ;
}
