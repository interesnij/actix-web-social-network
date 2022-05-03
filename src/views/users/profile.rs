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
    get_post_list,
    get_user_permission,
    get_anon_user_permission,
};

use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, Post};


pub fn user_routes(config: &mut web::ServiceConfig) {
    config.route("/id{id}/", web::get().to(user_page));
    config.route("/users/{id}/wall/", web::get().to(user_wall_page));
}

pub async fn user_wall_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::PaginationParams;
    use crate::models::PostList;

    let params_some = web::Query::<PaginationParams>::from_query(&req.query_string());
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
    let _type = get_folder(req);
    let _user = get_user(*_id);
    let mut next_page_number = 0;
    let object_list: Vec<Post>;
    let list = get_post_list(_user.get_selected_post_list_pk());

    let count = list.count;
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = list.get_paginate_items(20, step.into());
        if count > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = list.get_paginate_items(20, 0);
        if count > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);
        let _request_user_id = &_request_user.id;
        let is_user_can_see_post_list = list.is_user_can_see_el(*_request_user_id);
        let is_user_can_create_posts = list.is_user_can_create_el(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if _type == "desctop/".to_string() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lenta/list.stpl")]
            struct Template {
                list:         PostList,
                request_user: User,
                is_user_can_see_post_list: bool,
                is_user_can_create_posts: bool,
                object_list: Vec<Post>,
                user: User,
                next_page_number: i32,
            }
            let body = Template {
                list:                      list,
                request_user:              _request_user,
                is_user_can_see_post_list: is_user_can_see_post_list,
                is_user_can_create_posts:  is_user_can_create_posts,
                object_list: object_list,
                user: _user,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lenta/list.stpl")]
            struct Template {
                list:         PostList,
                request_user: User,
                is_user_can_see_post_list: bool,
                is_user_can_create_posts: bool,
                object_list: Vec<Post>,
                user: User,
                next_page_number: i32,
            }
            let body = Template {
                list:                      list,
                request_user:              _request_user,
                is_user_can_see_post_list: is_user_can_see_post_list,
                is_user_can_create_posts:  is_user_can_create_posts,
                object_list: object_list,
                user: _user,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        let is_user_can_see_post_list = list.is_anon_user_can_see_el();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if _type == "desctop/".to_string() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lenta/anon_list.stpl")]
            struct Template {
                list:         PostList,
                is_user_can_see_post_list: bool,
                object_list: Vec<Post>,
                user: User,
                next_page_number: i32,
            }
            let body = Template {
                list:                      list,
                is_user_can_see_post_list: is_user_can_see_post_list,
                object_list: object_list,
                user: _user,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lenta/anon_list.stpl")]
            struct Template {
                list:         PostList,
                is_user_can_see_post_list: bool,
                object_list: Vec<Post>,
                user: User,
                next_page_number: i32,
            }
            let body = Template {
                list:                      list,
                is_user_can_see_post_list: is_user_can_see_post_list,
                object_list: object_list,
                user: _user,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let _type = get_folder(req);
    let _user = get_user(*_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        if &_user.id == &_request_user.id {
            if _user.types > 10 {
                return my_bad_account(_type, _user, _request_user)
            }
            else {
                return my_user_account(_type, _user, _request_user)
            }
        }
        else if _user.types > 10 {
            return bad_account(_type, _user, _request_user)
        }
        else if _request_user.is_self_user_in_block(_user.id) {
            return self_block_account(_type, _user, _request_user)
        }
        else if !_user.is_user_can_see_all(_request_user.id) {
            return close_account(_type, _user, _request_user)
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        if !_user.is_anon_user_can_see_all() {
            return anon_close_account(_type, _user)
        }
        else if _user.types > 10 {
            return anon_bad_account(_type, _user)
        }
        else {
            return anon_user_account(_type, _user)
        }
    }
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
pub fn self_block_account(folder: String, user: User, request_user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/self_block_user.stpl")]
        struct UserPage {
            title: String,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/self_block_user.stpl")]
        struct UserPage {
            title: String,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn my_bad_account(folder: String, user: User, request_user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/my_bad_user.stpl")]
        struct UserPage {
            title: String,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/my_bad_user.stpl")]
        struct UserPage {
            title: String,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn bad_account(folder: String, user: User, request_user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/bad_user.stpl")]
        struct UserPage {
            title: String,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/bad_user.stpl")]
        struct UserPage {
            title: String,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn close_account(folder: String, user: User, request_user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/close_user.stpl")]
        struct UserPage {
            title: String,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/close_user.stpl")]
        struct UserPage {
            title: String,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_bad_account(folder: String, user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/anon_bad_user.stpl")]
        struct UserPage {
            title: String,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/anon_bad_user.stpl")]
        struct UserPage {
            title: String,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_close_account(folder: String, user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/anon_close_user.stpl")]
        struct UserPage {
            title: String,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/anon_close_user.stpl")]
        struct UserPage {
            title: String,
            user:  User,
        }
        let body = UserPage {
            title: user.get_full_name(),
            user:  user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
