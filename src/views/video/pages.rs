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
    get_video_list,
    get_user_permission,
    get_anon_user_permission,
    get_community_permission,
    get_anon_community_permission,
    get_list_variables,
};

use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, VideoList, Video};


pub fn videos_urls(config: &mut web::ServiceConfig) {
    config.route("/video/load_list/{list_id}/", web::get().to(load_list_page));
}

pub async fn load_list_page(session: Session, req: HttpRequest, list_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let mut is_open = false;
    let mut text = "".to_string();

    let _list = get_video_list(*list_id);

    let object_list: Vec<Video>;
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
        let is_user_can_see_video_list = _list.is_user_can_see_el(*_request_user_id);
        let is_user_can_create_videos = _list.is_user_can_create_el(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/video/list/list.stpl")]
            struct Template {
                list:         VideoList,
                request_user: User,
                is_user_can_see_video_list: bool,
                is_user_can_create_videos: bool,
                object_list: Vec<Video>,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_video_list: is_user_can_see_video_list,
                is_user_can_create_videos:  is_user_can_create_videos,
                object_list: object_list,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/video/list/list.stpl")]
            struct Template {
                list:         VideoList,
                request_user: User,
                is_user_can_see_video_list: bool,
                is_user_can_create_videos: bool,
                object_list: Vec<Video>,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_video_list: is_user_can_see_video_list,
                is_user_can_create_videos:  is_user_can_create_videos,
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
        let is_user_can_see_video_list = _list.is_anon_user_can_see_el();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/video/list/anon_list.stpl")]
            struct Template {
                list:         VideoList,
                is_user_can_see_video_list: bool,
                object_list: Vec<Video>,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                is_user_can_see_video_list: is_user_can_see_video_list,
                object_list: object_list,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/video/list/anon_list.stpl")]
            struct Template {
                list:         VideoList,
                is_user_can_see_video_list: bool,
                object_list: Vec<Video>,
                next_page_number: i32,
            }
            let body = Template {
                list:                      _list,
                is_user_can_see_video_list: is_user_can_see_video_list,
                object_list: object_list,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}