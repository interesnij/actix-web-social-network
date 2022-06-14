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
    get_user,
    get_user_with_link,
    get_post_list,
    get_user_permission,
    get_anon_user_permission,
    get_list_variables,
    get_device_and_ajax,
};

use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, Post};


pub fn profile_urls(config: &mut web::ServiceConfig) {
    config.route("/users/{user_id}/wall/{list_id}/", web::get().to(user_wall_page));
}

pub async fn user_wall_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::PostList;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    let user_id : i32 = param.0;
    let list_id : i32 = param.1;

    let _user = get_user(user_id);
    let _list = get_post_list(list_id);

    let object_list: Vec<Post>;
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = _list.get_paginate_items(20, step.into());
        if _list.count > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = _list.get_paginate_items(20, 0);
        if _list.count > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);
        let _request_user_id = &_request_user.id;
        let is_user_can_see_post_list = _list.is_user_can_see_el(*_request_user_id);
        let is_user_can_create_posts = _list.is_user_can_create_el(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
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
                list:                      _list,
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
                list:                      _list,
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
        let is_user_can_see_post_list = _list.is_anon_user_can_see_el();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
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
                list:                      _list,
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
                list:                      _list,
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

pub async fn user_page(session: Session, req: HttpRequest, link: String) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _user = get_user_with_link(link);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if &_user.id == &_request_user.id {
            if _user.types > 10 {
                return my_bad_account(is_desctop, _request_user, is_ajax)
            }
            else {
                return my_user_account(is_desctop, _user, _request_user, is_ajax)
            }
        }
        else if _user.types > 10 {
            return bad_account(is_desctop, _user, _request_user, is_ajax)
        }
        else if _request_user.is_self_user_in_block(_user.id) {
            return self_block_account(is_desctop, _user, _request_user, is_ajax)
        }
        else if !_user.is_user_can_see_all(_request_user.id) {
            return close_account(is_desctop, _user, _request_user, is_ajax)
        }
        else {
            return account(is_desctop, _user, _request_user, is_ajax)
        }
    } else {
        if !_user.is_anon_user_can_see_all() {
            return anon_close_account(is_desctop, _user, is_ajax)
        }
        else if _user.types > 10 {
            return anon_bad_account(is_desctop, _user, is_ajax)
        }
        else {
            return anon_user_account(is_desctop, _user, is_ajax)
        }
    }
}

pub fn my_user_account(is_desctop: bool, user: User, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/my_user.stpl")]
        struct UserPage {
            request_user: User,
            user:         User,
            is_ajax:      bool,
        }
        let body = UserPage {
            request_user: request_user,
            user:         user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    } else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/my_user.stpl")]
        struct UserPage {
            request_user: User,
            user:         User,
            is_ajax:      bool,
        }
        let body = UserPage {
            request_user: request_user,
            user:         user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_user_account(is_desctop: bool, user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/anon_user.stpl")]
        struct UserPage {
            private_bools: Vec<bool>,
            user:          User,
            is_ajax:       bool,
        }
        let body = UserPage {
            private_bools: user.get_anon_profile_all_can_see(),
            user:          user,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/anon_user.stpl")]
        struct UserPage {
            private_bools: Vec<bool>,
            user:          User,
            is_ajax:       bool,
        }
        let body = UserPage {
            private_bools: user.get_anon_profile_all_can_see(),
            user:          user,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn self_block_account(is_desctop: bool, user: User, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/self_block_user.stpl")]
        struct UserPage {
            user:         User,
            request_user: User,
            is_ajax:      bool,
        }
        let body = UserPage {
            user:         user,
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/self_block_user.stpl")]
        struct UserPage {
            user:         User,
            request_user: User,
            is_ajax:      bool,
        }
        let body = UserPage {
            user:         user,
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn my_bad_account(is_desctop: bool, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/my_bad_user.stpl")]
        struct UserPage {
            request_user: User,
            is_ajax:      bool,
        }
        let body = UserPage {
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/my_bad_user.stpl")]
        struct UserPage {
            request_user: User,
            is_ajax:      bool,
        }
        let body = UserPage {
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn bad_account(is_desctop: bool, user: User, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/bad_user.stpl")]
        struct UserPage {
            user:         User,
            request_user: User,
            is_ajax:      bool,
        }
        let body = UserPage {
            user:         user,
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/bad_user.stpl")]
        struct UserPage {
            user:         User,
            request_user: User,
            is_ajax:      bool,
        }
        let body = UserPage {
            user:         user,
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn close_account(is_desctop: bool, user: User, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/close_user.stpl")]
        struct UserPage {
            private_bools: Vec<bool>,
            request_user:  User,
            user:          User,
            is_ajax:       bool,
        }
        let body = UserPage {
            private_bools: user.get_profile_all_can_see(request_user.id),
            request_user:  request_user,
            user:          user,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/close_user.stpl")]
        struct UserPage {
            private_bools: Vec<bool>,
            request_user:  User,
            user:          User,
            is_ajax:       bool,
        }
        let body = UserPage {
            private_bools: user.get_profile_all_can_see(request_user.id),
            request_user:  request_user,
            user:          user,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_bad_account(is_desctop: bool, user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/anon_bad_user.stpl")]
        struct UserPage {
            user:    User,
            is_ajax: bool,
        }
        let body = UserPage {
            user:    user,
            is_ajax: is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/anon_bad_user.stpl")]
        struct UserPage {
            user:    User,
            is_ajax: bool,
        }
        let body = UserPage {
            user:    user,
            is_ajax: is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_close_account(is_desctop: bool, user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/anon_close_user.stpl")]
        struct UserPage {
            user:    User,
            is_ajax: bool,
        }
        let body = UserPage {
            user:    user,
            is_ajax: is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/anon_close_user.stpl")]
        struct UserPage {
            user:    User,
            is_ajax: bool,
        }
        let body = UserPage {
            user:    user,
            is_ajax: is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}


pub fn account(is_desctop: bool, user: User, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/account/user.stpl")]
        struct UserPage {
            private_bools: Vec<bool>,
            request_user:  User,
            user:          User,
            is_ajax:       bool,
        }
        let body = UserPage {
            private_bools: user.get_profile_all_can_see(request_user.id),
            request_user:  request_user,
            user:          user,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/users/account/user.stpl")]
        struct UserPage {
            private_bools: Vec<bool>,
            request_user:  User,
            user:          User,
            is_ajax:       bool,
        }
        let body = UserPage {
            private_bools: user.get_profile_all_can_see(request_user.id),
            request_user:  request_user,
            user:          user,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
