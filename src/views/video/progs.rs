use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use crate::utils::{
    is_signed_in,
    is_desctop,
    get_request_user_data,
    get_user,
    get_community,
    get_video_list,
    get_video,
    get_video_comment,
    get_community_permission,
    get_user_permission,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, VideoList, Video, VideoComment, Community};
use serde::{Deserialize, Serialize};

use std::str;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::{borrow::BorrowMut, io::Write};


pub fn progs_urls(config: &mut web::ServiceConfig) {
    config.route("/video/add_user_list/", web::post().to(add_user_list));
    config.route("/video/edit_user_list/{id}/", web::post().to(edit_user_list));
    config.route("/video/add_community_list/{id}/", web::post().to(add_community_list));
    config.route("/video/edit_community_list/{id}/", web::post().to(edit_community_list));
    config.route("/video/delete_user_list/{id}/", web::get().to(delete_user_list));
    config.route("/video/recover_user_list/{id}/", web::get().to(recover_user_list));
    config.route("/video/delete_community_list/{id}/", web::get().to(delete_community_list));
    config.route("/video/recover_community_list/{id}/", web::get().to(recover_community_list));

    config.route("/video/add_video_in_list/{id}/", web::post().to(add_video_in_list));
    config.route("/video/edit_video/{id}/", web::post().to(edit_video));
    config.route("/video/add_comment/{id}/", web::post().to(add_comment));
    config.route("/video/add_reply/{id}/", web::post().to(add_reply));
}

pub async fn add_user_list(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let _request_user = get_request_user_data(session);
        let form = post_list_form(payload.borrow_mut()).await;
        let new_list = VideoList::create_list (
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
        #[template(path = "desctop/video/user/new_list.stpl")]
        struct Template {
            list: VideoList,
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

pub async fn edit_user_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let list = get_video_list(*_id);
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
            #[template(path = "desctop/video/user/new_list.stpl")]
            struct Template {
                list: VideoList,
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

pub async fn add_community_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let community = get_community(*_id);
        let _request_user = get_request_user_data(session);
        if community.get_administrators_ids().iter().any(|&i| i==_request_user.id) {
            let form = post_list_form(payload.borrow_mut()).await;
            let new_list = VideoList::create_list (
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
            #[template(path = "desctop/video/community/new_list.stpl")]
            struct Template {
                list: VideoList,
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

pub async fn edit_community_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let list = get_video_list(*_id);
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
        #[template(path = "desctop/video/community/new_list.stpl")]
        struct Template {
            list: VideoList,
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


pub async fn delete_user_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_video_list(*_id);
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

pub async fn recover_user_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_video_list(*_id);
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

pub async fn delete_community_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_video_list(*_id);
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

pub async fn recover_community_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_video_list(*_id);
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
pub struct VideoForm {
    pub title: String,
    pub preview: Option<String>,
    pub image: Option<String>,
    pub file: String,
    pub description: Option<String>,
    pub comment_enabled: bool,
    pub votes_on: bool,
}

pub async fn video_form(
    payload: &mut Multipart,
    owner_path: String,
    owner_id: String
) -> VideoForm {
    use crate::utils::UploadedFiles;
    //use uuid::Uuid;

    let mut form: VideoForm = VideoForm {
        title: "".to_string(),
        preview: None,
        image: None,
        file: "".to_string(),
        description: None,
        comment_enabled: true,
        votes_on: true,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if field.name() == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            let file = UploadedFiles::new (
                owner_path.clone(),
                owner_id.to_string(),
                "video".to_string(),
                _new_path.to_string(),
            );
            let file_path = file.path.clone();
            let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .expect("E");
            };
            if field.content_type().to_string() == "image/jpeg".to_string() {
                form.image = Some(file.path.clone().replace("./","/"));
                form.preview = Some(file.path.clone().replace("./","/"));
            }
        }
        else if field.name() == "file" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            let file = UploadedFiles::new (
                owner_path.clone(),
                owner_id.to_string(),
                "video".to_string(),
                _new_path.to_string(),
            );
            let file_path = file.path.clone();
            let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .expect("E");
            };
            form.file = file.path.clone().replace("./","/");
        }
        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string;
                    }
                    else if field.name() == "description" {
                        form.description = Some(data_string);
                    }
                    else if field.name() == "comment_enabled" {
                        if data_string == "on" {
                            form.comment_enabled = true;
                        } else {
                            form.comment_enabled = false;
                        }
                    }
                    else if field.name() == "votes_on" {
                        if data_string == "on" {
                            form.votes_on = true;
                        } else {
                            form.votes_on = false;
                        }
                    }
                }
            }
        }
    }
    form
}
pub async fn add_video_in_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let user_id = _request_user.id;
        let _list = get_video_list(*_id);
        let mut owner_path = "".to_string();
        let mut owner_id = 0;
        let mut is_open = false;
        let mut text = "".to_string();
        let community_id = _list.community_id;

        if community_id.is_some() {
            let _tuple = get_community_permission(&_list.get_community(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
            owner_path = "communities".to_string();
            owner_id = community_id.unwrap();
        }
        else {
            let _tuple = get_user_permission(&_list.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
            owner_path = "users".to_string();
            owner_id = _request_user.id;
        }
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if _list.is_user_can_create_el(_request_user.id) {
            let form = video_form(
                payload.borrow_mut(),
                owner_path,
                owner_id.to_string()
            ).await;
            let new_video = _list.create_video (
                form.title,
                None,
                _request_user.id,
                form.preview,
                form.image,
                form.file,
                form.description,
                form.comment_enabled,
                form.votes_on,
            );

            #[derive(TemplateOnce)]
            #[template(path = "desctop/video/new_item.stpl")]
            struct Template {
                object: Video,
                request_user: User,
            }
            let body = Template {
                object: new_video,
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

pub async fn edit_video(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let user_id = _request_user.id;
        let _video = get_video(*_id);
        let _list = _video.get_list();
        let mut owner_path = "".to_string();
        let mut owner_id = 0;
        let mut is_open = false;
        let mut text = "".to_string();
        let community_id = _list.community_id;

        if community_id.is_some() {
            let _tuple = get_community_permission(&_list.get_community(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
            owner_path = "communities".to_string();
            owner_id = community_id.unwrap();
        }
        else {
            let _tuple = get_user_permission(&_list.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
            owner_path = "users".to_string();
            owner_id = _request_user.id;
        }
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if _video.is_user_can_edit_delete_item(_request_user.id) {
            let form = video_form(
                payload.borrow_mut(),
                owner_path,
                owner_id.to_string()
            ).await;
            let edit_video = _video.edit_video (
                form.title,
                form.preview,
                form.image,
                form.description,
                form.comment_enabled,
                form.votes_on,
            );

            #[derive(TemplateOnce)]
            #[template(path = "desctop/video/new_item.stpl")]
            struct Template {
                object: Video,
                request_user: User,
            }
            let body = Template {
                object: edit_video,
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
        let item = get_video(*_id);
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
            comment: VideoComment,
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
        let comment = get_video_comment(*_id);
        let item = get_video(comment.video_id);
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
            reply:           VideoComment,
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
