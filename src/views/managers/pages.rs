//use crate::schema;
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
    //establish_connection,
    get_device_and_ajax,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;
//use crate::diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/managers/", web::get().to(managers_page));
}

pub async fn managers_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("anon"))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_moderator() {
            let (is_desctop, is_ajax) = get_device_and_ajax(&req);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/managers/main.stpl")]
                struct Template {
                    request_user: User,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/managers/main.stpl")]
                struct Template {
                    request_user: User,
                    is_ajax:      bool,
                }
                let body = Template {
                    request_user: _request_user,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("not permitted"))
        }
    }
}
