use actix_web::{
    HttpResponse,
    HttpRequest,
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
    establish_connection,
};
use crate::diesel::RunQueryDsl;
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

    config.route("/video/add_video_in_list/{id}/", web::get().to(add_video_in_list_page));
    config.route("/video/edit_new_video/", web::get().to(edit_new_video_page));
    config.route("/video/edit_video/{id}/", web::get().to(edit_video_page));
}

pub async fn load_list_page(session: Session, req: HttpRequest, list_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let is_open : bool;
    let text : String;
    let owner_name : String;
    let owner_link : String;

    let _list = get_video_list(*list_id);

    let object_list: Vec<Video>;
    let lists: Vec<VideoList>;
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
            lists = community.get_video_lists();
            owner_name = community.name;
            owner_link = community.link;
        }
        else {
            let creator = _list.get_creator();
            let _tuple = get_user_permission(&_list.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
            lists = creator.get_video_lists();
            owner_name = creator.get_full_name();
            owner_link = creator.link;
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
                list:                       VideoList,
                request_user:               User,
                is_user_can_see_video_list: bool,
                is_user_can_create_videos:  bool,
                object_list:                Vec<Video>,
                next_page_number:           i32,
                owner_name:                 String,
                owner_link:                 String,
                lists:                      Vec<VideoList>,
            }
            let body = Template {
                list:                       _list,
                request_user:               _request_user,
                is_user_can_see_video_list: is_user_can_see_video_list,
                is_user_can_create_videos:  is_user_can_create_videos,
                object_list:                object_list,
                next_page_number:           next_page_number,
                owner_name:                 owner_name,
                owner_link:                 owner_link,
                lists:                      lists,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/video/list/list.stpl")]
            struct Template {
                list:                       VideoList,
                request_user:               User,
                is_user_can_see_video_list: bool,
                is_user_can_create_videos:  bool,
                object_list:                Vec<Video>,
                next_page_number:           i32,
                owner_name:                 String,
                owner_link:                 String,
                lists:                      Vec<VideoList>,
            }
            let body = Template {
                list:                       _list,
                request_user:               _request_user,
                is_user_can_see_video_list: is_user_can_see_video_list,
                is_user_can_create_videos:  is_user_can_create_videos,
                object_list:                object_list,
                next_page_number:           next_page_number,
                owner_name:                 owner_name,
                owner_link:                 owner_link,
                lists:                      lists,
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
            lists = community.get_video_lists();
            owner_name = community.name;
            owner_link = community.link;
        }
        else {
            let creator = _list.get_creator();
            let _tuple = get_anon_user_permission(&creator);
            is_open = _tuple.0;
            text = _tuple.1;
            lists = creator.get_video_lists();
            owner_name = creator.get_full_name();
            owner_link = creator.link;
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
                list:                       VideoList,
                is_user_can_see_video_list: bool,
                object_list:                Vec<Video>,
                next_page_number:           i32,
                owner_name:                 String,
                owner_link:                 String,
                lists:                      Vec<VideoList>,
            }
            let body = Template {
                list:                       _list,
                is_user_can_see_video_list: is_user_can_see_video_list,
                object_list:                object_list,
                next_page_number:           next_page_number,
                owner_name:                 owner_name,
                owner_link:                 owner_link,
                lists:                      lists,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/video/list/anon_list.stpl")]
            struct Template {
                list:                       VideoList,
                is_user_can_see_video_list: bool,
                object_list:                Vec<Video>,
                next_page_number:           i32,
                owner_name:                 String,
                owner_link:                 String,
                lists:                      Vec<VideoList>,
            }
            let body = Template {
                list:                       _list,
                is_user_can_see_video_list: is_user_can_see_video_list,
                object_list:                object_list,
                next_page_number:           next_page_number,
                owner_name:                 owner_name,
                owner_link:                 owner_link,
                lists:                      lists,
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
        use crate::schema::reactions::dsl::reactions;
        use crate::models::Reaction;

        let _connection = establish_connection();
        let reaction_list = reactions
            .load::<Reaction>(&_connection)
            .expect("E.");

        let _request_user = get_request_user_data(&session);
        #[derive(TemplateOnce)]
        #[template(path = "desctop/video/user/add_list.stpl")]
        struct Template {
            reaction_list: Vec<Reaction>,
        }
        let body = Template {
            reaction_list: reaction_list,
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
        use crate::schema::reactions::dsl::reactions;
        use crate::models::Reaction;

        let _connection = establish_connection();
        let reaction_list = reactions
            .load::<Reaction>(&_connection)
            .expect("E.");

        let _request_user = get_request_user_data(&session);
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
                list:          VideoList,
                reaction_list: Vec<Reaction>,
            }
            let body = YTemplate {
                list:          list,
                reaction_list: reaction_list,
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
        use crate::schema::reactions::dsl::reactions;
        use crate::models::Reaction;

        let _request_user = get_request_user_data(&session);
        let community = get_community(*_id);

        let _connection = establish_connection();
        let reaction_list = reactions
            .load::<Reaction>(&_connection)
            .expect("E.");

        #[derive(TemplateOnce)]
        #[template(path = "desctop/video/community/add_list.stpl")]
        struct Template {
            community:     Community,
            reaction_list: Vec<Reaction>,
        }
        let body = Template {
            community:     community,
            reaction_list: reaction_list,
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
        use crate::schema::reactions::dsl::reactions;
        use crate::models::Reaction;

        let _request_user = get_request_user_data(&session);
        let list = get_video_list(*_id);
        let community = get_community(list.community_id.unwrap());

        let _connection = establish_connection();
        let reaction_list = reactions
            .load::<Reaction>(&_connection)
            .expect("E.");

        #[derive(TemplateOnce)]
        #[template(path = "desctop/video/community/edit_list.stpl")]
        struct Template {
            community:     Community,
            list:          VideoList,
            reaction_list: Vec<Reaction>,
        }
        let body = Template {
            community:     community,
            list:          list,
            reaction_list: reaction_list,
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
    let is_open : bool;
    let text : String;

    let _video = get_video(*video_id);
    let _list = get_video_list(_video.video_list_id);

    let _videos = _list.get_paginate_items(50, 0);

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
        let _request_user = get_request_user_data(&session);
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
                videos:                      Vec<Video>,
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
                videos:                      _videos,
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
                videos:                      Vec<Video>,
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
                videos:                      _videos,
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
                list:                       VideoList,
                object:                     Video,
                is_user_can_see_video_list: bool,
                is_user_can_see_comments:   bool,
                object_list:                Vec<VideoComment>,
                next_page_number:           i32,
                videos:                     Vec<Video>,
            }
            let body = Template {
                list:                        _list,
                object:                      _video,
                is_user_can_see_video_list:  is_user_can_see_video_list,
                is_user_can_see_comments:    is_user_can_see_comments,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                videos:                      _videos,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/video/load/anon_video.stpl")]
            struct Template {
                list:                       VideoList,
                object:                     Video,
                is_user_can_see_video_list: bool,
                is_user_can_see_comments:   bool,
                object_list:                Vec<VideoComment>,
                next_page_number:           i32,
                videos:                     Vec<Video>,
            }
            let body = Template {
                list:                       _list,
                object:                     _video,
                is_user_can_see_video_list: is_user_can_see_video_list,
                is_user_can_see_comments:   is_user_can_see_comments,
                object_list:                object_list,
                next_page_number:           next_page_number,
                videos:                     _videos,
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
    let is_open : bool;
    let text : String;

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
        let _request_user = get_request_user_data(&session);
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
                object:                      Video,
                request_user:                User,
                is_user_can_create_comments: bool,
                object_list:                 Vec<VideoComment>,
                next_page_number:            i32,
            }
            let body = Template {
                list:                        _list,
                object:                      _video,
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
                object:                      Video,
                request_user:                User,
                is_user_can_create_comments: bool,
                object_list:                 Vec<VideoComment>,
                next_page_number:            i32,
            }
            let body = Template {
                list:                        _list,
                object:                      _video,
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
                object:                    Video,
                object_list:               Vec<VideoComment>,
                next_page_number:          i32,
            }
            let body = Template {
                list:                      _list,
                object:                    _video,
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
                object:                    Video,
                object_list:               Vec<VideoComment>,
                next_page_number:          i32,
            }
            let body = Template {
                list:                      _list,
                object:                    _video,
                object_list:               object_list,
                next_page_number:          next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn add_video_in_list_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        //use crate::models::VideoCategorie;
        //use crate::schema::video_categories::dsl::video_categories;

        //let _connection = establish_connection();

        //let categories = video_categories
        //    .load::<VideoCategorie>(&_connection)
        //    .expect("E.");

        let _request_user = get_request_user_data(&session);
        let list = get_video_list(*_id);
        if list.is_user_can_edit_delete_item(_request_user.id) {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/video/create_video.stpl")]
            struct Template {
                request_user: User,
                list:         VideoList,
                //categories:   Vec<VideoCategorie>,
            }
            let body = Template {
                request_user: _request_user,
                list:         list,
                //categories:   categories,
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

pub async fn edit_new_video_page(session: Session) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let _request_user = get_request_user_data(&session);
        #[derive(TemplateOnce)]
        #[template(path = "desctop/video/edit_new_video.stpl")]
        struct Template {}
        let body = Template {}
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_video_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let _request_user = get_request_user_data(&session);
        let video = get_video(*_id);
        if video.is_user_can_edit_delete_item(_request_user.id) {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/video/edit_video.stpl")]
            struct Template {
                object: Video,
            }
            let body = Template {
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
