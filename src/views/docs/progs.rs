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
    get_doc_list,
    get_doc,
    get_community_permission,
    get_user_permission,
    NewListValues,
    establish_connection,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, DocList, Doc, EditDoc};
use serde::{Deserialize, Serialize};

use std::str;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::{borrow::BorrowMut, io::Write};
use crate::diesel::{RunQueryDsl,ExpressionMethods};


pub fn progs_urls(config: &mut web::ServiceConfig) {
    config.route("/docs/add_user_list/", web::post().to(add_user_list));
    config.route("/docs/edit_user_list/{id}/", web::post().to(edit_user_list));
    config.route("/docs/add_community_list/{id}/", web::post().to(add_community_list));
    config.route("/docs/edit_community_list/{id}/", web::post().to(edit_community_list));
    config.route("/docs/delete_user_list/{id}/", web::get().to(delete_user_list));
    config.route("/docs/recover_user_list/{id}/", web::get().to(recover_user_list));
    config.route("/docs/delete_community_list/{id}/", web::get().to(delete_community_list));
    config.route("/docs/recover_community_list/{id}/", web::get().to(recover_community_list));

    config.route("/docs/add_docs_in_list/{id}/", web::post().to(add_doc_in_list));
    config.route("/docs/edit_doc/{id}/", web::post().to(edit_doc));
    config.route("/docs/delete_doc/{id}/", web::get().to(delete_doc));
    config.route("/docs/recover_doc/{id}/", web::get().to(recover_doc));
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
        let new_list = DocList::create_list (
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

        let list = get_doc_list(*_id);
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
        }
        else {
            return Json(NewListValues {
                pk: 0,
                name: "".to_string(),
                image: None,
            })
        }
    }
    else {
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
            let new_list = DocList::create_list (
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

        let list = get_doc_list(*_id);
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

        let list = get_doc_list(*_id);
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

        let list = get_doc_list(*_id);
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

        let list = get_doc_list(*_id);
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

        let list = get_doc_list(*_id);
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
pub struct DocsForm {
    pub files: Vec<String>,
}

pub async fn docs_form (
    payload: &mut Multipart,
    owner_path: String,
    owner_id: String
) -> DocsForm {
    use crate::utils::UploadedFiles;
    //use uuid::Uuid;

    let mut form: DocsForm = DocsForm {
        files: Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");


        let _new_path = field.content_disposition().get_filename().unwrap();
        let file = UploadedFiles::new (
            owner_path.clone(),
            owner_id.to_string(),
            "docs".to_string(),
            _new_path.to_string(),
        );
        println!("content_type {:?}", &_new_path);
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
    form
}

pub async fn add_doc_in_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _list = get_doc_list(*_id);
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
            let form = docs_form(
                payload.borrow_mut(),
                owner_path,
                owner_id.to_string()
            ).await;

            let mut files_list = Vec::new();
            let _connection = establish_connection();
            let mut count = 0;
            for file in form.files.iter() {
                let v: Vec<&str> = file.split('/').collect();
                let filename = v.last().unwrap().to_string();
                let new_doc = _list.create_doc (
                    filename,
                    community_id,
                    _request_user.id,
                    "a".to_string(),
                    file.to_string(),
                );
                files_list.push(new_doc);
                count += 1;
            }

            diesel::update(&_list)
              .set(schema::doc_lists::count.eq(_list.count + count))
              .get_result::<DocList>(&_connection)
              .expect("Error.");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/docs/new_docs.stpl")]
            struct Template {
                object_list: Vec<Doc>,
                request_user: User,
            }
            let body = Template {
                object_list: files_list,
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



pub async fn edit_doc(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> web::Json<EditDoc> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _doc = get_doc(*_id);
        let _list = _doc.get_list();
        let community_id = _doc.community_id;
        let is_open : bool;

        if community_id.is_some() {
            let _tuple = get_community_permission(&_list.get_community(), &_request_user);
            is_open = _tuple.0;
        }
        else {
            let _tuple = get_user_permission(&_list.get_creator(), &_request_user);
            is_open = _tuple.0;
        }

        if is_open == false {
            return Json(EditDoc {
                title: "".to_string(),
                types_2: "".to_string(),
            })
        }
        else if _doc.is_user_can_edit_delete_item(_request_user.id) {
            let mut form: EditDoc = EditDoc {
                title: "".to_string(),
                types_2: "".to_string(),
            };

            while let Some(item) = payload.next().await {
                let mut field: Field = item.expect("split_payload err");

                while let Some(chunk) = field.next().await {
                    let data = chunk.expect("split_payload err chunk");
                    if let Ok(s) = str::from_utf8(&data) {
                        let data_string = s.to_string();
                        if field.name() == "title" {
                            form.title = data_string;
                        }
                        else if field.name() == "types_2" {
                            form.types_2 = data_string;
                        }
                    }
                }
            }
            let _connection = establish_connection();
            diesel::update(&_doc)
                .set(&form)
                .get_result::<Doc>(&_connection)
                .expect("Error.");

            return Json(EditDoc {
                title: form.title,
                types_2: form.types_2,
            })
        } else {
            return Json(EditDoc {
                title: "".to_string(),
                types_2: "".to_string(),
            })
        }
    } else {
        return Json(EditDoc {
            title: "".to_string(),
            types_2: "".to_string(),
        })
    }
}


pub async fn delete_doc(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let doc = get_doc(*_id);
        let _request_user = get_request_user_data(&session);
        if doc.is_user_can_edit_delete_item(_request_user.id) {
            doc.delete_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn recover_doc(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let doc = get_doc(*_id);
        let _request_user = get_request_user_data(&session);
        if doc.is_user_can_edit_delete_item(_request_user.id) {
            doc.restore_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
