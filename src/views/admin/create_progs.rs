use crate::schema;
use actix_web::{
    HttpResponse,
    web,
    //web::Json,
};
use crate::utils::{
    is_signed_in,
    get_request_user_data,
    establish_connection,
    UploadedFiles,
};
use actix_session::Session;
use serde::{Deserialize, Serialize};

use std::str;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::{borrow::BorrowMut, io::Write};
use crate::diesel::{ExpressionMethods,RunQueryDsl, QueryDsl};


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

#[derive(Deserialize, Serialize, Debug)]
pub struct CategoryForm {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub category_id: Option<i32>,
    pub position: Option<i16>,
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
        position: None,
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
                        form.position = Some(_int);
                    }
                }
            }
        }
    }
    form
}

pub async fn create_communities_category(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            use crate::models::CommunityCategory;

            CommunityCategory::create_category (
                form.name,
                form.image,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_communities_category(session: Session, mut payload: Multipart, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::community_categorys::dsl::community_categorys;
            use crate::models::CommunityCategory;

            let _connection = establish_connection();
            let category = community_categorys
                .filter(schema::community_categorys::id.eq(*cat_id))
                .load::<CommunityCategory>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            category.edit_category (
                form.name,
                form.image,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn create_communities_subcategory(session: Session, mut payload: Multipart, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::community_categorys::dsl::community_categorys;
            use crate::models::CommunityCategory;

            let _connection = establish_connection();
            let category = community_categorys
                .filter(schema::community_categorys::id.eq(*cat_id))
                .load::<CommunityCategory>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            category.create_subcategory (
                form.name,
                form.image,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_communities_subcategory(session: Session, mut payload: Multipart, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::community_subcategorys::dsl::community_subcategorys;
            use crate::models::CommunitySubcategory;

            let _connection = establish_connection();
            let category = community_subcategorys
                .filter(schema::community_subcategorys::id.eq(*cat_id))
                .load::<CommunitySubcategory>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            category.edit_subcategory (
                form.name,
                form.category_id.unwrap(),
                form.image,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}


pub async fn create_goods_category(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            use crate::models::GoodCategorie;

            GoodCategorie::create_category (
                form.name,
                form.image,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_goods_category(session: Session, mut payload: Multipart, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::good_categories::dsl::good_categories;
            use crate::models::GoodCategorie;

            let _connection = establish_connection();
            let category = good_categories
                .filter(schema::good_categories::id.eq(*cat_id))
                .load::<GoodCategorie>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            category.edit_category (
                form.name,
                form.image,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn create_goods_subcategory(session: Session, mut payload: Multipart, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::good_categories::dsl::good_categories;
            use crate::models::GoodCategorie;

            let _connection = establish_connection();
            let category = good_categories
                .filter(schema::good_categories::id.eq(*cat_id))
                .load::<GoodCategorie>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            category.create_subcategory (
                form.name,
                form.image,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_goods_subcategory(session: Session, mut payload: Multipart, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::good_subcategories::dsl::good_subcategories;
            use crate::models::GoodSubcategorie;

            let _connection = establish_connection();
            let category = good_subcategories
                .filter(schema::good_subcategories::id.eq(*cat_id))
                .load::<GoodSubcategorie>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            category.edit_subcategory (
                form.name,
                form.category_id.unwrap(),
                form.image,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn create_sound_genre(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::SoundGenre;

        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            SoundGenre::create_genre (form.name);
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_sound_genre(session: Session, mut payload: Multipart, genre_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::sound_genres::dsl::sound_genres;
            use crate::models::SoundGenre;

            let _connection = establish_connection();
            let genre = sound_genres
                .filter(schema::sound_genres::id.eq(*genre_id))
                .load::<SoundGenre>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            genre.edit_genre(form.name);
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}


pub async fn create_artist(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::Artist;

        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            let form = category_form (
                payload.borrow_mut(),
                "music_artists".to_string(),
                _request_user.id.to_string()
            ).await;

            Artist::create_artist (
                form.name,
                form.description,
                form.image,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_artist(session: Session, mut payload: Multipart, artist_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::artists::dsl::artists;
            use crate::models::Artist;

            let _connection = establish_connection();
            let category = artists
                .filter(schema::artists::id.eq(*artist_id))
                .load::<Artist>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "music_artists".to_string(),
                _request_user.id.to_string()
            ).await;

            category.edit_artist (
                form.name,
                form.description,
                form.image,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn create_music_album(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::MusicAlbum;

        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            let form = category_form (
                payload.borrow_mut(),
                "music_albums".to_string(),
                _request_user.id.to_string()
            ).await;

            MusicAlbum::create_album (
                form.name,
                form.category_id,
                _request_user.id,
                form.description,
                form.image,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_music_album(session: Session, mut payload: Multipart, album_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::music_albums::dsl::music_albums;
            use crate::models::MusicAlbum;

            let _connection = establish_connection();
            let album = music_albums
                .filter(schema::music_albums::id.eq(*album_id))
                .load::<MusicAlbum>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "music_albums".to_string(),
                _request_user.id.to_string()
            ).await;

            album.edit_album (
                form.name,
                form.category_id,
                form.description,
                form.image,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn create_stickers_category(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::StickerCategorie;

        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            StickerCategorie::create_category (
                form.name,
                form.position.unwrap(),
                Some(_request_user.id),
                form.description,
                form.image
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_stickers_category(session: Session, mut payload: Multipart, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::sticker_categories::dsl::sticker_categories;
            use crate::models::StickerCategorie;

            let _connection = establish_connection();
            let category = sticker_categories
                .filter(schema::sticker_categories::id.eq(*cat_id))
                .load::<StickerCategorie>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            category.edit_category (
                form.name,
                form.position.unwrap(),
                Some(_request_user.id),
                form.description,
                form.image,
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn create_sticker(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::Sticker;

        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            let form = category_form (
                payload.borrow_mut(),
                "stickers".to_string(),
                _request_user.id.to_string()
            ).await;

            Sticker::create_sticker (
                form.name,
                form.position.unwrap(),
                form.category_id.unwrap(),
                form.image.unwrap()
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_sticker(session: Session, mut payload: Multipart, sticker_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::stickers::dsl::stickers;
            use crate::models::Sticker;

            let _connection = establish_connection();
            let sticker = stickers
                .filter(schema::stickers::id.eq(*sticker_id))
                .load::<Sticker>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "stickers".to_string(),
                _request_user.id.to_string()
            ).await;

            sticker.edit_sticker (
                form.name,
                form.position.unwrap(),
                form.category_id.unwrap(),
                form.image.unwrap()
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn create_smiles_category(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            use crate::models::SmileCategorie;

            SmileCategorie::create_category (
                form.name,
                form.position.unwrap(),
                form.description,
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_smiles_category(session: Session, mut payload: Multipart, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::smile_categories::dsl::smile_categories;
            use crate::models::SmileCategorie;

            let _connection = establish_connection();
            let category = smile_categories
                .filter(schema::smile_categories::id.eq(*cat_id))
                .load::<SmileCategorie>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            category.edit_category (
                form.name,
                form.position.unwrap(),
                form.description,
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn create_smile(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            let form = category_form (
                payload.borrow_mut(),
                "smiles".to_string(),
                _request_user.id.to_string()
            ).await;

            use crate::models::Smile;

            Smile::create_smile (
                form.name,
                form.position.unwrap(),
                form.category_id.unwrap(),
                form.image.unwrap()
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_smile(session: Session, mut payload: Multipart, smile_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::smiles::dsl::smiles;
            use crate::models::Smile;

            let _connection = establish_connection();
            let smile = smiles
                .filter(schema::smiles::id.eq(*smile_id))
                .load::<Smile>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "smiles".to_string(),
                _request_user.id.to_string()
            ).await;

            smile.edit_smile (
                form.name,
                form.position.unwrap(),
                form.category_id.unwrap(),
                form.image.unwrap()
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn create_post_category(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            use crate::models::PostCategorie;

            PostCategorie::create_category (
                form.name,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_post_category(session: Session, mut payload: Multipart, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::post_categories::dsl::post_categories;
            use crate::models::PostCategorie;

            let _connection = establish_connection();
            let category = post_categories
                .filter(schema::post_categories::id.eq(*cat_id))
                .load::<PostCategorie>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            category.edit_category (
                form.name,
                form.position.unwrap(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn create_video_category(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            use crate::models::VideoCategorie;

            VideoCategorie::create_category (
                form.name,
                form.position.unwrap().into(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_video_category(session: Session, mut payload: Multipart, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::video_categories::dsl::video_categories;
            use crate::models::VideoCategorie;

            let _connection = establish_connection();
            let category = video_categories
                .filter(schema::video_categories::id.eq(*cat_id))
                .load::<VideoCategorie>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = category_form (
                payload.borrow_mut(),
                "categories".to_string(),
                _request_user.id.to_string()
            ).await;

            category.edit_category (
                form.name,
                form.position.unwrap().into(),
            );
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReactionForm {
    pub types:     i16,
    pub image:     String,
    pub gif:       String,
    pub name:      String,
    pub is_active: bool,
    pub position:  i16,
}

pub async fn reaction_form (payload: &mut Multipart) -> ReactionForm {
    let mut form: ReactionForm = ReactionForm {
        types:     0,
        image:     "".to_string(),
        gif:       "".to_string(),
        name:      "".to_string(),
        is_active: true,
        position:  0,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "types" {
                    let _int: i16 = data_string.parse().unwrap();
                    form.types = _int;
                }
                else if field.name() == "image" {
                    form.image = data_string;
                }
                else if field.name() == "gif" {
                    form.gif = data_string;
                }
                else if field.name() == "name" {
                    form.name = data_string;
                }
                else if field.name() == "is_active" {
                    if data_string == "on" {
                        form.is_active = true;
                    } else {
                        form.is_active = false;
                    }
                }
                else if field.name() == "position" {
                    let _int: i16 = data_string.parse().unwrap();
                    form.position = _int;
                }
            }
        }
    }
    form
}

pub async fn create_reaction(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            let form = reaction_form(payload.borrow_mut()).await;

            use crate::models::Reaction;

            Reaction::create_reaction (
                form.types,
                form.image,
                form.gif,
                form.name,
                form.is_active,
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

pub async fn edit_reaction(session: Session, mut payload: Multipart, reaction_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_supermanager() {
            use crate::schema::reactions::dsl::reactions;
            use crate::models::Reaction;

            let _connection = establish_connection();
            let reaction = reactions
                .filter(schema::reactions::id.eq(*reaction_id))
                .load::<Reaction>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            let form = reaction_form (payload.borrow_mut()).await;

            reaction.edit_reaction (
                form.types,
                form.image,
                form.gif,
                form.name,
                form.is_active,
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
