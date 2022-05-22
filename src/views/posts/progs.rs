use crate::schema;
use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::Json,
    error::InternalError,
    http::StatusCode,
};
use crate::utils::{
    is_signed_in,
    get_request_user_data,
    get_community,
    get_post_list,
    get_post,
    get_post_comment,
    get_community_permission,
    get_user_permission,
    establish_connection,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, PostList, Post, PostComment};
use serde::{Deserialize, Serialize};
use crate::diesel::{ExpressionMethods,RunQueryDsl};
use std::str;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::borrow::BorrowMut;


pub fn progs_urls(config: &mut web::ServiceConfig) {
    config.route("/posts/add_user_list/", web::post().to(add_user_post_list));
    config.route("/posts/edit_user_list/{id}/", web::post().to(edit_user_post_list));
    config.route("/posts/add_community_list/{id}/", web::post().to(add_community_post_list));
    config.route("/posts/edit_community_list/{id}/", web::post().to(edit_community_post_list));
    config.route("/posts/delete_user_list/{id}/", web::get().to(delete_user_post_list));
    config.route("/posts/recover_user_list/{id}/", web::get().to(recover_user_post_list));
    config.route("/posts/delete_community_list/{id}/", web::get().to(delete_community_post_list));
    config.route("/posts/recover_community_list/{id}/", web::get().to(recover_community_post_list));

    config.route("/posts/add_post_in_list/{id}/", web::post().to(add_post_in_list));
    config.route("/posts/edit_post/{id}/", web::post().to(edit_post));
    config.route("/posts/delete_post/{id}/", web::post().to(delete_post));
    config.route("/posts/recover_post/{id}/", web::post().to(recover_post));
    config.route("/posts/on_comment/{id}/", web::post().to(on_comment));
    config.route("/posts/off_comment/{id}/", web::post().to(off_comment));
    config.route("/posts/on_votes/{id}/", web::post().to(on_votes));
    config.route("/posts/off_votes/{id}/", web::post().to(off_votes));
    config.route("/posts/fixed/{id}/", web::post().to(fixed));
    config.route("/posts/unfixed/{id}/", web::post().to(unfixed));

    config.route("/posts/add_comment/{id}/", web::post().to(add_comment));
    config.route("/posts/add_reply/{id}/", web::post().to(add_reply));
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
    pub is_signature: bool,
}

