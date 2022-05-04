use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    is_signed_in,
    establish_connection,
    get_folder,
    get_request_user_data,
    get_user,
    get_chat,
};

use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn chats_routes(config: &mut web::ServiceConfig) {
    config.route("/chats_list/", web::get().to(chats_list_page));
}

pub async fn chats_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let _type = get_folder(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let favourite_messages_count = _request_user.favourite_messages_count()
        if _type == "desctop/".to_string() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/chats/chat/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                favourite_messages_count: i32,
            }
            let body = Template {
                title:        "Сообщения".to_string(),
                request_user: _request_user,
                favourite_messages_count: favourite_messages_count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/chats/chat/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                favourite_messages_count: i32,
            }
            let body = Template {
                title:        "Сообщения".to_string(),
                request_user: _request_user,
                favourite_messages_count: favourite_messages_count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
