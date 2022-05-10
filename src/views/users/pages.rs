use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    is_signed_in,
    is_desctop,
    get_request_user_data,
    get_user,
    get_user_permission,
    get_anon_user_permission,
};

use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn user_pages_urls(config: &mut web::ServiceConfig) {
    config.route("/users/{user_id}/photos/", web::get().to(user_photos_page));
    config.route("/users/{user_id}/goods/", web::get().to(user_goods_page));
    config.route("/users/{user_id}/music/", web::get().to(user_music_page));
    config.route("/users/{user_id}/surveys/", web::get().to(user_surveys_page));
    config.route("/users/{user_id}/video/", web::get().to(user_video_page));
    config.route("/users/{user_id}/docs/", web::get().to(user_docs_page));

    config.route("/users/{user_id}/photos_list/{list_id}/", web::get().to(user_photos_list_page));
    config.route("/users/{user_id}/goods_list/{list_id}/", web::get().to(user_goods_list_page));
    config.route("/users/{user_id}/music_list/{list_id}/", web::get().to(user_music_list_page));
    config.route("/users/{user_id}/surveys_list/{list_id}/", web::get().to(user_surveys_list_page));
    config.route("/users/{user_id}/video_list/{list_id}/", web::get().to(user_video_list_page));
    config.route("/users/{user_id}/docs_list/{list_id}/", web::get().to(user_docs_list_page));
}

