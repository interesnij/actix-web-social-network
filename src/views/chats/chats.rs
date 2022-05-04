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
use crate::models::{User, Chat, Message};


pub fn chats_routes(config: &mut web::ServiceConfig) {
    config.route("/chats_list/", web::get().to(chats_list_page));
}

pub async fn chats_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::PaginationParams;

        let params_some = web::Query::<PaginationParams>::from_query(&req.query_string());
        let _type = get_folder(req);
        let mut page: i32 = 0;
        if params_some.is_ok() {
            let params = params_some.unwrap();
            if params.page.is_some() {
                page = params.page.unwrap();
            }
            else {
                page = 1;
            }
        }
        else {
            page = 1;
        }
        let mut next_page_number = 0;
        let object_list: Vec<Chat>;

        let _request_user = get_request_user_data(session);
        let favourite_messages_count = _request_user.favourite_messages_count();
        let count = _request_user.get_all_chats_count();

        if page > 1 {
            let step = (page - 1) * 20;
            object_list = _request_user.get_all_chats(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = _request_user.get_all_chats(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }
        if _type == "desctop/".to_string() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/chats/chat/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                favourite_messages_count: usize,
                count_chats: usize,
                next_page_number: i32,
                object_list: Vec<Chat>,
            }
            let body = Template {
                title:        "Сообщения".to_string(),
                request_user: _request_user,
                favourite_messages_count: favourite_messages_count,
                count_chats: count,
                next_page_number: next_page_number,
                object_list: object_list,
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
                favourite_messages_count: usize,
                count_chats: usize,
                next_page_number: i32,
                object_list: Vec<Chat>,
            }
            let body = Template {
                title:        "Сообщения".to_string(),
                request_user: _request_user,
                favourite_messages_count: favourite_messages_count,
                count_chats: count,
                next_page_number: next_page_number,
                object_list: object_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
