use actix_web::{
    HttpResponse,
    web,
    web::Json,
};
use crate::utils::{
    is_signed_in,
    get_request_user_data,
};
use actix_session::Session;
use serde::{Deserialize, Serialize};

use std::str;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::{borrow::BorrowMut, io::Write};
use crate::diesel::RunQueryDsl;

pub fn create_progs_urls(config: &mut web::ServiceConfig) {
    config.route("/admin/created/create_communities_category/", web::post().to(create_communities_category));
    config.route("/admin/created/create_communities_subcategory/{id}/", web::post().to(create_communities_subcategory));
    config.route("/admin/created/edit_communities_category/{id}/", web::post().to(edit_communities_category));
    config.route("/admin/created/edit_communities_subcategory/{id}/", web::post().to(edit_communities_subcategory));

    config.route("/admin/created/create_goods_category/", web::post().to(create_goods_category));
    config.route("/admin/created/create_goods_subcategory/", web::post().to(create_goods_subcategory));
    config.route("/admin/created/edit_goods_category/{id}/", web::post().to(edit_goods_category));
    config.route("/admin/created/edit_goods_subcategory/{id}/", web::post().to(edit_goods_subcategory));

    config.route("/admin/created/create_sound_genre/", web::post().to(create_sound_genre));
    config.route("/admin/created/edit_sound_genre/{id}/", web::post().to(edit_sound_genre));
    config.route("/admin/created/create_artist/", web::post().to(create_artist));
    config.route("/admin/created/edit_artist/{id}/", web::post().to(edit_artist));
    config.route("/admin/created/create_music_album/", web::post().to(create_music_album));
    config.route("/admin/created/edit_music_album/{id}/", web::post().to(edit_music_album));

    config.route("/admin/created/create_stickers_category/", web::post().to(create_stickers_category));
    config.route("/admin/created/edit_stickers_category/{id}/", web::post().to(edit_stickers_category));
    config.route("/admin/created/create_sticker/", web::post().to(create_sticker));
    config.route("/admin/created/edit_sticker/{id}/", web::post().to(edit_sticker));

    config.route("/admin/created/create_smiles_category/", web::post().to(create_smiles_category));
    config.route("/admin/created/edit_smiles_category/{id}/", web::post().to(edit_smiles_category));
    config.route("/admin/created/create_smile/", web::post().to(create_smile));
    config.route("/admin/created/edit_smile/{id}/", web::post().to(edit_smile));

    config.route("/admin/created/create_post_category/", web::post().to(create_post_category));
    config.route("/admin/created/edit_post_category/{id}/", web::post().to(edit_post_category));

    config.route("/admin/created/create_video_category/", web::post().to(create_video_category));
    config.route("/admin/created/edit_video_category/{id}/", web::post().to(edit_video_category));

    config.route("/admin/created/create_reaction/", web::post().to(create_reaction));
    config.route("/admin/created/edit_reaction/{id}/", web::post().to(edit_reaction));
}