pub async fn user_docs_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::DocList;
    use crate::utils::get_doc_list;

    let user_id : i32 = *user_id;
    let is_desctop = is_desctop(req);

    let _user = get_user(user_id);
    let _list = get_doc_list(_user.get_selected_doc_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/docs/main_list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         DocList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- список документов".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/docs/main_list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         DocList,
            }

            let body = Template {
                title:        _user.get_doc_list() + &"- список документов".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/docs/main_list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  DocList,
            }
            let body = Template {
                title:  _user.get_full_name() + &"- список документов".to_string(),
                user:   _user,
                list:   _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/docs/main_list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  DocList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- список документов".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_video_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::VideoList;
    use crate::utils::get_video_list;

    let user_id : i32 = *user_id;
    let is_desctop = is_desctop(req);

    let _user = get_user(user_id);
    let _list = get_video_list(_user.get_selected_video_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/video/main_list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         VideoList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- видеоальбомы".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/video/main_list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         VideoList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- видеоальбомы".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/video/main_list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  VideoList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- видеоальбомы".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/video/main_list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  VideoList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- видеоальбомы".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_surveys_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::SurveyList;
    use crate::utils::get_survey_list;

    let user_id : i32 = *user_id;
    let is_desctop = is_desctop(req);

    let _user = get_user(user_id);
    let _list = get_survey_list(_user.get_selected_survey_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/survey/main_list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         SurveyList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- список опрсоов".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/survey/main_list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         SurveyList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- список опрсоов".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/survey/main_list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  SurveyList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- список опрсоов".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/survey/main_list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  SurveyList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- список опрсоов".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_music_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::MusicList;
    use crate::utils::get_music_list;

    let user_id : i32 = *user_id;
    let is_desctop = is_desctop(req);

    let _user = get_user(user_id);
    let _list = get_music_list(_user.get_selected_music_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/music/main_list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         MusicList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- плейлисты".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/music/main_list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         MusicList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- плейлисты".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/music/main_list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  MusicList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- плейлисты".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/music/main_list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  MusicList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- плейлисты".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_goods_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::GoodList;
    use crate::utils::get_good_list;

    let user_id : i32 = *user_id;
    let is_desctop = is_desctop(req);

    let _user = get_user(user_id);
    let _list = get_good_list(_user.get_selected_good_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/goods/main_list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         GoodList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- подборки товаров".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/goods/main_list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         GoodList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- подборки товаров".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/goods/main_list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  GoodList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- подборки товаров".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/goods/main_list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  GoodList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- подборки товаров".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_photos_page(session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::PhotoList;
    use crate::utils::get_photo_list;

    let user_id : i32 = *user_id;
    let is_desctop = is_desctop(req);

    let _user = get_user(user_id);
    let _list = get_photo_list(_user.get_selected_photo_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/photos/main_list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         PhotoList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- фотоальбомы".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/photos/main_list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         PhotoList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- фотоальбомы".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/photos/main_list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  PhotoList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- фотоальбомы".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/photos/main_list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  PhotoList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- фотоальбомы".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_docs_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::DocList;
    use crate::utils::get_doc_list;

    let user_id : i32 = param.0;
    let list_id : i32 = param.1;
    let is_desctop = is_desctop(req);

    let _user = get_user(user_id);
    let _list = get_doc_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/docs/list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         DocList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- список документов".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/docs/list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         DocList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- список документов".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/docs/list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  DocList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- список документов".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/docs/list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  DocList,
            }
            let body = Template {
                title:  _user.get_full_name() + &"- список документов".to_string(),
                user:   _user,
                list:   _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_video_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::VideoList;
    use crate::utils::get_video_list;

    let user_id : i32 = param.0;
    let list_id : i32 = param.1;
    let is_desctop = is_desctop(req);

    let _user = get_user(user_id);
    let _list = get_video_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/video/list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         VideoList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- видеоальбомы".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/video/list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         VideoList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- видеоальбомы".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/video/list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  VideoList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- видеоальбомы".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/video/list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  VideoList,
            }
            let body = Template {
                title: _user.name.clone() + &"- видеоальбомы".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_surveys_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::SurveyList;
    use crate::utils::get_survey_list;

    let user_id : i32 = param.0;
    let list_id : i32 = param.1;
    let is_desctop = is_desctop(req);

    let _user = get_user(user_id);
    let _list = get_survey_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/survey/list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         SurveyList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- список опрсоов".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/survey/list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         SurveyList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- список опрсоов".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/survey/list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  SurveyList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- список опрсоов".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/survey/list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  SurveyList,
            }
            let body = Template {
                title:   _user.get_full_name() + &"- список опрсоов".to_string(),
                user:   _user,
                list:   _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_music_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::MusicList;
    use crate::utils::get_music_list;

    let user_id : i32 = param.0;
    let list_id : i32 = param.1;
    let is_desctop = is_desctop(req);

    let _user = get_user(user_id);
    let _list = get_music_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/music/list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         MusicList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- плейлисты".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/music/list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         MusicList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- плейлисты".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/music/list/anon_list.stpl")]
            struct Template {
                title:  String,
                user:   User,
                list:   MusicList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- плейлисты".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/music/list/anon_list.stpl")]
            struct Template {
                title:  String,
                user:   User,
                list:   MusicList,
            }
            let body = Template {
                title: _user.get_full_name() + &"- плейлисты".to_string(),
                user:  _user,
                list:  _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_goods_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::GoodList;
    use crate::utils::get_good_list;

    let user_id : i32 = param.0;
    let list_id : i32 = param.1;
    let is_desctop = is_desctop(req);

    let _user = get_user(user_id);
    let _list = get_good_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/goods/list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         GoodList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- подборки товаров".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/goods/list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         GoodList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- подборки товаров".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_user_permission(&_user);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/goods/list/anon_list.stpl")]
            struct Template {
                title:  String,
                user:   User,
                list:   GoodList,
            }
            let body = Template {
                title:  _user.get_full_name() + &"- подборки товаров".to_string(),
                user:   _user,
                list:   _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/goods/list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  GoodList,
            }
            let body = Template {
                title:  _user.get_full_name() + &"- подборки товаров".to_string(),
                user:   _user,
                list:   _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn user_photos_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::PhotoList;
    use crate::utils::get_photo_list;

    let user_id : i32 = param.0;
    let list_id : i32 = param.1;
    let is_desctop = is_desctop(req);

    let _user = get_user(user_id);
    let _list = get_photo_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (is_open, text) = get_user_permission(&_user, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/photos/list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         PhotoList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- фотоальбомы".to_string(),
                request_user: _request_user,
                user:         _user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/photos/list/list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                user:         User,
                list:         PhotoList,
            }

            let body = Template {
                title:        _user.get_full_name() + &"- фотоальбомы".to_string(),
                request_user: _request_user,
                user:         user,
                list:         _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_community_permission(&_community);
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/photos/list/anon_list.stpl")]
            struct Template {
                title:  String,
                user:   User,
                list:   PhotoList,
            }
            let body = Template {
                title:  _user.get_full_name() + &"- фотоальбомы".to_string(),
                user:   _user,
                list:   _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/photos/main_list/anon_list.stpl")]
            struct Template {
                title: String,
                user:  User,
                list:  PhotoList,
            }
            let body = Template {
                title:  _user.get_full_name() + &"- фотоальбомы".to_string(),
                user:   _user,
                list:   _list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}