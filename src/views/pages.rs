use diesel::prelude::*;
use crate::schema;
use actix_web::{
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    web,
};
use serde::Deserialize;
use crate::utils::{is_signed_in, establish_connection, get_folder, get_request_user_data,to_home,};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/featured/", web::get().to(featured_list_page));
}

pub async fn index_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "desctop/main/auth/auth.stpl")]
    struct DesctopAuthTemplate {
        title: String,
    }
    #[derive(TemplateOnce)]
    #[template(path = "desctop/main/lists/news_list.stpl")]
    struct DesctopNewsListTemplate {
        title:        String,
        request_user: User,
    }

    #[derive(TemplateOnce)]
    #[template(path = "mobile/main/auth/auth.stpl")]
    struct MobileAuthTemplate {
        title: String,
    }
    #[derive(TemplateOnce)]
    #[template(path = "mobile/main/lists/news_list.stpl")]
    struct MobileNewsListTemplate {
        title:        String,
        request_user: User,
    }

    let _connection = establish_connection();
    let _type = get_folder(req);
    if is_signed_in(&session) {

        let _request_user = get_request_user_data(session);

        if _type == "desctop/".to_string() {
            let body = DesctopNewsListTemplate {
                title:        "Новости".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            let body = MobileNewsListTemplate {
                title:        "Новости".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }

    } else {
        if _type == "desctop/".to_string() {
            let body = DesctopAuthTemplate { title: "Трезвый.рус | Вход".to_string() }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            let body = MobileAuthTemplate { title: "Трезвый.рус | Вход".to_string() }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
    }
}

pub async fn featured_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "desctop/main/lists/featured_list.stpl")]
    struct DesctopFeaturedListTemplate {
        title:        String,
        request_user: User,
    }
    #[derive(TemplateOnce)]
    #[template(path = "mobile/main/lists/featured_list.stpl")]
    struct MobileFeaturedListTemplate {
        title:        String,
        request_user: User,
    }

    let _connection = establish_connection();
    let _type = get_folder(req);
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);

        if _type == "desctop/".to_string() {
            let body = DesctopFeaturedListTemplate {
                title:      "Рекомендации".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            let body = MobileFeaturedListTemplate {
                title:      "Рекомендации".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
    } else {
        Ok(to_home())
    }
}
