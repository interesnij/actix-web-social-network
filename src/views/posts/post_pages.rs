use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use crate::utils::{
    is_signed_in,
    get_folder,
    get_request_user_data,
    get_user,
    get_community,
    get_post_list,
    to_home,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, PostList, Post, Community};
use serde::Deserialize;


pub fn post_routes(config: &mut web::ServiceConfig) {
    config.route("/posts/list/", web::get().to(post_list_page));
}

pub async fn post_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(Deserialize)]
    struct GetListParams {
        pub page: Option<i32>,
        pub user: Option<i32>,
        pub community: Option<i32>,
        pub list: Option<i32>,
    }

    let params_some = web::Query::<GetListParams>::from_query(&req.query_string());
    if !params_some.is_ok() {
        to_home();
    }
    let params = params_some.unwrap();
    let list: PostList;
    let is_page_list: bool;
    let object_list: Vec<Post>;
    let page_user: Option<User> = None;
    let page_community: Option<Community> = None;

    if params.user.is_some() {
        let user = get_user(params.user.unwrap());
        list = get_post_list(user.get_selected_post_list_pk());
        page_user = Some(user);
        is_page_list = true;
    }
    else if params.community.is_some() {
        let community = get_community(params.community.unwrap());
        list = get_post_list(community.get_selected_post_list_pk());
        page_community = Some(community);
        is_page_list = true;
    }
    else {
        list = get_post_list(params.list.unwrap());
        is_page_list = false;
    }

    if params.page.is_some() {
        object_list = list.get_paginate_items(20, ((params.page.unwrap() - 1) * 20).into());
    }
    else {
        object_list = list.get_paginate_items(20, 0);
    }
    let _type = get_folder(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;
        let is_user_can_see_post_list = list.is_user_can_see_el(*_request_user_id);
        let is_user_can_create_posts = list.is_user_can_create_el(*_request_user_id);

        if _type == "desctop/".to_string() {
            if list.community_id.is_some() {

                #[derive(TemplateOnce)]
                #[template(path = "desctop/communities/lenta/list.stpl")]
                struct UserPage {
                    list:         PostList,
                    request_user: User,
                    is_user_can_see_post_list: bool,
                    is_user_can_create_posts: bool,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    //is_page_list: bool,
                }
                let body = UserPage {
                    list:                      list,
                    request_user:              _request_user,
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_user_can_create_posts:  is_user_can_create_posts,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                    //is_page_list: is_page_list,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

            } else {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/users/lenta/list.stpl")]
                struct UserPage {
                    list:         PostList,
                    request_user: User,
                    is_user_can_see_post_list: bool,
                    is_user_can_create_posts: bool,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    list:                      list,
                    request_user:              _request_user,
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_user_can_create_posts:  is_user_can_create_posts,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                    is_page_list: is_page_list,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if list.community_id.is_some() {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/communities/lenta/list.stpl")]
                struct UserPage {
                    list:         PostList,
                    request_user: User,
                    is_user_can_see_post_list: bool,
                    is_user_can_create_posts: bool,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    list:                      list,
                    request_user:              _request_user,
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_user_can_create_posts:  is_user_can_create_posts,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                    is_page_list: is_page_list,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

            } else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/users/lenta/list.stpl")]
                struct UserPage {
                    list:         PostList,
                    request_user: User,
                    is_user_can_see_post_list: bool,
                    is_user_can_create_posts: bool,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    list:                      list,
                    request_user:              _request_user,
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_user_can_create_posts:  is_user_can_create_posts,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                    is_page_list: is_page_list,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }

    } else {
        let is_user_can_see_post_list = list.is_anon_user_can_see_el();

        if _type == "desctop/".to_string() {
            if list.community_id.is_some() {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/communities/lenta/anon_list.stpl")]
                struct UserPage {
                    is_user_can_see_post_list: bool,
                    list:  PostList,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    list:  list,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                    is_page_list: is_page_list,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            } else {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/users/lenta/anon_list.stpl")]
                struct UserPage {
                    is_user_can_see_post_list: bool,
                    list:  PostList,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    list:  list,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                    is_page_list: is_page_list,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if list.community_id.is_some() {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/communities/lenta/anon_list.stpl")]
                struct UserPage {
                    is_user_can_see_post_list: bool,
                    list:  PostList,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_page_list: is_page_list,
                    list:  list,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            } else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/users/lenta/anon_list.stpl")]
                struct UserPage {
                    is_user_can_see_post_list: bool,
                    list:  PostList,
                    object_list: Vec<Post>,
                    user: Option<User>,
                    community: Option<Community>,
                    is_page_list: bool,
                }
                let body = UserPage {
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_page_list: is_page_list,
                    list:  list,
                    object_list: object_list,
                    user: page_user,
                    community: page_community,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}
