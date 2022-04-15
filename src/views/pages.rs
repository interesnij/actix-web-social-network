use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    web,
};
use serde::Deserialize;
use crate::utils::{is_signed_in, establish_connection, get_folder, get_request_user_data};
use diesel::prelude::*;
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::PhoneCode;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index));
}

#[derive(Debug, Deserialize)]
pub struct SParams {
    pub q: String,
}

#[derive(TemplateOnce)]
#[template(path = "desctop/main/auth/auth.stpl")]
struct DesctopAuthTemplate {
    codes: Vec<PhoneCode>,
}
#[derive(TemplateOnce)]
#[template(path = "desctop/main/lists/news_list.stpl")]
struct DesctopNewsListTemplate {
    request_id: i32,
    fio:        String,
    types:      i16,
    gender:     String,
    language:   String,
    perm:       i16,
    have_link:  String,
    s_avatar:   String,
    background: String,
}

#[derive(TemplateOnce)]
#[template(path = "mobile/main/auth/auth.stpl")]
struct MobileAuthTemplate {
    test: bool,
}
#[derive(TemplateOnce)]
#[template(path = "mobile/main/lists/news_list.stpl")]
struct MobileNewsListTemplate {
    request_id: i32,
    fio: String,
    types:      i16,
    gender:     String,
    language:   String,
    perm:       i16,
    have_link:  String,
    s_avatar:   String,
    background: String,
}

pub async fn index(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let _connection = establish_connection();
    let _type = get_folder(req);
    if is_signed_in(&session) {

        let (_request_id, _fio, _types, _gender, _language, _perm, _have_link, _s_avatar, _background) = get_request_user_data(session);

        if _type == "desctop/".to_string() {
            let body = DesctopNewsListTemplate {
                request_id: _request_id,
                fio:        _fio,
                types:      _types,
                gender:     _gender,
                language:   _language,
                perm:       _perm,
                have_link:  _have_link,
                s_avatar:   _s_avatar,
                background: _background,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            let body = MobileNewsListTemplate {
                request_id: _request_id,
                fio:        _fio,
                types:      _types,
                gender:     _gender,
                language:   _language,
                perm:       _perm,
                have_link:  _have_link,
                s_avatar:   _s_avatar,
                background: _background,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }

    } else {
        if _type == "desctop/".to_string() {
            use crate::schema::phone_codes::dsl::phone_codes;

            let items = phone_codes
                .load::<PhoneCode>(&_connection)
                .expect("Error.");
            let body = DesctopAuthTemplate { codes: items }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            let body = MobileAuthTemplate { test: true }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
    }
}
