use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    is_signed_in,
    establish_connection,
    is_desctop,
    get_request_user_data,
    get_user,
    get_community,
    get_doc_list,
    get_user_permission,
    get_anon_user_permission,
    get_community_permission,
    get_anon_community_permission,
    get_list_variables,
};

use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, DocList, Doc};


pub fn profile_urls(config: &mut web::ServiceConfig) {
    config.route("/docs/load_list/{list_id}/", web::get().to(load_list_docs_page));
}
