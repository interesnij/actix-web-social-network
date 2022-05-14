use diesel::prelude::*;
use crate::schema;
use actix_web::{
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    web,
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
                    form.password = data_string;
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
