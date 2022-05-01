use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
//use serde::Deserialize;
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

pub fn my_user_account(folder: String, user: User, request_user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/my_user.stpl")]
        struct UserPage {
            title:        String,
            private_bools: Vec<bool>,
            request_user: User,
            user:         User,
            is_my_user:   bool
        }
        let body = UserPage {
            title:        user.get_full_name().clone(),
            private_bools: user.get_profile_all_can_see(request_user.id),
            request_user: request_user,
            user:         user,
            is_my_user:   true,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    } else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/my_user.stpl")]
        struct UserPage {
            title:        String,
            private_bools: Vec<bool>,
            request_user: User,
            user:         User,
            is_my_user:   bool
        }
        let body = UserPage {
            title:        user.get_full_name().clone(),
            private_bools: user.get_profile_all_can_see(request_user.id),
            request_user: request_user,
            user:         user,
            is_my_user:   true,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_user_account(folder: String, user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/anon_user.stpl")]
        struct UserPage {
            title: String,
            private_bools: Vec<bool>,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            private_bools: user.get_anon_profile_all_can_see(),
            user:  user,
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
            private_bools: Vec<bool>,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            private_bools: user.get_anon_profile_all_can_see(),
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn user_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::UserProfile;

    let _connection = establish_connection();
    let _type = get_folder(req);
    let _user = get_user(*_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);

        if &_user.id == &_request_user.id {
            return my_user_account(_type, _user, _request_user)
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        return anon_user_account(_type, _user)
    }
}
