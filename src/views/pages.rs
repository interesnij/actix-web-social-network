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
use crate::utils::{is_signed_in, establish_connection, get_folder, get_request_user_data,to_home,};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/featured/", web::get().to(featured_list_page));
    config.route("/add_list/", web::get().to(add_list_page));
    config.route("/edit_list/", web::get().to(edit_list_page));
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
            pub community_id: Option<String>,
        }

        let params_some = web::Query::<GetParams>::from_query(&req.query_string());
        if params_some.is_err() {
            to_home();
        }
        let params = params_some.unwrap();
        if params.community_id.is_some() {
            use crate::utils::get_community;

            let c_pk: i32 = params.community_id.unwrap().parse().unwrap();
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
                types: String,
                text: String,
                community_id: Option<String>,
            }
            let body = CreateListCommentTemplate {
                request_user: _request_user,
                types: params.types.clone(),
                text: text,
                community_id: params.community_id.clone(),
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
                types: String,
                text: String,
                community_id: Option<String>,
            }
            let body = CreateListTemplate {
                request_user: _request_user,
                types: params.types.clone(),
                text: text,
                community_id: params.community_id.clone(),
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

    let _connection = establish_connection();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let mut community: Option<Community> = None;

        #[derive(Deserialize)]
        struct GetParams {
            pub types: String,
            pub community_id: Option<String>,
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

        let mut text = "".to_string();
        let mut have_comments = false;
        let mut name = "".to_string();
        let mut description = None;

        let mut can_see_el = "".to_string();
        let mut can_see_comment = "".to_string();
        let mut create_el = "".to_string();
        let mut create_comment = "".to_string();
        let mut copy_el = "".to_string();

        let mut can_see_el_exclude_users: Option<Vec<User>> = None;
        let mut can_see_el_include_users: Option<Vec<User>> = None;
        let mut can_see_comment_exclude_users: Option<Vec<User>> = None;
        let mut can_see_comment_include_users: Option<Vec<User>> = None;
        let mut create_el_exclude_users: Option<Vec<User>> = None;
        let mut create_el_include_users: Option<Vec<User>> = None;
        let mut create_comment_exclude_users: Option<Vec<User>> = None;
        let mut create_comment_include_users: Option<Vec<User>> = None;
        let mut copy_el_exclude_users: Option<Vec<User>> = None;
        let mut copy_el_include_users: Option<Vec<User>> = None;

        if let suffix = "lpo".to_string() {
            use crate::utils::get_post_list;

            text = "Создание списка записей".to_string();
            have_comments = true;
            let list = get_post_list(pk);
            name = list.name;
            description = list.description;
            can_see_el = list.can_see_el;
            can_see_comment = list.can_see_comment;
            create_el = list.create_el;
            create_comment = list.create_comment;
            copy_el = list.copy_el;

            if can_see_el == "d".to_string() && can_see_el == "i".to_string() {
                use crate::schema::post_list_perms::dsl::post_list_perms;
                use crate::models::PostListPerm;
                use crate::utils::get_users_from_ids;
                let items = post_list_perms
                    .filter(schema::post_list_perms::post_list_id.eq(list.id))
                    .filter(schema::post_list_perms::can_see_item.eq("b"))
                    .load::<PostListPerm>(&_connection)
                    .expect("E");
                let mut stack = Vec::new();
                    for _item in items.iter() {
                        stack.push(_item.user_id);
                    };
                can_see_el_exclude_users = Some(get_users_from_ids(stack));
            }
            else if can_see_el == "e".to_string() && can_see_el == "j".to_string() {
                use crate::schema::post_list_perms::dsl::post_list_perms;
                use crate::models::PostListPerm;
                use crate::utils::get_users_from_ids;
                let items = post_list_perms
                    .filter(schema::post_list_perms::post_list_id.eq(list.id))
                    .filter(schema::post_list_perms::can_see_item.eq("a"))
                    .load::<PostListPerm>(&_connection)
                    .expect("E");
                let mut stack = Vec::new();
                    for _item in items.iter() {
                        stack.push(_item.user_id);
                    };
                can_see_el_include_users = Some(get_users_from_ids(stack));
            }

        }
        if let suffix = "lph".to_string() {
            use crate::utils::get_photo_list;

            text = "Создание фотоальбома".to_string();
            have_comments = true;
            let list = get_photo_list(pk);
            name = list.name;
            description = list.description;
            can_see_el = list.can_see_el;
            can_see_comment = list.can_see_comment;
            create_el = list.create_el;
            create_comment = list.create_comment;
            copy_el = list.copy_el;
        }
        if let suffix = "lgo".to_string() {
            use crate::utils::get_good_list;

            text = "Создание подборки товаров".to_string();
            have_comments = true;
            let list = get_good_list(pk);
            name = list.name;
            description = list.description;
            can_see_el = list.can_see_el;
            can_see_comment = list.can_see_comment;
            create_el = list.create_el;
            create_comment = list.create_comment;
            copy_el = list.copy_el;
        }
        if let suffix = "lvi".to_string() {
            use crate::utils::get_video_list;

            text = "Создание видеоальбома".to_string();
            have_comments = true;
            let list = get_video_list(pk);
            name = list.name;
            description = list.description;
            can_see_el = list.can_see_el;
            can_see_comment = list.can_see_comment;
            create_el = list.create_el;
            create_comment = list.create_comment;
            copy_el = list.copy_el;
        }
        if let suffix = "ldo".to_string() {
            use crate::utils::get_doc_list;

            text = "Создание списка документов".to_string();
            have_comments = false;
            let list = get_doc_list(pk);
            name = list.name;
            description = list.description;
            can_see_el = list.can_see_el;
            create_el = list.create_el;
            copy_el = list.copy_el;
        }
        if let suffix = "lmu".to_string() {
            use crate::utils::get_music_list;

            text = "Создание плейлиста".to_string();
            have_comments = false;
            let list = get_music_list(pk);
            name = list.name;
            description = list.description;
            can_see_el = list.can_see_el;
            create_el = list.create_el;
            copy_el = list.copy_el;
        }
        if let suffix = "lsu".to_string() {
            use crate::utils::get_survey_list;

            text = "Создание списка опросов".to_string();
            have_comments = false;
            let list = get_survey_list(pk);
            name = list.name;
            description = list.description;
            can_see_el = list.can_see_el;
            create_el = list.create_el;
            copy_el = list.copy_el;
        }

        if have_comments == true {
            #[derive(TemplateOnce)]
            #[template(path = "common/forms/edit_list_with_comment.stpl")]
            struct EditListCommentTemplate {
                request_user: User,
                suffix:       String,
                pk:           i32,
                text:         String,
                community_id: Option<String>,
                name:         String,
                description:  Option<String>,
                can_see_el:   String,
                can_see_comment:   String,
                create_el:    String,
                create_comment:    String,
                copy_el:      String,

                can_see_el_exclude_users: Option<Vec<User>>,
                can_see_el_include_users: Option<Vec<User>>,
                can_see_comment_exclude_users: Option<Vec<User>>,
                can_see_comment_include_users: Option<Vec<User>>,
                create_el_exclude_users: Option<Vec<User>>,
                create_el_include_users: Option<Vec<User>>,
                create_comment_exclude_users: Option<Vec<User>>,
                create_comment_include_users: Option<Vec<User>>,
                copy_el_exclude_users: Option<Vec<User>>,
                copy_el_include_users: Option<Vec<User>>,
            }
            let body = EditListCommentTemplate {
                request_user: _request_user,
                suffix: suffix.to_string(),
                pk: pk,
                text: text,
                community_id: params.community_id.clone(),

                name:         name,
                description:  description,
                can_see_el:   can_see_el,
                can_see_comment:   can_see_comment,
                create_el:    create_el,
                create_comment:    create_comment,
                copy_el:      copy_el,

                can_see_el_exclude_users: can_see_el_exclude_users,
                can_see_el_include_users: can_see_el_include_users,
                can_see_comment_exclude_users: can_see_comment_exclude_users,
                can_see_comment_include_users: can_see_comment_include_users,
                create_el_exclude_users: create_el_exclude_users,
                create_el_include_users: create_el_include_users,
                create_comment_exclude_users: create_comment_exclude_users,
                create_comment_include_users: create_comment_include_users,
                copy_el_exclude_users: copy_el_exclude_users,
                copy_el_include_users: copy_el_include_users,
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
                suffix:       String,
                pk:           i32,
                text:         String,
                community_id: Option<String>,
                name:         String,
                description:  Option<String>,
                can_see_el:   String,
                create_el:    String,
                copy_el:      String,

                can_see_el_exclude_users: Option<Vec<User>>,
                can_see_el_include_users: Option<Vec<User>>,
                create_el_exclude_users: Option<Vec<User>>,
                create_el_include_users: Option<Vec<User>>,
                copy_el_exclude_users: Option<Vec<User>>,
                copy_el_include_users: Option<Vec<User>>,
            }
            let body = EditListTemplate {
                request_user: _request_user,
                suffix: suffix.to_string(),
                pk: pk,
                text: text,
                community_id: params.community_id.clone(),

                name:         name,
                description:  description,
                can_see_el:   can_see_el,
                create_el:    create_el,
                copy_el:      copy_el,

                can_see_el_exclude_users: can_see_el_exclude_users,
                can_see_el_include_users: can_see_el_include_users,
                create_el_exclude_users: create_el_exclude_users,
                create_el_include_users: create_el_include_users,
                copy_el_exclude_users: copy_el_exclude_users,
                copy_el_include_users: copy_el_include_users,
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
