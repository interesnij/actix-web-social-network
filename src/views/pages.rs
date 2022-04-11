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
use crate::schema;
use diesel::prelude::*;
use crate::utils::is_signed_in;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index));
}

#[derive(Debug, Deserialize)]
pub struct SParams {
    pub q: String,
}

pub async fn index(session: Session, req: HttpRequest) -> impl Responder {
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
    if is_signed_in(&session) {
        data.insert("is_authenticated", true);
    } else {
        data.insert("is_authenticated", false);
    }
    let _all_users :Vec<User> = users.load(&_connection).expect("Error");
    data.insert("all_users", &_all_users);
    for user in _all_users {
        let phone = &user.phone;
        use rand::Rng;
        let xxx1: i32 = rand::thread_rng().gen_range(0..10000);
        let yyy1: String = xxx1.to_string();
        diesel::update(&user)
            .set(schema::users::phone.eq(yyy1))
            .get_result::<User>(&_connection)
            .expect("Error.");
    }
    data.insert("is_host_admin", &_is_host_admin);

    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
