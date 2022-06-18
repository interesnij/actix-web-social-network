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
    get_music_list,
    get_music,
    get_community_permission,
    get_user_permission,
    NewListValues,
    establish_connection,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{
    //User,
    MusicList,
    Music
};
use serde::{Deserialize, Serialize};

use std::str;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::{borrow::BorrowMut, io::Write};
use crate::diesel::{RunQueryDsl,ExpressionMethods};


pub fn progs_urls(config: &mut web::ServiceConfig) {
    config.route("/music/add_user_list/", web::post().to(add_user_list));
    config.route("/music/edit_user_list/{id}/", web::post().to(edit_user_list));
    config.route("/music/add_community_list/{id}/", web::post().to(add_community_list));
    config.route("/music/edit_community_list/{id}/", web::post().to(edit_community_list));
    config.route("/music/delete_user_list/{id}/", web::get().to(delete_user_list));
    config.route("/music/recover_user_list/{id}/", web::get().to(recover_user_list));
    config.route("/music/delete_community_list/{id}/", web::get().to(delete_community_list));
    config.route("/music/recover_community_list/{id}/", web::get().to(recover_community_list));

    config.route("/music/add_tracks_in_list/{id}/", web::post().to(add_tracks_in_list));
    config.route("/music/edit_track/{id}/", web::post().to(edit_track));
    config.route("/music/delete_track/{id}/", web::get().to(delete_track));
    config.route("/music/recover_track/{id}/", web::get().to(recover_track));
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
        let new_list = MusicList::create_list (
            _request_user,
            form.name,
            form.description,
            form.image,
            None,
            form.can_see_el,
            form.create_el,
            form.copy_el,
            Some(form.can_see_el_users),
            Some(form.create_el_users),
            Some(form.copy_el_users),
        );
        return Json(NewListValues {
            pk: new_list.id,
            name: new_list.name,
            image: new_list.image,
        })
    }
    else {
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

        let list = get_music_list(*_id);
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
                form.create_el,
                form.copy_el,
                Some(form.can_see_el_users),
                Some(form.create_el_users),
                Some(form.copy_el_users),
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
            let new_list = MusicList::create_list (
                _request_user,
                form.name,
                form.description,
                form.image,
                Some(*_id),
                form.can_see_el,
                form.create_el,
                form.copy_el,
                Some(form.can_see_el_users),
                Some(form.create_el_users),
                Some(form.copy_el_users),
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

        let list = get_music_list(*_id);
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
                form.create_el,
                form.copy_el,
                Some(form.can_see_el_users),
                Some(form.create_el_users),
                Some(form.copy_el_users),
            );

            return Json(NewListValues {
                pk: edit_list.id,
                name: edit_list.name.clone(),
                image: edit_list.image.clone(),
            })
        }
        else {
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

        let list = get_music_list(*_id);
        let _request_user = get_request_user_data(&session);
        if list.user_id == _request_user.id {
            list.delete_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn recover_user_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_music_list(*_id);
        let _request_user = get_request_user_data(&session);
        if list.user_id == _request_user.id {
            list.restore_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn delete_community_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_music_list(*_id);
        let _request_user = get_request_user_data(&session);
        if _request_user.is_administrator_of_community(list.community_id.unwrap()) {
            list.delete_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn recover_community_list(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let list = get_music_list(*_id);
        let _request_user = get_request_user_data(&session);
        if _request_user.is_administrator_of_community(list.community_id.unwrap()) {
            list.restore_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn add_tracks_in_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _list = get_music_list(*_id);
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
            use crate::utils::UploadedFiles;

            #[derive(Deserialize, Serialize, Debug)]
            pub struct TracksForm {
                pub files: Vec<String>,
            }

            let mut form: TracksForm = TracksForm {
                files: Vec::new(),
            };

            while let Some(item) = payload.next().await {
                let mut field: Field = item.expect("split_payload err");

                let _new_path = field.content_disposition().get_filename().unwrap();
                let file = UploadedFiles::new (
                    owner_path.clone(),
                    owner_id.to_string(),
                    "music".to_string(),
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
                form.files.push(file.path.clone().replace("./","/"));
            }

            let mut files_list = Vec::new();
            let mut count = 0;
            let _connection = establish_connection();
            for file in form.files.iter() {
                let v: Vec<&str> = file.split('/').collect();
                let filename = v.last().unwrap().to_string();
                let new_track = _list.create_track (
                    filename,
                    community_id,
                    _request_user.id,
                    None,
                    None,
                    file.to_string(),
                    None,
                );
                files_list.push(new_track);
                count += 1;
            }

            diesel::update(&_list)
              .set(schema::music_lists::count.eq(_list.count + count))
              .get_result::<MusicList>(&_connection)
              .expect("Error.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/music/tracks_list.stpl")]
            struct Template {
                object_list: Vec<Music>,
            }
            let body = Template {
                object_list: files_list,
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

#[derive(Deserialize, Serialize)]
pub struct EditTrackResponse {
    pub title: String,
    pub image: Option<String>,
}
pub async fn edit_track(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> web::Json<EditTrackResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _track = get_music(*_id);
        let _list = _track.get_list();
        let community_id = _track.community_id;
        let owner_path : String;
        let owner_id: i32;
        let is_open : bool;

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
            return Json(EditTrackResponse {
                title: "".to_string(),
                image: None,
            })
        }
        else if _track.is_user_can_edit_delete_item(_request_user.id) {
            use crate::models::EditMusic;

            let mut form: EditMusic = EditMusic {
                title: "".to_string(),
                genre_id: None,
                album_id: None,
                image: None,
            };

            while let Some(item) = payload.next().await {
                let mut field: Field = item.expect("split_payload err");

                if field.name() == "title" {
                    while let Some(chunk) = field.next().await {
                        let data = chunk.expect("split_payload err chunk");
                        if let Ok(s) = str::from_utf8(&data) {
                            let data_string = s.to_string();
                            form.title = data_string;
                        }
                    }
                }
                else if field.name() == "image" {
                    use crate::utils::UploadedFiles;

                    let _new_path = field.content_disposition().get_filename().unwrap();
                    let file = UploadedFiles::new (
                        owner_path.clone(),
                        owner_id.to_string(),
                        "music".to_string(),
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
                    form.image = Some(file.path.clone().replace("./","/"));
                }
            }

            let _connection = establish_connection();
            diesel::update(&_track)
                .set(&form)
                .get_result::<Music>(&_connection)
                .expect("Error.");

            return Json(EditTrackResponse {
                title: form.title,
                image: form.image,
            })
        } else {
            return Json(EditTrackResponse {
                title: "".to_string(),
                image: None,
            })
        }
    } else {
        return Json(EditTrackResponse {
            title: "".to_string(),
            image: None,
        })
    }
}

pub async fn delete_track(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let track = get_music(*_id);
        let _request_user = get_request_user_data(&session);
        if track.is_user_can_edit_delete_item(_request_user.id) {
            track.delete_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn recover_track(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let track = get_music(*_id);
        let _request_user = get_request_user_data(&session);
        if track.is_user_can_edit_delete_item(_request_user.id) {
            track.restore_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
