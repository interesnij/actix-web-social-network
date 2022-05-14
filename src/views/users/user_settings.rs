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
use crate::utils::{is_signed_in, establish_connection, is_desctop, get_request_user_data, to_home,};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn profile_settings_urls(config: &mut web::ServiceConfig) {
    config.route("/followings/", web::get().to(followings_page));
    config.route("/blacklist/", web::get().to(blacklist_page));
    config.route("/users/settings/", web::get().to(settings_page));
    config.route("/users/settings/design_settings/", web::get().to(design_settings_page));
    config.route("/users/settings/get_background/{color}/", web::get().to(get_background));
}

pub async fn followings_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    let object_list: Vec<User>;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let count = _request_user.count_followers();

        if page > 1 {
            let step = (page - 1) * 20;

            object_list = _request_user.get_followings(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = _request_user.get_followings(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/follows/following_list.stpl")]
            struct Template {
                title:                   String,
                request_user:            User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                count:                   i32,
            }

            let body = Template {
                title:                   _request_user.get_full_name() + &" - заявки в друзья".to_string(),
                request_user:            _request_user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                count:                   count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/follows/following_list.stpl")]
            struct Template {
                title:                   String,
                request_user:            User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                count:                   i32,
            }

            let body = Template {
                title:                   _request_user.get_full_name() + &" - заявки в друзья".to_string(),
                request_user:            _request_user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                count:                   count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn blacklist_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    let object_list: Vec<User>;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let count = _request_user.count_blacklist();

        if page > 1 {
            let step = (page - 1) * 20;

            object_list = _request_user.get_banned_user(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = _request_user.get_banned_user(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lists/blacklist.stpl")]
            struct Template {
                title:                   String,
                request_user:            User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                count:                   i32,
            }

            let body = Template {
                title:                   _request_user.get_full_name() + &" - черный список".to_string(),
                request_user:            _request_user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                count:                   count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/lists/blacklist.stpl")]
            struct Template {
                title:                   String,
                request_user:            User,
                object_list:             Vec<User>,
                next_page_number:        i32,
                count:                   i32,
            }

            let body = Template {
                title:                   _request_user.get_full_name() + &" - черный список".to_string(),
                request_user:            _request_user,
                object_list:             object_list,
                next_page_number:        next_page_number,
                count:                   count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn settings_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/settings.stpl")]
            struct Template {
                title:        String,
                request_user: User,
            }
            let body = Template {
                title:        "Настройки профиля".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/settings.stpl")]
            struct Template {
                title:        String,
                request_user: User,
            }
            let body = Template {
                title:        "Настройки профиля".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn design_settings_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(req);

    if is_signed_in(&session) {
        use crate::schema::design_settings::dsl::design_settings;
        use crate::models::DesignSetting;

        let _request_user = get_request_user_data(session);

        let _connection = establish_connection();
        let _designs = design_settings
            .filter(schema::design_settings::user_id.eq(_request_user.id))
            .load::<DesignSetting>(&_connection)
            .expect("E");

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/design_settings.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                color:        String,
            }
            let body = Template {
                title:        "Настройки профиля".to_string(),
                request_user: _request_user,
                color:        _designs[0].background.clone(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/design_settings.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                color:        String,
            }
            let body = Template {
                title:        "Настройки профиля".to_string(),
                request_user: _request_user,
                color:        _designs[0].background.clone(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn get_background(session: Session, color: web::Path<String>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::schema::design_settings::dsl::design_settings;
        use crate::models::{DesignSetting, EditDesignSetting};
        let _request_user = get_request_user_data(session);
        let _connection = establish_connection();
        let backgrounds = design_settings
            .filter(schema::design_settings::user_id.eq(_request_user.id))
            .load::<DesignSetting>(&_connection)
            .expect("E");

        let new_background = EditDesignSetting {
            background: color.into_inner(),
        };

        diesel::update(&backgrounds[0])
          .set(new_background)
             .get_result::<DesignSetting>(&_connection)
             .expect("Error.");

        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body("ok"))
        } else {
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(""))
        }
}
