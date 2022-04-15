use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    error::InternalError,
    web,
    http::{header::Header, StatusCode},
};
use serde::Deserialize;
use crate::utils::{is_signed_in, establish_connection, get_folder};
use crate::schema;
use diesel::prelude::*;
use actix_session::Session;
use sailfish::TemplateOnce;


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
    test: bool,
}
#[derive(TemplateOnce)]
#[template(path = "desctop/main/lists/news_list.stpl")]
struct DesctopNewsListTemplate {
    test: bool,
}

#[derive(TemplateOnce)]
#[template(path = "mobile/main/auth/auth.stpl")]
struct MobileAuthTemplate {
    test: bool,
}
#[derive(TemplateOnce)]
#[template(path = "mobile/main/lists/news_list.stpl")]
struct MobileNewsListTemplate {
    test: bool,
}

pub async fn index(session: Session, req: HttpRequest) -> impl Responder {
    let _connection = establish_connection();
    let mut _auth = false;
    if is_signed_in(&session) {
        _auth = true;
    }

    let _type = get_folder(req, session);
    if _auth == true {
        if _type == "desctop/".to_string() {
            let body = DesctopNewsListTemplate { test: true }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body))
        }
        else {
            let body = MobileNewsListTemplate { test: true }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body))
        }
    } else {
        if _type == "desctop/".to_string() {
            let body = DesctopAuthTemplate { test: true }
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
