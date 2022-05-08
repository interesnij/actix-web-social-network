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
    get_community,
    get_post_list,
    get_community_permission,
    get_anon_community_permission,
    get_list_variables,
};

use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, Post, Community};


pub fn community_urls(config: &mut web::ServiceConfig) {
    config.route("/public{id}/", web::get().to(community_page));
    config.route("/communities/{community_id}/wall/{list_id}/", web::get().to(community_wall_page));

    //config.route("/communities/{community_id}/photos/", web::get().to(community_photos_page));
    //config.route("/communities/{community_id}/goods/", web::get().to(community_goods_page));
    //config.route("/communities/{community_id}/music/", web::get().to(community_music_page));
    //config.route("/communities/{community_id}/surveys/", web::get().to(community_surveys_page));
    //config.route("/communities/{community_id}/video/", web::get().to(community_video_page));
}


pub async fn community_wall_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::PostList;

    let community_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    let _community = get_community(community_id);
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
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);
        let _request_user_id = &_request_user.id;
        let is_user_can_see_post_list = _list.is_user_can_see_el(*_request_user_id);
        let is_user_can_create_posts = _list.is_user_can_create_el(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/lenta/list.stpl")]
            struct Template {
                list:         PostList,
                request_user: User,
                is_user_can_see_post_list: bool,
                is_user_can_create_posts: bool,
                object_list: Vec<Post>,
                community: Community,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_post_list: is_user_can_see_post_list,
                is_user_can_create_posts:  is_user_can_create_posts,
                object_list: object_list,
                community: _community,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/lenta/list.stpl")]
            struct Template {
                list:         PostList,
                request_user: User,
                is_user_can_see_post_list: bool,
                is_user_can_create_posts: bool,
                object_list: Vec<Post>,
                community: Community,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_post_list: is_user_can_see_post_list,
                is_user_can_create_posts:  is_user_can_create_posts,
                object_list: object_list,
                community: _community,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_community_permission(&_community);
        let is_user_can_see_post_list = _list.is_anon_user_can_see_el();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/lenta/anon_list.stpl")]
            struct Template {
                list:         PostList,
                is_user_can_see_post_list: bool,
                object_list: Vec<Post>,
                community: Community,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                is_user_can_see_post_list: is_user_can_see_post_list,
                object_list: object_list,
                community: _community,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/lenta/anon_list.stpl")]
            struct Template {
                list:         PostList,
                is_user_can_see_post_list: bool,
                object_list: Vec<Post>,
                community: Community,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                is_user_can_see_post_list: is_user_can_see_post_list,
                object_list: object_list,
                community: _community,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let _type = get_folder(req);
    let _community = get_community(*_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        if _request_user.types > 10 {
            use crate::views::my_bad_account;
            return my_bad_account(_type, _request_user)
        }
        else if _request_user.is_administrator_of_community(_community.id) {
            if _community.types > 10 {
                return admin_bad_community(_type, _community, _request_user)
            }
            else {
                return admin_community(_type, _community, _request_user)
            }
        }
        else if _community.types > 10 {
            return bad_community(_type, _community, _request_user)
        }
        else if _request_user.is_follow_from_community(_community.id) {
            return follow_community(_type, _community, _request_user)
        }
        else if _request_user.is_child() && !_community.is_identified() {
            return no_child_safety_community(_type, _community, _request_user)
        }
        else if _request_user.is_member_of_community(_community.id) {
            return public_community(_type, _community, _request_user)
        }
        else if _community.is_public() {
            return public_community(_type, _community, _request_user)
        }
        else if _community.is_close() {
            return close_community(_type, _community, _request_user)
        }
        else if _community.is_private() {
            return private_community(_type, _community, _request_user)
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        if _community.types > 10 {
            return anon_bad_community(_type, _community)
        }
        else if _community.is_public() {
            return anon_community(_type, _community)
        }
        else if _community.is_close() {
            return anon_close_community(_type, _community)
        }
        else if _community.is_private() {
            return anon_private_community(_type, _community)
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub fn admin_community(folder: String, community: Community, request_user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/admin_community.stpl")]
        struct Template {
            title:         String,
            private_bools: Vec<bool>,
            request_user:  User,
            community:     Community,
        }
        let body = Template {
            title:         community.name.clone(),
            private_bools: community.get_community_all_can_see(request_user.id),
            request_user:  request_user,
            community:     community,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    } else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/admin_community.stpl")]
        struct Template {
            title:         String,
            private_bools: Vec<bool>,
            request_user:  User,
            community:     Community,
        }
        let body = Template {
            title:         community.name.clone(),
            private_bools: community.get_community_all_can_see(request_user.id),
            request_user:  request_user,
            community:     community,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_community(folder: String, community: Community) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/anon_community.stpl")]
        struct Template {
            title:         String,
            private_bools: Vec<bool>,
            community:     Community,
        }
        let body = Template {
            title: community.name.clone(),
            private_bools: community.get_anon_community_all_can_see(),
            community:  community,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/anon_community.stpl")]
        struct Template {
            title:         String,
            private_bools: Vec<bool>,
            community:     Community,
        }
        let body = Template {
            title: community.name.clone(),
            private_bools: community.get_anon_community_all_can_see(),
            community:  community,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn close_community(folder: String, community: Community, request_user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/close_community.stpl")]
        struct Template {
            title:        String,
            private_bools: Vec<bool>,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            private_bools: community.get_community_all_can_see(request_user.id),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/close_community.stpl")]
        struct Template {
            title:        String,
            private_bools: Vec<bool>,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            private_bools: community.get_community_all_can_see(request_user.id),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn private_community(folder: String, community: Community, request_user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/private_community.stpl")]
        struct Template {
            title:        String,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/private_community.stpl")]
        struct Template {
            title:        String,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn no_child_safety_community(folder: String, community: Community, request_user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/no_child_safety.stpl")]
        struct Template {
            title:        String,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/no_child_safety.stpl")]
        struct Template {
            title:        String,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn follow_community(folder: String, community: Community, request_user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/follow_community.stpl")]
        struct Template {
            title:        String,
            private_bools: Vec<bool>,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            private_bools: community.get_community_all_can_see(request_user.id),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/follow_community.stpl")]
        struct Template {
            title:        String,
            private_bools: Vec<bool>,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            private_bools: community.get_community_all_can_see(request_user.id),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn admin_bad_community(folder: String, community: Community, request_user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/admin_bad_community.stpl")]
        struct Template {
            title:        String,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/admin_bad_community.stpl")]
        struct Template {
            title:        String,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn bad_community(folder: String, community: Community, request_user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/bad_community.stpl")]
        struct Template {
            title:        String,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/bad_community.stpl")]
        struct Template {
            title:        String,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn public_community(folder: String, community: Community, request_user: User) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/public_community.stpl")]
        struct Template {
            title:        String,
            private_bools: Vec<bool>,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            private_bools: community.get_community_all_can_see(request_user.id),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/public_community.stpl")]
        struct Template {
            title:        String,
            private_bools: Vec<bool>,
            community:    Community,
            request_user: User,
        }
        let body = Template {
            title:        community.name.clone(),
            private_bools: community.get_community_all_can_see(request_user.id),
            community:    community,
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_bad_community(folder: String, community: Community) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/anon_bad_community.stpl")]
        struct Template {
            title:        String,
            community:    Community,
        }
        let body = Template {
            title:        community.name.clone(),
            community:    community,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/anon_bad_community.stpl")]
        struct Template {
            title:        String,
            community:    Community,
        }
        let body = Template {
            title:        community.name.clone(),
            community:    community,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_close_community(folder: String, community: Community) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/anon_close_community.stpl")]
        struct Template {
            title:         String,
            private_bools: Vec<bool>,
            community:     Community,
        }
        let body = Template {
            title: community.name.clone(),
            private_bools: community.get_anon_community_all_can_see(),
            community:  community,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/anon_close_community.stpl")]
        struct Template {
            title:         String,
            private_bools: Vec<bool>,
            community:     Community,
        }
        let body = Template {
            title: community.name.clone(),
            private_bools: community.get_anon_community_all_can_see(),
            community:  community,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_private_community(folder: String, community: Community) -> actix_web::Result<HttpResponse> {
    if folder == "desctop/".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/anon_private_community.stpl")]
        struct Template {
            title:        String,
            community:    Community,
        }
        let body = Template {
            title:        community.name.clone(),
            community:    community,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/anon_private_community.stpl")]
        struct Template {
            title:        String,
            community:    Community,
        }
        let body = Template {
            title:        community.name.clone(),
            community:    community,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}