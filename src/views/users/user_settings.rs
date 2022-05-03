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
use crate::utils::{is_signed_in, establish_connection, get_folder, get_request_user_data, to_home,};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn user_settings_routes(config: &mut web::ServiceConfig) {
    config.route("/users/settings/", web::get().to(settings_page));
    config.route("/users/design_settings/", web::get().to(design_settings_page));
    config.route("/users/settings/get_background/{color}/", web::get().to(get_background));
}

pub async fn settings_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let _type = get_folder(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        if _type == "desctop/".to_string() {
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
    let _type = get_folder(req);

    if is_signed_in(&session) {
        use crate::schema::design_settings::dsl::design_settings;
        use crate::models::DesignSetting;

        let _request_user = get_request_user_data(session);

        let _connection = establish_connection();
        let _designs = design_settings
            .filter(schema::design_settings::user_id.eq(_request_user.id))
            .load::<DesignSetting>(&_connection)
            .expect("E");

        if _type == "desctop/".to_string() {
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
            color: *color.clone(),
        }

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
