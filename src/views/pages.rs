use actix_web::{
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    web,
};
use serde::Deserialize;
use crate::utils::{is_signed_in, establish_connection, get_folder, get_request_user_data,to_home,};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/featured/", web::get().to(featured_list_page));
    config.route("/add_list/", web::get().to(add_list_page));
    //config.route("/add_list/", web::post().to(add_list));
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

pub async fn add_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::models::Community;

    let _connection = establish_connection();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let mut community: Option<Community> = None;

        #[derive(Deserialize)]
        struct GetParams {
            pub types: String,
            pub community_id: Option<i32>,
        }

        let params_some = web::Query::<GetParams>::from_query(&req.query_string());
        if !params_some.is_ok() {
            to_home();
        }
        let params = params_some.unwrap();
        if params.community_id.is_some() {
            use crate::utils::get_community;

            let _community = get_community(params.community_id.unwrap());
            if !_community.get_staff_users_ids().iter().any(|&i| i==_request_user.id) {
                to_home();
            }
            community = Some(_community);
        }
        let suffix = &params.types[..3];
        let text = match suffix {
            "lpo" => "Создание списка записей".to_string(),
            "lph" => "Создание фотоальбома".to_string(),
            "lgo" => "Создание подборки товаров".to_string(),
            "lvi" => "Создание видеоальбома".to_string(),
            "ldo" => "Создание списка документов".to_string(),
            "lmu" => "Создание плейлиста".to_string(),
            "lsu" => "Создание опросов".to_string(),
            _ => "".to_string(),
        };
        let have_comments = match suffix {
            "lpo" => true,
            "lph" => true,
            "lgo" => true,
            "lvi" => true,
            "ldo" => false,
            "lmu" => false,
            "lsu" => false,
            _ => false,
        };

        if have_comments == true {
            #[derive(TemplateOnce)]
            #[template(path = "common/forms/add_list_with_comment.stpl")]
            struct CreateListCommentTemplate {
                request_user: User,
                r#type: String,
                text: String,
                community: Option<Community>,
            }
            let body = CreateListCommentTemplate {
                request_user: _request_user,
                r#type: params.types.clone(),
                text: text,
                community: community,
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
                r#type: String,
                text: String,
                community: Option<Community>,
            }
            let body = CreateListTemplate {
                request_user: _request_user,
                r#type: params.types.clone(),
                text: text,
                community: community,
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

pub async fn edit_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::models::Community;
    use crate::utils::{
        get_post_list,
        get_photo_list,
        get_good_list,
        get_video_list,
        get_doc_list,
        get_music_list,
        get_survey_list,
    };

    let _connection = establish_connection();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let mut community: Option<Community> = None;

        #[derive(Deserialize)]
        struct GetParams {
            pub types: String,
            pub community_id: Option<i32>,
        }

        let params_some = web::Query::<GetParams>::from_query(&req.query_string());
        if !params_some.is_ok() {
            to_home();
        }
        let params = params_some.unwrap();
        if params.community_id.is_some() {
            use crate::utils::get_community;

            let _community = get_community(params.community_id.unwrap());
            if !_community.get_staff_users_ids().iter().any(|&i| i==_request_user.id) {
                to_home();
            }
            community = Some(_community);
        }
        let suffix = &params.types[..3];
        let pk: i32 = params.types[3..].parse().unwrap();
        if let suffix = "lpo".to_string() {
            let text = "Создание списка записей".to_string();
            let have_comments = true;
        }
        if let suffix = "lph".to_string() {
            "Создание фотоальбома".to_string();
            let have_comments = true;
        }
        if let suffix = "lgo".to_string() {
            let text = "Создание подборки товаров".to_string();
            let have_comments = true;
        }
        if let suffix = "lvi".to_string() {
            "Создание видеоальбома".to_string();
            let have_comments = true;
        }
        if let suffix = "ldo".to_string() {
            "Создание списка документов".to_string();
            let have_comments = false;
        }
        if let suffix = "lmu".to_string() {
            "Создание плейлиста".to_string();
            let have_comments = false;
        }
        if let suffix = "lsu".to_string() {
            "Создание списка опросов".to_string();
            let have_comments = false;
        }

        if have_comments == true {
            #[derive(TemplateOnce)]
            #[template(path = "common/forms/edit_list_with_comment.stpl")]
            struct EditListCommentTemplate {
                request_user: User,
                suffix: String,
                pk: i32,
                text: String,
                community: Option<Community>,
            }
            let body = EditListCommentTemplate {
                request_user: _request_user,
                suffix: suffix.to_string(),
                pk: pk,
                text: text,
                community: community,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "common/forms/edit_list_not_comment.stpl")]
            struct EditListTemplate {
                request_user: User,
                suffix: String,
                pk: i32,
                text: String,
                community: Option<Community>,
            }
            let body = EditListTemplate {
                request_user: _request_user,
                suffix: suffix.to_string(),
                pk: pk,
                text: text,
                community: community,
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
