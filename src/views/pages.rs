use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    error::InternalError,
    web,
    http::{header::Header, StatusCode},
};
use serde::Deserialize;
use crate::utils::{is_signed_in, establish_connection, get_data};
use crate::schema;
use diesel::prelude::*;
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{PhoneCode, User};


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
    request_user: User,
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
    test: bool,
}

pub async fn index(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let _connection = establish_connection();
    let _type = get_folder(req);
    if is_signed_in(&session) {
        use crate::schema::design_settings::dsl::design_settings;
        use crate::DesignSetting;

        let request_user = get_current_user(&session);
        let _design = design_settings
            .filter(schema::design_settings::user_id.eq(&request_user.id))
            .load::<DesignSetting>(&_connection)
            .expect("E");
        let background = _design[0].background;

        if _type == "desctop/".to_string() {
            let body = DesctopNewsListTemplate {request_user: request_user, background: background}
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            let body = DesctopNewsListTemplate {request_user: request_user, background: background}
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
