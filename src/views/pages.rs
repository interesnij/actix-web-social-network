use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web,
    http::header::Header
};
use serde::Deserialize;
use tera::Context;
use crate::utils::{establish_connection, get_default_template, TEMPLATES};
use crate::diesel::RunQueryDsl;
use crate::schema;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index));
}

#[derive(Debug, Deserialize)]
pub struct SParams {
    pub q: String,
}

pub async fn index(req: HttpRequest) -> impl Responder {
    use actix_web_httpauth::headers::authorization::{Authorization, Basic};
    use crate::schema::users::dsl::users;
    use crate::models::User;

    let _auth = Authorization::<Basic>::parse(&req);
    let _connection = establish_connection();
    let mut data = Context::new();
    let mut _template : String;

    let (_type, _is_host_admin) = get_default_template(req);
    if _auth.is_ok() {
        _template = _type + &"main/lists/news_list.html".to_string();
    } else {
        _template = _type + &"main/auth/auth.html".to_string();
    }
    let iner = 1;
    let find_users = users.filter(schema::users::id.eq(iner)).load::<User>(&_connection).expect("E");
    if find_users.len() > 1 {
        diesel::delete(&find_users[0]).execute(&_connection).expect("E");
    }

    let _all_users :Vec<User> = users.load(&_connection).expect("Error");
    data.insert("is_host_admin", &_is_host_admin);
    data.insert("all_users", &_all_users);
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
