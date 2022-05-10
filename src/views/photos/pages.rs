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
use crate::models::{User, PhotoList, Photo};


pub fn photos_urls(config: &mut web::ServiceConfig) {
    config.route("/photos/load_list/{list_id}/", web::get().to(load_list_page));
}

pub async fn load_list_page(session: Session, req: HttpRequest, list_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let mut is_open = false;
    let mut text = "".to_string();

    let _list = get_photo_list(*list_id);

    let object_list: Vec<Photo>;
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
        if _list.community_id.is_some() {
            let _tuple = get_community_permission(&_list.get_community(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_user_permission(&_list.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }

        let _request_user_id = &_request_user.id;
        let is_user_can_see_photo_list = _list.is_user_can_see_el(*_request_user_id);
        let is_user_can_create_photos = _list.is_user_can_create_el(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/photos/list/list.stpl")]
            struct Template {
                list:         PhotoList,
                request_user: User,
                is_user_can_see_photo_list: bool,
                is_user_can_create_photos: bool,
                object_list: Vec<Photo>,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_photo_list: is_user_can_see_photo_list,
                is_user_can_create_photos:  is_user_can_create_photos,
                object_list: object_list,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/photos/list/list.stpl")]
            struct Template {
                list:         PhotoList,
                request_user: User,
                is_user_can_see_photo_list: bool,
                is_user_can_create_photos: bool,
                object_list: Vec<Photo>,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_photo_list: is_user_can_see_photo_list,
                is_user_can_create_photos:  is_user_can_create_photos,
                object_list: object_list,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        if _list.community_id.is_some() {
            let _tuple = get_anon_community_permission(&_list.get_community());
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_anon_user_permission(&_list.get_creator());
            is_open = _tuple.0;
            text = _tuple.1;
        }
        let is_user_can_see_photo_list = _list.is_anon_user_can_see_el();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/photos/list/anon_list.stpl")]
            struct Template {
                list:         PhotoList,
                is_user_can_see_photo_list: bool,
                object_list: Vec<Photo>,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                is_user_can_see_photo_list: is_user_can_see_photo_list,
                object_list: object_list,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/photos/list/anon_list.stpl")]
            struct Template {
                list:         PhotoList,
                is_user_can_see_photo_list: bool,
                object_list: Vec<Photo>,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                is_user_can_see_photo_list: is_user_can_see_photo_list,
                object_list: object_list,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
