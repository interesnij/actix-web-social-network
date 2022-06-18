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
    get_photo_list,
    get_photo,
    get_photo_comment,
    get_user_permission,
    get_community_permission,
    establish_connection,
    NewListValues,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{PhotoList, Photo, PhotoComment, EditPhotoDescription};
use serde::{Deserialize, Serialize};
use std;

use std::{borrow::BorrowMut, io::Write};
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use crate::diesel::{RunQueryDsl,ExpressionMethods};


pub fn progs_urls(config: &mut web::ServiceConfig) {
    config.route("/photos/add_user_list/", web::post().to(add_user_photo_list));
    config.route("/photos/edit_user_list/{id}/", web::post().to(edit_user_photo_list));
    config.route("/photos/add_community_list/{id}/", web::post().to(add_community_photo_list));
    config.route("/photos/edit_community_list/{id}/", web::post().to(edit_community_photo_list));
    config.route("/photos/delete_user_list/{id}/", web::get().to(delete_user_photo_list));
    config.route("/photos/recover_user_list/{id}/", web::get().to(recover_user_photo_list));
    config.route("/photos/delete_community_list/{id}/", web::get().to(delete_community_photo_list));
    config.route("/photos/recover_community_list/{id}/", web::get().to(recover_community_photo_list));

    config.route("/photos/add_photos_in_list/{id}/", web::post().to(add_photos_in_list));
    config.route("/photos/edit_photo_description/{id}/", web::post().to(edit_photo_description));
    config.route("/photos/delete_photo/{id}/", web::get().to(delete_photo));
    config.route("/photos/recover_photo/{id}/", web::get().to(recover_photo));
    config.route("/photos/on_comment/{id}/", web::get().to(on_comment));
    config.route("/photos/off_comment/{id}/", web::get().to(off_comment));

    config.route("/photos/add_comment/{id}/", web::post().to(add_comment));
    config.route("/photos/add_reply/{id}/", web::post().to(add_reply));
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ImageForm {
    pub images: Vec<String>,
}

pub async fn images_form (
    payload: &mut Multipart,
    owner_path: String,
    owner_id: String
) -> ImageForm {
    use crate::utils::UploadedFiles;

    let mut form: ImageForm = ImageForm {
        images: Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        let _new_path = field.content_disposition().get_filename().unwrap();
        let file = UploadedFiles::new (
            owner_path.clone(),
            owner_id.to_string(),
            "photos".to_string(),
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
            form.images.push(file.path.clone().replace("./","/"));
        };
    }
    form
}

pub async fn add_photos_in_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _list = get_photo_list(*_id);
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

        else if _list.is_user_can_create_el(_request_user.id) {
            let form = images_form (
                payload.borrow_mut(),
                owner_path,
                owner_id.to_string()
            ).await;

            let mut image_list = Vec::new();
            let _connection = establish_connection();
            let mut count = 0;
            for image in form.images.iter() {
                let new_photo = _list.create_photo (
                    community_id,
                    _request_user.id,
                    image.to_string(),
                    image.to_string()
                );
                image_list.push(new_photo);
                count += 1;
            };
            diesel::update(&_list)
              .set(schema::photo_lists::count.eq(_list.count + count))
              .get_result::<PhotoList>(&_connection)
              .expect("Error.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/photos/new_photos.stpl")]
            struct Template {
                object_list: Vec<Photo>,
            }
            let body = Template {
                object_list: image_list,
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

pub async fn add_user_photo_list(session: Session, mut payload: Multipart) -> web::Json<NewListValues> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let _request_user = get_request_user_data(&session);
        let form = post_list_form(
            payload.borrow_mut(),
            "users".to_string(),
            _request_user.id.to_string()
        ).await;
        let new_list = PhotoList::create_list (
            _request_user,
            form.name,
            form.description,
            None,
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
            image: new_list.cover_photo,
        })

    } else {
        return Json(NewListValues {
            pk: 0,
            name: "".to_string(),
            image: None,
        })
    }
}

pub async fn edit_user_photo_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> web::Json<NewListValues> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let list = get_photo_list(*_id);
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
                image: edit_list.cover_photo.clone(),
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

pub async fn add_community_photo_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> web::Json<NewListValues> {
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
            let new_list = PhotoList::create_list (
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
                image: new_list.cover_photo,
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

pub async fn edit_community_photo_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> web::Json<NewListValues> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let list = get_photo_list(*_id);
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
                image: edit_list.cover_photo.clone(),
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


pub async fn delete_user_photo_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_photo_list(*_id);
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

pub async fn recover_user_photo_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_photo_list(*_id);
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

pub async fn delete_community_photo_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_photo_list(*_id);
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

pub async fn recover_community_photo_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_photo_list(*_id);
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

pub async fn edit_photo_description(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> web::Json<EditPhotoDescription> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _photo = get_photo(*_id);
        let _list = _photo.get_list();

        let is_open : bool;
        let community_id = _list.community_id;

        if community_id.is_some() {
            let _tuple = get_community_permission(&_list.get_community(), &_request_user);
            is_open = _tuple.0;
        }
        else {
            let _tuple = get_user_permission(&_list.get_creator(), &_request_user);
            is_open = _tuple.0;
        }
        if is_open == false {
            return Json(EditPhotoDescription {
                description: None,
            })
        }
        else if _photo.is_user_can_edit_delete_item(_request_user.id) {
            let mut form: EditPhotoDescription = EditPhotoDescription {
                description: None,
            };

            while let Some(item) = payload.next().await {
                let mut field: Field = item.expect("split_payload err");
                while let Some(chunk) = field.next().await {
                    let data = chunk.expect("split_payload err chunk");
                    if let Ok(s) = std::str::from_utf8(&data) {
                        let data_string = s.to_string();
                        if field.name() == "description" {
                            form.description = Some(data_string);
                        }
                    }
                }
            }

            let _connection = establish_connection();
            diesel::update(&_photo)
                .set(schema::photos::description.eq(form.description))
                .get_result::<Photo>(&_connection)
                .expect("Error.");
            return Json(EditPhotoDescription {
                description: _photo.description,
            })
        } else {
            return Json(EditPhotoDescription {
                description: None,
            })
        }
    } else {
        return Json(EditPhotoDescription {
            description: None,
        })
    }
}


pub async fn delete_photo(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let photo = get_photo(*_id);
        let _request_user = get_request_user_data(&session);
        if photo.is_user_can_edit_delete_item(_request_user.id) {
            photo.delete_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn recover_photo(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let photo = get_photo(*_id);
        let _request_user = get_request_user_data(&session);
        if photo.is_user_can_edit_delete_item(_request_user.id) {
            photo.restore_item();
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
        let photo = get_photo(*_id);
        let _request_user = get_request_user_data(&session);
        if photo.is_user_can_edit_delete_item(_request_user.id) {
            let _connection = establish_connection();

            diesel::update(&photo)
                .set(schema::photos::comment_enabled.eq(true))
                .get_result::<Photo>(&_connection)
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
        let photo = get_photo(*_id);
        let _request_user = get_request_user_data(&session);
        if photo.is_user_can_edit_delete_item(_request_user.id) {

            let _connection = establish_connection();
            diesel::update(&photo)
                .set(schema::photos::comment_enabled.eq(false))
                .get_result::<Photo>(&_connection)
                .expect("Error.");
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
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
        let item = get_photo(*_id);
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
            comment: PhotoComment,
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
        let comment = get_photo_comment(*_id);
        let item = get_photo(comment.photo_id);
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
            reply:           PhotoComment,
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
