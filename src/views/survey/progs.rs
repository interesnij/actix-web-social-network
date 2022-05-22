use actix_web::{
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use crate::utils::{
    is_signed_in,
    get_request_user_data,
    get_community,
    get_survey_list,
    get_survey,
    get_community_permission,
    get_user_permission,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, SurveyList, Survey};
use serde::{Deserialize, Serialize};

use std::str;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::{borrow::BorrowMut, io::Write};


pub fn progs_urls(config: &mut web::ServiceConfig) {
    config.route("/surveys/add_user_list/", web::post().to(add_user_list));
    config.route("/surveys/edit_user_list/{id}/", web::post().to(edit_user_list));
    config.route("/surveys/add_community_list/{id}/", web::post().to(add_community_list));
    config.route("/surveys/edit_community_list/{id}/", web::post().to(edit_community_list));
    config.route("/surveys/delete_user_list/{id}/", web::get().to(delete_user_list));
    config.route("/surveys/recover_user_list/{id}/", web::get().to(recover_user_list));
    config.route("/surveys/delete_community_list/{id}/", web::get().to(delete_community_list));
    config.route("/surveys/recover_community_list/{id}/", web::get().to(recover_community_list));

    config.route("/surveys/add_survey_in_list/{id}/", web::post().to(add_survey_in_list));
    config.route("/surveys/edit_survey/{id}/", web::post().to(edit_survey));
    config.route("/surveys/delete_survey/{id}/", web::post().to(delete_survey));
    config.route("/surveys/recover_survey/{id}/", web::post().to(recover_survey));
}

pub async fn add_user_list(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::post_list_form;

        let _request_user = get_request_user_data(session);
        let form = post_list_form(payload.borrow_mut()).await;
        let new_list = SurveyList::create_list (
            _request_user,
            form.name,
            form.description,
            None,
            form.can_see_el,
            form.create_el,
            form.copy_el,
            Some(form.can_see_el_users),
            Some(form.create_el_users),
            Some(form.copy_el_users),
        );

        #[derive(TemplateOnce)]
        #[template(path = "desctop/surveys/user/new_list.stpl")]
        struct Template {
            list: SurveyList,
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

        let list = get_survey_list(*_id);
        let _request_user = get_request_user_data(session);
        if list.user_id == _request_user.id {
            let form = post_list_form(payload.borrow_mut()).await;
            list.edit_list (
                form.name,
                form.description,
                form.can_see_el,
                form.create_el,
                form.copy_el,
                Some(form.can_see_el_users),
                Some(form.create_el_users),
                Some(form.copy_el_users),
            );

            #[derive(TemplateOnce)]
            #[template(path = "desctop/surveys/user/new_list.stpl")]
            struct Template {
                list: SurveyList,
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
            let new_list = SurveyList::create_list (
                _request_user,
                form.name,
                form.description,
                Some(*_id),
                form.can_see_el,
                form.create_el,
                form.copy_el,
                Some(form.can_see_el_users),
                Some(form.create_el_users),
                Some(form.copy_el_users),
            );

            #[derive(TemplateOnce)]
            #[template(path = "desctop/surveys/community/new_list.stpl")]
            struct Template {
                list: SurveyList,
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

        let list = get_survey_list(*_id);
        let community = get_community(list.community_id.unwrap());
        let _request_user = get_request_user_data(session);
        if community.get_administrators_ids().iter().any(|&i| i==_request_user.id) {
            let form = post_list_form(payload.borrow_mut()).await;
            list.edit_list (
                form.name,
                form.description,
                form.can_see_el,
                form.create_el,
                form.copy_el,
                Some(form.can_see_el_users),
                Some(form.create_el_users),
                Some(form.copy_el_users),
            );

        #[derive(TemplateOnce)]
        #[template(path = "desctop/surveys/community/new_list.stpl")]
        struct Template {
            list: SurveyList,
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

        let list = get_survey_list(*_id);
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

        let list = get_survey_list(*_id);
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

        let list = get_survey_list(*_id);
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

        let list = get_survey_list(*_id);
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
pub struct SurveyForm {
    pub title: String,
    pub image: Option<String>,
    pub is_anonymous: bool,
    pub is_multiple: bool,
    pub is_no_edited: bool,
    pub time_end: Option<String>,
}

pub async fn survey_form(
    payload: &mut Multipart,
    owner_path: String,
    owner_id: String
) -> SurveyForm {
    use crate::utils::UploadedFiles;
    //use uuid::Uuid;

    let mut form: SurveyForm = SurveyForm {
        title: "".to_string(),
        image: None,
        is_anonymous: true,
        is_multiple: true,
        is_no_edited: true,
        time_end: None,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if field.name() == "image"{
            let _new_path = field.content_disposition().get_filename().unwrap();
            let file = UploadedFiles::new (
                owner_path.clone(),
                owner_id.to_string(),
                "surveys".to_string(),
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
        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string;
                    }
                    else if field.name() == "time_end" {
                        form.time_end = Some(data_string);
                    }
                    else if field.name() == "is_anonymous" {
                        if data_string == "on" {
                            form.is_anonymous = true;
                        } else {
                            form.is_anonymous = false;
                        }
                    }
                    else if field.name() == "is_multiple" {
                        if data_string == "on" {
                            form.is_multiple = true;
                        } else {
                            form.is_multiple = false;
                        }
                    }
                    else if field.name() == "is_no_edited" {
                        if data_string == "on" {
                            form.is_no_edited = true;
                        } else {
                            form.is_no_edited = false;
                        }
                    }
                }
            }
        }
    }
    form
}

pub async fn add_survey_in_list(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let user_id = _request_user.id;
        let _list = get_survey_list(*_id);

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
            let form = survey_form(
                payload.borrow_mut(),
                owner_path,
                owner_id.to_string()
            ).await;
            let new_survey = _list.create_survey (
                form.title,
                None,
                _request_user.id,
                form.image,
                form.is_anonymous,
                form.is_multiple,
                form.is_no_edited,
                form.time_end,
            );

            #[derive(TemplateOnce)]
            #[template(path = "desctop/surveys/new_item.stpl")]
            struct Template {
                object: Survey,
                request_user: User,
            }
            let body = Template {
                object: new_survey,
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

pub async fn edit_survey(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let user_id = _request_user.id;
        let _survey = get_survey(*_id);
        let _list = _survey.get_list();

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
        else if _survey.is_user_can_edit_delete_item(_request_user.id) {
            let form = survey_form(
                payload.borrow_mut(),
                owner_path,
                owner_id.to_string()
            ).await;
            _survey.edit_survey (
                form.title,
                form.image,
                form.is_anonymous,
                form.is_multiple,
                form.is_no_edited,
                form.time_end,
            );

            #[derive(TemplateOnce)]
            #[template(path = "desctop/surveys/new_item.stpl")]
            struct Template {
                object: Survey,
                request_user: User,
            }
            let body = Template {
                object: _survey,
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

pub async fn delete_survey(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let survey = get_survey(*_id);
        let _request_user = get_request_user_data(session);
        if survey.is_user_can_edit_delete_item(_request_user.id) {
            survey.delete_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn recover_survey(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let survey = get_survey(*_id);
        let _request_user = get_request_user_data(session);
        if survey.is_user_can_edit_delete_item(_request_user.id) {
            survey.restore_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
