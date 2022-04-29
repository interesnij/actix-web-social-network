use actix_web::{
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    web,
};
//use serde::Deserialize;
use crate::utils::{is_signed_in, establish_connection, get_folder, get_request_user_data,to_home,};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/featured/", web::get().to(featured_list_page));
    config.route("/add_list/", web::get().to(get_add_list_page));
    //config.route("/add_list/", web::post().to(post_add_list_page));
}

// контекст шаблонов входа или страницы новостей, в зависимости
// от статуса аутентификации пользователя
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

pub async fn index_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let _connection = establish_connection();
    let _type = get_folder(req);
    if is_signed_in(&session) {

        let _request_user = get_request_user_data(session);

        if _type == "desctop/".to_string() {
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
        if _type == "desctop/".to_string() {
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

// контекст шаблонов рекомендаций записей
//DesctopFeaturedListTemplate<'a> == request_user: &'a User
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
pub async fn featured_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let _connection = establish_connection();
    let _type = get_folder(req);
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);

        if _type == "desctop/".to_string() {
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

pub async fn get_add_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let _connection = establish_connection();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let community: Option<Community> = None;

        #[derive(Deserialize)]
        struct GetParams {
            pub type: String,
            pub community_id: Option<i32>,
        }

        let params_some = web::Query::<GetListParams>::from_query(&req.query_string());
        if !params_some.is_ok() {
            to_home();
        }
        let params = params_some.unwrap();
        if params.community_id.is_some() {
            use crate::utils::get_community;

            community = get_community(params.community_id.unwrap());
            if !community.get_staff_users_ids().iter().any(|&i| i==_request_user.id) {
                to_home();
            }
        }
        let suffix = params.type[..3];
        let (text, have_comments) = match code {
            "lpo" => {"Создание списка записей".to_string(); true},
            "lph" => {"Создание фотоальбома".to_string(); true}
            "lgo" => {"Создание подборки товаров".to_string(); true}
            "lvi" => {"Создание видеоальбома".to_string(); true}
            "ldo" => {"Создание списка документов".to_string(); false}
            "lmu" => {"Создание плейлиста".to_string(); false}
            "lsu" => {"Создание опросов".to_string(); false}
        }

        if have_comments == true {
            #[derive(TemplateOnce)]
            #[template(path = "common/forms/add_list_with_comment.stpl")]
            struct CreateListCommentTemplate {
                request_user: User,
                type: String,
                text: String,
                community: Option<Community>,
            }
            let body = CreateListCommentTemplate {
                request_user: _request_user,
                type: params.type,
                text: text,
                community: community;
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "common/forms/add_list_not_comment.stpl")]
            struct CreateListTemplate {
                request_user: User,
                type: String,
                text: String,
                community: Option<Community>,
            }
            let body = CreateListTemplate {
                request_user: _request_user,
                type: params.type,
                text: text,
                community: community;
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
