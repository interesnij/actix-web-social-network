use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use crate::utils::{
    is_signed_in,
    get_folder,
    get_request_user_data,
    get_user,
    get_community,
    get_post_list,
    get_post,
    get_post_comment,
    to_home,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, PostList, Post, Community};
use serde::{Deserialize, Serialize};
use actix_multipart::{Field, Multipart};
use std::{borrow::BorrowMut, str};


pub fn post_routes(config: &mut web::ServiceConfig) {
    config.route("/posts/list/", web::get().to(post_list_page));

    config.route("/posts/add_user_list/", web::get().to(add_user_post_list_page));
    config.route("/posts/edit_user_list/{id}/", web::get().to(edit_user_post_list_page));
    config.route("/posts/add_user_list/", web::post().to(add_user_post_list));
    //config.route("/posts/edit_user_list/{id}/", web::post().to(edit_user_post_list));
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostListForm {
    pub name: String,
    pub description: Option<String>,
    pub can_see_el: String,
    pub can_see_comment: String,
    pub create_el: String,
    pub create_comment: String,
    pub copy_el: String,
    pub can_see_el_users: Vec<i32>,
    pub can_see_comment_users: Vec<i32>,
    pub create_el_users: Vec<i32>,
    pub create_comment_users: Vec<i32>,
    pub copy_el_users: Vec<i32>,
}

pub async fn post_list_form(payload: &mut Multipart) -> PostListForm {
    let mut form: PostListForm = PostListForm {
        name: "".to_string(),
        description: None,
        can_see_el: "".to_string(),
        can_see_comment: "".to_string(),
        create_el: "".to_string(),
        create_comment: "".to_string(),
        copy_el: "".to_string(),
        can_see_el_users: Vec::new(),
        can_see_comment_users: Vec::new(),
        create_el_users: Vec::new(),
        create_comment_users: Vec::new(),
        copy_el_users: Vec::new(),
    };
    let _list = [
        "can_see_el_users",
        "can_see_comment_users",
        "create_el_users",
        "create_comment_users",
        "copy_el_users",
    ];

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if _list.contains(&field.name()) {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    let _int: i32 = data_string.parse().unwrap();
                    if field.name() == "can_see_el_users" {
                        form.can_see_el_users.push(_int);
                    }
                    else if field.name() == "can_see_comment_users" {
                        form.can_see_comment_users.push(_int);
                    }
                    else if field.name() == "create_el_users" {
                        form.create_el_users.push(_int);
                    }
                    else if field.name() == "create_comment_users" {
                        form.create_comment_users.push(_int);
                    }
                    else if field.name() == "copy_el_users" {
                        form.copy_el_users.push(_int);
                    }
                }
            }
        }
        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string
                    } else if field.name() == "description" {
                        form.description = Some(data_string)
                    }
                }
            }
        }
    }
    form
}
pub async fn add_user_post_list(session: Session, req: HttpRequest, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let _request_user = get_request_user_data(session);
        let form = post_list_form(payload.borrow_mut()).await;
        let new_list = PostList::create_list (
            _request_user,
            form.name,
            Some(form.description),
            None,
            form.can_see_el,
            form.can_see_comment,
            form.create_el,
            form.create_comment,
            form.copy_el,
            Some(form.can_see_el_users),
            Some(form.can_see_comment_users),
            Some(form.create_el_users),
            Some(form.create_comment_users),
            Some(form.copy_el_users),
        );

        #[derive(TemplateOnce)]
        #[template(path = "desctop/users/lenta/my_list.stpl")]
        struct Template {
            request_user: User,
            list: PostList,
        }
        let body = Template {
            request_user: _request_user,
            list: list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body))
    } else {
        Ok(to_home())
    }
}

pub async fn add_user_post_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/posts/post_user/add_list.stpl")]
        struct Template {
            request_user: User,
        }
        let body = Template {
            request_user: _request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body))
    } else {
        Ok(to_home())
    }
}
pub async fn edit_user_post_list_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _list_id : i32 = *_id;
        let list = get_post_list(_list_id);
        //let creator = get_user(list.user_id);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/posts/post_user/edit_list.stpl")]
        struct YTemplate {
            request_user: User,
            list: PostList,
            //creator: User,
        }
        let body = YTemplate {
            request_user: _request_user,
            list: list,
            //creator: creator,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body))
    } else {
        Ok(to_home())
    }
}
pub async fn add_community_post_list_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let community = get_community(*_id);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/posts/post_community/add_list.stpl")]
        struct Template {
            request_user: User,
            community: Community,
        }
        let body = Template {
            request_user: _request_user,
            community: community,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body))
    } else {
        Ok(to_home())
    }
}
pub async fn edit_community_post_list_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let list = get_post_list(*_id);
        let community = get_community(list.community_id.unwrap());

        #[derive(TemplateOnce)]
        #[template(path = "desctop/posts/post_community/edit_list.stpl")]
        struct Template {
            request_user: User,
            community: Community,
            list: PostList,
        }
        let body = Template {
            request_user: _request_user,
            community: community,
            list: list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body))
    } else {
        Ok(to_home())
    }
}

