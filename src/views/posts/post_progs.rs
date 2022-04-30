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
//use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use actix_multipart::Multipart;


pub fn post_progs(config: &mut web::ServiceConfig) {
    config.route("/posts/add_user_list/", web::post().to(add_user_post_list));
    config.route("/posts/edit_user_list/{id}/", web::post().to(edit_user_post_list));
    config.route("/posts/add_community_list/{id}/", web::post().to(add_community_post_list));
    config.route("/posts/edit_community_list/{id}/", web::post().to(edit_community_post_list));
    config.route("/posts/delete_list/{id}/", web::post().to(delete_post_list));
    config.route("/posts/recover_list/{id}/", web::post().to(recover_post_list));
}

pub async fn add_user_post_list(session: Session, req: HttpRequest, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
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

pub async fn edit_user_post_list(session: Session, req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
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

pub async fn add_community_post_list(session: Session, req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
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

pub async fn edit_community_post_list(session: Session, req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
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


pub async fn delete_post_list(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_post_list(*_id);
        let _request_user = get_request_user_data(session);
        if list.user_id == _request_user.id {
            let res = list.delete_item();
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(res))
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

pub async fn recover_post_list(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_post_list(*_id);
        let _request_user = get_request_user_data(session);
        if list.user_id == _request_user.id {
            let res = list.restore_item();
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(res))
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
