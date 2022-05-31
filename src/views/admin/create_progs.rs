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
    //config.route("/admin/created/create_communities_subcategory/{id}/", web::post().to(create_communities_subcategory));
    //config.route("/admin/created/edit_communities_category/{id}/", web::post().to(edit_communities_category));
    //config.route("/admin/created/edit_communities_subcategory/{id}/", web::post().to(edit_communities_subcategory));

    //config.route("/admin/created/create_goods_category/", web::post().to(create_goods_category));
    //config.route("/admin/created/create_goods_subcategory/", web::post().to(create_goods_subcategory));
    //config.route("/admin/created/edit_goods_category/{id}/", web::post().to(edit_goods_category));
    //config.route("/admin/created/edit_goods_subcategory/{id}/", web::post().to(edit_goods_subcategory));

    //config.route("/admin/created/create_sound_genre/", web::post().to(create_sound_genre));
    //config.route("/admin/created/edit_sound_genre/{id}/", web::post().to(edit_sound_genre));
    //config.route("/admin/created/create_artist/", web::post().to(create_artist));
    //config.route("/admin/created/edit_artist/{id}/", web::post().to(edit_artist));
    //config.route("/admin/created/create_music_album/", web::post().to(create_music_album));
    //config.route("/admin/created/edit_music_album/{id}/", web::post().to(edit_music_album));

    //config.route("/admin/created/create_stickers_category/", web::post().to(create_stickers_category));
    //config.route("/admin/created/edit_stickers_category/{id}/", web::post().to(edit_stickers_category));
    //config.route("/admin/created/create_sticker/", web::post().to(create_sticker));
    //config.route("/admin/created/edit_sticker/{id}/", web::post().to(edit_sticker));

    //config.route("/admin/created/create_smiles_category/", web::post().to(create_smiles_category));
    //config.route("/admin/created/edit_smiles_category/{id}/", web::post().to(edit_smiles_category));
    //config.route("/admin/created/create_smile/", web::post().to(create_smile));
    //config.route("/admin/created/edit_smile/{id}/", web::post().to(edit_smile));

    //config.route("/admin/created/create_post_category/", web::post().to(create_post_category));
    //config.route("/admin/created/edit_post_category/{id}/", web::post().to(edit_post_category));

    //config.route("/admin/created/create_video_category/", web::post().to(create_video_category));
    //config.route("/admin/created/edit_video_category/{id}/", web::post().to(edit_video_category));

    //config.route("/admin/created/create_reaction/", web::post().to(create_reaction));
    //config.route("/admin/created/edit_reaction/{id}/", web::post().to(edit_reaction));
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CategoryForm {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub category_id: Option<i32>,
    pub position: Option<i32>,
}

pub async fn category_form (
    payload: &mut Multipart,
    owner_path: String,
    owner_id: String
) -> CategoryForm {
    let mut form: CategoryForm = CategoryForm {
        name: "".to_string(),
        description: None,
        image: None,
        category_id: None,
        position: 0,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if field.name() == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            let file = UploadedFiles::new (
                owner_path.clone(),
                owner_id.to_string(),
                "admin_images".to_string(),
                _new_path.to_string(),
            );
            let file_path = file.path.clone();
            let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .expect("E");
            };
            form.image = Some(file.path.clone().replace("./","/"));
        }
        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string
                    } else if field.name() == "description" {
                        form.description = Some(data_string)
                    }
                    else if field.name() == "category_id" {
                        let _int: i32 = data_string.parse().unwrap();
                        form.category_id = Some(_int);
                    }
                    else if field.name() == "position" {
                        let _int: i16 = data_string.parse().unwrap();
                        form.position = _int;
                    }
                }
            }
        }
    }
    form
}

pub async fn create_communities_category(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            let form = category_form (
                payload.borrow_mut(),
                "communities_category".to_string(),
                _request_user.id.to_string()
            ).await;

            use crate::models::CommunityCategory;

            let new_list = CommunityCategory::create_category (
                form.name,
                form.image,
                form.position,
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
