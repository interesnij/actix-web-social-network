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
use crate::models::NewUser;


pub fn global_routes(config: &mut web::ServiceConfig) {
    config.route("/phone_send/{phone}/", web::post().to(phone_send));
    config.route("/phone_verify/{phone}/{code}/", web::post().to(phone_verify));
    config.route("/signup/", web::post().to(signup));
}

pub async fn process_signup(data: web::Form<NewUser>) -> impl Responder {
    use crate::schema::users;
    use crate::models::User;

    let connection = establish_connection();

    diesel::insert_into(users::table)
        .values(&*data)
        .get_result::<User>(&connection)
        .expect("Error registering user.");

    println!("{:?}", data);
    HttpResponse::Ok().body(format!("ok"))
}

pub async fn phone_send(req: HttpRequest, _phone: web::Path<String>) -> impl Responder {
    use crate::schema::{users, phone_codes};
    use crate::models::{User, PhoneCode};
    use {http_req::error, http_req::request, std::io, std::io::Write};

    let connection = establish_connection();
    let mut data = Context::new();
    let mut a = Vec::new();
    let _url = "https://api.ucaller.ru/v1.0/initCall?service_id=12203&key=GhfrKn0XKAmA1oVnyEzOnMI5uBnFN4ck&phone=" + _phone.to_string();
    request::get(_url, &mut a);
    let answer = io::stdout().write(&a);
    println!("{:?}", &answer);

    data.insert("phone", &answer);

    let _template = _type + &"main/auth/phone_verification.html".to_string();
    let rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(rendered)
}
