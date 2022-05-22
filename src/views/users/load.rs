use actix_web::{
    HttpResponse,
    HttpRequest,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    is_signed_in,
    is_desctop,
    get_request_user_data,
    get_list_variables,
    get_user,
};

use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn load_urls(config: &mut web::ServiceConfig) {
    config.route("/load/photos/", web::get().to(photos_load));
    config.route("/load/photos_list/{id}/", web::get().to(photos_list_load));

    config.route("/load/video/", web::get().to(video_load));
    config.route("/load/video_list/{id}/", web::get().to(video_list_load));

    config.route("/load/docs/", web::get().to(docs_load));
    config.route("/load/docs_list/{id}/", web::get().to(docs_list_load));

    config.route("/load/surveys/", web::get().to(surveys_load));
    config.route("/load/surveys_list/{id}/", web::get().to(surveys_list_load));

    config.route("/load/music/", web::get().to(music_load));
    config.route("/load/music_list/{id}/", web::get().to(music_list_load));

    config.route("/load/goods/", web::get().to(goods_load));
    config.route("/load/goods_list/{id}/", web::get().to(goods_list_load));

    config.route("/load/lists_for_copy/", web::get().to(lists_for_copy_load));
    config.route("/load/communities_lists_for_copy/", web::get().to(communities_lists_for_copy_load));
    config.route("/load/chat_items/", web::get().to(chat_items_load));

    config.route("/load/friends/", web::get().to(friends_load));
    config.route("/load/smiles/", web::get().to(smiles_load));
    config.route("/load/smiles_stickers/", web::get().to(smiles_stickers_load));
    config.route("/load/chats/", web::get().to(chats_load));
    config.route("/load/communities/", web::get().to(communities_load));

    config.route("/load/list_include_users/", web::get().to(list_include_users_load));
    config.route("/load/list_exclude_users/", web::get().to(list_exclude_users_load));
}

