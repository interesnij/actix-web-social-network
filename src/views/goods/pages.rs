use crate::schema;
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
    get_good_list,
    get_good,
    get_user_permission,
    get_anon_user_permission,
    get_community_permission,
    get_anon_community_permission,
    get_list_variables,
    establish_connection,
};
use crate::diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, GoodList, Good, GoodComment, Community};


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/goods/load_list/{list_id}/", web::get().to(load_list_page));
    config.route("/goods/load_good/{id}/", web::get().to(load_good_page));
    config.route("/goods/load_comments/{id}/", web::get().to(load_comments_page));

    config.route("/goods/add_user_list/", web::get().to(add_user_list_page));
    config.route("/goods/edit_user_list/{id}/", web::get().to(edit_user_list_page));
    config.route("/goods/add_community_list//{id}", web::get().to(add_community_list_page));
    config.route("/goods/edit_community_list/{id}/", web::get().to(edit_community_list_page));

    config.route("/goods/add_good_in_list/{id}/", web::get().to(add_good_in_list_page));
    config.route("/goods/edit_good/{id}/", web::get().to(edit_good_page));
}

pub async fn load_list_page(session: Session, req: HttpRequest, list_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let is_open : bool;
    let text : String;
    let owner_name : String;
    let owner_link : String;

    let _list = get_good_list(*list_id);

    let object_list: Vec<Good>;
    let lists: Vec<GoodList>;
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
            lists = community.get_good_lists();
            owner_name = community.name;
            owner_link = community.link;
        }
        else {
            let creator = _list.get_creator();
            let _tuple = get_user_permission(&creator, &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
            lists = creator.get_good_lists();
            owner_name = creator.get_full_name();
            owner_link = creator.link;
        }

        let _request_user_id = &_request_user.id;
        let is_user_can_see_good_list = _list.is_user_can_see_el(*_request_user_id);
        let is_user_can_create_goods = _list.is_user_can_create_el(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/goods/list/list.stpl")]
            struct Template {
                list:                      GoodList,
                request_user:              User,
                is_user_can_see_good_list: bool,
                is_user_can_create_goods:  bool,
                object_list:               Vec<Good>,
                next_page_number:          i32,
                owner_name:                String,
                owner_link:                String,
                lists:                     Vec<GoodList>,
            }
            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_good_list: is_user_can_see_good_list,
                is_user_can_create_goods:  is_user_can_create_goods,
                object_list:               object_list,
                next_page_number:          next_page_number,
                owner_name:                owner_name,
                owner_link:                owner_link,
                lists:                     lists,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/goods/list/list.stpl")]
            struct Template {
                list:                      GoodList,
                request_user:              User,
                is_user_can_see_good_list: bool,
                is_user_can_create_goods:  bool,
                object_list:               Vec<Good>,
                next_page_number:          i32,
                owner_name:                String,
                owner_link:                String,
                lists:                     Vec<GoodList>,
            }
            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_good_list: is_user_can_see_good_list,
                is_user_can_create_goods:  is_user_can_create_goods,
                object_list:               object_list,
                next_page_number:          next_page_number,
                owner_name:                owner_name,
                owner_link:                owner_link,
                lists:                     lists,
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
            lists = community.get_good_lists();
            owner_name = community.name;
            owner_link = community.link;
        }
        else {
            let creator = _list.get_creator();
            let _tuple = get_anon_user_permission(&creator);
            is_open = _tuple.0;
            text = _tuple.1;
            lists = creator.get_good_lists();
            owner_name = creator.get_full_name();
            owner_link = creator.link;
        }
        let is_user_can_see_good_list = _list.is_anon_user_can_see_el();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/goods/list/anon_list.stpl")]
            struct Template {
                list:                      GoodList,
                is_user_can_see_good_list: bool,
                object_list:               Vec<Good>,
                next_page_number:          i32,
                owner_name:                String,
                owner_link:                String,
                lists:                     Vec<GoodList>,
            }
            let body = Template {
                list:                      _list,
                is_user_can_see_good_list: is_user_can_see_good_list,
                object_list:               object_list,
                next_page_number:          next_page_number,
                owner_name:                owner_name,
                owner_link:                owner_link,
                lists:                     lists,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/goods/list/anon_list.stpl")]
            struct Template {
                list:                      GoodList,
                is_user_can_see_good_list: bool,
                object_list:               Vec<Good>,
                next_page_number:          i32,
                owner_name:                String,
                owner_link:                String,
                lists:                     Vec<GoodList>,
            }
            let body = Template {
                list:                      _list,
                is_user_can_see_good_list: is_user_can_see_good_list,
                object_list:               object_list,
                next_page_number:          next_page_number,
                owner_name:                owner_name,
                owner_link:                owner_link,
                lists:                     lists,
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
        #[template(path = "desctop/goods/user/add_list.stpl")]
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
        let list = get_good_list(_list_id);
        if list.user_id != _request_user.id {
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(""))
        }
        else {

            #[derive(TemplateOnce)]
            #[template(path = "desctop/goods/user/edit_list.stpl")]
            struct YTemplate {
                list:          GoodList,
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
        #[template(path = "desctop/goods/community/add_list.stpl")]
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
        let list = get_good_list(*_id);
        let community = get_community(list.community_id.unwrap());

        let _connection = establish_connection();
        let reaction_list = reactions
            .load::<Reaction>(&_connection)
            .expect("E.");

        #[derive(TemplateOnce)]
        #[template(path = "desctop/goods/community/edit_list.stpl")]
        struct Template {
            community:     Community,
            list:          GoodList,
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

pub async fn load_good_page(session: Session, req: HttpRequest, good_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let is_open : bool;
    let text : String;
    let mut prev: Option<i32> = None;
    let mut next: Option<i32> = None;

    let _good = get_good(*good_id);
    let _list = get_good_list(_good.good_list_id);

    let _goods = _list.get_items();
    for (i, item) in _goods.iter().enumerate().rev() {
        if item.id == _good.id {
            if (i + 1) != _goods.len() {
                prev = Some(_goods[i + 1].id);
            };
            if i != 0 {
                next = Some(_goods[i - 1].id);
            };
            break;
        }
    };

    let object_list: Vec<GoodComment>;
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = _good.get_comments(20, step.into());
        if _good.comment > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = _good.get_comments(20, 0);
        if _good.comment > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _good.community_id.is_some() {
            let _tuple = get_community_permission(&_good.get_community(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_user_permission(&_good.get_creator(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }

        let _request_user_id = &_request_user.id;
        let is_user_can_see_good_list = _list.is_user_can_see_el(*_request_user_id);
        let is_user_can_see_comments = _list.is_user_can_see_comment(*_request_user_id);
        let is_user_can_create_comments = _list.is_user_can_create_comment(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/goods/load/good.stpl")]
            struct Template {
                list:                        GoodList,
                object:                      Good,
                request_user:                User,
                is_user_can_see_good_list:   bool,
                is_user_can_see_comments:    bool,
                is_user_can_create_comments: bool,
                object_list:                 Vec<GoodComment>,
                next_page_number:            i32,
                prev:                        Option<i32>,
                next:                        Option<i32>,
            }
            let body = Template {
                list:                       _list,
                object:                     _good,
                request_user:               _request_user,
                is_user_can_see_good_list:   is_user_can_see_good_list,
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
            #[template(path = "mobile/goods/load/good.stpl")]
            struct Template {
                list:                        GoodList,
                object:                      Good,
                request_user:                User,
                is_user_can_see_good_list:   bool,
                is_user_can_see_comments:    bool,
                is_user_can_create_comments: bool,
                object_list:                 Vec<GoodComment>,
                next_page_number:            i32,
                prev:                        Option<i32>,
                next:                        Option<i32>,
            }
            let body = Template {
                list:                        _list,
                object:                      _good,
                request_user:                _request_user,
                is_user_can_see_good_list:   is_user_can_see_good_list,
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
        if _good.community_id.is_some() {
            let _tuple = get_anon_community_permission(&_good.get_community());
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_anon_user_permission(&_good.get_creator());
            is_open = _tuple.0;
            text = _tuple.1;
        }
        let is_user_can_see_good_list = _list.is_anon_user_can_see_el();
        let is_user_can_see_comments = _list.is_anon_user_can_see_comment();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/goods/load/anon_good.stpl")]
            struct Template {
                list:                      GoodList,
                object:                    Good,
                is_user_can_see_good_list: bool,
                is_user_can_see_comments:  bool,
                object_list:               Vec<GoodComment>,
                next_page_number:          i32,
                prev:                      Option<i32>,
                next:                      Option<i32>,
            }
            let body = Template {
                list:                      _list,
                object:                    _good,
                is_user_can_see_good_list: is_user_can_see_good_list,
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
            #[template(path = "mobile/goods/load/anon_good.stpl")]
            struct Template {
                list:                      GoodList,
                object:                    Good,
                is_user_can_see_good_list: bool,
                is_user_can_see_comments:  bool,
                object_list:               Vec<GoodComment>,
                next_page_number:          i32,
                prev:                      Option<i32>,
                next:                      Option<i32>,
            }
            let body = Template {
                list:                      _list,
                object:                    _good,
                is_user_can_see_good_list: is_user_can_see_good_list,
                is_user_can_see_comments:  is_user_can_see_comments,
                object_list:               object_list,
                next_page_number:          next_page_number,
                prev:                      prev,
                next:                      next,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn load_comments_page(session: Session, req: HttpRequest, good_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let is_open : bool;
    let text : String;

    let _good = get_good(*good_id);
    let _list = get_good_list(_good.good_list_id);

    let object_list: Vec<GoodComment>;
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = _good.get_comments(20, step.into());
        if _good.comment > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = _good.get_comments(20, 0);
        if _good.comment > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _good.community_id.is_some() {
            let _tuple = get_community_permission(&_good.get_community(), &_request_user);
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_user_permission(&_good.get_creator(), &_request_user);
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
                list:                        GoodList,
                object:                      Good,
                request_user:                User,
                is_user_can_create_comments: bool,
                object_list:                 Vec<GoodComment>,
                next_page_number:            i32,
            }
            let body = Template {
                list:                        _list,
                object:                      _good,
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
                list:                        GoodList,
                object:                      Good,
                request_user:                User,
                is_user_can_create_comments: bool,
                object_list:                 Vec<GoodComment>,
                next_page_number:            i32,
            }
            let body = Template {
                list:                        _list,
                object:                      _good,
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
        if _good.community_id.is_some() {
            let _tuple = get_anon_community_permission(&_good.get_community());
            is_open = _tuple.0;
            text = _tuple.1;
        }
        else {
            let _tuple = get_anon_user_permission(&_good.get_creator());
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
                list:                      GoodList,
                object:                    Good,
                object_list:               Vec<GoodComment>,
                next_page_number:          i32,
            }
            let body = Template {
                list:                      _list,
                object:                    _good,
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
                list:                      GoodList,
                object:                    Good,
                object_list:               Vec<GoodComment>,
                next_page_number:          i32,
            }
            let body = Template {
                list:                      _list,
                object:                    _good,
                object_list:               object_list,
                next_page_number:          next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn add_good_in_list_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::GoodSubcategorie;
        use crate::schema::good_subcategories::dsl::good_subcategories;

        let _connection = establish_connection();

        let categories = good_subcategories
            .order(schema::good_subcategories::position.desc())
            .load::<GoodSubcategorie>(&_connection)
            .expect("E.");

        let _request_user = get_request_user_data(&session);
        let list = get_good_list(*_id);
        if list.is_user_can_edit_delete_item(_request_user.id) {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/goods/create_good.stpl")]
            struct Template {
                list:         GoodList,
                categories:   Vec<GoodSubcategorie>,
            }
            let body = Template {
                list:         list,
                categories:   categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }

    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_good_page(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::GoodSubcategorie;
        use crate::schema::good_subcategories::dsl::good_subcategories;

        let _connection = establish_connection();

        let categories = good_subcategories
            .order(schema::good_subcategories::position.desc())
            .load::<GoodSubcategorie>(&_connection)
            .expect("E.");

        let _request_user = get_request_user_data(&session);
        let good = get_good(*_id);
        let list = good.get_list();
        if list.is_user_can_edit_delete_item(_request_user.id) {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/goods/edit_good.stpl")]
            struct Template {
                object:     Good,
                categories: Vec<GoodSubcategorie>,
            }
            let body = Template {
                object:     good,
                categories: categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }

    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
