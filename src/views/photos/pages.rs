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
    get_photo_list,
    get_photo,
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
use crate::models::{User, PhotoList, Photo, PhotoComment, Community};


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/photos/load_list/{list_id}/", web::get().to(load_list_page));
    config.route("/photos/load_photo/{id}/", web::get().to(load_photo_page));
    config.route("/photos/load_comments/{id}/", web::get().to(load_comments_page));

    config.route("/photos/add_user_list/", web::get().to(add_user_list_page));
    config.route("/photos/edit_user_list/{id}/", web::get().to(edit_user_list_page));
    config.route("/photos/add_community_list//{id}", web::get().to(add_community_list_page));
    config.route("/photos/edit_community_list/{id}/", web::get().to(edit_community_list_page));
}

pub async fn load_list_page(session: Session, req: HttpRequest, list_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let is_open : bool;
    let text : String;
    let owner_name : String;
    let owner_link : String;

    let _list = get_photo_list(*list_id);

    let object_list: Vec<Photo>;
    let lists: Vec<PhotoList>;
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
            lists = community.get_photo_lists();
            owner_name = community.name;
            owner_link = community.link;
        }
        else {
            let creator = _list.get_creator();
            let _tuple = get_user_permission(&_list.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
            lists = creator.get_photo_lists();
            owner_name = creator.get_full_name();
            owner_link = creator.link;
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
                list:                       PhotoList,
                request_user:               User,
                is_user_can_see_photo_list: bool,
                is_user_can_create_photos:  bool,
                object_list:                Vec<Photo>,
                next_page_number:           i32,
                owner_name:                 String,
                owner_link:                 String,
                lists:                      Vec<PhotoList>,
            }
            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_photo_list: is_user_can_see_photo_list,
                is_user_can_create_photos:  is_user_can_create_photos,
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
            #[template(path = "mobile/photos/list/list.stpl")]
            struct Template {
                list:                       PhotoList,
                request_user:               User,
                is_user_can_see_photo_list: bool,
                is_user_can_create_photos:  bool,
                object_list:                Vec<Photo>,
                next_page_number:           i32,
                owner_name:                 String,
                owner_link:                 String,
                lists:                      Vec<PhotoList>,
            }
            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_photo_list: is_user_can_see_photo_list,
                is_user_can_create_photos:  is_user_can_create_photos,
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
            lists = community.get_photo_lists();
            owner_name = community.name;
            owner_link = community.link;
        }
        else {
            let creator = _list.get_creator();
            let _tuple = get_anon_user_permission(&creator);
            is_open = _tuple.0;
            text = _tuple.1;
            lists = creator.get_photo_lists();
            owner_name = creator.get_full_name();
            owner_link = creator.link;
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
                list:                       PhotoList,
                is_user_can_see_photo_list: bool,
                object_list:                Vec<Photo>,
                next_page_number:           i32,
                owner_name:                 String,
                owner_link:                 String,
                lists:                      Vec<PhotoList>,
            }
            let body = Template {
                list:                       _list,
                is_user_can_see_photo_list: is_user_can_see_photo_list,
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
            #[template(path = "mobile/photos/list/anon_list.stpl")]
            struct Template {
                list:                       PhotoList,
                is_user_can_see_photo_list: bool,
                object_list:                Vec<Photo>,
                next_page_number:           i32,
                owner_name:                 String,
                owner_link:                 String,
                lists:                      Vec<PhotoList>,
            }
            let body = Template {
                list:                       _list,
                is_user_can_see_photo_list: is_user_can_see_photo_list,
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

        let _request_user = get_request_user_data(&session);
        let _connection = establish_connection();
        let reaction_list = reactions
            .load::<Reaction>(&_connection)
            .expect("E.");

        #[derive(TemplateOnce)]
        #[template(path = "desctop/photos/user/add_list.stpl")]
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
        let list = get_photo_list(_list_id);
        if list.user_id != _request_user.id {
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(""))
        }
        else {

            #[derive(TemplateOnce)]
            #[template(path = "desctop/photos/user/edit_list.stpl")]
            struct YTemplate {
                list:          PhotoList,
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
        #[template(path = "desctop/photos/community/add_list.stpl")]
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
        let list = get_photo_list(*_id);
        let community = get_community(list.community_id.unwrap());

        let _connection = establish_connection();
        let reaction_list = reactions
            .load::<Reaction>(&_connection)
            .expect("E.");

        #[derive(TemplateOnce)]
        #[template(path = "desctop/photos/community/edit_list.stpl")]
        struct Template {
            community:     Community,
            list:          PhotoList,
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

pub async fn load_photo_page(session: Session, req: HttpRequest, photo_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let is_open : bool;
    let text : String;
    let mut prev: Option<i32> = None;
    let mut next: Option<i32> = None;

    let _photo = get_photo(*photo_id);
    let _list = get_photo_list(_photo.photo_list_id);

    let _photos = _list.get_items();
    for (i, item) in _photos.iter().enumerate().rev() {
        if item.id == _photo.id {
            if (i + 1) != _photos.len() {
                prev = Some(_photos[i + 1].id);
            };
            if i != 0 {
                next = Some(_photos[i - 1].id);
            };
            break;
        }
    };

    let object_list: Vec<PhotoComment>;
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = _photo.get_comments(20, step.into());
        if _photo.comment > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = _photo.get_comments(20, 0);
        if _photo.comment > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _photo.community_id.is_some() {
            let _tuple = get_community_permission(&_photo.get_community(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_user_permission(&_photo.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }

        let _request_user_id = &_request_user.id;
        let is_user_can_see_photo_list = _list.is_user_can_see_el(*_request_user_id);
        let is_user_can_see_comments = _list.is_user_can_see_comment(*_request_user_id);
        let is_user_can_create_comments = _list.is_user_can_create_comment(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/photos/load/photo.stpl")]
            struct Template {
                list:                        PhotoList,
                object:                      Photo,
                request_user:                User,
                is_user_can_see_photo_list:   bool,
                is_user_can_see_comments:    bool,
                is_user_can_create_comments: bool,
                object_list:                 Vec<PhotoComment>,
                next_page_number:            i32,
                prev:                        Option<i32>,
                next:                        Option<i32>,
            }
            let body = Template {
                list:                        _list,
                object:                      _photo,
                request_user:                _request_user,
                is_user_can_see_photo_list:  is_user_can_see_photo_list,
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
            #[template(path = "mobile/photos/load/photo.stpl")]
            struct Template {
                list:                        PhotoList,
                object:                      Photo,
                request_user:                User,
                is_user_can_see_photo_list:   bool,
                is_user_can_see_comments:    bool,
                is_user_can_create_comments: bool,
                object_list:                 Vec<PhotoComment>,
                next_page_number:            i32,
                prev:                        Option<i32>,
                next:                        Option<i32>,
            }
            let body = Template {
                list:                        _list,
                object:                      _photo,
                request_user:                _request_user,
                is_user_can_see_photo_list:  is_user_can_see_photo_list,
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
        if _photo.community_id.is_some() {
            let _tuple = get_anon_community_permission(&_photo.get_community());
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_anon_user_permission(&_photo.get_creator());
            is_open = _tuple.0;
            text = _tuple.1;
        }
        let is_user_can_see_photo_list = _list.is_anon_user_can_see_el();
        let is_user_can_see_comments = _list.is_anon_user_can_see_comment();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/photos/load/anon_photo.stpl")]
            struct Template {
                list:                      PhotoList,
                object:                    Photo,
                is_user_can_see_photo_list: bool,
                is_user_can_see_comments:  bool,
                object_list:               Vec<PhotoComment>,
                next_page_number:          i32,
                prev:                      Option<i32>,
                next:                      Option<i32>,
            }
            let body = Template {
                list:                      _list,
                object:                    _photo,
                is_user_can_see_photo_list: is_user_can_see_photo_list,
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
            #[template(path = "mobile/photos/load/anon_photo.stpl")]
            struct Template {
                list:                      PhotoList,
                object:                    Photo,
                is_user_can_see_photo_list: bool,
                is_user_can_see_comments:  bool,
                object_list:               Vec<PhotoComment>,
                next_page_number:          i32,
                prev:                      Option<i32>,
                next:                      Option<i32>,
            }
            let body = Template {
                list:                       _list,
                object:                     _photo,
                is_user_can_see_photo_list: is_user_can_see_photo_list,
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


pub async fn load_comments_page(session: Session, req: HttpRequest, photo_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let is_open : bool;
    let text : String;

    let _photo = get_photo(*photo_id);
    let _list = get_photo_list(_photo.photo_list_id);

    let object_list: Vec<PhotoComment>;
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = _photo.get_comments(20, step.into());
        if _photo.comment > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = _photo.get_comments(20, 0);
        if _photo.comment > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _photo.community_id.is_some() {
            let _tuple = get_community_permission(&_photo.get_community(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_user_permission(&_photo.get_creator(), &_request_user);
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
                list:                        PhotoList,
                object:                      Photo,
                request_user:                User,
                is_user_can_create_comments: bool,
                object_list:                 Vec<PhotoComment>,
                next_page_number:            i32,
            }
            let body = Template {
                list:                        _list,
                object:                      _photo,
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
                list:                        PhotoList,
                object:                      Photo,
                request_user:                User,
                is_user_can_create_comments: bool,
                object_list:                 Vec<PhotoComment>,
                next_page_number:            i32,
            }
            let body = Template {
                list:                        _list,
                object:                      _photo,
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
        if _photo.community_id.is_some() {
            let _tuple = get_anon_community_permission(&_photo.get_community());
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_anon_user_permission(&_photo.get_creator());
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
                list:                      PhotoList,
                object:                    Photo,
                object_list:               Vec<PhotoComment>,
                next_page_number:          i32,
            }
            let body = Template {
                list:                      _list,
                object:                    _photo,
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
                list:                      PhotoList,
                object:                    Photo,
                object_list:               Vec<PhotoComment>,
                next_page_number:          i32,
            }
            let body = Template {
                list:                      _list,
                object:                    _photo,
                object_list:               object_list,
                next_page_number:          next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
