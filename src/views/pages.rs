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
use crate::utils::{
    is_signed_in,
    establish_connection,
    get_request_user_data,
    get_list_variables,
    get_type,
    get_device_and_ajax,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/featured/", web::get().to(featured_list_page));
    config.route("/all-users/", web::get().to(all_users_page));
    config.route("/all-communities/", web::get().to(all_communities_page));
    config.route("/mobile-menu/", web::get().to(mobile_menu_page));

    config.route("/progs/check_custom_link/{slug}/", web::get().to(check_custom_link));
    config.route("/progs/create_repost/", web::get().to(create_repost_page));
    config.route("/progs/create_claim/", web::get().to(create_claim_page));
    config.route("/load/reactions/", web::get().to(all_reactions_page));

    config.route("/{slug}/", web::get().to(link_page));
}

pub async fn link_page(session: Session, req: HttpRequest, slug: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::views::users::user_page;
    use crate::views::communities::community_page;

    let link = slug.clone();
    if &link[..2] == "id".to_string() {
        return user_page(session, req, link).await
    }
    else if &link.len() > &5 && &link[..6] == "public".to_string() {
        return community_page(session, req, link).await
    }
    else {
        use crate::schema::custom_links::dsl::custom_links;
        use crate::models::CustomLink;

        let _connection = establish_connection();
        let link_some = custom_links
            .filter(schema::custom_links::link.eq(&link))
            .limit(1)
            .load::<CustomLink>(&_connection)
            .expect("E.");

        if link_some.len() == 0 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            if link_some[0].owner == 1 {
                return user_page(session, req, link).await
            }
            else if link_some[0].owner == 2 {
                return community_page(session, req, link).await
            }
            else {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
            }
        }
    }
}

