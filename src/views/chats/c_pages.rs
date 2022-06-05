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
    get_message,
    get_list_variables,
};
use serde::Deserialize;
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, Chat, Message};


pub fn c_pages_urls(config: &mut web::ServiceConfig) {
    config.route("/chats_list/", web::get().to(chats_list_page));
    config.route("/chat/{id}/", web::get().to(chat_page));
    config.route("/chat/create_chat/", web::get().to(create_chat_page));
    config.route("/chat/edit_chat/{id}/", web::get().to(edit_chat_page));
    config.route("/chat/create_message/{id}/", web::get().to(create_message_page));
    config.route("/chat/load_chat_message/{id}/", web::get().to(load_chat_message_page));
    config.route("/chat/load_message/{id}/", web::get().to(load_message_page));
    config.route("/chat/edit_message/{id}/", web::get().to(edit_message_page));

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
        let _request_user = get_request_user_data(session);
        let _message = get_message(*_id);
        let _chat = _message.get_chat();

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
        let _request_user = get_request_user_data(session);
        let _message = get_message(*_id);
        _message.get_chat().read_messages(&_request_user.id);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/chats/messages/message.stpl")]
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

pub async fn edit_message_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        let _message = get_message(*_id);
        let _chat = _message.get_chat();
        if (_chat.is_private() && _message.user_id != _request_user.id)
            || (_chat.is_group() || _chat.is_public()) && !_request_user.is_administrator_of_chat(_chat.id) {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission denied."))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/chats/create/edit_message.stpl")]
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
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn edit_chat_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        let _chat = get_chat(*_id);
        if (_chat.is_group() || _chat.is_public()) && !_chat.is_user_can_see_settings(_request_user.id) {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission denied."))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/chats/create/edit_chat.stpl")]
            struct Template {
                request_user: User,
                chat:         Chat,
            }
            let body = Template {
                request_user: _request_user,
                chat:         _chat,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn private_chat_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        let _chat = get_chat(*_id);
        if (_chat.is_group() || _chat.is_public()) && !_chat.is_user_can_see_settings(_request_user.id) {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission denied."))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/chats/chat/info/private.stpl")]
            struct Template {
                request_user: User,
                chat:         Chat,
            }
            let body = Template {
                request_user: _request_user,
                chat:         _chat,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn chat_exclude_users_load(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    #[derive(Debug, Deserialize)]
    pub struct ZParams {
        pub page:  Option<i32>,
        pub types: Option<String>,
    }

    let params_some = web::Query::<ZParams>::from_query(&req.query_string());
    let (is_desctop, page) = get_list_variables(req);

    if is_signed_in(&session) {
        let mut page: i32 = 1;
        let mut next_page_number: i32 = 0;
        let mut text =  "".to_string();
        let mut types = "".to_string();
        let _chat = get_chat(*_id);

        if params_some.is_ok() {
            let params = params_some.unwrap();
            if params.page.is_some() {
                page = params.page.unwrap();
            }
            if params.types.is_some() {
                types = params.types.as_ref().unwrap().to_string();
            }
        }

        let _request_user = get_request_user_data(session);
        let mut users_list: Vec<User> = Vec::new();
        let mut object_list: Vec<User> = Vec::new();

        let count = _chat.members;
            if page > 1 {
                let step = (page - 1) * 20;
                object_list = _chat.get_members(20, step.into());
                if count > (page * 20).try_into().unwrap() {
                    next_page_number = page + 1;
                }
            }
            else {
                object_list = _chat.get_members(20, 0);
                if count > 20.try_into().unwrap() {
                    next_page_number = 2;
                }
        }

        if types == "can_add_members".to_string() {
            text = "добавлять участников".to_string();
            users_list = _chat.get_can_add_in_chat_exclude_users()
        }
        else if types == "can_fix_item".to_string() {
            text = "закреплять сообщения".to_string();
            users_list = _chat.get_can_fix_item_exclude_users()
        }
        else if types == "can_mention".to_string() {
            text = "упоминать о чате".to_string();
            users_list = _chat.get_can_send_mention_exclude_users()
        }
        else if types == "can_add_admin".to_string() {
            text = "добавлять админов".to_string();
            users_list = _chat.get_can_add_admin_exclude_users()
        }
        else if types == "can_add_design".to_string() {
            text = "работать с дизайном".to_string();
            users_list = _chat.get_can_add_design_exclude_users()
        }
        else if types == "can_see_settings".to_string() {
            text = "видеть настройки".to_string();
            users_list = _chat.get_can_see_settings_exclude_users()
        }
        else if types == "can_see_log".to_string() {
            text = "видеть логи чата".to_string();
            users_list = _chat.get_can_see_log_exclude_users()
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/chats/chat/info/exclude_users.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<User>,
                users:            Vec<User>,
                next_page_number: i32,
                types:            String,
                count:            i32,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                users:            users_list,
                next_page_number: next_page_number,
                types:            types,
                count:            count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {

            #[derive(TemplateOnce)]
            #[template(path = "mobile/chats/chat/info/exclude_users.stpl")]
            struct Template {
                request_user:        User,
                object_list:         Vec<User>,
                users:               Vec<User>,
                next_page_number:    i32,
                types:               String,
                count:               i32,
            }

            let body = Template {
                request_user:        _request_user,
                object_list:         object_list,
                users:               users_list,
                next_page_number:    next_page_number,
                types:               types,
                count:               count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
