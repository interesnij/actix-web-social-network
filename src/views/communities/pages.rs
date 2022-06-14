use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    is_signed_in,
    get_request_user_data,
    get_community,
    get_community_with_link,
    get_community_permission,
    get_anon_community_permission,
    get_list_variables,
    establish_connection,
};
use crate::diesel::RunQueryDsl;
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, Post, Community};


pub fn community_urls(config: &mut web::ServiceConfig) {
    config.route("/communities/{community_id}/wall/{list_id}/", web::get().to(community_wall_page));

    config.route("/communities/{community_id}/photos/", web::get().to(community_photos_page));
    config.route("/communities/{community_id}/goods/", web::get().to(community_goods_page));
    config.route("/communities/{community_id}/music/", web::get().to(community_music_page));
    config.route("/communities/{community_id}/surveys/", web::get().to(community_surveys_page));
    config.route("/communities/{community_id}/video/", web::get().to(community_video_page));
    config.route("/communities/{community_id}/docs/", web::get().to(community_docs_page));

    config.route("/communities/{community_id}/photos_list/{list_id}/", web::get().to(community_photos_list_page));
    config.route("/communities/{community_id}/goods_list/{list_id}/", web::get().to(community_goods_list_page));
    config.route("/communities/{community_id}/music_list/{list_id}/", web::get().to(community_music_list_page));
    config.route("/communities/{community_id}/surveys_list/{list_id}/", web::get().to(community_surveys_list_page));
    config.route("/communities/{community_id}/video_list/{list_id}/", web::get().to(community_video_list_page));
    config.route("/communities/{community_id}/docs_list/{list_id}/", web::get().to(community_docs_list_page));
    config.route("/communities/create_community/", web::get().to(create_community_page));
}

