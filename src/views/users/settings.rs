use diesel::prelude::*;
use crate::schema;
use actix_web::{
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    web,
    Responder,
};
use crate::utils::{
    is_signed_in,
    establish_connection,
    is_desctop,
    get_request_user_data,
    get_list_variables,
    get_device_and_ajax,
    get_device_and_page_and_ajax,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;
use serde::{Deserialize,Serialize};
use std::str;
use std::borrow::BorrowMut;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;


pub fn settings_urls(config: &mut web::ServiceConfig) {
    /////// pages
    config.route("/users/followings/", web::get().to(followings_page));
    config.route("/users/blacklist/", web::get().to(blacklist_page));
    config.route("/users/settings/", web::get().to(settings_page));
    config.route("/users/settings/design/", web::get().to(design_settings_page));

    config.route("/users/settings/private/", web::get().to(private_settings_page));
    config.route("/users/settings/load_include_users/", web::get().to(load_include_users_page));
    config.route("/users/settings/load_exclude_users/", web::get().to(load_exclude_users_page));

    config.route("/users/settings/edit_link/", web::get().to(edit_link_page));
    config.route("/users/settings/edit_name/", web::get().to(edit_name_page));
    config.route("/users/settings/edit_password/", web::get().to(edit_password_page));
    config.route("/users/settings/edit_phone/", web::get().to(edit_phone_page));
    config.route("/users/settings/remove_profile/", web::get().to(remove_profile_page));

    /////// progs
    config.route("/users/settings/change_phone_send/{phone}/", web::get().to(change_phone_send));
    config.route("/users/settings/change_phone_verify/{phone}/{code}/", web::get().to(change_phone_verify));
    config.route("/users/settings/get_background/{color}/", web::get().to(get_background));

    config.route("/users/settings/edit_link/", web::post().to(edit_link));
    config.route("/users/settings/edit_name/", web::post().to(edit_name));
    config.route("/users/settings/edit_password/", web::post().to(edit_password));
    config.route("/users/settings/edit_phone/", web::post().to(edit_phone));

    config.route("/users/settings/remove_profile/", web::post().to(remove_profile));

    config.route("/users/settings/private/", web::post().to(private_settings));
    config.route("/users/settings/load_include_users/", web::post().to(post_include_users));
    config.route("/users/settings/load_exclude_users/", web::post().to(post_exclude_users));
}

pub async fn followings_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let mut next_page_number = 0;

    let object_list: Vec<User>;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let count = _request_user.count_followers();

        if page > 1 {
            let step = (page - 1) * 20;

            object_list = _request_user.get_followings(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = _request_user.get_followings(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/follows/following_list.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<User>,
                next_page_number: i32,
                count:            i32,
                is_ajax:          bool,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                next_page_number: next_page_number,
                count:            count,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/follows/following_list.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<User>,
                next_page_number: i32,
                count:            i32,
                is_ajax:          bool,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                next_page_number: next_page_number,
                count:            count,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn blacklist_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let mut next_page_number = 0;

    let object_list: Vec<User>;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let count = _request_user.count_blacklist();

        if page > 1 {
            let step = (page - 1) * 20;

            object_list = _request_user.get_blocked_users(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = _request_user.get_blocked_users(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lists/blacklist.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<User>,
                next_page_number: i32,
                count:            usize,
                is_ajax:          bool,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                next_page_number: next_page_number,
                count:            count,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/lists/blacklist.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<User>,
                next_page_number: i32,
                count:            usize,
                is_ajax:          bool,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                next_page_number: next_page_number,
                count:            count,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn settings_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/settings.stpl")]
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
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/settings.stpl")]
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
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn design_settings_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_signed_in(&session) {
        use crate::schema::design_settings::dsl::design_settings;
        use crate::models::DesignSetting;

        let _request_user = get_request_user_data(&session);

        let _connection = establish_connection();
        let _designs = design_settings
            .filter(schema::design_settings::user_id.eq(_request_user.id))
            .load::<DesignSetting>(&_connection)
            .expect("E");

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/design_settings.stpl")]
            struct Template {
                request_user: User,
                color:        String,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                color:        _designs[0].background.clone(),
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/design_settings.stpl")]
            struct Template {
                request_user: User,
                color:        String,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                color:        _designs[0].background.clone(),
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
pub async fn private_settings_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_signed_in(&session) {
        use crate::schema::user_privates::dsl::user_privates;
        use crate::models::UserPrivate;

        let _request_user = get_request_user_data(&session);

        let _connection = establish_connection();
        let _private = user_privates
            .filter(schema::user_privates::user_id.eq(_request_user.id))
            .load::<UserPrivate>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/private_settings.stpl")]
            struct Template {
                request_user: User,
                private:      UserPrivate,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                private:      _private,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/private_settings.stpl")]
            struct Template {
                request_user: User,
                private:      UserPrivate,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                private:      _private,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn load_exclude_users_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(Debug, Deserialize)]
    pub struct ZParams {
        pub action:       Option<String>,
    }

    let params_some = web::Query::<ZParams>::from_query(&req.query_string());
    let (is_desctop, page) = get_list_variables(req);

    if is_signed_in(&session) {
        let mut next_page_number: i32 = 0;
        let mut types = "".to_string();
        let mut text =  "".to_string();
        if params_some.is_ok() {
            let params = params_some.unwrap();
            if params.action.is_some() {
                types = params.action.as_ref().unwrap().to_string();
            }
        }

        let _request_user = get_request_user_data(&session);
        let mut users_list: Vec<User> = Vec::new();
        let step: i32;

        let count = _request_user.count_friends();
        if page > 1 {
            step = (page - 1) * 20;
            if count > (page * 20) {
                next_page_number = page + 1;
            }
        }
        else {
            step = 0;
            if count > 20 {
                next_page_number = 2;
            }
        }
        let object_list = _request_user.get_friends(20, step.into());

        if types == "can_see_all".to_string() {
            text = "видеть страницы".to_string();
            users_list = _request_user.get_can_see_all_exclude_users();
        }
        else if types == "can_see_community".to_string() {
            text = "видеть сообщества".to_string();
            users_list = _request_user.get_can_see_community_exclude_users();
        }
        else if types == "can_see_info".to_string() {
            text = "видеть информацию профиля".to_string();
            users_list = _request_user.get_can_see_info_exclude_users();
        }
        else if types == "can_see_friend".to_string() {
            text = "видеть друзей".to_string();
            users_list = _request_user.get_can_see_friend_exclude_users();
        }
        else if types == "can_send_message".to_string() {
            text = "писать сообщения".to_string();
            users_list = _request_user.get_can_send_message_exclude_users();
        }
        else if types == "can_add_in_chat".to_string() {
            text = "добавлять в беседы".to_string();
            users_list = _request_user.get_can_add_in_chat_exclude_users();
        }
        else if types == "can_see_post".to_string() {
            text = "видеть записи".to_string();
            users_list = _request_user.get_can_see_post_exclude_users();
        }
        else if types == "can_see_photo".to_string() {
            text = "видеть фотографии".to_string();
            users_list = _request_user.get_can_see_photo_exclude_users();
        }
        else if types == "can_see_good".to_string() {
            text = "видеть товары".to_string();
            users_list = _request_user.get_can_see_good_exclude_users();
        }
        else if types == "can_see_video".to_string() {
            text = "видеть видеозаписи".to_string();
            users_list = _request_user.get_can_see_video_exclude_users();
        }
        else if types == "can_see_music".to_string() {
            text = "видеть аудиозаписи".to_string();
            users_list = _request_user.get_can_see_music_exclude_users();
        }
        else if types == "can_see_planner".to_string() {
            text = "видеть раздел планирования".to_string();
            users_list = _request_user.get_can_see_planner_exclude_users();
        }
        else if types == "can_see_doc".to_string() {
            text = "видеть документы".to_string();
            users_list = _request_user.get_can_see_doc_exclude_users();
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/perm/exclude_users.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<User>,
                users:            Vec<User>,
                next_page_number: i32,
                types:            String,
                //count:            i32,
                text:             String,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                users:            users_list,
                next_page_number: next_page_number,
                types:            types,
                //count:            count,
                text:             text,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {

            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/perm/exclude_users.stpl")]
            struct Template {
                request_user:        User,
                object_list:         Vec<User>,
                users:               Vec<User>,
                next_page_number:    i32,
                types:               String,
                //count:               i32,
                text:                String,
            }

            let body = Template {
                request_user:        _request_user,
                object_list:         object_list,
                users:               users_list,
                next_page_number:    next_page_number,
                types:               types,
                //count:               count,
                text:                text,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
pub async fn load_include_users_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(Debug, Deserialize)]
    pub struct ZParams {
        pub action: Option<String>,
    }

    let params_some = web::Query::<ZParams>::from_query(&req.query_string());
    let (is_desctop, page) = get_list_variables(req);

    if is_signed_in(&session) {
        let mut next_page_number: i32 = 0;
        let mut types = "".to_string();
        let mut text =  "".to_string();
        if params_some.is_ok() {
            let params = params_some.unwrap();
            if params.action.is_some() {
                types = params.action.as_ref().unwrap().to_string();
            }
        }

        let _request_user = get_request_user_data(&session);
        let mut users_list: Vec<User> = Vec::new();
        let step: i32;

        let count = _request_user.count_friends();
        if page > 1 {
            step = (page - 1) * 20;
            if count > (page * 20) {
                next_page_number = page + 1;
            }
        }
        else {
            step = 0;
            if count > 20 {
                next_page_number = 2;
            }
        }
        let object_list = _request_user.get_friends(20, step.into());

        if types == "can_see_all".to_string() {
            text = "видеть страницы".to_string();
            users_list = _request_user.get_can_see_all_include_users();
        }
        else if types == "can_see_community".to_string() {
            text = "видеть сообщества".to_string();
            users_list = _request_user.get_can_see_community_include_users();
        }
        else if types == "can_see_info".to_string() {
            text = "видеть информацию профиля".to_string();
            users_list = _request_user.get_can_see_info_include_users();
        }
        else if types == "can_see_friend".to_string() {
            text = "видеть друзей".to_string();
            users_list = _request_user.get_can_see_friend_include_users();
        }
        else if types == "can_send_message".to_string() {
            text = "писать сообщения".to_string();
            users_list = _request_user.get_can_send_message_include_users();
        }
        else if types == "can_add_in_chat".to_string() {
            text = "добавлять в беседы".to_string();
            users_list = _request_user.get_can_add_in_chat_include_users();
        }
        else if types == "can_see_post".to_string() {
            text = "видеть записи".to_string();
            users_list = _request_user.get_can_see_post_include_users();
        }
        else if types == "can_see_photo".to_string() {
            text = "видеть фотографии".to_string();
            users_list = _request_user.get_can_see_photo_include_users();
        }
        else if types == "can_see_good".to_string() {
            text = "видеть товары".to_string();
            users_list = _request_user.get_can_see_good_include_users();
        }
        else if types == "can_see_video".to_string() {
            text = "видеть видеозаписи".to_string();
            users_list = _request_user.get_can_see_video_include_users();
        }
        else if types == "can_see_music".to_string() {
            text = "видеть аудиозаписи".to_string();
            users_list = _request_user.get_can_see_music_include_users();
        }
        else if types == "can_see_planner".to_string() {
            text = "видеть раздел планирования".to_string();
            users_list = _request_user.get_can_see_planner_include_users();
        }
        else if types == "can_see_doc".to_string() {
            text = "видеть документы".to_string();
            users_list = _request_user.get_can_see_doc_include_users();
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/perm/include_users.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<User>,
                users:            Vec<User>,
                next_page_number: i32,
                types:            String,
                //count:            i32,
                text:             String,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                users:            users_list,
                next_page_number: next_page_number,
                types:            types,
                //count:            count,
                text:             text,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {

            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/perm/include_users.stpl")]
            struct Template {
                request_user:        User,
                object_list:         Vec<User>,
                users:               Vec<User>,
                next_page_number:    i32,
                types:               String,
                //count:               i32,
                text:                String,
            }

            let body = Template {
                request_user:        _request_user,
                object_list:         object_list,
                users:               users_list,
                next_page_number:    next_page_number,
                types:               types,
                //count:               count,
                text:                text,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_link_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/edit_link.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/edit_link.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
pub async fn edit_name_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/edit_name.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/edit_name.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
pub async fn edit_password_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/edit_password.stpl")]
            struct Template {
                //request_user: User,
            }
            let body = Template {
                //request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/edit_password.stpl")]
            struct Template {
                //request_user: User,
            }
            let body = Template {
                //request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
pub async fn edit_phone_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/edit_phone.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/edit_phone.stpl")]
            struct Template {
                request_user: User,
            }
            let body = Template {
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
pub async fn remove_profile_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/remove_profile.stpl")]
            struct Template {
                //request_user: User,
            }
            let body = Template {
                //request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/remove_profile.stpl")]
            struct Template {
                //request_user: User,
            }
            let body = Template {
                //request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}


pub async fn get_background(session: Session, color: web::Path<String>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::schema::design_settings::dsl::design_settings;
        use crate::models::{DesignSetting, EditDesignSetting};
        let _request_user = get_request_user_data(&session);
        let _connection = establish_connection();
        let backgrounds = design_settings
            .filter(schema::design_settings::user_id.eq(_request_user.id))
            .load::<DesignSetting>(&_connection)
            .expect("E");

        let new_background = EditDesignSetting {
            background: color.into_inner(),
        };

        diesel::update(&backgrounds[0])
          .set(new_background)
             .get_result::<DesignSetting>(&_connection)
             .expect("Error.");

        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body("ok"))
        } else {
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(""))
        }
}

pub async fn edit_link(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::schema::custom_links::dsl::custom_links;
        use crate::models::{EditLinkUser, CustomLink, NewCustomLink};

        let _request_user = get_request_user_data(&session);
        let _connection = establish_connection();

        let mut form: EditLinkUser = EditLinkUser {
            link:  "".to_string(),
        };

        while let Some(item) = payload.next().await {
            let mut field: Field = item.expect("split_payload err");

            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    form.link = data_string;
                }
            }
        }

        diesel::update(&_request_user)
          .set(schema::users::link.eq("/".to_owned() + &form.link + &"/".to_string()))
             .get_result::<User>(&_connection)
             .expect("Error.");

        let link = &form.link;

        let link_some = custom_links
            .filter(schema::custom_links::link.eq(link))
            .limit(1)
            .load::<CustomLink>(&_connection)
            .expect("E.")
            .len() == 0;

        if link_some {
            let new_link = NewCustomLink {
                link: form.link,
                owner: 1,
            };
            diesel::insert_into(schema::custom_links::table)
                .values(&new_link)
                .get_result::<CustomLink>(&_connection)
                .expect("Error.");
        }

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
}
pub async fn edit_name(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::EditNameUser;

        let _request_user = get_request_user_data(&session);
        let _connection = establish_connection();

        let mut form: EditNameUser = EditNameUser {
            first_name: "".to_string(),
            last_name: "".to_string(),
        };

        while let Some(item) = payload.next().await {
            let mut field: Field = item.expect("split_payload err");

            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "first_name" {
                        form.first_name = data_string;
                    }
                    else if field.name() == "last_name" {
                        form.last_name = data_string;
                    }
                }
            }
        }

        diesel::update(&_request_user)
          .set(&form)
             .get_result::<User>(&_connection)
             .expect("Error.");

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
}

pub async fn edit_password(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::EditPasswordUser;
        use crate::utils::hash_password;

        let _request_user = get_request_user_data(&session);
        let _connection = establish_connection();

        let mut form: EditPasswordUser = EditPasswordUser {
            password: "".to_string(),
        };

        while let Some(item) = payload.next().await {
            let mut field: Field = item.expect("split_payload err");

            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    form.password = hash_password(&data_string.clone());
                }
            }
        }

        diesel::update(&_request_user)
          .set(&form)
             .get_result::<User>(&_connection)
             .expect("Error.");

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
}
pub async fn edit_phone(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::EditPhoneUser;

        let _request_user = get_request_user_data(&session);
        let _connection = establish_connection();

        let mut form: EditPhoneUser = EditPhoneUser {
            phone: "".to_string(),
        };

        while let Some(item) = payload.next().await {
            let mut field: Field = item.expect("split_payload err");

            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    form.phone = data_string;
                }
            }
        }

        diesel::update(&_request_user)
          .set(&form)
             .get_result::<User>(&_connection)
             .expect("Error.");

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
}

pub async fn remove_profile(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::{UserDeleteAnketa, NewUserDeleteAnketa};

        let _request_user = get_request_user_data(&session);
        let _connection = establish_connection();

        let mut form: NewUserDeleteAnketa = NewUserDeleteAnketa {
            user_id: _request_user.id,
            answer: "".to_string(),
            other: None,
            created: chrono::Local::now().naive_utc(),
        };

        while let Some(item) = payload.next().await {
            let mut field: Field = item.expect("split_payload err");

            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "answer" {
                        form.answer = data_string;
                    }
                    else if field.name() == "other" {
                        form.other = Some(data_string);
                    }
                }
            }
        }

        diesel::insert_into(schema::user_delete_anketas::table)
            .values(&form)
            .get_result::<UserDeleteAnketa>(&_connection)
            .expect("Error.");
        _request_user.delete_item();

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
}

pub async fn change_phone_send(session: Session, _phone: web::Path<String>) -> impl Responder {
    use crate::utils::PhoneJson;

    if is_signed_in(&session) {
        let req_phone = _phone.to_string();
        if req_phone.len() > 8 {
            use crate::models::{PhoneCode, NewPhoneCode};
            use crate::schema::users::dsl::users;

            let _connection = establish_connection();
            let _some_user = users
                .filter(schema::users::phone.eq(&req_phone))
                .load::<User>(&_connection)
                .expect("E");
            if _some_user.len() > 0 {
                let rendered = "Пользователь с таким номером уже зарегистрирован. Используйте другой номер или напишите в службу поддержки, если этот номер Вы не использовали ранее.";
                HttpResponse::Ok().body(rendered)
            } else {

                let _url = "https://api.ucaller.ru/v1.0/initCall?service_id=12203&key=GhfrKn0XKAmA1oVnyEzOnMI5uBnFN4ck&phone=".to_owned() + &req_phone;
                let __request = reqwest::get(_url).await.expect("E.");
                let new_request = __request.text().await.unwrap();
                println!("{:?}", new_request);

                let phone200: PhoneJson = serde_json::from_str(&new_request).unwrap();
                let code_i32: i32 = phone200.code.parse().unwrap();
                let new_phone_code = NewPhoneCode {
                    phone: _phone.to_string(),
                    code:  code_i32,
                };
                diesel::insert_into(schema::phone_codes::table)
                    .values(&new_phone_code)
                    .get_result::<PhoneCode>(&_connection)
                    .expect("E.");

                let rendered = "Мы Вам звоним. Последние 4 цифры нашего номера - код подтверждения,
                который нужно ввести в поле 'Последние 4 цифры' и нажать 'Подтвердить'
                <div class='row block_verify mt-5'>
                    <div class='col-md-2'></div>
                    <div class='col-md-4'>
                        <input type='number' id='code' onkeyup='code_check();' class='form-control border-0' placeholder='Последние 4 цифры'>
                        <hr class='my-0'>
                    </div>
                    <div class='mb-3 col-md-4'>
                        <button type='button' disabled='disabled' id='change_code_send' class='btn btn-primary pink-gradient'>Подтвердить</button>
                    </div>
                    <div class='col-md-2'></div>
                </div>";
            HttpResponse::Ok().body(rendered)
            }
        }
        else {
            let rendered = "Введите, пожалуйста, корректное количество цифр Вашего телефона";
            HttpResponse::Ok().body(rendered)
        }

    } else {
        HttpResponse::Ok().body("")
    }
}

pub async fn change_phone_verify(session: Session, param: web::Path<(String,i32)>) -> impl Responder {
    use crate::schema::phone_codes::dsl::phone_codes;
    use crate::models::{PhoneCode, EditPhoneUser};

    let response_text : String;

    if is_signed_in(&session) {
        let _connection = establish_connection();
        let _request_user = get_request_user_data(&session);
        let _phone = param.0.to_string();
        let _code = param.1;

        let _phone_codes = phone_codes
            .filter(schema::phone_codes::phone.eq(&_phone))
            .filter(schema::phone_codes::code.eq(&_code))
            .load::<PhoneCode>(&_connection)
            .expect("E");
        if _phone_codes.len() > 0 {
            diesel::delete(phone_codes
                .filter(schema::phone_codes::phone.eq(&_phone))
                .filter(schema::phone_codes::code.eq(&_code))
            ).execute(&_connection)
            .expect("E");
            response_text = "ok".to_string();
            let new_phone = EditPhoneUser {
                phone: _phone,
            };
            diesel::update(&_request_user)
              .set(new_phone)
              .get_result::<User>(&_connection)
              .expect("Error.");
        } else {
            response_text = "Код подтверждения неверный. Проверьте, пожалуйста, номер, с которого мы Вам звонили. Последние 4 цифры этого номера и есть код подтверждения, который нужно ввести с поле 'Последние 4 цифры'. Если не можете найти номер, нажмите на кнопку 'Перезвонить повторно.'".to_string();
        }
    }
    else {
        response_text = "".to_string();
    }

    HttpResponse::Ok().body(response_text)
}

pub async fn private_settings(session: Session, mut payload: Multipart) -> impl Responder {
    // программа изменяет значение полей приватности пользователя
    // кроме "друзья кроме" и "некоторые друзья" (это написано ниже)
    if is_signed_in(&session) {
        use crate::models::UserPrivate;

        struct PrivateForm {
            pub action: String,
            pub value:  String,
        }
        let mut form: PrivateForm = PrivateForm {
            action: "".to_string(),
            value: "".to_string(),
        };

        while let Some(item) = payload.next().await {
            let mut field: Field = item.expect("split_payload err");
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "action" {
                        form.action = data_string;
                    }
                    else if field.name() == "value" {
                        form.value = data_string;
                    }
                }
            }
        }
        let _connection = establish_connection();
        let _request_user = get_request_user_data(&session);

        let user_private = _request_user.get_private_model();
        let action_9: &str = &form.action[..3];
        if action_9 == "can" {
            if form.action == "can_see_all".to_string() {
                diesel::update(&user_private)
                    .set(schema::user_privates::can_see_all.eq(form.value))
                    .get_result::<UserPrivate>(&_connection)
                    .expect("Error.");
            }
            else if form.action == "can_see_community".to_string() {
                diesel::update(&user_private)
                    .set(schema::user_privates::can_see_community.eq(form.value))
                    .get_result::<UserPrivate>(&_connection)
                    .expect("Error.");
            }
            else if form.action == "can_see_info".to_string() {
                diesel::update(&user_private)
                    .set(schema::user_privates::can_see_info.eq(form.value))
                    .get_result::<UserPrivate>(&_connection)
                    .expect("Error.");
            }
            else if form.action == "can_see_friend".to_string() {
                diesel::update(&user_private)
                    .set(schema::user_privates::can_see_friend.eq(form.value))
                    .get_result::<UserPrivate>(&_connection)
                    .expect("Error.");
            }
            else if form.action == "can_send_message".to_string() {
                diesel::update(&user_private)
                    .set(schema::user_privates::can_send_message.eq(form.value))
                    .get_result::<UserPrivate>(&_connection)
                    .expect("Error.");
            }
            else if form.action == "can_add_in_chat".to_string() {
                diesel::update(&user_private)
                    .set(schema::user_privates::can_add_in_chat.eq(form.value))
                    .get_result::<UserPrivate>(&_connection)
                    .expect("Error.");
            }
            else if form.action == "can_see_post".to_string() {
                diesel::update(&user_private)
                    .set(schema::user_privates::can_see_post.eq(form.value))
                    .get_result::<UserPrivate>(&_connection)
                    .expect("Error.");
            }
            else if form.action == "can_see_photo".to_string() {
                diesel::update(&user_private)
                    .set(schema::user_privates::can_see_photo.eq(form.value))
                    .get_result::<UserPrivate>(&_connection)
                    .expect("Error.");
            }
            else if form.action == "can_see_good".to_string() {
                diesel::update(&user_private)
                    .set(schema::user_privates::can_see_good.eq(form.value))
                    .get_result::<UserPrivate>(&_connection)
                    .expect("Error.");
            }
            else if form.action == "can_see_video".to_string() {
                diesel::update(&user_private)
                    .set(schema::user_privates::can_see_video.eq(form.value))
                    .get_result::<UserPrivate>(&_connection)
                    .expect("Error.");
            }
            else if form.action == "can_see_music".to_string() {
                diesel::update(&user_private)
                    .set(schema::user_privates::can_see_music.eq(form.value))
                    .get_result::<UserPrivate>(&_connection)
                    .expect("Error.");
            }
            else if form.action == "can_see_planner".to_string() {
                diesel::update(&user_private)
                    .set(schema::user_privates::can_see_planner.eq(form.value))
                    .get_result::<UserPrivate>(&_connection)
                    .expect("Error.");
            }
            else if form.action == "can_see_doc".to_string() {
                diesel::update(&user_private)
                    .set(schema::user_privates::can_see_doc.eq(form.value))
                    .get_result::<UserPrivate>(&_connection)
                    .expect("Error.");
            }
        }
    }
    HttpResponse::Ok().body("")
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserPrivateIEForm {
    pub users:  Option<String>,
    pub action: Option<String>,
}
pub async fn user_private_ie_form(payload: &mut Multipart) -> UserPrivateIEForm {
    let mut form: UserPrivateIEForm = UserPrivateIEForm {
        users:  None,
        action: None,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "users" {
                    form.users = Some(data_string);
                }
                else if field.name() == "action" {
                    form.action = Some(data_string);
                }
            }
        }
    }
    form
}

pub async fn post_exclude_users(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let _request_user = get_request_user_data(&session);
        let _connection = establish_connection();
        let form = user_private_ie_form(payload.borrow_mut()).await;
        if form.action.is_some() && form.users.is_some() {
            _request_user.set_friends_visible_perms(form.action.as_deref().unwrap().to_string(), form.users.as_deref().unwrap().to_string(), "b".to_string());
        }
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
pub async fn post_include_users(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let _request_user = get_request_user_data(&session);
        let _connection = establish_connection();
        let form = user_private_ie_form(payload.borrow_mut()).await;
        if form.action.is_some() && form.users.is_some() {
            _request_user.set_friends_visible_perms(form.action.as_deref().unwrap().to_string(), form.users.as_deref().unwrap().to_string(), "a".to_string());
        }
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
