use crate::schema;
use actix_web::{
    //HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    is_signed_in,
    get_request_user_data,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn create_pages_urls(config: &mut web::ServiceConfig) {
    config.route("/admin/created/create_communities_category/", web::get().to(create_communities_category_page));
    config.route("/admin/created/create_communities_subcategory/{id}/", web::get().to(create_communities_subcategory_page));
    config.route("/admin/created/edit_communities_category/{id}/", web::get().to(edit_communities_category_page));
    config.route("/admin/created/edit_communities_subcategory/{id}/", web::get().to(edit_communities_subcategory_page));

    config.route("/admin/created/create_goods_category/", web::get().to(create_goods_category_page));
    config.route("/admin/created/create_goods_subcategory/", web::get().to(create_goods_subcategory_page));
    config.route("/admin/created/edit_goods_category/{id}/", web::get().to(edit_goods_category_page));
    config.route("/admin/created/edit_goods_subcategory/{id}/", web::get().to(edit_goods_subcategory_page));

    config.route("/admin/created/create_sound_genre/", web::get().to(create_sound_genre_page));
    config.route("/admin/created/edit_sound_genre/{id}/", web::get().to(edit_sound_genre_page));
    config.route("/admin/created/create_artist/", web::get().to(create_artist_page));
    config.route("/admin/created/edit_artist/{id}/", web::get().to(edit_artist_page));
    config.route("/admin/created/create_music_album/", web::get().to(create_music_album_page));
    config.route("/admin/created/edit_music_album/{id}/", web::get().to(edit_music_album_page));

    config.route("/admin/created/create_stickers_category/", web::get().to(create_stickers_category_page));
    config.route("/admin/created/edit_stickers_category/{id}/", web::get().to(edit_stickers_category_page));
    config.route("/admin/created/create_sticker/", web::get().to(create_sticker_page));
    config.route("/admin/created/edit_sticker/{id}/", web::get().to(edit_sticker_page));

    config.route("/admin/created/create_smiles_category/", web::get().to(create_smiles_category_page));
    config.route("/admin/created/edit_smiles_category/{id}/", web::get().to(edit_smiles_category_page));
    config.route("/admin/created/create_smile/", web::get().to(create_smile_page));
    config.route("/admin/created/edit_smile/{id}/", web::get().to(edit_smile_page));

    config.route("/admin/created/create_post_category/", web::get().to(create_post_category_page));
    config.route("/admin/created/edit_post_category/{id}/", web::get().to(edit_post_category_page));

    config.route("/admin/created/create_video_category/", web::get().to(create_video_category_page));
    config.route("/admin/created/edit_video_category/{id}/", web::get().to(edit_video_category_page));

    config.route("/admin/created/create_reaction/", web::get().to(create_reaction_page));
    config.route("/admin/created/edit_reaction/{id}/", web::get().to(edit_reaction_page));
}

pub async fn create_communities_category_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_communities_category.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_communities_category_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::community_categorys::dsl::community_categorys;
            use crate::models::CommunityCategory;

            let _connection = establish_connection();
            let category = community_categorys
                .filter(schema::community_categorys::id.eq(*cat_id))
                .load::<CommunityCategory>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_communities_category.stpl")]
            struct Template {
                request_user: User,
                category:     CommunityCategory,
            }
            let body = Template {
                request_user: _request_user,
                category:     category,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn create_communities_subcategory_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::community_categorys::dsl::community_categorys;
            use crate::models::CommunityCategory;

            let _connection = establish_connection();
            let category = community_categorys
                .filter(schema::community_categorys::id.eq(*cat_id))
                .load::<CommunityCategory>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            let categories = community_categorys
                .load::<CommunityCategory>(&_connection)
                .expect("E.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_communities_subcategory.stpl")]
            struct Template {
                request_user: User,
                category:     CommunityCategory,
                categories:   Vec<CommunityCategory>,
            }
            let body = Template {
                request_user: _request_user,
                category:     category,
                categories:   categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_communities_subcategory_page(session: Session, subcat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::community_categorys::dsl::community_categorys;
            use crate::schema::community_subcategorys::dsl::community_subcategorys;
            use crate::models::{CommunityCategory, CommunitySubcategory};

            let _connection = establish_connection();
            let subcategory = community_subcategorys
                .filter(schema::community_subcategorys::id.eq(*subcat_id))
                .load::<CommunitySubcategory>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            let categories = community_categorys
                .load::<CommunityCategory>(&_connection)
                .expect("E.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_communities_subcategory.stpl")]
            struct Template {
                request_user: User,
                subcategory:  CommunitySubcategory,
                categories:   Vec<CommunityCategory>,
            }
            let body = Template {
                request_user: _request_user,
                subcategory:  subcategory,
                categories:   categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}


pub async fn create_goods_category_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_goods_category.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_goods_category_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::good_categories::dsl::good_categories;
            use crate::models::GoodCategorie;

            let _connection = establish_connection();
            let category = good_categories
                .filter(schema::good_categories::id.eq(*cat_id))
                .load::<GoodCategorie>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_goods_category.stpl")]
            struct Template {
                request_user: User,
                category:     GoodCategorie,
            }
            let body = Template {
                request_user: _request_user,
                category:     category,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn create_goods_subcategory_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::good_categories::dsl::good_categories;
            use crate::models::GoodCategorie;

            let _connection = establish_connection();
            let category = good_categories
                .filter(schema::good_categories::id.eq(*cat_id))
                .load::<GoodCategorie>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            let categories = good_categories
                .load::<GoodCategorie>(&_connection)
                .expect("E.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_goods_subcategory.stpl")]
            struct Template {
                request_user: User,
                category:     GoodCategorie,
                categories:   Vec<GoodCategorie>,
            }
            let body = Template {
                request_user: _request_user,
                category:     category,
                categories:   categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_goods_subcategory_page(session: Session, subcat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::good_categories::dsl::good_categories;
            use crate::schema::good_subcategories::dsl::good_subcategories;
            use crate::models::{GoodCategorie, GoodSubcategorie};

            let _connection = establish_connection();
            let subcategory = good_subcategories
                .filter(schema::good_subcategories::id.eq(*subcat_id))
                .load::<GoodSubcategorie>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            let categories = good_categories
                .load::<GoodCategorie>(&_connection)
                .expect("E.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_goods_subcategory.stpl")]
            struct Template {
                request_user: User,
                subcategory:  GoodSubcategorie,
                categories:   Vec<GoodCategorie>,
            }
            let body = Template {
                request_user: _request_user,
                subcategory:  subcategory,
                categories:   categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}


pub async fn create_sound_genre_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_sound_genre.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_sound_genre_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::sound_genres::dsl::sound_genres;
            use crate::models::SoundGenre;

            let _connection = establish_connection();
            let category = sound_genres
                .filter(schema::sound_genres::id.eq(*cat_id))
                .load::<SoundGenre>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_sound_genre.stpl")]
            struct Template {
                request_user: User,
                sound_genre:  CommunityCategory,
            }
            let body = Template {
                request_user: _request_user,
                sound_genre:  sound_genre,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn create_artist_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_artist.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_artist_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::artists::dsl::artists;
            use crate::models::Artist;

            let _connection = establish_connection();
            let artist = artists
                .filter(schema::artists::id.eq(*cat_id))
                .load::<Artist>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_artist.stpl")]
            struct Template {
                request_user: User,
                artist:     Artist,
            }
            let body = Template {
                request_user: _request_user,
                artist:       artist,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn create_music_album_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);

        if _request_user.is_supermanager() {
            use crate::schema::artists::dsl::artists;
            use crate::models::Artist;

            let _connection = establish_connection();
            let all_artists = artists
                .load::<Artist>(&_connection)
                .expect("E.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_music_album.stpl")]
            struct Template {
                request_user: User,
                all_artists:  Vec<Artist>,
            }
            let body = Template {
                request_user: _request_user,
                all_artists:  all_artists,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_music_album_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::music_albums::dsl::music_albums;
            use crate::schema::artists::dsl::artists;
            use crate::models::{MusicAlbum,Artist};

            let _connection = establish_connection();
            let music_album = music_albums
                .filter(schema::music_albums::id.eq(*cat_id))
                .load::<MusicAlbum>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            let all_artists = artists
                .load::<Artist>(&_connection)
                .expect("E.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_music_album.stpl")]
            struct Template {
                request_user: User,
                music_album:  MusicAlbum,
                all_artists:  Vec<Artist>,
            }
            let body = Template {
                request_user: _request_user,
                music_album:  music_album,
                all_artists:  all_artists,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn create_stickers_category_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_stickers_category.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_stickers_category_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::sticker_categories::dsl::sticker_categories;
            use crate::models::StickerCategorie;

            let _connection = establish_connection();
            let category = sticker_categories
                .filter(schema::sticker_categories::id.eq(*cat_id))
                .load::<StickerCategorie>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_stickers_category.stpl")]
            struct Template {
                request_user: User,
                category:     StickerCategorie,
            }
            let body = Template {
                request_user: _request_user,
                category:     category,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn create_sticker_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::sticker_categories::dsl::sticker_categories;
            use crate::models::StickerCategorie;

            let _connection = establish_connection();
            let categories = sticker_categories
                .load::<StickerCategorie>(&_connection)
                .expect("E.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_sticker.stpl")]
            struct Template {
                request_user: User,
                categories:   Vec<StickerCategorie>,
            }
            let body = Template {
                request_user: _request_user,
                categories:   categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_sticker_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::stickers::dsl::stickers;
            use crate::schema::sticker_categories::dsl::sticker_categories;
            use crate::models::{Sticker,StickerCategorie};

            let _connection = establish_connection();
            let sticker = stickers
                .filter(schema::stickers::id.eq(*cat_id))
                .load::<Sticker>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            let categories = sticker_categories
                .load::<StickerCategorie>(&_connection)
                .expect("E.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_sticker.stpl")]
            struct Template {
                request_user: User,
                sticker:      Sticker,
                categories:   Vec<StickerCategorie>,
            }
            let body = Template {
                request_user: _request_user,
                sticker:      sticker,
                categories:   categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}


pub async fn create_smiles_category_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_smiles_category.stpl")]
            struct Template {
                request_user: User,
                categories:   Vec<SmileCategorie>,
            }
            let body = Template {
                request_user: _request_user,
                categories:   categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_smiles_category_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::smile_categories::dsl::smile_categories;
            use crate::models::SmileCategorie;

            let _connection = establish_connection();
            let category = smile_categories
                .filter(schema::smile_categories::id.eq(*cat_id))
                .load::<SmileCategorie>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_smiles_category.stpl")]
            struct Template {
                request_user: User,
                category:     SmileCategorie,
            }
            let body = Template {
                request_user: _request_user,
                category:     category,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn create_smile_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::smile_categories::dsl::smile_categories;
            use crate::models::SmileCategorie;

            let _connection = establish_connection();
            let categories = smile_categories
                .load::<SmileCategorie>(&_connection)
                .expect("E.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_smile.stpl")]
            struct Template {
                request_user: User,
                categories:   Vec<SmileCategorie>
            }
            let body = Template {
                request_user: _request_user,
                categories:   categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_smile_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::smiles::dsl::smiles;
            use crate::models::Smile;

            let _connection = establish_connection();
            let smile = smiles
                .filter(schema::smiles::id.eq(*cat_id))
                .load::<Smile>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            let categories = smile_categories
                .load::<SmileCategorie>(&_connection)
                .expect("E.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_smile.stpl")]
            struct Template {
                request_user: User,
                smile:        Smile,
                categories:   Vec<SmileCategorie>
            }
            let body = Template {
                request_user: _request_user,
                smile:        smile,
                categories:   categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn create_post_category_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_post_category.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_post_category_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::post_categories::dsl::post_categories;
            use crate::models::PostCategorie;

            let _connection = establish_connection();
            let category = post_categories
                .filter(schema::post_categories::id.eq(*cat_id))
                .load::<PostCategorie>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            let categories = post_categories
                .load::<PostCategorie>(&_connection)
                .expect("E.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_post_category.stpl")]
            struct Template {
                request_user: User,
                category:     PostCategorie,
                categories:   Vec<PostCategorie>
            }
            let body = Template {
                request_user: _request_user,
                category:     category,
                categories:   categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn create_video_category_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_video_category.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_video_category_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::video_categories::dsl::video_categories;
            use crate::models::VideoCategorie;

            let _connection = establish_connection();
            let category = video_categories
                .filter(schema::video_categories::id.eq(*cat_id))
                .load::<VideoCategorie>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            let categories = post_categories
                .load::<VideoCategorie>(&_connection)
                .expect("E.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_video_category.stpl")]
            struct Template {
                request_user: User,
                category:     VideoCategorie,
                categories:   Vec<VideoCategorie>,
            }
            let body = Template {
                request_user: _request_user,
                category:     category,
                categories:   categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn create_reaction_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_reaction.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn edit_reaction_page(session: Session, cat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::reactions::dsl::reactions;
            use crate::models::Reaction;

            let _connection = establish_connection();
            let category = reactions
                .filter(schema::reactions::id.eq(*cat_id))
                .load::<Reaction>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_reaction.stpl")]
            struct Template {
                request_user: User,
                reaction:     Reaction,
            }
            let body = Template {
                request_user: _request_user,
                reaction:     reaction,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}
