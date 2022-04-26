use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use serde::Deserialize;
use crate::utils::{is_signed_in, establish_connection, get_folder, get_request_user_data};
//use diesel::prelude::*;
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn user_routes(config: &mut web::ServiceConfig) {
    config.route("/id{id}/", web::get().to(user_page));
}

pub async fn user_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let _connection = establish_connection();
    let _type = get_folder(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);


        #[derive(TemplateOnce)]
        if _type == "desctop/".to_string() {
            #[template(path = "desctop/users/account/user.stpl")];
        }
        else {
            #[template(path = "mobile/users/account/user.stpl")];
        }
        struct UserPage {
            title:        String,
            request_user: User,
        }
        let body = UserPage {
            title:        "Новости".to_string(),
            request_user: _request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body))

    } else {
        let _template = _type + &"users/account/anon_user.stpl".to_string();
        #[derive(TemplateOnce)]
        #[template(path = _template)]
        struct AnonUserPage {
            title:  String,
        }
        let body = AnonUserPage { title: "Трезвый.рус | Вход".to_string() }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body))
    }
}