pub async fn community_docs_page(session: Session, req: HttpRequest, community_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::DocList;
    use crate::utils::{get_doc_list, get_device_and_ajax};

    let community_id : i32 = *community_id;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _community = get_community(community_id);
    let _list = get_doc_list(_community.get_selected_doc_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/docs/main_list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         DocList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/docs/main_list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         DocList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
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
            #[template(path = "desctop/communities/docs/main_list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      DocList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/docs/main_list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      DocList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_video_page(session: Session, req: HttpRequest, community_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::VideoList;
    use crate::utils::{get_video_list, get_device_and_ajax};

    let community_id : i32 = *community_id;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _community = get_community(community_id);
    let _list = get_video_list(_community.get_selected_video_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/video/main_list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         VideoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/video/main_list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         VideoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
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
            #[template(path = "desctop/communities/video/main_list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      VideoList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/video/main_list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      VideoList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_surveys_page(session: Session, req: HttpRequest, community_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::SurveyList;
    use crate::utils::{get_survey_list, get_device_and_ajax};

    let community_id : i32 = *community_id;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _community = get_community(community_id);
    let _list = get_survey_list(_community.get_selected_survey_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/survey/main_list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         SurveyList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/survey/main_list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         SurveyList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
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
            #[template(path = "desctop/communities/survey/main_list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      SurveyList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/survey/main_list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      SurveyList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_music_page(session: Session, req: HttpRequest, community_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::MusicList;
    use crate::utils::{get_music_list, get_device_and_ajax};

    let community_id : i32 = *community_id;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _community = get_community(community_id);
    let _list = get_music_list(_community.get_selected_music_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/music/main_list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         MusicList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/music/main_list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         MusicList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
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
            #[template(path = "desctop/communities/music/main_list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      MusicList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/music/main_list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      MusicList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_goods_page(session: Session, req: HttpRequest, community_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::GoodList;
    use crate::utils::{get_good_list, get_device_and_ajax};

    let community_id : i32 = *community_id;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _community = get_community(community_id);
    let _list = get_good_list(_community.get_selected_good_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/goods/main_list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         GoodList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/goods/main_list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         GoodList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
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
            #[template(path = "desctop/communities/goods/main_list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      GoodList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/goods/main_list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      GoodList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_photos_page(session: Session, req: HttpRequest, community_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::PhotoList;
    use crate::utils::{get_photo_list, get_device_and_ajax};

    let community_id : i32 = *community_id;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _community = get_community(community_id);
    let _list = get_photo_list(_community.get_selected_photo_list_pk());

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/photos/main_list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         PhotoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/photos/main_list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         PhotoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
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
            #[template(path = "desctop/communities/photos/main_list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      PhotoList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/photos/main_list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      PhotoList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_docs_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::DocList;
    use crate::utils::{get_doc_list, get_device_and_ajax};

    let community_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _community = get_community(community_id);
    let _list = get_doc_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/docs/list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         DocList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/docs/list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         DocList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
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
            #[template(path = "desctop/communities/docs/list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      DocList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/docs/list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      DocList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_video_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::VideoList;
    use crate::utils::{get_video_list, get_device_and_ajax};

    let community_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _community = get_community(community_id);
    let _list = get_video_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/video/list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         VideoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/video/list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         VideoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
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
            #[template(path = "desctop/communities/video/list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      VideoList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/video/list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      VideoList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_surveys_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::SurveyList;
    use crate::utils::{get_survey_list, get_device_and_ajax};

    let community_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _community = get_community(community_id);
    let _list = get_survey_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/survey/list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         SurveyList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/survey/list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         SurveyList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
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
            #[template(path = "desctop/communities/survey/list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      SurveyList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/survey/list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      SurveyList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_music_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::MusicList;
    use crate::utils::{get_music_list, get_device_and_ajax};

    let community_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _community = get_community(community_id);
    let _list = get_music_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/music/list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         MusicList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/music/list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         MusicList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
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
            #[template(path = "desctop/communities/music/list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      MusicList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/music/list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      MusicList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_goods_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::GoodList;
    use crate::utils::{get_good_list, get_device_and_ajax};

    let community_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _community = get_community(community_id);
    let _list = get_good_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/goods/list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         GoodList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/goods/list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         GoodList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
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
            #[template(path = "desctop/communities/goods/list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      GoodList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/goods/list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      GoodList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_photos_list_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::PhotoList;
    use crate::utils::{get_photo_list, get_device_and_ajax};

    let community_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let _community = get_community(community_id);
    let _list = get_photo_list(list_id);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/photos/list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         PhotoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/photos/list/list.stpl")]
            struct Template {
                request_user: User,
                community:    Community,
                list:         PhotoList,
                is_ajax:      bool,
            }

            let body = Template {
                request_user: _request_user,
                community:    _community,
                list:         _list,
                is_ajax:      is_ajax,
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
            #[template(path = "desctop/communities/photos/list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      PhotoList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/photos/list/anon_list.stpl")]
            struct Template {
                community: Community,
                list:      PhotoList,
                is_ajax:   bool,
            }
            let body = Template {
                community: _community,
                list:      _list,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_wall_page(session: Session, req: HttpRequest, param: web::Path<(i32,i32)>) -> actix_web::Result<HttpResponse> {
    use crate::models::PostList;
    use crate::utils::get_post_list;

    let community_id : i32 = param.0;
    let list_id : i32 = param.1;
    let (is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;

    let _community = get_community(community_id);
    let _list = get_post_list(list_id);

    let object_list: Vec<Post>;
    if page > 1 {
        let step = (page - 1) * 20;
        object_list = _list.get_paginate_items(20, step.into());
        if _list.count > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        object_list = _list.get_paginate_items(20, 0);
        if _list.count > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (is_open, text) = get_community_permission(&_community, &_request_user);
        let _request_user_id = &_request_user.id;
        let is_user_can_see_post_list = _list.is_user_can_see_el(*_request_user_id);
        let is_user_can_create_posts = _list.is_user_can_create_el(*_request_user_id);

        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }

        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/lenta/list.stpl")]
            struct Template {
                list:                      PostList,
                request_user:              User,
                is_user_can_see_post_list: bool,
                is_user_can_create_posts:  bool,
                object_list:               Vec<Post>,
                community:                 Community,
                next_page_number:          i32,
            }

            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_post_list: is_user_can_see_post_list,
                is_user_can_create_posts:  is_user_can_create_posts,
                object_list:               object_list,
                community:                 _community,
                next_page_number:          next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/lenta/list.stpl")]
            struct Template {
                list:                      PostList,
                request_user:              User,
                is_user_can_see_post_list: bool,
                is_user_can_create_posts:  bool,
                object_list:               Vec<Post>,
                community:                 Community,
                next_page_number:          i32,
            }
            let body = Template {
                list:                      _list,
                request_user:              _request_user,
                is_user_can_see_post_list: is_user_can_see_post_list,
                is_user_can_create_posts:  is_user_can_create_posts,
                object_list:               object_list,
                community:                 _community,
                next_page_number:          next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        }
    } else {
        let (is_open, text) = get_anon_community_permission(&_community);
        let is_user_can_see_post_list = _list.is_anon_user_can_see_el();
        if is_open == false {
            use crate::views::close_item;
            return close_item(text)
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/communities/lenta/anon_list.stpl")]
            struct Template {
                list:                      PostList,
                is_user_can_see_post_list: bool,
                object_list:               Vec<Post>,
                community:                 Community,
                next_page_number:          i32,
            }
            let body = Template {
                list:                      _list,
                is_user_can_see_post_list: is_user_can_see_post_list,
                object_list:               object_list,
                community:                 _community,
                next_page_number:          next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

        } else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/communities/lenta/anon_list.stpl")]
            struct Template {
                list:                      PostList,
                is_user_can_see_post_list: bool,
                object_list:               Vec<Post>,
                community:                 Community,
                next_page_number:          i32,
            }
            let body = Template {
                list:                      _list,
                is_user_can_see_post_list: is_user_can_see_post_list,
                object_list:               object_list,
                community:                 _community,
                next_page_number:          next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn community_page(session: Session, req: HttpRequest, link: String) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _community = get_community_with_link(link);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.types > 10 {
            use crate::views::my_bad_account;
            return my_bad_account(is_desctop, _request_user, is_ajax)
        }
        else if _request_user.is_administrator_of_community(_community.id) {
            if _community.types > 10 {
                return admin_bad_community(is_desctop, _community, _request_user, is_ajax)
            }
            else {
                return admin_community(is_desctop, _community, _request_user, is_ajax)
            }
        }
        else if _community.types > 10 {
            return bad_community(is_desctop, _community, _request_user, is_ajax)
        }
        else if _request_user.is_follow_from_community(_community.id) {
            return follow_community(is_desctop, _community, _request_user, is_ajax)
        }
        else if _request_user.is_child() && !_community.is_identified() {
            return no_child_safety_community(is_desctop, _community, _request_user, is_ajax)
        }
        else if _request_user.is_member_of_community(_community.id) {
            return public_community(is_desctop, _community, _request_user, is_ajax)
        }
        else if _community.is_public() {
            return public_community(is_desctop, _community, _request_user, is_ajax)
        }
        else if _community.is_close() {
            return close_community(is_desctop, _community, _request_user, is_ajax)
        }
        else if _community.is_private() {
            return private_community(is_desctop, _community, _request_user, is_ajax)
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        if _community.types > 10 {
            return anon_bad_community(is_desctop, _community, is_ajax)
        }
        else if _community.is_public() {
            return anon_community(is_desctop, _community, is_ajax)
        }
        else if _community.is_close() {
            return anon_close_community(is_desctop, _community, is_ajax)
        }
        else if _community.is_private() {
            return anon_private_community(is_desctop, _community, is_ajax)
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    }
}

pub fn admin_community(is_desctop: bool, community: Community, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/admin_community.stpl")]
        struct Template {
            request_user: User,
            community:    Community,
            is_ajax:      bool,
        }
        let body = Template {
            request_user: request_user,
            community:    community,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    } else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/admin_community.stpl")]
        struct Template {
            request_user: User,
            community:    Community,
            is_ajax:      bool,
        }
        let body = Template {
            request_user: request_user,
            community:    community,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_community(is_desctop: bool, community: Community, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/anon_community.stpl")]
        struct Template {
            private_bools: Vec<bool>,
            community:     Community,
            is_ajax:       bool,
        }
        let body = Template {
            private_bools: community.get_anon_community_all_can_see(),
            community:     community,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/anon_community.stpl")]
        struct Template {
            private_bools: Vec<bool>,
            community:     Community,
            is_ajax:       bool,
        }
        let body = Template {
            private_bools: community.get_anon_community_all_can_see(),
            community:     community,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn close_community(is_desctop: bool, community: Community, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/close_community.stpl")]
        struct Template {
            private_bools: Vec<bool>,
            community:     Community,
            request_user:  User,
            is_ajax:       bool,
        }
        let body = Template {
            private_bools: community.get_community_all_can_see(request_user.id),
            community:     community,
            request_user:  request_user,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/close_community.stpl")]
        struct Template {
            private_bools: Vec<bool>,
            community:     Community,
            request_user:  User,
            is_ajax:       bool,
        }
        let body = Template {
            private_bools: community.get_community_all_can_see(request_user.id),
            community:     community,
            request_user:  request_user,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn private_community(is_desctop: bool, community: Community, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/private_community.stpl")]
        struct Template {
            community:    Community,
            request_user: User,
            is_ajax:      bool,
        }
        let body = Template {
            community:    community,
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/private_community.stpl")]
        struct Template {
            community:    Community,
            request_user: User,
            is_ajax:      bool,
        }
        let body = Template {
            community:    community,
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn no_child_safety_community(is_desctop: bool, community: Community, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/no_child_safety.stpl")]
        struct Template {
            community:    Community,
            request_user: User,
            is_ajax:      bool,
        }
        let body = Template {
            community:    community,
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/no_child_safety.stpl")]
        struct Template {
            community:    Community,
            request_user: User,
            is_ajax:      bool,
        }
        let body = Template {
            community:    community,
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn follow_community(is_desctop: bool, community: Community, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/follow_community.stpl")]
        struct Template {
            private_bools: Vec<bool>,
            community:    Community,
            request_user: User,
            is_ajax:      bool,
        }
        let body = Template {
            private_bools: community.get_community_all_can_see(request_user.id),
            community:     community,
            request_user:  request_user,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/follow_community.stpl")]
        struct Template {
            private_bools: Vec<bool>,
            community:     Community,
            request_user:  User,
            is_ajax:       bool,
        }
        let body = Template {
            private_bools: community.get_community_all_can_see(request_user.id),
            community:     community,
            request_user:  request_user,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn admin_bad_community(is_desctop: bool, community: Community, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/admin_bad_community.stpl")]
        struct Template {
            community:    Community,
            request_user: User,
            is_ajax:      bool,
        }
        let body = Template {
            community:    community,
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/admin_bad_community.stpl")]
        struct Template {
            community:    Community,
            request_user: User,
            is_ajax:      bool,
        }
        let body = Template {
            community:    community,
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn bad_community(is_desctop: bool, community: Community, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/bad_community.stpl")]
        struct Template {
            community:    Community,
            request_user: User,
            is_ajax:      bool,
        }
        let body = Template {
            community:    community,
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/bad_community.stpl")]
        struct Template {
            community:    Community,
            request_user: User,
            is_ajax:      bool,
        }
        let body = Template {
            community:    community,
            request_user: request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub fn public_community(is_desctop: bool, community: Community, request_user: User, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/public_community.stpl")]
        struct Template {
            private_bools: Vec<bool>,
            community:     Community,
            request_user:  User,
            is_ajax:       bool,
        }
        let body = Template {
            private_bools: community.get_community_all_can_see(request_user.id),
            community:     community,
            request_user:  request_user,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/public_community.stpl")]
        struct Template {
            private_bools: Vec<bool>,
            community:     Community,
            request_user:  User,
            is_ajax:       bool,
        }
        let body = Template {
            private_bools: community.get_community_all_can_see(request_user.id),
            community:     community,
            request_user:  request_user,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_bad_community(is_desctop: bool, community: Community, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/anon_bad_community.stpl")]
        struct Template {
            community: Community,
            is_ajax:   bool,
        }
        let body = Template {
            community: community,
            is_ajax:   is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/anon_bad_community.stpl")]
        struct Template {
            community: Community,
            is_ajax:   bool,
        }
        let body = Template {
            community: community,
            is_ajax:   is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_close_community(is_desctop: bool, community: Community, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/anon_close_community.stpl")]
        struct Template {
            private_bools: Vec<bool>,
            community:     Community,
            is_ajax:       bool,
        }
        let body = Template {
            private_bools: community.get_anon_community_all_can_see(),
            community:     community,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/anon_close_community.stpl")]
        struct Template {
            private_bools: Vec<bool>,
            community:     Community,
            is_ajax:       bool,
        }
        let body = Template {
            private_bools: community.get_anon_community_all_can_see(),
            community:     community,
            is_ajax:       is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn anon_private_community(is_desctop: bool, community: Community, is_ajax: bool) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/detail/anon_private_community.stpl")]
        struct Template {
            community: Community,
            is_ajax:   bool,
        }
        let body = Template {
            community: community,
            is_ajax:   is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/communities/detail/anon_private_community.stpl")]
        struct Template {
            community: Community,
            is_ajax:   bool,
        }
        let body = Template {
            community: community,
            is_ajax:   is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn create_community_page(session: Session) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::schema::community_categorys::dsl::community_categorys;
        use crate::models::CommunityCategory;

        let _request_user = get_request_user_data(&session);
        let _connection = establish_connection();
        let categories = community_categorys
            .load::<CommunityCategory>(&_connection)
            .expect("E.");

        #[derive(TemplateOnce)]
        #[template(path = "desctop/communities/manage/create_community.stpl")]
        struct Template {
            request_user: User,
            categories:   Vec<CommunityCategory>,
        }
        let body = Template {
            request_user: _request_user,
            categories:   categories,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
    }
}
