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
use futures::StreamExt;


pub fn post_progs(config: &mut web::ServiceConfig) {
    config.route("/posts/add_user_list/", web::post().to(add_user_post_list));
    config.route("/posts/edit_user_list/{id}/", web::post().to(edit_user_post_list));
    config.route("/posts/add_community_list/{id}/", web::post().to(add_community_post_list));
    config.route("/posts/edit_community_list/{id}/", web::post().to(edit_community_post_list));
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
            form.description,
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
        #[template(path = "desctop/users/lenta/new_list.stpl")]
        struct Template {
            list: PostList,
        }
        let body = Template {
            list: new_list,
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

pub async fn edit_user_post_list(session: Session, req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_post_list(*_id);
        let _request_user = get_request_user_data(session);
        let form = post_list_form(payload.borrow_mut()).await;
        list.edit_list (
            form.name,
            form.description,
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
        #[template(path = "desctop/users/lenta/new_list.stpl")]
        struct Template {
            list: PostList,
        }
        let body = Template {
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

pub async fn add_community_post_list(session: Session, req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let community = get_community(*_id);
        let _request_user = get_request_user_data(session);
        if !community.get_administrators_ids().iter().any(|&i| i==_request_user.id) {
            Ok(to_home())
        }
        let form = post_list_form(payload.borrow_mut()).await;
        let new_list = PostList::create_list (
            _request_user,
            form.name,
            form.description,
            *_id,
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
        #[template(path = "desctop/communities/lenta/new_list.stpl")]
        struct Template {
            list: PostList,
        }
        let body = Template {
            list: new_list,
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

pub async fn edit_community_post_list(session: Session, req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_post_list(*_id);
        let community = get_community(list.community_id.unwrap());
        let _request_user = get_request_user_data(session);
        if !community.get_administrators_ids().iter().any(|&i| i==_request_user.id) {
            Ok(to_home())
        }
        let form = post_list_form(payload.borrow_mut()).await;
        list.edit_list (
            form.name,
            form.description,
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
        #[template(path = "desctop/communities/lenta/new_list.stpl")]
        struct Template {
            list: PostList,
        }
        let body = Template {
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
