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
    get_community,
    get_video_list,
    get_video,
    get_user_permission,
    get_anon_user_permission,
    get_community_permission,
    get_anon_community_permission,
    get_list_variables,
};

use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, VideoList, Video, VideoComment, Community};


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/video/load_list/{list_id}/", web::get().to(load_list_page));
    config.route("/video/load_video/{id}/", web::get().to(load_video_page));
    config.route("/video/load_comments/{id}/", web::get().to(load_comments_page));

    config.route("/video/add_user_list/", web::get().to(add_user_list_page));
    config.route("/video/edit_user_list/{id}/", web::get().to(edit_user_list_page));
    config.route("/video/add_community_list//{id}", web::get().to(add_community_list_page));
    config.route("/video/edit_community_list/{id}/", web::get().to(edit_community_list_page));
    config.route("/video/edit_video/{id}/", web::get().to(edit_video_page));
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


pub async fn add_user_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        #[derive(TemplateOnce)]
        #[template(path = "desctop/video/user/add_list.stpl")]
        struct Template {
            request_user: User,
        }
        let body = Template {
            request_user: _request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
    }
}
pub async fn edit_user_list_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(session);
        let _list_id : i32 = *_id;
        let list = get_video_list(_list_id);
        if list.user_id != _request_user.id {
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(""))
        }
        else {

            #[derive(TemplateOnce)]
            #[template(path = "desctop/video/user/edit_list.stpl")]
            struct YTemplate {
                request_user: User,
                list: VideoList,
            }
            let body = YTemplate {
                request_user: _request_user,
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
pub async fn add_community_list_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let community = get_community(*_id);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/video/community/add_list.stpl")]
        struct Template {
            request_user: User,
            community: Community,
        }
        let body = Template {
            request_user: _request_user,
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
pub async fn edit_community_list_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let list = get_video_list(*_id);
        let community = get_community(list.community_id.unwrap());

        #[derive(TemplateOnce)]
        #[template(path = "desctop/video/community/edit_list.stpl")]
        struct Template {
            request_user: User,
            community: Community,
            list: VideoList,
        }
        let body = Template {
            request_user: _request_user,
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

pub async fn load_video_page(session: Session, req: HttpRequest, video_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let mut is_open = false;
    let mut text = "".to_string();
    let mut prev: Option<i32> = None;
    let mut next: Option<i32> = None;

    let _video = get_video(*video_id);
    let _list = get_video_list(_video.video_list_id);

    let _videos = _list.get_items();
    for (i, item) in _videos.iter().enumerate().rev() {
        if item.id == _video.id {
            if (i + 1) != _videos.len() {
                prev = Some(_videos[i + 1].id);
            };
            if i != 0 {
                next = Some(_videos[i - 1].id);
            };
            break;
        }
    };

    let object_list: Vec<VideoComment>;
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = _video.get_comments(20, step.into());
        if _video.comment > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = _video.get_comments(20, 0);
        if _video.comment > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        if _video.community_id.is_some() {
            let _tuple = get_community_permission(&_video.get_community(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_user_permission(&_video.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }

        let _request_user_id = &_request_user.id;
        let is_user_can_see_video_list = _list.is_user_can_see_el(*_request_user_id);
        let is_user_can_see_comments = _list.is_user_can_see_comment(*_request_user_id);
        let is_user_can_create_comments = _list.is_user_can_create_comment(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/video/load/video.stpl")]
            struct Template {
                list:                        VideoList,
                object:                      Video,
                request_user:                User,
                is_user_can_see_video_list:  bool,
                is_user_can_see_comments:    bool,
                is_user_can_create_comments: bool,
                object_list:                 Vec<VideoComment>,
                next_page_number:            i32,
                prev:                        Option<i32>,
                next:                        Option<i32>,
            }
            let body = Template {
                list:                       _list,
                object:                     _video,
                request_user:               _request_user,
                is_user_can_see_video_list:   is_user_can_see_video_list,
                is_user_can_see_comments:    is_user_can_see_comments,
                is_user_can_create_comments: is_user_can_create_comments,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                prev:                        prev,
                next:                        next,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/video/load/video.stpl")]
            struct Template {
                list:                        VideoList,
                object:                      Video,
                request_user:                User,
                is_user_can_see_video_list:  bool,
                is_user_can_see_comments:    bool,
                is_user_can_create_comments: bool,
                object_list:                 Vec<VideoComment>,
                next_page_number:            i32,
                prev:                        Option<i32>,
                next:                        Option<i32>,
            }
            let body = Template {
                list:                        _list,
                object:                      _video,
                request_user:                _request_user,
                is_user_can_see_video_list:   is_user_can_see_video_list,
                is_user_can_see_comments:    is_user_can_see_comments,
                is_user_can_create_comments: is_user_can_create_comments,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                prev:                        prev,
                next:                        next,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        if _video.community_id.is_some() {
            let _tuple = get_anon_community_permission(&_video.get_community());
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_anon_user_permission(&_video.get_creator());
            is_open = _tuple.0;
            text = _tuple.1;
        }
        let is_user_can_see_video_list = _list.is_anon_user_can_see_el();
        let is_user_can_see_comments = _list.is_anon_user_can_see_comment();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/video/load/anon_video.stpl")]
            struct Template {
                list:                      VideoList,
                object:                    Video,
                is_user_can_see_video_list: bool,
                is_user_can_see_comments:  bool,
                object_list:               Vec<VideoComment>,
                next_page_number:          i32,
                prev:                      Option<i32>,
                next:                      Option<i32>,
            }
            let body = Template {
                list:                      _list,
                object:                    _video,
                is_user_can_see_video_list: is_user_can_see_video_list,
                is_user_can_see_comments:  is_user_can_see_comments,
                object_list:               object_list,
                next_page_number:          next_page_number,
                prev:                      prev,
                next:                      next,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/video/load/anon_video.stpl")]
            struct Template {
                list:                      VideoList,
                object:                    Video,
                is_user_can_see_video_list: bool,
                is_user_can_see_comments:  bool,
                object_list:               Vec<VideoComment>,
                next_page_number:          i32,
                prev:                      Option<i32>,
                next:                      Option<i32>,
            }
            let body = Template {
                list:                       _list,
                object:                     _video,
                is_user_can_see_video_list: is_user_can_see_video_list,
                is_user_can_see_comments:   is_user_can_see_comments,
                object_list:                object_list,
                next_page_number:           next_page_number,
                prev:                       prev,
                next:                       next,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


pub async fn load_comments_page(session: Session, req: HttpRequest, video_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let mut is_open = false;
    let mut text = "".to_string();

    let _video = get_video(*video_id);
    let _list = get_video_list(_video.video_list_id);

    let object_list: Vec<VideoComment>;
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = _video.get_comments(20, step.into());
        if _video.comment > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = _video.get_comments(20, 0);
        if _video.comment > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        if _video.community_id.is_some() {
            let _tuple = get_community_permission(&_video.get_community(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_user_permission(&_video.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }

        let _request_user_id = &_request_user.id;
        let is_user_can_create_comments = _list.is_user_can_create_comment(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if !_list.is_user_can_see_el(*_request_user_id) && !_list.is_user_can_see_comment(*_request_user_id) {
            use crate::views::close_list;
            return close_list()
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/generic/items/comment/comments.stpl")]
            struct Template {
                list:                        VideoList,
                item:                        Video,
                request_user:                User,
                is_user_can_create_comments: bool,
                object_list:                 Vec<VideoComment>,
                next_page_number:            i32,
            }
            let body = Template {
                list:                        _list,
                item:                        _video,
                request_user:                _request_user,
                is_user_can_create_comments: is_user_can_create_comments,
                object_list:                 object_list,
                next_page_number:            next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/generic/items/comment/comments.stpl")]
            struct Template {
                list:                        VideoList,
                item:                        Video,
                request_user:                User,
                is_user_can_create_comments: bool,
                object_list:                 Vec<VideoComment>,
                next_page_number:            i32,
            }
            let body = Template {
                list:                        _list,
                item:                        _video,
                request_user:                _request_user,
                is_user_can_create_comments: is_user_can_create_comments,
                object_list:                 object_list,
                next_page_number:            next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        if _video.community_id.is_some() {
            let _tuple = get_anon_community_permission(&_video.get_community());
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_anon_user_permission(&_video.get_creator());
            is_open = _tuple.0;
            text = _tuple.1;
        }

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if !_list.is_anon_user_can_see_el() && !_list.is_anon_user_can_see_comment() {
            use crate::views::close_list;
            return close_list()
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/generic/items/comment/anon_comments.stpl")]
            struct Template {
                list:                      VideoList,
                item:                      Video,
                object_list:               Vec<VideoComment>,
                next_page_number:          i32,
            }
            let body = Template {
                list:                      _list,
                item:                      _video,
                object_list:               object_list,
                next_page_number:          next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/generic/items/comment/anon_comments.stpl")]
            struct Template {
                list:                      VideoList,
                item:                      Video,
                object_list:               Vec<VideoComment>,
                next_page_number:          i32,
            }
            let body = Template {
                list:                      _list,
                item:                      _video,
                object_list:               object_list,
                next_page_number:          next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn edit_video_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let video = get_video(*_id);
        if video.is_user_can_edit_delete_item(_request_user.id) {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/video/edit_video.stpl")]
            struct Template {
                request_user: User,
                object: Video,
            }
            let body = Template {
                request_user: _request_user,
                object: video,
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
