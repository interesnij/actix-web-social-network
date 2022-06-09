use crate::schema;
use actix_web::{
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
    get_video_list,
    get_video,
    get_video_comment,
    get_community_permission,
    get_user_permission,
    establish_connection,
    NewListValues,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{
    //User,
    VideoList,
    Video,
    VideoComment};
use serde::{Deserialize, Serialize};
use crate::diesel::{ExpressionMethods,RunQueryDsl};
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
    config.route("/video/delete_video/{id}/", web::get().to(delete_video));
    config.route("/video/recover_video/{id}/", web::get().to(recover_video));
    config.route("/video/on_comment/{id}/", web::get().to(on_comment));
    config.route("/video/off_comment/{id}/", web::get().to(off_comment));

    config.route("/video/add_comment/{id}/", web::post().to(add_comment));
    config.route("/video/add_reply/{id}/", web::post().to(add_reply));
}

pub async fn add_user_list(session: Session, mut payload: Multipart) -> web::Json<NewListValues> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let _request_user = get_request_user_data(&session);
        let form = post_list_form(
            payload.borrow_mut(),
            "users".to_string(),
            _request_user.id.to_string()
        ).await;
        let new_list = VideoList::create_list (
            _request_user,
            form.name,
            form.description,
            form.image,
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
            form.reactions,
        );

        return Json(NewListValues {
            pk: new_list.id,
            name: new_list.name,
            image: new_list.image,
        })
    } else {
        return Json(NewListValues {
            pk: 0,
            name: "".to_string(),
            image: None,
        })
    }
}

pub async fn edit_user_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> web::Json<NewListValues> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let list = get_video_list(*_id);
        let _request_user = get_request_user_data(&session);
        if list.user_id == _request_user.id {
            let form = post_list_form(
                payload.borrow_mut(),
                "users".to_string(),
                _request_user.id.to_string()
            ).await;
            let edit_list = list.edit_list (
                form.name,
                form.description,
                form.image,
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
                form.reactions,
            );
            return Json(NewListValues {
                pk: edit_list.id,
                name: edit_list.name.clone(),
                image: edit_list.image.clone(),
            })

        } else {
            return Json(NewListValues {
                pk: 0,
                name: "".to_string(),
                image: None,
            })
        }
    } else {
        return Json(NewListValues {
            pk: 0,
            name: "".to_string(),
            image: None,
        })
    }
}

pub async fn add_community_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> web::Json<NewListValues> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let community = get_community(*_id);
        let _request_user = get_request_user_data(&session);
        if community.get_administrators_ids().iter().any(|&i| i==_request_user.id) {
            let form = post_list_form(
                payload.borrow_mut(),
                "communities".to_string(),
                community.id.to_string()
            ).await;
            let new_list = VideoList::create_list (
                _request_user,
                form.name,
                form.description,
                form.image,
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
                form.reactions,
            );

            return Json(NewListValues {
                pk: new_list.id,
                name: new_list.name,
                image: new_list.image,
            })
        } else {
            return Json(NewListValues {
                pk: 0,
                name: "".to_string(),
                image: None,
            })
        }

    } else {
        return Json(NewListValues {
            pk: 0,
            name: "".to_string(),
            image: None,
        })
    }
}

pub async fn edit_community_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> web::Json<NewListValues> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let list = get_video_list(*_id);
        let community = get_community(list.community_id.unwrap());
        let _request_user = get_request_user_data(&session);
        if community.get_administrators_ids().iter().any(|&i| i==_request_user.id) {
            let form = post_list_form(
                payload.borrow_mut(),
                "communities".to_string(),
                community.id.to_string()
            ).await;
            let edit_list = list.edit_list (
                form.name,
                form.description,
                form.image,
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
                form.reactions,
            );

            return Json(NewListValues {
                pk: edit_list.id,
                name: edit_list.name.clone(),
                image: edit_list.image.clone(),
            })
        } else {
            return Json(NewListValues {
                pk: 0,
                name: "".to_string(),
                image: None,
            })
        }
    } else {
        return Json(NewListValues {
            pk: 0,
            name: "".to_string(),
            image: None,
        })
    }
}


