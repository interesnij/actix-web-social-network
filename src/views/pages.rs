use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web
};
use serde::Deserialize;
use tera::Context;
use crate::utils::{establish_connection, get_default_template, TEMPLATES};
use crate::diesel::RunQueryDsl;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index));
}

#[derive(Debug, Deserialize)]
pub struct SParams {
    pub q: String,
}

pub async fn index(req: HttpRequest) -> impl Responder {
    use actix_web_httpauth::headers::authorization::{Authorization, Basic};

    let auth = Authorization::<Basic>::parse(req);
    let _connection = establish_connection();
    let mut data = Context::new();
    let mut _template : String;

    let (_type, _is_host_admin) = get_default_template(req);
    if auth {
        _template = _type + &"main/lists/news_list.html".to_string();
    } else {
        _template = _type + &"main/auth/auth.html".to_string();
    }

    data.insert("is_host_admin", &_is_host_admin);
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
