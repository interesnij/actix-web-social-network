use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    is_signed_in,
    establish_connection,
    is_desctop,
    get_request_user_data,
    get_user,
    get_community,
    get_doc_list,
    get_user_permission,
    get_anon_user_permission,
    get_community_permission,
    get_anon_community_permission,
    get_list_variables,
};

use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, DocList, Doc};


pub fn docs_urls(config: &mut web::ServiceConfig) {
    config.route("/docs/load_list/{list_id}/", web::get().to(load_list_page));
}

pub async fn load_list_page(session: Session, req: HttpRequest, list_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    let _list = get_doc_list(*list_id);

    let object_list: Vec<Doc>;
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
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        let _request_user_id = &_request_user.id;
        let is_user_can_see_doc_list = _list.is_user_can_see_el(_request_user_id);
        let is_user_can_create_docs = _list.is_community_can_create_el(_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/docs/list.stpl")]
            struct Template {
                list:         DocList,
                request_user: User,
                is_user_can_see_doc_list: bool,
                is_user_can_create_docs: bool,
                object_list: Vec<Doc>,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_doc_list: is_user_can_see_doc_list,
                is_user_can_create_docs:  is_user_can_create_docs,
                object_list: object_list,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/docs/list.stpl")]
            struct Template {
                list:         DocList,
                request_user: User,
                is_user_can_see_doc_list: bool,
                is_user_can_create_docs: bool,
                object_list: Vec<Doc>,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_doc_list: is_user_can_see_doc_list,
                is_user_can_create_docs:  is_user_can_create_docs,
                object_list: object_list,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        let is_user_can_see_post_list = _list.is_anon_user_can_see_el();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/docs/anon_list.stpl")]
            struct Template {
                list:         DocList,
                is_user_can_see_doc_list: bool,
                object_list: Vec<Doc>,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                is_user_can_see_doc_list: is_user_can_see_doc_list,
                object_list: object_list,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/docs/anon_list.stpl")]
            struct Template {
                list:         DocList,
                is_user_can_see_doc_list: bool,
                object_list: Vec<Doc>,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                is_user_can_see_doc_list: is_user_can_see_doc_list,
                object_list: object_list,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