pub async fn photos_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::models::PhotoList;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        let object_list: Vec<Photo>;
        let list = _request_user.get_photo_list();
        let count = list.count;
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_paginate_items(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_paginate_items(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/photos.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Photo>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/photos.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Photo>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn photos_list_load(session: Session, req: HttpRequest, list_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::PhotoList;
    use crate::utils::get_photo_list;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        let object_list: Vec<Photo>;
        let list = get_photo_list(*list_id);
        let count = list.count;
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_paginate_items(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_paginate_items(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/photos_list.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Photo>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/photos_list.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Photo>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn video_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::models::VideoList;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        let object_list: Vec<Video>;
        let list = _request_user.get_video_list();
        let count = list.count;
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_paginate_items(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_paginate_items(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/video.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Video>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/video.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Video>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn video_list_load(session: Session, req: HttpRequest, list_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::VideoList;
    use crate::utils::get_video_list;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        let object_list: Vec<Video>;
        let list = get_video_list(*list_id);
        let count = list.count;
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_paginate_items(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_paginate_items(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/video_list.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Video>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/video_list.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Video>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn docs_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::models::DocList;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        let object_list: Vec<Doc>;
        let list = _request_user.get_doc_list();
        let count = list.count;
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_paginate_items(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_paginate_items(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/docs.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Doc>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/docs.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Doc>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn docs_list_load(session: Session, req: HttpRequest, list_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::DocList;
    use crate::utils::get_doc_list;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        let object_list: Vec<Doc>;
        let list = get_doc_list(*list_id);
        let count = list.count;
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_paginate_items(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_paginate_items(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/docs_list.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Doc>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/docs_list.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Doc>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}


pub async fn surveys_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::models::SurveyList;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        let object_list: Vec<Survey>;
        let list = _request_user.get_survey_list();
        let count = list.count;
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_paginate_items(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_paginate_items(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/surveys.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Survey>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/surveys.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Survey>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn surveys_list_load(session: Session, req: HttpRequest, list_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::SurveyList;
    use crate::utils::get_survey_list;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        let object_list: Vec<Survey>;
        let list = get_survey_list(*list_id);
        let count = list.count;
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_paginate_items(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_paginate_items(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/surveys_list.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Survey>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/surveys_list.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Survey>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}


pub async fn music_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::models::MusicList;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        let object_list: Vec<Music>;
        let list = _request_user.get_music_list();
        let count = list.count;
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_paginate_items(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_paginate_items(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/music.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Music>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/music.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Music>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn music_list_load(session: Session, req: HttpRequest, list_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::MusicList;
    use crate::utils::get_music_list;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        let object_list: Vec<Music>;
        let list = get_music_list(*list_id);
        let count = list.count;
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_paginate_items(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_paginate_items(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/music_list.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Music>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/music_list.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Music>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}


pub async fn goods_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::models::GoodList;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        let object_list: Vec<Good>;
        let list = _request_user.get_good_list();
        let count = list.count;
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_paginate_items(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_paginate_items(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/goods.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Good>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/goods.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Good>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn goods_list_load(session: Session, req: HttpRequest, list_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::GoodList;
    use crate::utils::get_good_list;

    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        let object_list: Vec<Good>;
        let list = get_good_list(*list_id);
        let count = list.count;
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_paginate_items(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_paginate_items(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/goods_list.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Good>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/goods_list.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<Good>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

fn get_load_type(req: &HttpRequest) -> (bool, i32, String) {
    #[derive(Debug, Deserialize)]
    pub struct TypesParams {
        pub types: Option<String>,
    }
    let params_some = web::Query::<TypesParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.types.is_some() {
            let item = params.types.as_deref().unwrap();
            let pk: i32 = item[3..].parse().unwrap();
            let code = &item[..3].to_string();

            return (true, pk, code.to_string());
        }
        else {
            return (false, 0, "".to_string());
        }
    }
    else {
        return (false, 0, "".to_string());
    }
}
pub async fn lists_for_copy_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (type_exists, comment_id, types) = get_load_type(&req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);

        if types == "pos".to_string() {
            use crate::models::PostList;

            let object_list: _request_user.get_post_lists();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/lists_for_copy.stpl")]
            struct Template { object_list: Vec<PostList>, }
            let body = Template { object_list: object_list, }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else if types == "pho".to_string() {
            use crate::models::PhotoList;

            let object_list: _request_user.get_photo_lists();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/lists_for_copy.stpl")]
            struct Template { object_list: Vec<PhotoList>, }
            let body = Template { object_list: object_list, }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else if types == "goo".to_string() {
            use crate::models::GoodList;

            let object_list: _request_user.get_good_lists();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/lists_for_copy.stpl")]
            struct Template { object_list: Vec<GoodList>, }
            let body = Template { object_list: object_list, }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else if types == "vid".to_string() {
            use crate::models::VideoList;

            let object_list: _request_user.get_video_lists();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/lists_for_copy.stpl")]
            struct Template { object_list: Vec<VideoList>, }
            let body = Template { object_list: object_list, }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else if types == "doc".to_string() {
            use crate::models::DocList;

            let object_list: _request_user.get_doc_lists();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/lists_for_copy.stpl")]
            struct Template { object_list: Vec<DocList>, }
            let body = Template { object_list: object_list, }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else if types == "mus".to_string() {
            use crate::models::MusicList;

            let object_list: _request_user.get_music_lists();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/lists_for_copy.stpl")]
            struct Template { object_list: Vec<MusicList>, }
            let body = Template { object_list: object_list, }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn communities_lists_for_copy_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (type_exists, comment_id, types) = get_load_type(&req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);

        if types == "pos".to_string() {
            use crate::models::PostList;

            let object_list: _request_user.get_post_lists_from_staffed_comunities();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/communities_lists_for_copy.stpl")]
            struct Template { object_list: Vec<PostList>, }
            let body = Template { object_list: object_list, }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else if types == "pho".to_string() {
            use crate::models::PhotoList;

            let object_list: _request_user.get_photo_lists_from_staffed_comunities();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/communities_lists_for_copy.stpl")]
            struct Template { object_list: Vec<PhotoList>, }
            let body = Template { object_list: object_list, }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else if types == "goo".to_string() {
            use crate::models::GoodList;

            let object_list: _request_user.get_good_lists_from_staffed_comunities();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/communities_lists_for_copy.stpl")]
            struct Template { object_list: Vec<GoodList>, }
            let body = Template { object_list: object_list, }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else if types == "vid".to_string() {
            use crate::models::VideoList;

            let object_list: _request_user.get_video_lists_from_staffed_comunities();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/communities_lists_for_copy.stpl")]
            struct Template { object_list: Vec<VideoList>, }
            let body = Template { object_list: object_list, }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else if types == "doc".to_string() {
            use crate::models::DocList;

            let object_list: _request_user.get_doc_lists_from_staffed_comunities();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/communities_lists_for_copy.stpl")]
            struct Template { object_list: Vec<DocList>, }
            let body = Template { object_list: object_list, }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else if types == "mus".to_string() {
            use crate::models::MusicList;

            let object_list: _request_user.get_music_lists_from_staffed_comunities();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/communities_lists_for_copy.stpl")]
            struct Template { object_list: Vec<MusicList>, }
            let body = Template { object_list: object_list, }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}


pub async fn chat_items_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        use crate::models::Chat;

        #[derive(Debug, Deserialize)]
        pub struct ChatItemsParams {
            pub chats_page: Option<i32>,
            pub friends_page: Option<i32>,
        }

        let params_some = web::Query::<ChatItemsParams>::from_query(&req.query_string());
        let mut chats_page: i32 = 0;
        let mut friends_page: i32 = 0;
        if params_some.is_ok() {
            let params = params_some.unwrap();
            if params.chats_page.is_some() {
                chats_page = params.chats_page.unwrap();
            }
            if params.friends_page.is_some() {
                chats_page = params.friends_page.unwrap();
            }
        }
        else {
            chats_page = 1;
            friends_page = 1;
        }

        let _request_user = get_request_user_data(session);
        let _request_user_id = &_request_user.id;

        let chats_list: Vec<Chat>;
        let friends_list: Vec<User>;
        let chats_count = _request_user.get_all_chats_count();
        let friends_count = _request_user.count_friends();

        if chats_page > 1 {
            let step = (chats_page - 1) * 20;
            chats_list = _request_user.get_all_chats(20, step.into());
            if chats_count > (chats_page * 20).try_into().unwrap() {
                chats_next_page_number = chats_page + 1;
            }
        }
        else {
            chats_list = _request_user.get_all_chats(20, 0);
            if chats_count > 20.try_into().unwrap() {
                chats_next_page_number = 2;
            }
        }

        if friends_page > 1 {
            let step = (friends_page - 1) * 20;
            friends_list = _request_user.get_friends(20, step.into());
            if friends_count > (friends_page * 20).try_into().unwrap() {
                friends_next_page_number = friends_page + 1;
            }
        }
        else {
            friends_list = _request_user.get_friends(20, 0);
            if friends_count > 20.try_into().unwrap() {
                friends_next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/chat_items.stpl")]
            struct Template {
                request_user:             User,
                chats_list:               Vec<Chat>,
                friends_list:             Vec<User>,
                chats_next_page_number:   i32,
                friends_next_page_number: i32,
                chats_count:              usize,
                friends_count:            i32,
            }

            let body = Template {
                request_user:             _request_user,
                chats_list:               chats_list,
                friends_list:             friends_list,
                chats_next_page_number:   chats_next_page_number,
                friends_next_page_number: friends_next_page_number,
                chats_count:              chats_count,
                friends_count:            friends_count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {

            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/chat_items.stpl")]
            struct Template {
                request_user:             User,
                chats_list:               Vec<Chat>,
                friends_list:             Vec<User>,
                chats_next_page_number:   i32,
                friends_next_page_number: i32,
                chats_count:              usize,
                friends_count:            i32,
            }

            let body = Template {
                request_user:             _request_user,
                chats_list:               chats_list,
                friends_list:             friends_list,
                chats_next_page_number:   chats_next_page_number,
                friends_next_page_number: friends_next_page_number,
                chats_count:              chats_count,
                friends_count:            friends_count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}


pub async fn friends_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);

        let object_list: Vec<User>;
        let count = _request_user.count_friends();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_friends(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_friends(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/friends.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<User>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/friends.stpl")]
            struct Template {
                request_user:                User,
                object_list:                 Vec<User>,
                next_page_number:            i32,
                count:                       i32,
            }

            let body = Template {
                request_user:                _request_user,
                object_list:                 object_list,
                next_page_number:            next_page_number,
                count:                       count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn smiles_stickers_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::{Smile,StickerCategory};

        let (is_desctop, page) = get_list_variables(req);
        let mut next_page_number = 0;
        let _request_user = get_request_user_data(session);

        let object_list: Vec<Smile>;
        let categories = _request_user.get_sticker_categories(30, 0);
        let count = _request_user.get_smiles();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_smiles(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_friends(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/smiles_stickers.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<Smile>,
                categories:       Vec<StickerCategory>,
                next_page_number: i32,
                count:            usize,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                categories:       categories,
                next_page_number: next_page_number,
                count:            count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/smiles_stickers.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<Smile>,
                categories:       Vec<StickerCategory>,
                next_page_number: i32,
                count:            usize,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                categories:       categories,
                next_page_number: next_page_number,
                count:            count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn smiles_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::Smile;

        let (is_desctop, page) = get_list_variables(req);
        let mut next_page_number = 0;
        let _request_user = get_request_user_data(session);

        let object_list: Vec<Smile>;
        let count = _request_user.get_smiles();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_smiles(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_friends(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/smiles.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<Smile>,
                next_page_number: i32,
                count:            usize,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                next_page_number: next_page_number,
                count:            count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/smiles.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<Smile>,
                next_page_number: i32,
                count:            usize,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                next_page_number: next_page_number,
                count:            count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn chats_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::Chat;

        let (is_desctop, page) = get_list_variables(req);
        let mut next_page_number = 0;
        let _request_user = get_request_user_data(session);

        let object_list: Vec<Chat>;
        let count = _request_user.get_all_chats_count();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_all_chats(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_all_chats(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/chats.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<Chat>,
                next_page_number: i32,
                count:            usize,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                next_page_number: next_page_number,
                count:            count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/chats.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<Chat>,
                next_page_number: i32,
                count:            usize,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                next_page_number: next_page_number,
                count:            count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn communities_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::models::Community;

        let (is_desctop, page) = get_list_variables(req);
        let mut next_page_number = 0;
        let _request_user = get_request_user_data(session);

        let object_list: Vec<Community>;
        let count = _request_user.get_all_chats_count();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = list.get_communities(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = list.get_communities(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/communities.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<Community>,
                next_page_number: i32,
                count:            i32,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                next_page_number: next_page_number,
                count:            count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/communities.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<Community>,
                next_page_number: i32,
                count:            i32,
            }

            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                next_page_number: next_page_number,
                count:            count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn list_exclude_users_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);

    if is_signed_in(&session) {

        #[derive(Debug, Deserialize)]
        pub struct ZParams {
            pub page:         Option<i32>,
            pub action:       Option<String>,
            pub community_id: Option<i32>,
            pub list:         Option<String>,
        }

        let params_some = web::Query::<ChatItemsParams>::from_query(&req.query_string());
        let mut page: i32 = 1;
        let mut count: i32 = 0;
        let mut community_id: Option<i32> = None;
        let mut types =  "".to_string();
        let mut list =   "".to_string();
        let mut text =   "".to_string();
        if params_some.is_ok() {
            let params = params_some.unwrap();
            if params.page.is_some() {
                page = params.page.unwrap();
            }
            if params.action.is_some() {
                types = params.action.unwrap();
            }
            if params.community_pk.is_some() {
                community_id = params.community_id;
            }
            if params.list.is_some() {
                list = params.list.unwrap();
            }
        }

        let _request_user = get_request_user_data(session);
        let object_list: Vec<User>;
        let users: Vec<User>;

        if community_id.is_some() {
            use crate::utils::get_community;

            let community = get_community(community_id.unwrap());
            if _request_user.is_administrator_of_community(community.id) {
                count = community.count_members();
                if page > 1 {
                    let step = (page - 1) * 20;
                    object_list = community.get_members(20, step.into());
                    if count > (page * 20).try_into().unwrap() {
                        next_page_number = page + 1;
                    }
                }
                else {
                    object_list = community.get_members(20, 0);
                    if count > 20.try_into().unwrap() {
                        next_page_number = 2;
                    }
                }
            }
        }
        else {
            count = _request_user.count_friends();
            if page > 1 {
                let step = (page - 1) * 20;
                object_list = community.get_friends(20, step.into());
                if count > (page * 20).try_into().unwrap() {
                    next_page_number = page + 1;
                }
            }
            else {
                object_list = community.get_friends(20, 0);
                if count > 20.try_into().unwrap() {
                    next_page_number = 2;
                }
            }
        }
        let pk: i32 = list[3..].parse().unwrap();
        let code = &list[..3].to_string();

        if types == "can_see_el".to_string() {
            text = "видеть записи".to_string();
            if code == "pos".to_string() {
                use crate::utils::get_post_list;
                let current_list = get_post_list(pk);
                users_list = current_list.get_can_see_el_exclude_users()
            }
            else if code == "pho".to_string() {
                use crate::utils::get_photo_list;
                let current_list = get_photo_list(pk);
                users_list = current_list.get_can_see_el_exclude_users()
            }
            else if code == "doc".to_string() {
                use crate::utils::get_doc_list;
                let current_list = get_doc_list(pk);
                users_list = current_list.get_can_see_el_exclude_users()
            }
            else if code == "goo".to_string() {
                use crate::utils::get_good_list;
                let current_list = get_good_list(pk);
                users_list = current_list.get_can_see_el_exclude_users()
            }
            else if code == "vid".to_string() {
                use crate::utils::get_video_list;
                let current_list = get_video_list(pk);
                users_list = current_list.get_can_see_el_exclude_users()
            }
            else if code == "mus".to_string() {
                use crate::utils::get_music_list;
                let current_list = get_music_list(pk);
                users_list = current_list.get_can_see_el_exclude_users()
            }
        }
        else if types == "can_see_comment".to_string() {
            text = "видеть комментарии".to_string();
            if code == "pos".to_string() {
                use crate::utils::get_post_list;
                let current_list = get_post_list(pk);
                users_list = current_list.get_can_see_comment_exclude_users()
            }
            else if code == "pho".to_string() {
                use crate::utils::get_photo_list;
                let current_list = get_photo_list(pk);
                users_list = current_list.get_can_see_comment_exclude_users()
            }
            else if code == "doc".to_string() {
                use crate::utils::get_doc_list;
                let current_list = get_doc_list(pk);
                users_list = current_list.get_can_see_comment_exclude_users()
            }
            else if code == "goo".to_string() {
                use crate::utils::get_good_list;
                let current_list = get_good_list(pk);
                users_list = current_list.get_can_see_comment_exclude_users()
            }
            else if code == "vid".to_string() {
                use crate::utils::get_video_list;
                let current_list = get_video_list(pk);
                users_list = current_list.get_can_see_comment_exclude_users()
            }
            else if code == "mus".to_string() {
                use crate::utils::get_music_list;
                let current_list = get_music_list(pk);
                users_list = current_list.get_can_see_comment_exclude_users()
            }
        }
        else if types == "create_el".to_string() {
            text = "создавать элементы".to_string();
            if code == "pos".to_string() {
                use crate::utils::get_post_list;
                let current_list = get_post_list(pk);
                users_list = current_list.get_create_el_exclude_users()
            }
            else if code == "pho".to_string() {
                use crate::utils::get_photo_list;
                let current_list = get_photo_list(pk);
                users_list = current_list.get_create_el_exclude_users()
            }
            else if code == "doc".to_string() {
                use crate::utils::get_doc_list;
                let current_list = get_doc_list(pk);
                users_list = current_list.get_create_el_exclude_users()
            }
            else if code == "goo".to_string() {
                use crate::utils::get_good_list;
                let current_list = get_good_list(pk);
                users_list = current_list.get_create_el_exclude_users()
            }
            else if code == "vid".to_string() {
                use crate::utils::get_video_list;
                let current_list = get_video_list(pk);
                users_list = current_list.get_create_el_exclude_users()
            }
            else if code == "mus".to_string() {
                use crate::utils::get_music_list;
                let current_list = get_music_list(pk);
                users_list = current_list.get_create_el_exclude_users()
            }
        }
        else if types == "create_comment".to_string() {
            text = "создавать комментарии".to_string();
            if code == "pos".to_string() {
                use crate::utils::get_post_list;
                let current_list = get_post_list(pk);
                users_list = current_list.get_create_comment_exclude_users()
            }
            else if code == "pho".to_string() {
                use crate::utils::get_photo_list;
                let current_list = get_photo_list(pk);
                users_list = current_list.get_create_comment_exclude_users()
            }
            else if code == "doc".to_string() {
                use crate::utils::get_doc_list;
                let current_list = get_doc_list(pk);
                users_list = current_list.get_create_comment_exclude_users()
            }
            else if code == "goo".to_string() {
                use crate::utils::get_good_list;
                let current_list = get_good_list(pk);
                users_list = current_list.get_create_comment_exclude_users()
            }
            else if code == "vid".to_string() {
                use crate::utils::get_video_list;
                let current_list = get_video_list(pk);
                users_list = current_list.get_create_comment_exclude_users()
            }
            else if code == "mus".to_string() {
                use crate::utils::get_music_list;
                let current_list = get_music_list(pk);
                users_list = current_list.get_create_comment_exclude_users()
            }
        }
        else if types == "copy_el".to_string() {
            text = "копировать записи и список".to_string();
            if code == "pos".to_string() {
                use crate::utils::get_post_list;
                let current_list = get_post_list(pk);
                users_list = current_list.get_copy_el_exclude_users()
            }
            else if code == "pho".to_string() {
                use crate::utils::get_photo_list;
                let current_list = get_photo_list(pk);
                users_list = current_list.get_copy_el_exclude_users()
            }
            else if code == "doc".to_string() {
                use crate::utils::get_doc_list;
                let current_list = get_doc_list(pk);
                users_list = current_list.get_copy_el_exclude_users()
            }
            else if code == "goo".to_string() {
                use crate::utils::get_good_list;
                let current_list = get_good_list(pk);
                users_list = current_list.get_copy_el_exclude_users()
            }
            else if code == "vid".to_string() {
                use crate::utils::get_video_list;
                let current_list = get_video_list(pk);
                users_list = current_list.get_copy_el_exclude_users()
            }
            else if code == "mus".to_string() {
                use crate::utils::get_music_list;
                let current_list = get_music_list(pk);
                users_list = current_list.get_copy_el_exclude_users()
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/list_exclude_users.stpl")]
            struct Template {
                request_user:        User,
                object_list:         Vec<User>,
                users:               Vec<User>,
                next_page_number:    i32,
                types:               String,
                count:               i32,
            }

            let body = Template {
                request_user:        _request_user,
                object_list:         object_list,
                users:               users_list,
                next_page_number:    next_page_number,
                types:               types,
                count:               count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {

            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/list_exclude_users.stpl")]
            struct Template {
                request_user:        User,
                object_list:         Vec<User>,
                users:               Vec<User>,
                next_page_number:    i32,
                types:               String,
                count:               i32,
            }

            let body = Template {
                request_user:        _request_user,
                object_list:         object_list,
                users:               users_list,
                next_page_number:    next_page_number,
                types:               types,
                count:               count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn list_include_users_load(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, page) = get_list_variables(req);

    if is_signed_in(&session) {

        #[derive(Debug, Deserialize)]
        pub struct ZParams {
            pub page:         Option<i32>,
            pub action:       Option<String>,
            pub community_id: Option<i32>,
            pub list:         Option<String>,
        }

        let params_some = web::Query::<ChatItemsParams>::from_query(&req.query_string());
        let mut page: i32 = 1;
        let mut count: i32 = 0;
        let mut community_id: Option<i32> = None;
        let mut types =  "".to_string();
        let mut list =   "".to_string();
        let mut text =   "".to_string();
        if params_some.is_ok() {
            let params = params_some.unwrap();
            if params.page.is_some() {
                page = params.page.unwrap();
            }
            if params.action.is_some() {
                types = params.action.unwrap();
            }
            if params.community_pk.is_some() {
                community_id = params.community_id;
            }
            if params.list.is_some() {
                list = params.list.unwrap();
            }
        }

        let _request_user = get_request_user_data(session);
        let object_list: Vec<User>;
        let users: Vec<User>;

        if community_id.is_some() {
            use crate::utils::get_community;

            let community = get_community(community_id.unwrap());
            if _request_user.is_administrator_of_community(community.id) {
                count = community.count_members();
                if page > 1 {
                    let step = (page - 1) * 20;
                    object_list = community.get_members(20, step.into());
                    if count > (page * 20).try_into().unwrap() {
                        next_page_number = page + 1;
                    }
                }
                else {
                    object_list = community.get_members(20, 0);
                    if count > 20.try_into().unwrap() {
                        next_page_number = 2;
                    }
                }
            }
        }
        else {
            count = _request_user.count_friends();
            if page > 1 {
                let step = (page - 1) * 20;
                object_list = community.get_friends(20, step.into());
                if count > (page * 20).try_into().unwrap() {
                    next_page_number = page + 1;
                }
            }
            else {
                object_list = community.get_friends(20, 0);
                if count > 20.try_into().unwrap() {
                    next_page_number = 2;
                }
            }
        }
        let pk: i32 = list[3..].parse().unwrap();
        let code = &list[..3].to_string();

        if types == "can_see_el".to_string() {
            text = "видеть записи".to_string();
            if code == "pos".to_string() {
                use crate::utils::get_post_list;
                let current_list = get_post_list(pk);
                users_list = current_list.get_can_see_el_include_users()
            }
            else if code == "pho".to_string() {
                use crate::utils::get_photo_list;
                let current_list = get_photo_list(pk);
                users_list = current_list.get_can_see_el_include_users()
            }
            else if code == "doc".to_string() {
                use crate::utils::get_doc_list;
                let current_list = get_doc_list(pk);
                users_list = current_list.get_can_see_el_include_users()
            }
            else if code == "goo".to_string() {
                use crate::utils::get_good_list;
                let current_list = get_good_list(pk);
                users_list = current_list.get_can_see_el_include_users()
            }
            else if code == "vid".to_string() {
                use crate::utils::get_video_list;
                let current_list = get_video_list(pk);
                users_list = current_list.get_can_see_el_include_users()
            }
            else if code == "mus".to_string() {
                use crate::utils::get_music_list;
                let current_list = get_music_list(pk);
                users_list = current_list.get_can_see_el_include_users()
            }
        }
        else if types == "can_see_comment".to_string() {
            text = "видеть комментарии".to_string();
            if code == "pos".to_string() {
                use crate::utils::get_post_list;
                let current_list = get_post_list(pk);
                users_list = current_list.get_can_see_comment_include_users()
            }
            else if code == "pho".to_string() {
                use crate::utils::get_photo_list;
                let current_list = get_photo_list(pk);
                users_list = current_list.get_can_see_comment_include_users()
            }
            else if code == "doc".to_string() {
                use crate::utils::get_doc_list;
                let current_list = get_doc_list(pk);
                users_list = current_list.get_can_see_comment_include_users()
            }
            else if code == "goo".to_string() {
                use crate::utils::get_good_list;
                let current_list = get_good_list(pk);
                users_list = current_list.get_can_see_comment_include_users()
            }
            else if code == "vid".to_string() {
                use crate::utils::get_video_list;
                let current_list = get_video_list(pk);
                users_list = current_list.get_can_see_comment_include_users()
            }
            else if code == "mus".to_string() {
                use crate::utils::get_music_list;
                let current_list = get_music_list(pk);
                users_list = current_list.get_can_see_comment_include_users()
            }
        }
        else if types == "create_el".to_string() {
            text = "создавать элементы".to_string();
            if code == "pos".to_string() {
                use crate::utils::get_post_list;
                let current_list = get_post_list(pk);
                users_list = current_list.get_create_el_include_users()
            }
            else if code == "pho".to_string() {
                use crate::utils::get_photo_list;
                let current_list = get_photo_list(pk);
                users_list = current_list.get_create_el_include_users()
            }
            else if code == "doc".to_string() {
                use crate::utils::get_doc_list;
                let current_list = get_doc_list(pk);
                users_list = current_list.get_create_el_include_users()
            }
            else if code == "goo".to_string() {
                use crate::utils::get_good_list;
                let current_list = get_good_list(pk);
                users_list = current_list.get_create_el_include_users()
            }
            else if code == "vid".to_string() {
                use crate::utils::get_video_list;
                let current_list = get_video_list(pk);
                users_list = current_list.get_create_el_include_users()
            }
            else if code == "mus".to_string() {
                use crate::utils::get_music_list;
                let current_list = get_music_list(pk);
                users_list = current_list.get_create_el_include_users()
            }
        }
        else if types == "create_comment".to_string() {
            text = "создавать комментарии".to_string();
            if code == "pos".to_string() {
                use crate::utils::get_post_list;
                let current_list = get_post_list(pk);
                users_list = current_list.get_create_comment_include_users()
            }
            else if code == "pho".to_string() {
                use crate::utils::get_photo_list;
                let current_list = get_photo_list(pk);
                users_list = current_list.get_create_comment_include_users()
            }
            else if code == "doc".to_string() {
                use crate::utils::get_doc_list;
                let current_list = get_doc_list(pk);
                users_list = current_list.get_create_comment_include_users()
            }
            else if code == "goo".to_string() {
                use crate::utils::get_good_list;
                let current_list = get_good_list(pk);
                users_list = current_list.get_create_comment_include_users()
            }
            else if code == "vid".to_string() {
                use crate::utils::get_video_list;
                let current_list = get_video_list(pk);
                users_list = current_list.get_create_comment_include_users()
            }
            else if code == "mus".to_string() {
                use crate::utils::get_music_list;
                let current_list = get_music_list(pk);
                users_list = current_list.get_create_comment_include_users()
            }
        }
        else if types == "copy_el".to_string() {
            text = "копировать записи и список".to_string();
            if code == "pos".to_string() {
                use crate::utils::get_post_list;
                let current_list = get_post_list(pk);
                users_list = current_list.get_copy_el_include_users()
            }
            else if code == "pho".to_string() {
                use crate::utils::get_photo_list;
                let current_list = get_photo_list(pk);
                users_list = current_list.get_copy_el_include_users()
            }
            else if code == "doc".to_string() {
                use crate::utils::get_doc_list;
                let current_list = get_doc_list(pk);
                users_list = current_list.get_copy_el_include_users()
            }
            else if code == "goo".to_string() {
                use crate::utils::get_good_list;
                let current_list = get_good_list(pk);
                users_list = current_list.get_copy_el_include_users()
            }
            else if code == "vid".to_string() {
                use crate::utils::get_video_list;
                let current_list = get_video_list(pk);
                users_list = current_list.get_copy_el_include_users()
            }
            else if code == "mus".to_string() {
                use crate::utils::get_music_list;
                let current_list = get_music_list(pk);
                users_list = current_list.get_copy_el_include_users()
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/load/list_include_users.stpl")]
            struct Template {
                request_user:        User,
                object_list:         Vec<User>,
                users:               Vec<User>,
                next_page_number:    i32,
                types:               String,
                count:               i32,
            }

            let body = Template {
                request_user:        _request_user,
                object_list:         object_list,
                users:               users_list,
                next_page_number:    next_page_number,
                types:               types,
                count:               count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {

            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/load/list_include_users.stpl")]
            struct Template {
                request_user:        User,
                object_list:         Vec<User>,
                users:               Vec<User>,
                next_page_number:    i32,
                types:               String,
                count:               i32,
            }

            let body = Template {
                request_user:        _request_user,
                object_list:         object_list,
                users:               users_list,
                next_page_number:    next_page_number,
                types:               types,
                count:               count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
