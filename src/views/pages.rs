use diesel::prelude::*;
use crate::schema;
use actix_web::{
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    web,
};
use serde::Deserialize;
use crate::utils::{
    is_signed_in,
    establish_connection,
    is_desctop,
    get_request_user_data,
    to_home,
    get_list_variables,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/featured/", web::get().to(featured_list_page));
    config.route("/all-users/", web::get().to(all_users_page));
    config.route("/all-communities/", web::get().to(all_communities_page));
    config.route("/check_custom_link/{slug}/", web::get().to(check_custom_link));
    config.route("/{slug}/", web::get().to(link_page));
}

pub async fn link_page(session: Session, req: HttpRequest, slug: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let link = slug.clone();
    if &link[..2] == "id".to_string() {
        use crate::views::users::user_page;

        let pk = link[2..].parse().unwrap();
        return user_page(session, req, link).await
    }
    else if &link.len() > &5 && &link[..6] == "public".to_string() {
        use crate::views::communities::community_page;

        return community_page(session, req, link).await
    }
    else {
        use crate::schema::custom_links::dsl::custom_links;
        use crate::models::CustomLink;

        link_some = custom_links
            .filter(schema::custom_links::link.eq(&link))
            .limit(1)
            .load::<CustomLink>(&_connection)
            .expect("E.");

        if link_some.len() == 0 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        if link_some[0].owner == 1 {
            return user_page(session, req, link).await
        }
        else if link_some[0].owner == 1 {
            return community_page(session, req, link).await
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub async fn index_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "desctop/main/auth/auth.stpl")]
    struct DesctopAuthTemplate {
        title: String,
    }
    #[derive(TemplateOnce)]
    #[template(path = "desctop/main/lists/news_list.stpl")]
    struct DesctopNewsListTemplate {
        title:        String,
        request_user: User,
    }

    #[derive(TemplateOnce)]
    #[template(path = "mobile/main/auth/auth.stpl")]
    struct MobileAuthTemplate {
        title: String,
    }
    #[derive(TemplateOnce)]
    #[template(path = "mobile/main/lists/news_list.stpl")]
    struct MobileNewsListTemplate {
        title:        String,
        request_user: User,
    }

    let _connection = establish_connection();
    let is_desctop = is_desctop(req);
    if is_signed_in(&session) {

        let _request_user = get_request_user_data(session);

        if is_desctop {
            let body = DesctopNewsListTemplate {
                title:        "Новости".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            let body = MobileNewsListTemplate {
                title:        "Новости".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }

    } else {
        if is_desctop {
            let body = DesctopAuthTemplate { title: "Трезвый.рус | Вход".to_string() }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            let body = MobileAuthTemplate { title: "Трезвый.рус | Вход".to_string() }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
    }
}

pub async fn featured_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "desctop/main/lists/featured_list.stpl")]
    struct DesctopFeaturedListTemplate {
        title:        String,
        request_user: User,
    }
    #[derive(TemplateOnce)]
    #[template(path = "mobile/main/lists/featured_list.stpl")]
    struct MobileFeaturedListTemplate {
        title:        String,
        request_user: User,
    }

    let _connection = establish_connection();
    let is_desctop = is_desctop(req);
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);

        if is_desctop {
            let body = DesctopFeaturedListTemplate {
                title:      "Рекомендации".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            let body = MobileFeaturedListTemplate {
                title:      "Рекомендации".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
    } else {
        Ok(to_home())
    }
}


pub async fn all_users_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let object_list: Vec<User>;

    let _connection = establish_connection();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let count = _request_user.get_all_users_count();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = _request_user.get_users(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = _request_user.get_users(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lists/all_users.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                next_page_number: i32,
                object_list: Vec<User>,
                count_users: usize,
            }

            let body = Template {
                title:        "Пользователи".to_string(),
                request_user: _request_user,
                next_page_number: next_page_number,
                object_list: object_list,
                count_users: count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/lists/all_users.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                next_page_number: i32,
                object_list: Vec<User>,
                count_users: usize,
            }

            let body = Template {
                title:        "Пользователи".to_string(),
                request_user: _request_user,
                next_page_number: next_page_number,
                object_list: object_list,
                count_users: count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }

    } else {
        let count = User::get_anon_users_count();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = User::get_anon_users(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = User::get_anon_users(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lists/anon_all_users.stpl")]
            struct Template {
                title:        String,
                next_page_number: i32,
                object_list: Vec<User>,
                count_users: usize,
            }
            let body = Template {
                title: "Пользователи".to_string(),
                next_page_number: next_page_number,
                object_list: object_list,
                count_users: count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/lists/anon_all_users.stpl")]
            struct Template {
                title:        String,
                next_page_number: i32,
                object_list: Vec<User>,
                count_users: usize,
            }
            let body = Template {
                title: "Пользователи".to_string(),
                next_page_number: next_page_number,
                object_list: object_list,
                count_users: count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
    }
}


pub async fn all_communities_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::models::Community;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    let object_list: Vec<Community>;

    let _connection = establish_connection();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let count = User::get_all_communities_count();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = User::get_all_communities(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = User::get_all_communities(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lists/all_communities.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                next_page_number: i32,
                object_list: Vec<Community>,
                count_communities: usize,
            }

            let body = Template {
                title:        "Пользователи".to_string(),
                request_user: _request_user,
                next_page_number: next_page_number,
                object_list: object_list,
                count_communities: count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/lists/all_communities.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                next_page_number: i32,
                object_list: Vec<Community>,
                count_communities: usize,
            }

            let body = Template {
                title:        "Пользователи".to_string(),
                request_user: _request_user,
                next_page_number: next_page_number,
                object_list: object_list,
                count_communities: count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }

    } else {
        let count = User::get_all_communities_count();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = User::get_all_communities(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = User::get_all_communities(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lists/anon_all_communities.stpl")]
            struct Template {
                title:        String,
                next_page_number: i32,
                object_list: Vec<Community>,
                count_communities: usize,
            }
            let body = Template {
                title: "Пользователи".to_string(),
                next_page_number: next_page_number,
                object_list: object_list,
                count_communities: count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/lists/anon_all_communities.stpl")]
            struct Template {
                title:        String,
                next_page_number: i32,
                object_list: Vec<Community>,
                count_communities: usize,
            }
            let body = Template {
                title: "Пользователи".to_string(),
                next_page_number: next_page_number,
                object_list: object_list,
                count_communities: count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
    }
}

pub async fn check_custom_link(session: Session, req: HttpRequest, slug: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::custom_link_check;

    let link = slug.clone();
    let (_bool, _string) = custom_link_check(&link);
    let answer = "
    <div>
        <span id='bool'>".to_owned() + &_bool.to_string() + &"</span>
        <span id='string'>".to_string() + &_string.to_string() + &"</span>
    </div>".to_string();
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(answer))
}
