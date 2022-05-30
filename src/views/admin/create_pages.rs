use actix_web::{
    HttpRequest,
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
    config.route("/admin/created/create_music_album/", web::get().to(create_music_album_)page));
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
            category = community_categorys
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
            category = community_categorys
                .filter(schema::community_categorys::id.eq(*cat_id))
                .load::<CommunityCategory>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_communities_subcategory.stpl")]
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

pub async fn edit_communities_subcategory_page(session: Session, subcat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::community_subcategorys::dsl::community_subcategorys;
            use crate::models::CommunitySubcategory;

            let _connection = establish_connection();
            subcategory = community_subcategorys
                .filter(schema::community_subcategorys::id.eq(*subcat_id))
                .load::<CommunityCategory>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_communities_subcategory.stpl")]
            struct Template {
                request_user: User,
                subcategory:  CommunitySubcategory,
            }
            let body = Template {
                request_user: _request_user,
                subcategory:  subcategory,
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
            category = good_categories
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
            use crate::models::CommunityCategory;

            let _connection = establish_connection();
            category = good_categories
                .filter(schema::good_categories::id.eq(*cat_id))
                .load::<GoodCategorie>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/create_goods_subcategory.stpl")]
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

pub async fn edit_goods_subcategory_page(session: Session, subcat_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        if _request_user.is_supermanager() {
            use crate::schema::good_subcategories::dsl::good_subcategories;
            use crate::models::GoodSubcategorie;

            let _connection = establish_connection();
            subcategory = good_subcategories
                .filter(schema::good_subcategories::id.eq(*subcat_id))
                .load::<GoodSubcategorie>(&_connection)
                .expect("E.")
                .into_iter()
                .nth(0)
                .unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/admin/created/edit_goods_subcategory.stpl")]
            struct Template {
                request_user: User,
                subcategory:  GoodSubcategorie,
            }
            let body = Template {
                request_user: _request_user,
                subcategory:  subcategory,
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
