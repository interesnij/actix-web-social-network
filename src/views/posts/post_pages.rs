use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use serde::Deserialize;
use crate::utils::{
    is_signed_in,
    get_folder,
    get_request_user_data,
    get_user,
    get_community,
    get_user,
    get_post_list,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, PostList, Post};
use serde::Deserialize;


pub fn user_routes(config: &mut web::ServiceConfig) {
    config.route("/posts/list/", web::get().to(post_list_page));
}

pub async fn post_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(Deserialize)]
    struct GetListParams {
        pub user: Option<i32>,
        pub community: Option<i32>,
        pub list: Option<i32>,
    }

    let params_some = web::Query::<GetListParams>::from_query(&req.query_string());
    if !params_some.is_ok() {
        HttpResponse::Ok().body("Not ok".to_string())
    }
    let params = params_some.unwrap();
    let mut _list: PostList;
    let mut is_page_list: bool;
    if params.user.is_some() {
        let user = get_user(params.user.unwrap());
        list = get_post_list(user.get_selected_post_list_pk());
        is_page_list = true;
    }
    else if params.community.is_some() {
        let community = get_community(params.community.unwrap());
        list = get_post_list(community.get_selected_post_list_pk());
        is_page_list = true;
    }
    else {
        list = get_post_list(params.list.unwrap());
        is_page_list = false;
    }

    let _type = get_folder(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;
        let is_user_can_see_post_list = list.is_user_can_see_el(_request_user_id);
        let is_user_can_create_posts = list.is_user_can_create_el(_request_user_id);

        if _type == "desctop/".to_string() {
            if list.community_id.is_some() {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/communties/lenta/list.stpl")]
                struct UserPage {
                    title:        String,
                    list:         PostList,
                    request_user: User,
                    is_page_list: bool,
                    is_user_can_see_post_list: bool,
                    is_user_can_create_posts: bool,
                }
                let body = UserPage {
                    title:        _user.get_full_name().clone(),
                    private_bools: _user.get_profile_all_can_see(*_request_user_id).clone(),
                    request_user: _request_user,
                    user:         _user,
                    user_profile: _profile,
                    is_my_user:   is_my_user,
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_user_can_create_posts: is_user_can_create_posts,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

            } else {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/users/lenta/list.stpl")]
                struct UserPage {
                    title:        String,
                    list:         PostList,
                    request_user: User,
                    is_page_list: bool,
                    is_user_can_see_post_list: bool,
                    is_user_can_create_posts: bool,
                }
                let body = UserPage {
                    title:        _user.get_full_name().clone(),
                    private_bools: _user.get_profile_all_can_see(*_request_user_id).clone(),
                    request_user: _request_user,
                    user:         _user,
                    user_profile: _profile,
                    is_my_user:   is_my_user,
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    is_user_can_create_posts: is_user_can_create_posts,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if list.community_id.is_some() {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/communties/lenta/list.stpl")]
                struct UserPage {
                    title:        String,
                    list:         PostList,
                    request_user: User,
                    is_page_list: bool,
                    is_user_can_see_post_list: bool,
                    is_user_can_create_posts: bool,
                }
                let body = UserPage {
                    title:        _user.get_full_name().clone(),
                    private_bools: _user.get_profile_all_can_see(*_request_user_id).clone(),
                    request_user: _request_user,
                    user:         _user,
                    user_profile: _profile,
                    is_my_user:   is_my_user,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

            } else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/users/lenta/list.stpl")]
                struct UserPage {
                    title:        String,
                    list:         PostList,
                    request_user: User,
                    is_page_list: bool,
                    is_user_can_see_post_list: bool,
                    is_user_can_create_posts: bool,
                }
                let body = UserPage {
                    title:        _user.get_full_name().clone(),
                    private_bools: _user.get_profile_all_can_see(*_request_user_id).clone(),
                    request_user: _request_user,
                    user:         _user,
                    user_profile: _profile,
                    is_my_user:   is_my_user,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }

    } else {
        let is_user_can_see_post_list = list.is_anon_user_can_see_el(_request_user_id);

        if _type == "desctop/".to_string() {
            if list.community_id.is_some() {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/communties/lenta/anon_list.stpl")]
                struct UserPage {
                    title: String,
                    is_user_can_see_post_list: bool,
                    user:  User,
                }
                let body = UserPage {
                    title: _user.get_full_name(),
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    user:  _user,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            } else {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/users/lenta/anon_list.stpl")]
                struct UserPage {
                    title: String,
                    is_user_can_see_post_list: bool,
                    user:  User,
                }
                let body = UserPage {
                    title: _user.get_full_name(),
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    user:  _user,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if list.community_id.is_some() {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/communties/lenta/anon_list.stpl")]
                struct UserPage {
                    title: String,
                    is_user_can_see_post_list: bool,
                    user:  User,
                }
                let body = UserPage {
                    title: _user.get_full_name(),
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    user:  _user,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            } else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/users/lenta/anon_list.stpl")]
                struct UserPage {
                    title: String,
                    is_user_can_see_post_list: bool,
                    user:  User,
                }
                let body = UserPage {
                    title: _user.get_full_name(),
                    is_user_can_see_post_list: is_user_can_see_post_list,
                    user:  _user,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}
