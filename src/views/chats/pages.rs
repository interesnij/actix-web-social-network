use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    is_signed_in,
    is_desctop,
    get_request_user_data,
    get_chat,
};

use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, Chat, Message};


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/chats_list/", web::get().to(chats_list_page));
    config.route("/chat/{id}/", web::get().to(chat_page));
    config.route("/create_chat/", web::get().to(create_chat_page));
    config.route("/create_message/{id}/", web::get().to(create_message_page));
    config.route("/load_chat_message/{id}/", web::get().to(load_chat_message_page));
    config.route("/load_message/{id}/", web::get().to(load_message_page));
}

pub async fn chats_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::PaginationParams;

        let params_some = web::Query::<PaginationParams>::from_query(&req.query_string());
        let is_desctop = is_desctop(req);
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
        if is_desctop {
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
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Ошибка доступа."))
    }
}

pub async fn chat_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::PaginationParams;

        let _chat = get_chat(*_id);

        let params_some = web::Query::<PaginationParams>::from_query(&req.query_string());
        let is_desctop = is_desctop(req);
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
        let object_list: Vec<Message>;

        let _request_user = get_request_user_data(session);
        let favourite_messages_count = _request_user.favourite_messages_count();
        let count = _chat.count_messages_for_user(_request_user.id);

        if page > 1 {
            let step = (page - 1) * 20;
            object_list = _chat.get_messages_for_user(20, step.into(), _request_user.id);
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = _chat.get_messages_for_user(20, 0, _request_user.id);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/chats/chat/detail/chat.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                favourite_messages_count: usize,
                count_messages: usize,
                next_page_number: i32,
                object_list: Vec<Message>,
                chat: Chat,
            }
            let body = Template {
                title:        "Сообщения".to_string(),
                request_user: _request_user,
                favourite_messages_count: favourite_messages_count,
                count_messages: count,
                next_page_number: next_page_number,
                object_list: object_list,
                chat: _chat,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/chats/chat/detail/chat.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                favourite_messages_count: usize,
                count_messages: usize,
                next_page_number: i32,
                object_list: Vec<Message>,
                chat: Chat,
            }
            let body = Template {
                title:        "Сообщения".to_string(),
                request_user: _request_user,
                favourite_messages_count: favourite_messages_count,
                count_messages: count,
                next_page_number: next_page_number,
                object_list: object_list,
                chat: _chat,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn create_chat_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/chats/create/create_chat.stpl")]
        struct Template {
            request_user: User,
        }
        let body = Template {
            request_user:  _request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
    }
}

pub async fn create_message_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::utils::get_user;

        let _request_user = get_request_user_data(session);
        let _recipient = get_user(*_id);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/chats/create/add_message.stpl")]
        struct Template {
            request_user: User,
            recipient:    User,
        }
        let body = Template {
            request_user: _request_user,
            recipient:    _recipient,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
    }
}

pub async fn load_message_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::utils::get_message;

        let _request_user = get_request_user_data(session);
        let _chat = get_message(*_id).get_chat();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/chats/create/load_message.stpl")]
        struct Template {
            request_user: User,
            object:       Chat,
        }
        let body = Template {
            request_user: _request_user,
            object:       _chat,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
    }
}

pub async fn load_chat_message_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::utils::get_message;

        let _request_user = get_request_user_data(session);
        let _message = get_message(*_id);
        //_message.get_chat().read_messages(&_request_user.id);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/chats/create/load_chat_message.stpl")]
        struct Template {
            request_user: User,
            object:       Message,
        }
        let body = Template {
            request_user: _request_user,
            object:       _message,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
    }
}
