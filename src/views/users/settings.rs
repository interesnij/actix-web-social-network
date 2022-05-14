use diesel::prelude::*;
use crate::schema;
use actix_web::{
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    web,
    Responder,
};
use crate::utils::{
    is_signed_in,
    establish_connection,
    is_desctop,
    get_request_user_data,
    get_list_variables,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;

use std::{str, borrow::BorrowMut};
use actix_multipart::{Field, Multipart};
use futures::StreamExt;


pub fn settings_urls(config: &mut web::ServiceConfig) {
    config.route("/followings/", web::get().to(followings_page));
    config.route("/blacklist/", web::get().to(blacklist_page));
    config.route("/users/settings/", web::get().to(settings_page));
    config.route("/users/settings/design/", web::get().to(design_settings_page));
    config.route("/users/settings/private/", web::get().to(private_settings_page));

    config.route("/users/settings/edit_link/", web::get().to(edit_link_page));
    config.route("/users/settings/edit_name/", web::get().to(edit_name_page));
    config.route("/users/settings/edit_password/", web::get().to(edit_password_page));
    config.route("/users/settings/edit_phone/", web::get().to(edit_phone_page));
    config.route("/users/settings/remove_profile/", web::get().to(remove_profile_page));

    config.route("/users/settings/change_phone_send/{phone}/", web::get().to(change_phone_send));
    config.route("/users/settings/change_phone_verify/{phone}/{code}/", web::get().to(change_phone_verify));
    config.route("/users/settings/get_background/{color}/", web::get().to(change_phone_verify));

    config.route("/users/settings/edit_link/", web::post().to(edit_link));
    config.route("/users/settings/edit_name/", web::post().to(edit_name));
    config.route("/users/settings/edit_password/", web::post().to(edit_password));
    config.route("/users/settings/edit_phone/", web::post().to(edit_phone));
    config.route("/users/settings/remove_profile/", web::post().to(remove_profile));
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

            object_list = _request_user.get_blocked_users(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = _request_user.get_blocked_users(20, 0);
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
                count:                   usize,
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
                count:                   usize,
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
pub async fn private_settings_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(req);

    if is_signed_in(&session) {
        use crate::schema::user_privates::dsl::user_privates;
        use crate::models::UserPrivate;

        let _request_user = get_request_user_data(session);

        let _connection = establish_connection();
        let _private = user_privates
            .filter(schema::user_privates::user_id.eq(_request_user.id))
            .load::<UserPrivate>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/private_settings.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                private:      UserPrivate,
            }
            let body = Template {
                title:        "Настройки приватности".to_string(),
                request_user: _request_user,
                private:      _private,
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
                private:      UserPrivate,
            }
            let body = Template {
                title:        "Настройки приватности".to_string(),
                request_user: _request_user,
                private:      _private,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_link_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/edit_link.stpl")]
            struct Template {
                title:        String,
                request_user: User,
            }
            let body = Template {
                title:        "Изменение ссылки профиля".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/edit_link.stpl")]
            struct Template {
                title:        String,
                request_user: User,
            }
            let body = Template {
                title:        "Изменение ссылки профиля".to_string(),
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
pub async fn edit_name_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/edit_name.stpl")]
            struct Template {
                title:        String,
                request_user: User,
            }
            let body = Template {
                title:        "Изменение имени / фамилии".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/edit_name.stpl")]
            struct Template {
                title:        String,
                request_user: User,
            }
            let body = Template {
                title:        "Изменение имени / фамилии".to_string(),
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
pub async fn edit_password_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/edit_password.stpl")]
            struct Template {
                title:        String,
                request_user: User,
            }
            let body = Template {
                title:        "Изменение пароля".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/edit_password.stpl")]
            struct Template {
                title:        String,
                request_user: User,
            }
            let body = Template {
                title:        "Изменение пароля".to_string(),
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
pub async fn edit_phone_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/edit_phone.stpl")]
            struct Template {
                title:        String,
                request_user: User,
            }
            let body = Template {
                title:        "Изменение телефона".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/edit_phone.stpl")]
            struct Template {
                title:        String,
                request_user: User,
            }
            let body = Template {
                title:        "Изменение телефона".to_string(),
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
pub async fn remove_profile_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/settings/remove_profile.stpl")]
            struct Template {
                title:        String,
                request_user: User,
            }
            let body = Template {
                title:        "Удаление аккаунта".to_string(),
                request_user: _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/settings/remove_profile.stpl")]
            struct Template {
                title:        String,
                request_user: User,
            }
            let body = Template {
                title:        "Удаление аккаунта".to_string(),
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

pub async fn edit_link(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::schema::users::dsl::users;
        use crate::models::EditLinkUser;

        let _request_user = get_request_user_data(session);
        let _connection = establish_connection();

        let mut form: EditLinkUser = EditLinkUser {
            have_link: None,
        };

        while let Some(item) = payload.next().await {
            let mut field: Field = item.expect("split_payload err");

            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    form.have_link = Some(data_string);
                }
            }
        }

        diesel::update(&_request_user)
          .set(form)
             .get_result::<User>(&_connection)
             .expect("Error.");

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
}
pub async fn edit_name(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::schema::users::dsl::users;
        use crate::models::EditNameUser;

        let _request_user = get_request_user_data(session);
        let _connection = establish_connection();

        let mut form: EditNameUser = EditNameUser {
            first_name: "".to_string(),
            last_name: "".to_string(),
        };

        while let Some(item) = payload.next().await {
            let mut field: Field = item.expect("split_payload err");

            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "first_name" {
                        form.first_name = data_string;
                    }
                    else if field.name() == "last_name" {
                        form.last_name = data_string;
                    }
                }
            }
        }

        diesel::update(&_request_user)
          .set(form)
             .get_result::<User>(&_connection)
             .expect("Error.");

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
}

pub async fn edit_password(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::schema::users::dsl::users;
        use crate::models::EditPasswordUser;
        use crate::utils::hash_password;

        let _request_user = get_request_user_data(session);
        let _connection = establish_connection();

        let mut form: EditPasswordUser = EditPasswordUser {
            password: "".to_string(),
        };

        while let Some(item) = payload.next().await {
            let mut field: Field = item.expect("split_payload err");

            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    form.password = hash_password(&data_string.clone());
                }
            }
        }

        diesel::update(&_request_user)
          .set(form)
             .get_result::<User>(&_connection)
             .expect("Error.");

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
}
pub async fn edit_phone(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::schema::users::dsl::users;
        use crate::models::EditPhoneUser;

        let _request_user = get_request_user_data(session);
        let _connection = establish_connection();

        let mut form: EditPhoneUser = EditPhoneUser {
            phone: "".to_string(),
        };

        while let Some(item) = payload.next().await {
            let mut field: Field = item.expect("split_payload err");

            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    form.phone = data_string;
                }
            }
        }

        diesel::update(&_request_user)
          .set(form)
             .get_result::<User>(&_connection)
             .expect("Error.");

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
}

pub async fn remove_profile(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::schema::users::dsl::users;
        use crate::models::{UserDeleteAnketa, NewUserDeleteAnketa};

        let _request_user = get_request_user_data(session);
        let _connection = establish_connection();

        let mut form: NewUserDeleteAnketa = NewUserDeleteAnketa {
            user_id: _request_user.id,
            answer: "".to_string(),
            other: None,
            created: chrono::Local::now().naive_utc(),
        };

        while let Some(item) = payload.next().await {
            let mut field: Field = item.expect("split_payload err");

            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "answer" {
                        form.answer = data_string;
                    }
                    else if field.name() == "other" {
                        form.other = Some(data_string);
                    }
                }
            }
        }

        diesel::insert_into(schema::user_delete_anketas::table)
            .values(&form)
            .get_result::<UserDeleteAnketa>(&_connection)
            .expect("Error.");
        _request_user.delete_item();

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        } else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
}

pub async fn change_phone_send(session: Session, _phone: web::Path<String>) -> impl Responder {
    use crate::utils::PhoneJson;

    if is_signed_in(&session) {
        let req_phone = _phone.to_string();
        if req_phone.len() > 8 {
            use crate::models::{PhoneCode, NewPhoneCode};
            use crate::schema::users::dsl::users;

            let _connection = establish_connection();
            let _some_user = users
                .filter(schema::users::phone.eq(&req_phone))
                .load::<User>(&_connection)
                .expect("E");
            if _some_user.len() > 0 {
                let rendered = "Пользователь с таким номером уже зарегистрирован. Используйте другой номер или напишите в службу поддержки, если этот номер Вы не использовали ранее.";
                HttpResponse::Ok().body(rendered)
            } else {

                let _url = "https://api.ucaller.ru/v1.0/initCall?service_id=12203&key=GhfrKn0XKAmA1oVnyEzOnMI5uBnFN4ck&phone=".to_owned() + &req_phone;
                let __request = reqwest::get(_url).await.expect("E.");
                let new_request = __request.text().await.unwrap();
                println!("{:?}", new_request);

                let phone200: PhoneJson = serde_json::from_str(&new_request).unwrap();
                let code_i32: i32 = phone200.code.parse().unwrap();
                let new_phone_code = NewPhoneCode {
                    phone: _phone.to_string(),
                    code:  code_i32,
                };
                diesel::insert_into(schema::phone_codes::table)
                    .values(&new_phone_code)
                    .get_result::<PhoneCode>(&_connection)
                    .expect("E.");

                let rendered = "Мы Вам звоним. Последние 4 цифры нашего номера - код подтверждения,
                который нужно ввести в поле 'Последние 4 цифры' и нажать 'Подтвердить'
                <div class='row block_verify mt-5'>
                    <div class='col-md-2'></div>
                    <div class='col-md-4'>
                        <input type='number' id='code' onkeyup='code_check();' class='form-control border-0' placeholder='Последние 4 цифры'>
                        <hr class='my-0'>
                    </div>
                    <div class='mb-3 col-md-4'>
                        <button type='button' disabled='disabled' id='change_code_send' class='btn btn-primary pink-gradient'>Подтвердить</button>
                    </div>
                    <div class='col-md-2'></div>
                </div>";
            HttpResponse::Ok().body(rendered)
            }
        }
        else {
            let rendered = "Введите, пожалуйста, корректное количество цифр Вашего телефона";
            HttpResponse::Ok().body(rendered)
        }

    } else {
        HttpResponse::Ok().body("")
    }
}

pub async fn change_phone_verify(session: Session, param: web::Path<(String,i32)>) -> impl Responder {
    use crate::schema::phone_codes::dsl::phone_codes;
    use crate::models::{PhoneCode, EditPhoneUser};

    let response_text : String;

    if is_signed_in(&session) {
        let _connection = establish_connection();
        let _request_user = get_request_user_data(session);
        let _phone = param.0.to_string();
        let _code = param.1;

        let _phone_codes = phone_codes
            .filter(schema::phone_codes::phone.eq(&_phone))
            .filter(schema::phone_codes::code.eq(&_code))
            .load::<PhoneCode>(&_connection)
            .expect("E");
        if _phone_codes.len() > 0 {
            diesel::delete(phone_codes
                .filter(schema::phone_codes::phone.eq(&_phone))
                .filter(schema::phone_codes::code.eq(&_code))
            ).execute(&_connection)
            .expect("E");
            response_text = "ok".to_string();
            let new_phone = EditPhoneUser {
                phone: _phone,
            };
            diesel::update(&_request_user)
              .set(new_phone)
              .get_result::<User>(&_connection)
              .expect("Error.");
        } else {
            response_text = "Код подтверждения неверный. Проверьте, пожалуйста, номер, с которого мы Вам звонили. Последние 4 цифры этого номера и есть код подтверждения, который нужно ввести с поле 'Последние 4 цифры'. Если не можете найти номер, нажмите на кнопку 'Перезвонить повторно.'".to_string();
        }
    }
    else {
        response_text = "".to_string();
    }

    HttpResponse::Ok().body(response_text)
}
