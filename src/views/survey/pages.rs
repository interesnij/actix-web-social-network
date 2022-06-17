use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    is_signed_in,
    get_request_user_data,
    get_survey,
    get_community,
    get_survey_list,
    get_user_permission,
    get_anon_user_permission,
    get_community_permission,
    get_anon_community_permission,
    get_list_variables,
};

use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, SurveyList, Survey, Community};


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/survey/load_list/{list_id}/", web::get().to(load_list_page));

    config.route("/survey/add_user_list/", web::get().to(add_user_list_page));
    config.route("/survey/edit_user_list/{id}/", web::get().to(edit_user_list_page));
    config.route("/survey/add_community_list//{id}", web::get().to(add_community_list_page));
    config.route("/survey/edit_community_list/{id}/", web::get().to(edit_community_list_page));

    config.route("/survey/add_survey_in_list/{id}/", web::get().to(add_survey_in_list_page));
    config.route("/survey/edit_survey/{id}/", web::get().to(edit_survey_page));
}

pub async fn load_list_page(session: Session, req: HttpRequest, list_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let is_open : bool;
    let text : String;
    let owner_name : String;
    let owner_link : String;

    let _list = get_survey_list(*list_id);

    let object_list: Vec<Survey>;
    let lists: Vec<SurveyList>;
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = _list.get_paginate_items(20, step.into());
        if _list.count > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = _list.get_paginate_items(20, 0);
        if _list.count > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _list.community_id.is_some() {
            let community = _list.get_community();
            let _tuple = get_community_permission(&community, &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
            lists = community.get_survey_lists();
            owner_name = community.name;
            owner_link = community.link;
        }
        else {
            let creator = _list.get_creator();
            let _tuple = get_user_permission(&_list.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
            lists = creator.get_survey_lists();
            owner_name = creator.get_full_name();
            owner_link = creator.link;
        }

        let _request_user_id = &_request_user.id;
        let is_user_can_see_survey_list = _list.is_user_can_see_el(*_request_user_id);
        let is_user_can_create_surveys = _list.is_user_can_create_el(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

         else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/surveys/list/list.stpl")]
            struct Template {
                list:                        SurveyList,
                request_user:                User,
                is_user_can_see_survey_list: bool,
                is_user_can_create_surveys:  bool,
                object_list:                 Vec<Survey>,
                next_page_number:            i32,
                owner_name:                  String,
                owner_link:                  String,
                lists:                       Vec<SurveyList>,
            }
            let body = Template {
                list:                        _list,
                request_user:                _request_user,
                is_user_can_see_survey_list: is_user_can_see_survey_list,
                is_user_can_create_surveys:  is_user_can_create_surveys,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                owner_name:                  owner_name,
                owner_link:                  owner_link,
                lists:                       lists,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/surveys/list/list.stpl")]
            struct Template {
                list:                        SurveyList,
                request_user:                User,
                is_user_can_see_survey_list: bool,
                is_user_can_create_surveys:  bool,
                object_list:                 Vec<Survey>,
                next_page_number:            i32,
                owner_name:                  String,
                owner_link:                  String,
                lists:                       Vec<SurveyList>,
            }
            let body = Template {
                list:                        _list,
                request_user:                _request_user,
                is_user_can_see_survey_list: is_user_can_see_survey_list,
                is_user_can_create_surveys:  is_user_can_create_surveys,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                owner_name:                  owner_name,
                owner_link:                  owner_link,
                lists:                       lists,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        if _list.community_id.is_some() {
            let community = _list.get_community();
            let _tuple = get_anon_community_permission(&community);
            is_open = _tuple.0;
            text = _tuple.1;
            lists = community.get_survey_lists();
            owner_name = community.name;
            owner_link = community.link;
        }
        else {
            let creator = _list.get_creator();
            let _tuple = get_anon_user_permission(&creator);
            is_open = _tuple.0;
            text = _tuple.1;
            lists = creator.get_survey_lists();
            owner_name = creator.get_full_name();
            owner_link = creator.link;
        }
        let is_user_can_see_survey_list = _list.is_anon_user_can_see_el();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/surveys/list/anon_list.stpl")]
            struct Template {
                list:                        SurveyList,
                is_user_can_see_survey_list: bool,
                object_list:                 Vec<Survey>,
                next_page_number:            i32,
                owner_name:                  String,
                owner_link:                  String,
                lists:                       Vec<SurveyList>,
            }
            let body = Template {
                list:                        _list,
                is_user_can_see_survey_list: is_user_can_see_survey_list,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                owner_name:                  owner_name,
                owner_link:                  owner_link,
                lists:                       lists,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/surveys/list/anon_list.stpl")]
            struct Template {
                list:                        SurveyList,
                is_user_can_see_survey_list: bool,
                object_list:                 Vec<Survey>,
                next_page_number:            i32,
                owner_name:                  String,
                owner_link:                  String,
                lists:                       Vec<SurveyList>,
            }
            let body = Template {
                list:                        _list,
                is_user_can_see_survey_list: is_user_can_see_survey_list,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                owner_name:                  owner_name,
                owner_link:                  owner_link,
                lists:                       lists,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


pub async fn add_user_list_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        #[derive(TemplateOnce)]
        #[template(path = "desctop/surveys/user/add_list.stpl")]
        struct Template {
        }
        let body = Template {
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
    }
}
pub async fn edit_user_list_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        let _list_id : i32 = *_id;
        let list = get_survey_list(_list_id);
        if list.user_id != _request_user.id {
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(""))
        }
        else {

            #[derive(TemplateOnce)]
            #[template(path = "desctop/surveys/user/edit_list.stpl")]
            struct YTemplate {
                list: SurveyList,
            }
            let body = YTemplate {
                list: list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
    }
}
pub async fn add_community_list_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let community = get_community(*_id);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/surveys/community/add_list.stpl")]
        struct Template {
            community: Community,
        }
        let body = Template {
            community: community,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
pub async fn edit_community_list_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let list = get_survey_list(*_id);
        let community = get_community(list.community_id.unwrap());

        #[derive(TemplateOnce)]
        #[template(path = "desctop/surveys/community/edit_list.stpl")]
        struct Template {
            community: Community,
            list: SurveyList,
        }
        let body = Template {
            community: community,
            list: list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body))
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn add_survey_in_list_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let list = get_survey_list(*_id);
        if list.is_user_can_edit_delete_item(_request_user.id) {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/surveys/create_survey.stpl")]
            struct Template {
                list: SurveyList,
            }
            let body = Template {
                list: list,
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

pub async fn edit_survey_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let survey = get_survey(*_id);
        if survey.is_user_can_edit_delete_item(_request_user.id) {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/surveys/edit_survey.stpl")]
            struct Template {
                survey: Survey,
            }
            let body = Template {
                survey: survey,
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