pub async fn post_form(payload: &mut Multipart) -> PostForm {
    let mut form: PostForm = PostForm {
        content: None,
        cat: None,
        attach: None,
        comment_enabled: true,
        votes_on: true,
        is_signature: false,
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
        else if field.name() == "is_signature" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    if s.to_string() == "on" {
                        form.is_signature = true;
                    } else {
                        form.is_signature = false;
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
pub async fn add_post_in_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let user_id = _request_user.id;
        let _list = get_post_list(*_id);
        let community_id = _list.community_id;
        let mut is_open = false;
        let mut text = "".to_string();

        if community_id.is_some() {
            let _tuple = get_community_permission(&_list.get_community(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_user_permission(&_list.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if _list.is_user_can_create_el(_request_user.id) {
            let form = post_form(payload.borrow_mut()).await;
            let new_post = _list.create_post (
                user_id,
                form.content,
                form.cat,
                form.attach,
                None,
                form.comment_enabled,
                form.is_signature,
                form.votes_on,
                Some("a".to_string()),
            );

            #[derive(TemplateOnce)]
            #[template(path = "desctop/posts/post.stpl")]
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
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_post(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let user_id = _request_user.id;
        let _post = get_post(*_id);
        let _list = _post.get_list();
        let community_id = _list.community_id;
        let mut is_open = false;
        let mut text = "".to_string();

        if community_id.is_some() {
            let _tuple = get_community_permission(&_list.get_community(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_user_permission(&_list.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if _post.is_user_can_edit_delete_item(_request_user.id) {
            let form = post_form(payload.borrow_mut()).await;
            let edit_post = _post.edit_post (
                form.content,
                form.cat,
                form.attach,
                form.comment_enabled,
                form.votes_on,
                form.is_signature,
            );

            #[derive(TemplateOnce)]
            #[template(path = "desctop/posts/post.stpl")]
            struct Template {
                object: Post,
                request_user: User,
            }
            let body = Template {
                object: _post,
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn add_comment(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;
        let item = get_post(*_id);
        let list = item.get_list();
        let mut is_open = false;
        let mut text = "".to_string();

        if item.community_id.is_some() {
            let _tuple = get_community_permission(&item.get_community(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_user_permission(&item.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if !list.is_user_can_create_comment(*_request_user_id) {
            use crate::views::close_list;
            return close_list()
        }

        use crate::utils::comment_form;
        let form = comment_form(payload.borrow_mut()).await;
        let new_comment = item.create_comment(
            &_request_user,
            form.attach,
            None,
            form.content,
            form.sticker_id,
        );

        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/items/comment/new_parent.stpl")]
        struct Template {
            comment: PostComment,
            request_user_id: i32,
        }
        let body = Template {
            comment: new_comment,
            request_user_id: *_request_user_id,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn add_reply(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;
        let comment = get_post_comment(*_id);
        let item = get_post(comment.post_id);
        let list = item.get_list();
        let mut is_open = false;
        let mut text = "".to_string();

        if item.community_id.is_some() {
            let _tuple = get_community_permission(&item.get_community(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_user_permission(&item.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if !list.is_user_can_create_comment(*_request_user_id) {
            use crate::views::close_list;
            return close_list()
        }

        use crate::utils::comment_form;
        let form = comment_form(payload.borrow_mut()).await;
        let new_comment = item.create_comment(
            &_request_user,
            form.attach,
            Some(comment.id),
            form.content,
            form.sticker_id,
        );

        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/items/comment/new_reply.stpl")]
        struct Template {
            reply:           PostComment,
            request_user_id: i32,
        }
        let body = Template {
            reply:           new_comment,
            request_user_id: *_request_user_id,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}


pub async fn delete_post(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let post = get_post(*_id);
        let _request_user = get_request_user_data(session);
        if post.is_user_can_edit_delete_item(_request_user.id) {
            post.delete_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn recover_post(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let post = get_post(*_id);
        let _request_user = get_request_user_data(session);
        if post.is_user_can_edit_delete_item(_request_user.id) {
            post.restore_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}


pub async fn on_comment(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let post = get_post(*_id);
        let _request_user = get_request_user_data(session);
        if post.is_user_can_edit_delete_item(_request_user.id) {
            use crate::schema::posts::dsl::posts;
            let _connection = establish_connection();

            diesel::update(&post)
                .set(schema::posts::comment_enabled.eq(true))
                .get_result::<Post>(&_connection)
                .expect("Error.");
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn off_comment(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let post = get_post(*_id);
        let _request_user = get_request_user_data(session);
        if post.is_user_can_edit_delete_item(_request_user.id) {
            use crate::schema::posts::dsl::posts;

            let _connection = establish_connection();
            diesel::update(&post)
                .set(schema::posts::comment_enabled.eq(false))
                .get_result::<Post>(&_connection)
                .expect("Error.");
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn on_votes(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let post = get_post(*_id);
        let _request_user = get_request_user_data(session);
        if post.is_user_can_edit_delete_item(_request_user.id) {
            use crate::schema::posts::dsl::posts;

            let _connection = establish_connection();
            diesel::update(&post)
                .set(schema::posts::votes_on.eq(true))
                .get_result::<Post>(&_connection)
                .expect("Error.");
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn off_votes(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let post = get_post(*_id);
        let _request_user = get_request_user_data(session);
        if post.is_user_can_edit_delete_item(_request_user.id) {
            use crate::schema::posts::dsl::posts;

            let _connection = establish_connection();
            diesel::update(&post)
                .set(schema::posts::votes_on.eq(false))
                .get_result::<Post>(&_connection)
                .expect("Error.");
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn fixed(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let post = get_post(*_id);
        let _request_user = get_request_user_data(session);
        if post.is_user_can_edit_delete_item(_request_user.id) {
            post.fixed_post(_request_user);
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn unfixed(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let post = get_post(*_id);
        let _request_user = get_request_user_data(session);
        if post.is_user_can_edit_delete_item(_request_user.id) {
            post.unfixed_post(_request_user);
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
