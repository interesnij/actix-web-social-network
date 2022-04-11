use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web,
    http::header::Header
};
use serde::Deserialize;
use tera::Context;
use crate::utils::{is_signed_in, get_current_user, establish_connection, get_default_template, TEMPLATES};
use crate::schema;
use diesel::prelude::*;
use actix_session::Session;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index));
}

#[derive(Debug, Deserialize)]
pub struct SParams {
    pub q: String,
}

pub async fn index(session: Session, req: HttpRequest) -> impl Responder {
    use crate::schema::users::dsl::users;
    use crate::models::User;

    let _connection = establish_connection();
    let mut data = Context::new();
    let mut _template : String;

    let (_type, _is_host_admin) = get_default_template(req);
    let mut _request_user :String;
    if is_signed_in(&session) {
        _template = _type + &"main/lists/news_list.html".to_string();
        _request_user = Some(get_current_user(&session));
        data.insert("request_user", &_request_user);
    } else {
        _template = _type + &"main/auth/auth.html".to_string();
    }
    if is_signed_in(&session) {
        data.insert("is_authenticated", &true);
    } else {
        data.insert("is_authenticated", &false);
    }
    data.insert("is_host_admin", &_is_host_admin);
    data.insert("request.user", &_request_user);

    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
