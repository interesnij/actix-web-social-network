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
    get_community,
    //get_community_with_link,
    //get_community_permission,
    get_list_variables,
    //establish_connection,
};
use crate::diesel::RunQueryDsl;
use actix_session::Session;
use sailfish::TemplateOnce;
//use crate::models::{User, Post, Community};


pub fn community_urls(config: &mut web::ServiceConfig) {
    config.route("/communities/{id}/settings/", web::get().to(settings_page));
    config.route("/communities/{id}/settings/private/", web::get().to(private_settings_page));
    config.route("/communities/{id}/settings/followers/", web::get().to(followers_settings_page));
    config.route("/communities/{id}/settings/blacklist/", web::get().to(blacklist_settings_page));
    config.route("/communities/{id}/settings/members/", web::get().to(members_settings_page));

    config.route("/communities/{id}/settings/administrators/", web::get().to(members_settings_page));
    config.route("/communities/{id}/settings/editors/", web::get().to(members_settings_page));
    config.route("/communities/{id}/settings/moderators/", web::get().to(members_settings_page));
    config.route("/communities/{id}/settings/advertisers/", web::get().to(members_settings_page));
}
