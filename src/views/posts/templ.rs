use actix_web::{
    error::InternalError,
    http::StatusCode,
    HttpResponse,
};
use sailfish::TemplateOnce;
use crate::models::{User, PostList};


pub fn close_post_list (
    folder: String, list: PostList, request_user: User, text: String, is_window: bool,
) -> actix_web::Result<HttpResponse> {
    if is_window == true {
        #[derive(TemplateOnce)]
        #[template(path = "common/close_item.stpl")]
        struct Template {
            text: String,
        }
        let body = Template {
            text:  text,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/anon_close_user.stpl")]
        struct Template {
            user:  User,
        }
        let body = Template {
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/anon_close_user.stpl")]
        struct Template {
            user:  User,
        }
        let body = Template {
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