pub async fn post_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(Deserialize)]
    struct GetListParams {
        pub page: Option<i32>,
        pub user: Option<i32>,
        pub community: Option<i32>,
        pub list: Option<i32>,
    }

    let params_some = web::Query::<GetListParams>::from_query(&req.query_string());
    if !params_some.is_ok() {
        to_home();
    }
    let params = params_some.unwrap();
    let list: PostList;
    let is_page_list: bool;
    let object_list: Vec<Post>;
    let mut page_user: Option<User> = None;
    let mut page_community: Option<Community> = None;

    if params.user.is_some() {
        let user = get_user(params.user.unwrap());
        list = get_post_list(user.get_selected_post_list_pk());
        page_user = Some(user);
        is_page_list = true;
    }
    else if params.community.is_some() {
        let community = get_community(params.community.unwrap());
        list = get_post_list(community.get_selected_post_list_pk());
        page_community = Some(community);
        is_page_list = true;
    }
    else {
        list = get_post_list(params.list.unwrap());
        is_page_list = false;
    }

    if params.page.is_some() {
        object_list = list.get_paginate_items(20, ((params.page.unwrap() - 1) * 20).into());
    }
    else {
        object_list = list.get_paginate_items(20, 0);
    }
    let _type = get_folder(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;
        let is_user_can_see_post_list = list.is_user_can_see_el(*_request_user_id);
        let is_user_can_create_posts = list.is_user_can_create_el(*_request_user_id);

        if _type == "desctop/".to_string() {
            if list.community_id.is_some() {

                #[derive(TemplateOnce)]
                #[template(path = "desctop/communities/lenta/list.stpl")]
                struct UserPage {
                    list:         PostList,
                    request_user: User,
                    is_user_can_see_post_list: bool,
                    is_user_can_create_posts: bool,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    //is_page_list: bool,
                }
                let body = UserPage {
                    list:                      list,
                    request_user:              _request_user,
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_user_can_create_posts:  is_user_can_create_posts,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                    //is_page_list: is_page_list,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

            } else {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/users/lenta/list.stpl")]
                struct UserPage {
                    list:         PostList,
                    request_user: User,
                    is_user_can_see_post_list: bool,
                    is_user_can_create_posts: bool,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    list:                      list,
                    request_user:              _request_user,
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_user_can_create_posts:  is_user_can_create_posts,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                    is_page_list: is_page_list,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if list.community_id.is_some() {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/communities/lenta/list.stpl")]
                struct UserPage {
                    list:         PostList,
                    request_user: User,
                    is_user_can_see_post_list: bool,
                    is_user_can_create_posts: bool,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    list:                      list,
                    request_user:              _request_user,
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_user_can_create_posts:  is_user_can_create_posts,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                    is_page_list: is_page_list,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

            } else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/users/lenta/list.stpl")]
                struct UserPage {
                    list:         PostList,
                    request_user: User,
                    is_user_can_see_post_list: bool,
                    is_user_can_create_posts: bool,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    list:                      list,
                    request_user:              _request_user,
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_user_can_create_posts:  is_user_can_create_posts,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                    is_page_list: is_page_list,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }

    } else {
        let is_user_can_see_post_list = list.is_anon_user_can_see_el();

        if _type == "desctop/".to_string() {
            if list.community_id.is_some() {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/communities/lenta/anon_list.stpl")]
                struct UserPage {
                    is_user_can_see_post_list: bool,
                    list:  PostList,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    list:  list,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                    is_page_list: is_page_list,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            } else {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/users/lenta/anon_list.stpl")]
                struct UserPage {
                    is_user_can_see_post_list: bool,
                    list:  PostList,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    list:  list,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                    is_page_list: is_page_list,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if list.community_id.is_some() {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/communities/lenta/anon_list.stpl")]
                struct UserPage {
                    is_user_can_see_post_list: bool,
                    list:  PostList,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_page_list: is_page_list,
                    list:  list,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            } else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/users/lenta/anon_list.stpl")]
                struct UserPage {
                    is_user_can_see_post_list: bool,
                    list:  PostList,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_page_list: is_page_list,
                    list:  list,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}
