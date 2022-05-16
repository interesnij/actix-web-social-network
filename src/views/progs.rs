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
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/check_custom_link/{slug}/", web::get().to(check_custom_link));
}

pub async fn check_custom_link(session: Session, req: HttpRequest, slug: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::custom_link_check;

    let link = slug.clone();
    let (_bool, _string) = custom_link_check(&link);
    let answer = "
    <div>
        <span id='bool'>".to_owned() + &_bool.to_string() + &"</span>
        <span id='string'>".to_string() + &"</span>
    </div>".to_string();
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(answer))
}
