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

use std::str;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::borrow::BorrowMut;


pub fn post_progs(config: &mut web::ServiceConfig) {
    config.route("/posts/add_user_list/", web::post().to(add_user_post_list));
    config.route("/posts/edit_user_list/{id}/", web::post().to(edit_user_post_list));
    config.route("/posts/add_community_list/{id}/", web::post().to(add_community_post_list));
    config.route("/posts/edit_community_list/{id}/", web::post().to(edit_community_post_list));
    config.route("/posts/delete_user_list/{id}/", web::get().to(delete_user_post_list));
    config.route("/posts/recover_user_list/{id}/", web::get().to(recover_user_post_list));
    config.route("/posts/delete_community_list/{id}/", web::get().to(delete_community_post_list));
    config.route("/posts/recover_community_list/{id}/", web::get().to(recover_community_post_list));

    config.route("/posts/add_user_post/{id}/", web::post().to(add_user_post));
    config.route("/posts/add_community_post/{id}/", web::post().to(add_community_post));
}

pub async fn add_user_post_list(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

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
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}

pub async fn edit_user_post_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let list = get_post_list(*_id);
        let _request_user = get_request_user_data(session);
        if list.user_id == _request_user.id {
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
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
        }
    } else {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}

pub async fn add_community_post_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let community = get_community(*_id);
        let _request_user = get_request_user_data(session);
        if community.get_administrators_ids().iter().any(|&i| i==_request_user.id) {
            let form = post_list_form(payload.borrow_mut()).await;
            let new_list = PostList::create_list (
                _request_user,
                form.name,
                form.description,
                Some(*_id),
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
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }

} else {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(""))
    }
}

pub async fn edit_community_post_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let list = get_post_list(*_id);
        let community = get_community(list.community_id.unwrap());
        let _request_user = get_request_user_data(session);
        if community.get_administrators_ids().iter().any(|&i| i==_request_user.id) {
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
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
} else {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(""))
}
}


pub async fn delete_user_post_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_post_list(*_id);
        let _request_user = get_request_user_data(session);
        if list.user_id == _request_user.id {
            let res = list.delete_item();
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body("ok"))
        } else {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
        }
    } else {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}

pub async fn recover_user_post_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_post_list(*_id);
        let _request_user = get_request_user_data(session);
        if list.user_id == _request_user.id {
            list.restore_item();
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body("ok"))
        } else {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
        }
    } else {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}

pub async fn delete_community_post_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_post_list(*_id);
        let _request_user = get_request_user_data(session);
        if _request_user.is_administrator_of_community(list.community_id.unwrap()) {
            let res = list.delete_item();
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body("ok"))
        } else {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
        }
    } else {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}

pub async fn recover_community_post_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_post_list(*_id);
        let _request_user = get_request_user_data(session);
        if _request_user.is_administrator_of_community(list.community_id.unwrap()) {
            list.restore_item();
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body("ok"))
        } else {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
        }
    } else {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostForm {
    pub content: Option<String>,
    pub cat: Option<i32>,
    pub attach: Option<String>,
    pub comment_enabled: bool,
    pub votes_on: bool,
}

pub async fn post_form(payload: &mut Multipart) -> PostForm {
    let mut form: PostForm = PostForm {
        content: None,
        cat: None,
        attach: None,
        comment_enabled: true,
        votes_on: true,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if field.name() == "cat" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    let _int: i32 = data_string.parse().unwrap();
                    form.cat = Some(_int);
                }
            }
        }
        else if field.name() == "content" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    form.content = Some(data_string);
                }
            }
        }
        else if field.name() == "attach" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    form.attach = Some(data_string);
                }
            }
        }
        else if field.name() == "comment_enabled" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    if s.to_string() == "on" {
                        form.comment_enabled = true;
                    } else {
                        form.comment_enabled = false;
                    }
                }
            }
        }
        else if field.name() == "votes_on" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    if s.to_string() == "on" {
                        form.votes_on = true;
                    } else {
                        form.votes_on = false;
                    }
                }
            }
        }
    }
    form
}
pub async fn add_user_post(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let user_id = _request_user.id;
        let list = get_post_list(*_id);
        if list.is_user_can_create_el(_request_user.id) {
            let form = post_form(payload.borrow_mut()).await;
            let new_post = Post::create_post (
                user_id,
                form.content,
                form.cat,
                list,
                form.attach,
                None,
                form.comment_enabled,
                false,
                form.votes_on,
                None,
                Some("a".to_string()),
            );

            #[derive(TemplateOnce)]
            #[template(path = "desctop/posts/post_user/new_post.stpl")]
            struct Template {
                object: Post,
                request_user: User,
            }
            let body = Template {
                object: new_post,
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        } else {
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(""))
        }
    } else {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}

pub async fn add_community_post(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let list = get_post_list(*_id);
        let community_id = list.community_id;
        let user_id = _request_user.id;
        if list.is_user_can_create_el(_request_user.id) {
            let form = post_form(payload.borrow_mut()).await;
            let new_post = Post::create_post (
                user_id,
                form.content,
                form.cat,
                list,
                form.attach,
                None,
                form.comment_enabled,
                false,
                form.votes_on,
                community_id,
                Some("a".to_string()),
            );

            #[derive(TemplateOnce)]
            #[template(path = "desctop/posts/post_community/new_post.stpl")]
            struct Template {
                object: Post,
                request_user: User,
            }
            let body = Template {
                object: new_post,
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        } else {
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(""))
        }
    } else {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""))
    }
}
