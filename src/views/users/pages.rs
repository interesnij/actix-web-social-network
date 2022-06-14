use actix_web::{
    HttpResponse,
    HttpRequest,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    is_signed_in,
    get_request_user_data,
    get_user,
    get_user_permission,
    get_anon_user_permission,
    get_device_and_ajax,
    get_device_and_page_and_ajax,
};

use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/id{user_id}/communities/", web::get().to(user_communities_page));
    config.route("/id{user_id}/staff-communities/", web::get().to(user_staff_communities_page));
    config.route("/id{user_id}/friends/", web::get().to(user_friends_page));
    config.route("/id{user_id}/friends-online/", web::get().to(user_friends_online_page));
    config.route("/id{user_id}/friends-common/", web::get().to(user_friends_common_page));
    config.route("/id{user_id}/follows/", web::get().to(user_follows_page));

    config.route("/id{user_id}/photos/", web::get().to(user_photos_page));
    config.route("/id{user_id}/goods/", web::get().to(user_goods_page));
    config.route("/id{user_id}/music/", web::get().to(user_music_page));
    config.route("/id{user_id}/surveys/", web::get().to(user_surveys_page));
    config.route("/id{user_id}/video/", web::get().to(user_video_page));
    config.route("/id{user_id}/docs/", web::get().to(user_docs_page));

    config.route("/users/{user_id}/photos_list/{list_id}/", web::get().to(user_photos_list_page));
    config.route("/users/{user_id}/goods_list/{list_id}/", web::get().to(user_goods_list_page));
    config.route("/users/{user_id}/music_list/{list_id}/", web::get().to(user_music_list_page));
    config.route("/users/{user_id}/surveys_list/{list_id}/", web::get().to(user_surveys_list_page));
    config.route("/users/{user_id}/video_list/{list_id}/", web::get().to(user_video_list_page));
    config.route("/users/{user_id}/docs_list/{list_id}/", web::get().to(user_docs_list_page));
}

