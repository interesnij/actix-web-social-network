use actix_web::web;

use crate::views::{
    pages,
    auth,
    user_routes,
    community_routes,
    post_routes,
    chat_routes,
    docs_routes,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages::pages_routes)
    .configure(auth::auth_routes)
    .configure(user_routes)
    .configure(community_routes)
    .configure(post_routes)
    .configure(chat_routes)
    .configure(docs_routes)
    ;
}