pub async fn delete_user_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_video_list(*_id);
        let _request_user = get_request_user_data(&session);
        if list.user_id == _request_user.id {
            list.delete_item();
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
        let _request_user = get_request_user_data(&session);
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
        let _request_user = get_request_user_data(&session);
        if _request_user.is_administrator_of_community(list.community_id.unwrap()) {
            list.delete_item();
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
        let _request_user = get_request_user_data(&session);
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
    pub category_id: Option<i32>,
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
        category_id: None,
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
                    if field.name() == "title" {
                        let data_string = s.to_string();
                        form.title = data_string;
                    }
                    else if field.name() == "category_id" {
                        let data_string = s.to_string();
                        let _int: i32 = data_string.parse().unwrap();
                        form.category_id = Some(_int);
                    }
                    else if field.name() == "description" {
                        let data_string = s.to_string();
                        form.description = Some(data_string);
                    }
                    else if field.name() == "comment_enabled" {
                        let data_string = s.to_string();
                        if data_string == "on" {
                            form.comment_enabled = true;
                        } else {
                            form.comment_enabled = false;
                        }
                    }
                }
            }
        }
    }
    form
}

#[derive(Serialize)]
pub struct NewVideoResponse {
    pub pk: i32,
}
pub async fn add_video_in_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> web::Json<NewVideoResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _list = get_video_list(*_id);
        let owner_path : String;
        let owner_id: i32;
        let is_open : bool;
        let community_id = _list.community_id;

        if community_id.is_some() {
            let _tuple = get_community_permission(&_list.get_community(), &_request_user);
            is_open = _tuple.0;
            owner_path = "communities".to_string();
            owner_id = community_id.unwrap();
        }
        else {
            let _tuple = get_user_permission(&_list.get_creator(), &_request_user);
            is_open = _tuple.0;
            owner_path = "users".to_string();
            owner_id = _request_user.id;
        }
        if is_open == false {
            return Json(NewVideoResponse {
                pk: 0,
            })
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
                form.category_id,
            );
            return Json(NewVideoResponse {
                pk: new_video.id,
            })
        } else {
            return Json(NewVideoResponse {
                pk: 0,
            })
        }
    } else {
        return Json(NewVideoResponse {
            pk: 0,
        })
    }
}

pub async fn edit_video(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _video = get_video(*_id);
        let _list = _video.get_list();
        let owner_path : String;
        let owner_id: i32;
        let is_open : bool;
        let text : String;
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
            let edited_video = _video.edit_video (
                form.title,
                form.preview,
                form.image,
                form.description,
                form.comment_enabled,
                form.category_id,
            );

            #[derive(TemplateOnce)]
            #[template(path = "desctop/video/video.stpl")]
            struct Template {
                object: Video,
            }
            let body = Template {
                object: edited_video,
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
        let _request_user = get_request_user_data(&session);
        let _request_user_id = &_request_user.id;
        let item = get_video(*_id);
        let list = item.get_list();
        let is_open : bool;
        let text : String;

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
        let _request_user = get_request_user_data(&session);
        let _request_user_id = &_request_user.id;
        let comment = get_video_comment(*_id);
        let item = get_video(comment.video_id);
        let list = item.get_list();
        let is_open : bool;
        let text : String;

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

pub async fn delete_video(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let video = get_video(*_id);
        let _request_user = get_request_user_data(&session);
        if video.is_user_can_edit_delete_item(_request_user.id) {
            video.delete_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn recover_video(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let video = get_video(*_id);
        let _request_user = get_request_user_data(&session);
        if video.is_user_can_edit_delete_item(_request_user.id) {
            video.restore_item();
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
        let video = get_video(*_id);
        let _request_user = get_request_user_data(&session);
        if video.is_user_can_edit_delete_item(_request_user.id) {
            let _connection = establish_connection();

            diesel::update(&video)
                .set(schema::videos::comment_enabled.eq(true))
                .get_result::<Video>(&_connection)
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
        let video = get_video(*_id);
        let _request_user = get_request_user_data(&session);
        if video.is_user_can_edit_delete_item(_request_user.id) {
            let _connection = establish_connection();
            diesel::update(&video)
                .set(schema::videos::comment_enabled.eq(false))
                .get_result::<Video>(&_connection)
                .expect("Error.");
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