pub async fn user_communities_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::Community;

    let user_id : i32 = *user_id;
    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let mut next_page_number = 0;

    let _user = get_user(user_id);
    let object_list: Vec<Community>;
    let count = _user.count_communities();
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = _user.get_communities(20, step.into());
        if count > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = _user.get_communities(20, 0);
        if count > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _request_user_id = &_request_user.id;
        let (is_open, text) = get_user_permission(&_user, &_request_user);
        let is_user_can_see_communities = _user.is_user_can_see_community(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/communities/list.stpl")]
            struct Template {
                request_user:                User,
                user:                        User,
                object_list:                 Vec<Community>,
                next_page_number:            i32,
                is_user_can_see_communities: bool,
                count:                       i32,
                is_ajax:                     bool,
            }

            let body = Template {
                request_user:                _request_user,
                user:                        _user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                is_user_can_see_communities: is_user_can_see_communities,
                count:                       count,
                is_ajax:                     is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/communities/list.stpl")]
            struct Template {
                request_user:                User,
                user:                        User,
                object_list:                 Vec<Community>,
                next_page_number:            i32,
                is_user_can_see_communities: bool,
                count:                       i32,
                is_ajax:                     bool,
            }

            let body = Template {
                request_user:                _request_user,
                user:                        _user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                is_user_can_see_communities: is_user_can_see_communities,
                count:                       count,
                is_ajax:                     is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        let is_user_can_see_communities = _user.is_anon_user_can_see_community();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/communities/anon_list.stpl")]
            struct Template {
                user:  User,
                object_list:                 Vec<Community>,
                next_page_number:            i32,
                is_user_can_see_communities: bool,
                count:                       i32,
                is_ajax:                     bool,
            }
            let body = Template {
                user:                        _user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                is_user_can_see_communities: is_user_can_see_communities,
                count:                       count,
                is_ajax:                     is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/communities/anon_list.stpl")]
            struct Template {
                user:  User,
                object_list:                 Vec<Community>,
                next_page_number:            i32,
                is_user_can_see_communities: bool,
                count:                       i32,
                is_ajax:                     bool,
            }
            let body = Template {
                user:                        _user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                is_user_can_see_communities: is_user_can_see_communities,
                count:                       count,
                is_ajax:                     is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
pub async fn user_staff_communities_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::models::Community;

    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _request_user_id = &_request_user.id;

        let object_list: Vec<Community>;
        let count = _request_user.get_staffed_communities_ids().len();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = _request_user.get_staffed_communities(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = _request_user.get_staffed_communities(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/communities/staff_list.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<Community>,
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
            #[template(path = "mobile/users/communities/staff_list.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<Community>,
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

pub async fn user_friends_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id : i32 = *user_id;
    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let mut next_page_number = 0;

    let _user = get_user(user_id);
    let object_list: Vec<User>;
    let count = _user.count_friends();
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = _user.get_friends(20, step.into());
        if count > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = _user.get_friends(20, 0);
        if count > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _request_user_id = &_request_user.id;
        let (is_open, text) = get_user_permission(&_user, &_request_user);
        let is_user_can_see_friends = _user.is_user_can_see_friend(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/friends/list.stpl")]
            struct Template {
                request_user:            User,
                user:                    User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                is_user_can_see_friends: bool,
                count:                   i32,
                is_ajax:                 bool,
            }

            let body = Template {
                request_user:            _request_user,
                user:                    _user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                is_user_can_see_friends: is_user_can_see_friends,
                count:                   count,
                is_ajax:                 is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/friends/list.stpl")]
            struct Template {
                request_user:            User,
                user:                    User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                is_user_can_see_friends: bool,
                count:                   i32,
                is_ajax:                 bool,
            }

            let body = Template {
                request_user:            _request_user,
                user:                    _user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                is_user_can_see_friends: is_user_can_see_friends,
                count:                   count,
                is_ajax:                 is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        let is_user_can_see_friends = _user.is_anon_user_can_see_friend();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/friends/anon_list.stpl")]
            struct Template {
                user:                    User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                is_user_can_see_friends: bool,
                count:                   i32,
                is_ajax:                 bool,
            }
            let body = Template {
                user:                    _user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                is_user_can_see_friends: is_user_can_see_friends,
                count:                   count,
                is_ajax:                 is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/friends/anon_list.stpl")]
            struct Template {
                user:                    User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                is_user_can_see_friends: bool,
                count:                   i32,
                is_ajax:                 bool,
            }
            let body = Template {
                user:                    _user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                is_user_can_see_friends: is_user_can_see_friends,
                count:                   count,
                is_ajax:                 is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_friends_online_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id : i32 = *user_id;
    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let mut next_page_number = 0;

    let _user = get_user(user_id);
    let object_list: Vec<User>;
    let count = _user.get_online_friends_count();
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = _user.get_online_friends(20, step.into());
        if count > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = _user.get_online_friends(20, 0);
        if count > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _request_user_id = &_request_user.id;
        let (is_open, text) = get_user_permission(&_user, &_request_user);
        let is_user_can_see_friends = _user.is_user_can_see_friend(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/friends/online_list.stpl")]
            struct Template {
                request_user:            User,
                user:                    User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                is_user_can_see_friends: bool,
                count:                   usize,
                is_ajax:                 bool,
            }

            let body = Template {
                request_user:            _request_user,
                user:                    _user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                is_user_can_see_friends: is_user_can_see_friends,
                count:                   count,
                is_ajax:                 is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/friends/online_list.stpl")]
            struct Template {
                request_user:            User,
                user:                    User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                is_user_can_see_friends: bool,
                count:                   usize,
                is_ajax:                 bool,
            }

            let body = Template {
                request_user:            _request_user,
                user:                    _user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                is_user_can_see_friends: is_user_can_see_friends,
                count:                   count,
                is_ajax:                 is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        let is_user_can_see_friends = _user.is_anon_user_can_see_friend();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/friends/anon_online_list.stpl")]
            struct Template {
                user:                    User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                is_user_can_see_friends: bool,
                count:                   usize,
                is_ajax:                 bool,
            }
            let body = Template {
                user:                    _user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                is_user_can_see_friends: is_user_can_see_friends,
                count:                   count,
                is_ajax:                 is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/friends/anon_online_list.stpl")]
            struct Template {
                user:                    User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                is_user_can_see_friends: bool,
                count:                   usize,
                is_ajax:                 bool,
            }
            let body = Template {
                user:                    _user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                is_user_can_see_friends: is_user_can_see_friends,
                count:                   count,
                is_ajax:                 is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_friends_common_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id : i32 = *user_id;
    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let mut next_page_number = 0;

    let _user = get_user(user_id);
    let object_list: Vec<User>;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);
        let count = _user.count_common_friends_of_user(&_request_user);

        if page > 1 {
            let step = (page - 1) * 20;

            object_list = _user.get_common_friends_of_user(&_request_user, 20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = _user.get_common_friends_of_user(&_request_user, 20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/friends/common_list.stpl")]
            struct Template {
                request_user:            User,
                user:                    User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                count:                   usize,
                is_ajax:                 bool,
            }

            let body = Template {
                request_user:            _request_user,
                user:                    _user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                count:                   count,
                is_ajax:                 is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/friends/common_list.stpl")]
            struct Template {
                request_user:            User,
                user:                    User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                count:                   usize,
                is_ajax:                 bool,
            }

            let body = Template {
                request_user:            _request_user,
                user:                    _user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                count:                   count,
                is_ajax:                 is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn user_follows_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id : i32 = *user_id;
    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let mut next_page_number = 0;

    let _user = get_user(user_id);
    let object_list: Vec<User>;
    let count = _user.count_followers();
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = _user.get_followers(20, step.into());
        if count > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = _user.get_followers(20, 0);
        if count > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/follows/list.stpl")]
            struct Template {
                request_user:     User,
                user:             User,
                object_list:      Vec<User>,
                next_page_number: i32,
                count:            i32,
                is_ajax:          bool,
            }

            let body = Template {
                request_user:     _request_user,
                user:             _user,
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
            #[template(path = "mobile/users/follows/list.stpl")]
            struct Template {
                request_user:     User,
                user:             User,
                object_list:      Vec<User>,
                next_page_number: i32,
                count:            i32,
                is_ajax:          bool,
            }

            let body = Template {
                request_user:     _request_user,
                user:             _user,
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
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/follows/anon_list.stpl")]
            struct Template {
                user:             User,
                object_list:      Vec<User>,
                next_page_number: i32,
                count:            i32,
                is_ajax:          bool,
            }
            let body = Template {
                user:                    _user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                count:                   count,
                is_ajax:                 is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/follows/anon_list.stpl")]
            struct Template {
                user:             User,
                object_list:      Vec<User>,
                next_page_number: i32,
                count:            i32,
                is_ajax:          bool,
            }
            let body = Template {
                user:                    _user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                count:                   count,
                is_ajax:                 is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_docs_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::DocList;
    use crate::utils::get_doc_list;

    let user_id : i32 = *user_id;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _user = get_user(user_id);
    let _list = get_doc_list(_user.get_selected_doc_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/docs/main_list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         DocList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/docs/main_list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         DocList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/docs/main_list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    DocList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/docs/main_list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    DocList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_video_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::VideoList;
    use crate::utils::get_video_list;

    let user_id : i32 = *user_id;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _user = get_user(user_id);
    let _list = get_video_list(_user.get_selected_video_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/video/main_list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         VideoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/video/main_list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         VideoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/video/main_list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    VideoList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/video/main_list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    VideoList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_surveys_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::SurveyList;
    use crate::utils::get_survey_list;

    let user_id : i32 = *user_id;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _user = get_user(user_id);
    let _list = get_survey_list(_user.get_selected_survey_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/survey/main_list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         SurveyList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/survey/main_list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         SurveyList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/survey/main_list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    SurveyList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/survey/main_list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    SurveyList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_music_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::MusicList;
    use crate::utils::get_music_list;

    let user_id : i32 = *user_id;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _user = get_user(user_id);
    let _list = get_music_list(_user.get_selected_music_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/music/main_list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         MusicList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/music/main_list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         MusicList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/music/main_list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    MusicList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/music/main_list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    MusicList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_goods_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::GoodList;
    use crate::utils::get_good_list;

    let user_id : i32 = *user_id;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _user = get_user(user_id);
    let _list = get_good_list(_user.get_selected_good_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/goods/main_list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         GoodList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/goods/main_list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         GoodList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/goods/main_list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    GoodList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/goods/main_list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    GoodList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_photos_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::PhotoList;
    use crate::utils::get_photo_list;

    let user_id : i32 = *user_id;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _user = get_user(user_id);
    let _list = get_photo_list(_user.get_selected_photo_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/photos/main_list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         PhotoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/photos/main_list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         PhotoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/photos/main_list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    PhotoList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/photos/main_list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    PhotoList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_docs_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::DocList;
    use crate::utils::get_doc_list;

    let user_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _user = get_user(user_id);
    let _list = get_doc_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/docs/list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         DocList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/docs/list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         DocList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/docs/list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    DocList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/docs/list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    DocList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_video_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::VideoList;
    use crate::utils::get_video_list;

    let user_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _user = get_user(user_id);
    let _list = get_video_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/video/list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         VideoList,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/video/list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         VideoList,
                is_ajax:      bool,
            }
            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/video/list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    VideoList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/video/list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    VideoList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_surveys_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::SurveyList;
    use crate::utils::get_survey_list;

    let user_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _user = get_user(user_id);
    let _list = get_survey_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/survey/list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         SurveyList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/survey/list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         SurveyList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/survey/list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    SurveyList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/survey/list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    SurveyList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_music_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::MusicList;
    use crate::utils::get_music_list;

    let user_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _user = get_user(user_id);
    let _list = get_music_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/music/list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         MusicList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/music/list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         MusicList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/music/list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    MusicList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/music/list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    MusicList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_goods_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::GoodList;
    use crate::utils::get_good_list;

    let user_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _user = get_user(user_id);
    let _list = get_good_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/goods/list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         GoodList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/goods/list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         GoodList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/goods/list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    GoodList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/goods/list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    GoodList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_photos_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::PhotoList;
    use crate::utils::get_photo_list;

    let user_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _user = get_user(user_id);
    let _list = get_photo_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/photos/list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         PhotoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/photos/list/list.stpl")]
            struct Template {
                request_user: User,
                user:         User,
                list:         PhotoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                user:         _user,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/photos/list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    PhotoList,
                is_ajax: bool,
            }
            let body = Template {
                user:    _user,
                list:    _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/photos/main_list/anon_list.stpl")]
            struct Template {
                user:    User,
                list:    PhotoList,
                is_ajax: bool,
            }
            let body = Template {
                user:   _user,
                list:   _list,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
