use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use serde::Deserialize;
use crate::utils::{
    is_signed_in,
    establish_connection,
    get_folder,
    get_request_user_data,
    get_user,
};
//use diesel::prelude::*;
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn user_routes(config: &mut web::ServiceConfig) {
    config.route("/id{id}/", web::get().to(user_page));
}

pub async fn user_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let _connection = establish_connection();
    let _type = get_folder(req);
    let _user = get_user(*_id);


    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        if _type == "desctop/".to_string() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/account/user.stpl")]
            struct UserPage {
                title:        String,
                request_user: User,
                user:         User,
                private_bools: Vec<bool>,
            }
            let body = UserPage {
                title:        _user.get_full_name().clone(),
                request_user: _request_user,
                private_bools: _user.get_profile_all_can_see(*_request_user_id).clone(),
                user:         _user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/account/user.stpl")]
            struct UserPage {
                title:        String,
                request_user: User,
                user:         User,
                private_bools: Vec<bool>,
            }
            let body = UserPage {
                title:        _user.get_full_name().clone(),
                request_user: _request_user,
                private_bools: _user.get_profile_all_can_see(*_request_user_id).clone(),
                user:         _user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

    } else {
        if _type == "desctop/".to_string() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/account/anon_user.stpl")]
            struct UserPage {
                title: String,
                user:  User,
            }
            let body = UserPage {
                title: _user.get_full_name(),
                user:  _user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/account/anon_user.stpl")]
            struct UserPage {
                title: String,
                user:  User,
            }
            let body = UserPage {
                title: _user.get_full_name(),
                user:  _user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