pub async fn index_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let _connection = establish_connection();

    #[derive(TemplateOnce)]
    #[template(path = "desctop/main/auth/auth.stpl")]
    struct DesctopAuthTemplate {
        is_ajax: bool,
    }
    #[derive(TemplateOnce)]
    #[template(path = "desctop/main/lists/news_list.stpl")]
    struct DesctopNewsListTemplate {
        request_user: User,
        is_ajax:      bool,
    }

    #[derive(TemplateOnce)]
    #[template(path = "mobile/main/auth/auth.stpl")]
    struct MobileAuthTemplate {
        is_ajax: bool,
    }
    #[derive(TemplateOnce)]
    #[template(path = "mobile/main/lists/news_list.stpl")]
    struct MobileNewsListTemplate {
        request_user: User,
        is_ajax:      bool,
    }

    let _connection = establish_connection();
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_signed_in(&session) {

        let _request_user = get_request_user_data(&session);

        if is_desctop {
            let body = DesctopNewsListTemplate {
                request_user: _request_user,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            let body = MobileNewsListTemplate {
                request_user: _request_user,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

    } else {
        if is_desctop {
            let body = DesctopAuthTemplate {
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            let body = MobileAuthTemplate {is_ajax: is_ajax,}
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn featured_list_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let _connection = establish_connection();
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/lists/featured_list.stpl")]
            struct DesctopFeaturedListTemplate {
                request_user: User,
                is_ajax:      bool,
            }
            let body = DesctopFeaturedListTemplate {
                request_user: _request_user,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/lists/featured_list.stpl")]
            struct MobileFeaturedListTemplate {
                request_user: User,
                is_ajax:      bool,
            }
            let body = MobileFeaturedListTemplate {
                request_user: _request_user,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}


pub async fn all_users_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_page_and_ajax;

    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let mut next_page_number = 0;
    let object_list: Vec<User>;

    let _connection = establish_connection();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let count = _request_user.get_all_users_count();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = _request_user.get_users(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = _request_user.get_users(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lists/all_users.stpl")]
            struct Template {
                request_user:     User,
                next_page_number: i32,
                object_list:      Vec<User>,
                count:            usize,
                is_ajax:          bool,
            }

            let body = Template {
                request_user:     _request_user,
                next_page_number: next_page_number,
                object_list:      object_list,
                count:            count,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/lists/all_users.stpl")]
            struct Template {
                request_user:     User,
                next_page_number: i32,
                object_list:      Vec<User>,
                count:            usize,
                is_ajax:          bool,
            }

            let body = Template {
                request_user:     _request_user,
                next_page_number: next_page_number,
                object_list:      object_list,
                count:            count,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

    } else {
        let count = User::get_anon_users_count();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = User::get_anon_users(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = User::get_anon_users(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lists/anon_all_users.stpl")]
            struct Template {
                next_page_number: i32,
                object_list:      Vec<User>,
                count:            usize,
                is_ajax:          bool,
            }
            let body = Template {
                next_page_number: next_page_number,
                object_list:      object_list,
                count:            count,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/lists/anon_all_users.stpl")]
            struct Template {
                next_page_number: i32,
                object_list:      Vec<User>,
                count:            usize,
                is_ajax:          bool,
            }
            let body = Template {
                next_page_number: next_page_number,
                object_list:      object_list,
                count:            count,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


pub async fn all_communities_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::models::Community;
    use crate::utils::get_device_and_page_and_ajax;

    let (is_desctop, page, is_ajax) = get_device_and_page_and_ajax(&req);
    let mut next_page_number = 0;

    let object_list: Vec<Community>;

    let _connection = establish_connection();
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let count = User::get_all_communities_count();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = User::get_all_communities(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = User::get_all_communities(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lists/all_communities.stpl")]
            struct Template {
                request_user:     User,
                next_page_number: i32,
                object_list:      Vec<Community>,
                count:            usize,
                is_ajax:          bool,
            }

            let body = Template {
                request_user:     _request_user,
                next_page_number: next_page_number,
                object_list:      object_list,
                count:            count,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/lists/all_communities.stpl")]
            struct Template {
                request_user:     User,
                next_page_number: i32,
                object_list:      Vec<Community>,
                count:            usize,
                is_ajax:          bool,
            }

            let body = Template {
                request_user:     _request_user,
                next_page_number: next_page_number,
                object_list:      object_list,
                count:            count,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

    } else {
        let count = User::get_all_communities_count();
        if page > 1 {
            let step = (page - 1) * 20;
            object_list = User::get_all_communities(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            object_list = User::get_all_communities(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lists/anon_all_communities.stpl")]
            struct Template {
                next_page_number: i32,
                object_list:      Vec<Community>,
                count:            usize,
                is_ajax:          bool,
            }
            let body = Template {
                next_page_number: next_page_number,
                object_list:      object_list,
                count:            count,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/lists/anon_all_communities.stpl")]
            struct Template {
                next_page_number: i32,
                object_list:      Vec<Community>,
                count:            usize,
                is_ajax:          bool,
            }
            let body = Template {
                next_page_number: next_page_number,
                object_list:      object_list,
                count:            count,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn check_custom_link(slug: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::custom_link_check;

    let link = slug.clone();
    let (_bool, _string) = custom_link_check(&link);
    let answer = "
    <div>
        <span id='bool'>".to_owned() + &_bool.to_string() + &"</span>
        <span id='string'>".to_string() + &_string.to_string() + &"</span>
    </div>".to_string();
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(answer))
}

pub async fn create_repost_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::{get_user_permission,get_community_permission};

        let (_type_exists, item_id, types) = get_type(&req);
        let _request_user = get_request_user_data(&session);
        let _request_user_id = &_request_user.id;
        let text = "".to_string();
        let mut creator_id = 0;
        let mut is_list = false;
        let mut can_copy_item = false;
        let mut permission_check = false;

        let pre_types = &types[..1];

        if pre_types == "l".to_string() {
            is_list = true;
            if types == "lpo".to_string() {
                use crate::utils::get_post_list;

                let list = get_post_list(item_id);
                can_copy_item = list.is_user_can_see_el(*_request_user_id) && list.is_user_can_copy_el(*_request_user_id);
                creator_id = list.user_id;
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "ldo".to_string() {
                use crate::utils::get_doc_list;

                let list = get_doc_list(item_id);
                can_copy_item = list.is_user_can_see_el(*_request_user_id) && list.is_user_can_copy_el(*_request_user_id);
                creator_id = list.user_id;
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "lgo".to_string() {
                use crate::utils::get_good_list;

                let list = get_good_list(item_id);
                can_copy_item = list.is_user_can_see_el(*_request_user_id) && list.is_user_can_copy_el(*_request_user_id);
                creator_id = list.user_id;
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "lmu".to_string() {
                use crate::utils::get_music_list;

                let list = get_music_list(item_id);
                can_copy_item = list.is_user_can_see_el(*_request_user_id) && list.is_user_can_copy_el(*_request_user_id);
                creator_id = list.user_id;
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "lph".to_string() {
                use crate::utils::get_photo_list;

                let list = get_photo_list(item_id);
                can_copy_item = list.is_user_can_see_el(*_request_user_id) && list.is_user_can_copy_el(*_request_user_id);
                creator_id = list.user_id;
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "lsu".to_string() {
                use crate::utils::get_survey_list;

                let list = get_survey_list(item_id);
                can_copy_item = list.is_user_can_see_el(*_request_user_id) && list.is_user_can_copy_el(*_request_user_id);
                creator_id = list.user_id;
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "lvi".to_string() {
                use crate::utils::get_video_list;

                let list = get_video_list(item_id);
                can_copy_item = list.is_user_can_see_el(*_request_user_id) && list.is_user_can_copy_el(*_request_user_id);
                creator_id = list.user_id;
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
        }
        else if pre_types == "u".to_string() {
            use crate::utils::get_user;

            let user = get_user(item_id);
            permission_check = get_user_permission(&user, &_request_user).0;
        }
        else if pre_types == "c".to_string() {
            use crate::utils::get_community;

            let community = get_community(item_id);
            permission_check = get_community_permission(&community, &_request_user).0;
        }
        else {
            if types == "pos".to_string() {
                use crate::utils::get_post;

                let list = get_post(item_id).get_list();
                can_copy_item = list.is_user_can_see_el(*_request_user_id) && list.is_user_can_copy_el(*_request_user_id);
                creator_id = list.user_id;
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "doc".to_string() {
                use crate::utils::get_doc;

                let list = get_doc(item_id).get_list();
                can_copy_item = list.is_user_can_see_el(*_request_user_id) && list.is_user_can_copy_el(*_request_user_id);
                creator_id = list.user_id;
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "goo".to_string() {
                use crate::utils::get_good;

                let list = get_good(item_id).get_list();
                can_copy_item = list.is_user_can_see_el(*_request_user_id) && list.is_user_can_copy_el(*_request_user_id);
                creator_id = list.user_id;
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "mus".to_string() {
                use crate::utils::get_music;

                let list = get_music(item_id).get_list();
                can_copy_item = list.is_user_can_see_el(*_request_user_id) && list.is_user_can_copy_el(*_request_user_id);
                creator_id = list.user_id;
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "sur".to_string() {
                use crate::utils::get_survey;

                let list = get_survey(item_id).get_list();
                creator_id = list.user_id;
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "vid".to_string() {
                use crate::utils::get_video;

                let list = get_video(item_id).get_list();
                creator_id = list.user_id;
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
        }
        if permission_check == false {
            #[derive(TemplateOnce)]
            #[template(path = "base_block/close/close_item.stpl")]
            struct Template {
                text: String,
            }
            let body = Template {
                text:  "Permission Denied.".to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/generic/repost/repost.stpl")]
            struct Template {
                request_user:  User,
                text:          String,
                types:         String,
                creator_id:    i32,
                item_id:       i32,
                is_list:       bool,
                can_copy_item: bool,
            }
            let body = Template {
                request_user:  _request_user,
                text:          text,
                types:         types,
                creator_id:    creator_id,
                item_id:       item_id,
                is_list:       is_list,
                can_copy_item: can_copy_item,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn create_claim_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::{get_user_permission,get_community_permission};

        let (_type_exists, item_id, types) = get_type(&req);
        let _request_user = get_request_user_data(&session);
        let _request_user_id = &_request_user.id;
        let text = "".to_string();
        let mut permission_check = false;
        let pre_types = &types[..1];

        if pre_types == "l".to_string() {
            if types == "lpo".to_string() {
                use crate::utils::get_post_list;

                let list = get_post_list(item_id);
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "ldo".to_string() {
                use crate::utils::get_doc_list;

                let list = get_doc_list(item_id);
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "lgo".to_string() {
                use crate::utils::get_good_list;

                let list = get_good_list(item_id);
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "lmu".to_string() {
                use crate::utils::get_music_list;

                let list = get_music_list(item_id);
                if list.community_id.is_some() {
                    permission_check = get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "lph".to_string() {
                use crate::utils::get_photo_list;

                let list = get_photo_list(item_id);
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "lsu".to_string() {
                use crate::utils::get_survey_list;

                let list = get_survey_list(item_id);
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "lvi".to_string() {
                use crate::utils::get_video_list;

                let list = get_video_list(item_id);
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
        }
        else if types == "use".to_string() {
            use crate::utils::get_user;

            let user = get_user(item_id);
            permission_check = get_user_permission(&user, &_request_user).0;
        }
        else if types == "com".to_string() {
            use crate::utils::get_community;

            let community = get_community(item_id);
            permission_check = get_community_permission(&community, &_request_user).0;
        }
        else if pre_types == "c".to_string() {
            if types == "cpo".to_string() {
                use crate::utils::get_post_comment;

                let list = get_post_comment(item_id).get_list();
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "cph".to_string() {
                use crate::utils::get_photo_comment;

                let list = get_photo_comment(item_id).get_list();
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "cgo".to_string() {
                use crate::utils::get_good_comment;

                let list = get_good_comment(item_id).get_list();
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "cvi".to_string() {
                use crate::utils::get_video_comment;

                let list = get_video_comment(item_id).get_list();
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
        }
        else {
            if types == "pos".to_string() {
                use crate::utils::get_post;

                let list = get_post(item_id).get_list();
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "doc".to_string() {
                use crate::utils::get_doc;

                let list = get_doc(item_id).get_list();
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "goo".to_string() {
                use crate::utils::get_good;

                let list = get_good(item_id).get_list();
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "mus".to_string() {
                use crate::utils::get_music;

                let list = get_music(item_id).get_list();
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "sur".to_string() {
                use crate::utils::get_survey;

                let list = get_survey(item_id).get_list();
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else if types == "vid".to_string() {
                use crate::utils::get_video;

                let list = get_video(item_id).get_list();
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(*_request_user_id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
        }
        if permission_check == false {
            #[derive(TemplateOnce)]
            #[template(path = "base_block/close/close_item.stpl")]
            struct Template {
                text: String,
            }
            let body = Template {
                text:  "Permission Denied.".to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/generic/user/report.stpl")]
            struct Template {
                //request_user:  User,
                text:          String,
                types:         String,
            }
            let body = Template {
                //request_user:  _request_user,
                text:          text,
                types:         types,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn all_reactions_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::{
        get_user_permission,
        get_community_permission,
        get_anon_user_permission,
        get_anon_community_permission,
    };

    #[derive(Debug, Deserialize)]
    pub struct TypesParams2 {
        pub types:    Option<String>,
        pub reaction: Option<i16>,
    }
    let mut item_id: i32 = 0;
    let mut code = "".to_string();
    let mut reaction: i16 = 0;

    let params_some = web::Query::<TypesParams2>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.types.is_some() {
            let item = params.types.as_deref().unwrap();
            item_id = item[3..].parse().unwrap();
            code = item[..3].to_string();
        }
        if params.reaction.is_some() {
            reaction = params.reaction.unwrap();
        }
    }
    let is_auth = is_signed_in(&session);
    let (_is_desctop, page) = get_list_variables(req);
    let mut next_page_number = 0;
    let mut step = 0;
    let mut text = "".to_string();
    let mut permission_check = false;
    let mut object_list: Vec<User> = Vec::new();
    let mut reaction_list: Vec<i16> = Vec::new();

    if page > 1 {
        step = (page - 1) * 20;
    }

    if code == "c".to_string() {
        if code == "cpo".to_string() {
            use crate::utils::get_post_comment;

            let comment = get_post_comment(item_id);
            let list = comment.get_list();
            if is_auth {
                let _request_user = get_request_user_data(&session);
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(_request_user.id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(_request_user.id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else {
                if list.community_id.is_some() {
                    permission_check = list.is_anon_user_can_see_el() && get_anon_community_permission(&list.get_community()).0;
                }
                else {
                    permission_check = list.is_anon_user_can_see_el() && get_anon_user_permission(&list.get_creator()).0;
                }
            }

            if permission_check {
                object_list = comment.get_reactions_users_of_types(20, step.into(), reaction);
                if page > 1 && comment.reactions > (page * 20) {
                    next_page_number = page + 1;
                }
                else {
                    if comment.reactions > 20 {
                        next_page_number = 2;
                    }
                }
                text = comment.count_reactions_of_types_ru(reaction);
                reaction_list = list.get_reactions_list();
            }
        }
        else if code == "cph".to_string() {
            use crate::utils::get_photo_comment;

            let comment = get_photo_comment(item_id);
            let list = comment.get_list();
            if is_auth {
                let _request_user = get_request_user_data(&session);
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(_request_user.id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(_request_user.id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else {
                if list.community_id.is_some() {
                    permission_check = list.is_anon_user_can_see_el() && get_anon_community_permission(&list.get_community()).0;
                }
                else {
                    permission_check = list.is_anon_user_can_see_el() && get_anon_user_permission(&list.get_creator()).0;
                }
            }

            if permission_check {
                object_list = comment.get_reactions_users_of_types(20, step.into(), reaction);
                if page > 1 && comment.reactions > (page * 20) {
                    next_page_number = page + 1;
                }
                else {
                    if comment.reactions > 20 {
                        next_page_number = 2;
                    }
                }
                text = comment.count_reactions_of_types_ru(reaction);
                reaction_list = list.get_reactions_list();
            }
        }
        else if code == "cgo".to_string() {
            use crate::utils::get_good_comment;

            let comment = get_good_comment(item_id);
            let list = comment.get_list();
            if is_auth {
                let _request_user = get_request_user_data(&session);
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(_request_user.id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(_request_user.id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else {
                if list.community_id.is_some() {
                    permission_check = list.is_anon_user_can_see_el() && get_anon_community_permission(&list.get_community()).0;
                }
                else {
                    permission_check = list.is_anon_user_can_see_el() && get_anon_user_permission(&list.get_creator()).0;
                }
            }

            if permission_check {
                object_list = comment.get_reactions_users_of_types(20, step.into(), reaction);
                if page > 1 && comment.reactions > (page * 20) {
                    next_page_number = page + 1;
                }
                else {
                    if comment.reactions > 20 {
                        next_page_number = 2;
                    }
                }
                text = comment.count_reactions_of_types_ru(reaction);
                reaction_list = list.get_reactions_list();
            }
        }
        else if code == "cvi".to_string() {
            use crate::utils::get_video_comment;

            let comment = get_video_comment(item_id);
            let list = comment.get_list();
            if is_auth {
                let _request_user = get_request_user_data(&session);
                if list.community_id.is_some() {
                    permission_check = list.is_user_can_see_el(_request_user.id) && get_community_permission(&list.get_community(), &_request_user).0;
                }
                else {
                    permission_check = list.is_user_can_see_el(_request_user.id) && get_user_permission(&list.get_creator(), &_request_user).0;
                }
            }
            else {
                if list.community_id.is_some() {
                    permission_check = list.is_anon_user_can_see_el() && get_anon_community_permission(&list.get_community()).0;
                }
                else {
                    permission_check = list.is_anon_user_can_see_el() && get_anon_user_permission(&list.get_creator()).0;
                }
            }

            if permission_check {
                object_list = comment.get_reactions_users_of_types(20, step.into(), reaction);
                if page > 1 && comment.reactions > (page * 20) {
                    next_page_number = page + 1;
                }
                else {
                    if comment.reactions > 20 {
                        next_page_number = 2;
                    }
                }
                text = comment.count_reactions_of_types_ru(reaction);
                reaction_list = list.get_reactions_list();
            }
        }
    }
    else {

    if code == "pos".to_string() {
        use crate::utils::get_post;

        let item = get_post(item_id);
        let list = item.get_list();
        if is_auth {
            let _request_user = get_request_user_data(&session);
            if list.community_id.is_some() {
                permission_check = list.is_user_can_see_el(_request_user.id) && get_community_permission(&list.get_community(), &_request_user).0;
            }
            else {
                permission_check = list.is_user_can_see_el(_request_user.id) && get_user_permission(&list.get_creator(), &_request_user).0;
            }
        }
        else {
            if list.community_id.is_some() {
                permission_check = list.is_anon_user_can_see_el() && get_anon_community_permission(&list.get_community()).0;
            }
            else {
                permission_check = list.is_anon_user_can_see_el() && get_anon_user_permission(&list.get_creator()).0;
            }
        }

        if permission_check {
            object_list = item.get_reactions_users_of_types(20, step.into(), reaction);
            if page > 1 && item.reactions > (page * 20) {
                next_page_number = page + 1;
            }
            else {
                if item.reactions > 20 {
                    next_page_number = 2;
                }
            }
            text = item.count_reactions_of_types_ru(reaction);
            reaction_list = list.get_reactions_list();
        }
    }
    else if code == "pho".to_string() {
        use crate::utils::get_photo;

        let item = get_photo(item_id);
        let list = item.get_list();
        if is_auth {
            let _request_user = get_request_user_data(&session);
            if list.community_id.is_some() {
                permission_check = list.is_user_can_see_el(_request_user.id) && get_community_permission(&list.get_community(), &_request_user).0;
            }
            else {
                permission_check = list.is_user_can_see_el(_request_user.id) && get_user_permission(&list.get_creator(), &_request_user).0;
            }
        }
        else {
            if list.community_id.is_some() {
                permission_check = list.is_anon_user_can_see_el() && get_anon_community_permission(&list.get_community()).0;
            }
            else {
                permission_check = list.is_anon_user_can_see_el() && get_anon_user_permission(&list.get_creator()).0;
            }
        }
        if permission_check {
            object_list = item.get_reactions_users_of_types(20, step.into(), reaction);
            if page > 1 && item.reactions > (page * 20) {
                next_page_number = page + 1;
            }
            else {
                if item.reactions > 20 {
                    next_page_number = 2;
                }
            }
            text = item.count_reactions_of_types_ru(reaction);
            reaction_list = list.get_reactions_list();
        }
    }
    else if code == "goo".to_string() {
        use crate::utils::get_good;

        let item = get_good(item_id);
        let list = item.get_list();
        if is_auth {
            let _request_user = get_request_user_data(&session);
            if list.community_id.is_some() {
                permission_check = list.is_user_can_see_el(_request_user.id) && get_community_permission(&list.get_community(), &_request_user).0;
            }
            else {
                permission_check = list.is_user_can_see_el(_request_user.id) && get_user_permission(&list.get_creator(), &_request_user).0;
            }
        }
        else {
            if list.community_id.is_some() {
                permission_check = list.is_anon_user_can_see_el() && get_anon_community_permission(&list.get_community()).0;
            }
            else {
                permission_check = list.is_anon_user_can_see_el() && get_anon_user_permission(&list.get_creator()).0;
            }
        }
        if permission_check {
            object_list = item.get_reactions_users_of_types(20, step.into(), reaction);
            if page > 1 && item.reactions > (page * 20) {
                next_page_number = page + 1;
            }
            else {
                if item.reactions > 20 {
                    next_page_number = 2;
                }
            }
            text = item.count_reactions_of_types_ru(reaction);
            reaction_list = list.get_reactions_list();
        }
    }
    else if code == "vid".to_string() {
        use crate::utils::get_video;

        let item = get_video(item_id);
        let list = item.get_list();
        if is_auth {
            let _request_user = get_request_user_data(&session);
            if list.community_id.is_some() {
                permission_check = list.is_user_can_see_el(_request_user.id) && get_community_permission(&list.get_community(), &_request_user).0;
            }
            else {
                permission_check = list.is_user_can_see_el(_request_user.id) && get_user_permission(&list.get_creator(), &_request_user).0;
            }
        }
        else {
            if list.community_id.is_some() {
                permission_check = list.is_anon_user_can_see_el() && get_anon_community_permission(&list.get_community()).0;
            }
            else {
                permission_check = list.is_anon_user_can_see_el() && get_anon_user_permission(&list.get_creator()).0;
            }
        }
        if permission_check {
            object_list = item.get_reactions_users_of_types(20, step.into(), reaction);
            if page > 1 && item.reactions > (page * 20) {
                next_page_number = page + 1;
            }
            else {
                if item.reactions > 20 {
                    next_page_number = 2;
                }
            }
            text = item.count_reactions_of_types_ru(reaction);
            reaction_list = list.get_reactions_list();
        }
    }
    }

    if permission_check == false {
        #[derive(TemplateOnce)]
        #[template(path = "base_block/close/close_item.stpl")]
        struct Template {
            text: String,
        }
        let body = Template {
            text:  "Permission Denied.".to_string(),
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/items/reactions.stpl")]
        struct Template {
            text:             String,
            types:            String,
            object_list:      Vec<User>,
            next_page_number: i32,
            reaction:         i16,
            reaction_list:    Vec<i16>,
        }
        let body = Template {
            text:             text,
            types:            code + &item_id.to_string(),
            object_list:      object_list,
            next_page_number: next_page_number,
            reaction:         reaction,
            reaction_list:    reaction_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn mobile_menu_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::utils::get_ajax;

        let is_ajax = get_ajax(&req);
        #[derive(TemplateOnce)]
        #[template(path = "mobile/main/menu.stpl")]
        struct Template {
            request_user: User,
            is_ajax:      bool,
        }

        let _request_user = get_request_user_data(&session);
        let body = Template {
            request_user: _request_user,
            is_ajax:      is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("not permission"))
    }
}
