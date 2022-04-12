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
    use crate::utils::get_default_template_2;

    let _connection = establish_connection();
    let mut _template : String;

    let (_type, data) = get_default_template_2(req, session);
    if is_signed_in(&session) {
        _template = _type + &"main/lists/news_list.html".to_string();
    } else {
        _template = _type + &"main/auth/auth.html".to_string();
    }

    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
