use actix_web::web;

use crate::views::{
    pages,
    manager_routes,
    progs,
    auth,
    user_routes,
    community_routes,
    post_routes,
    chat_routes,
    docs_routes,
    goods_routes,
    music_routes,
    photos_routes,
    survey_routes,
    video_routes,
    admin_routes,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(auth::auth_routes)
    .configure(chat_routes)
    .configure(user_routes)
    .configure(manager_routes)
    .configure(community_routes)
    .configure(post_routes)

    .configure(docs_routes)
    .configure(goods_routes)
    .configure(music_routes)
    .configure(photos_routes)
    .configure(survey_routes)
    .configure(video_routes)
    .configure(progs::progs_routes)
    .configure(admin_routes)
    .configure(pages::pages_routes)
    ;
}
