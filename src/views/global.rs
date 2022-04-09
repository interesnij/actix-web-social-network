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
use crate::schema;


pub fn global_routes(config: &mut web::ServiceConfig) {
    config.route("/phone_window/", web::get().to(phone_window));
    config.route("/phone_send/{phone}/", web::get().to(phone_send));
    config.route("/phone_verify/{phone}/{code}/", web::get().to(phone_verify));
    //config.route("/signup/", web::post().to(signup));
}

pub async fn process_signup() -> impl Responder {
    let connection = establish_connection();
    //diesel::insert_into(users::table)
    //    .values(&*data)
    //    .get_result::<User>(&connection)
    //    .expect("Error registering user.");

    //println!("{:?}", data);
    HttpResponse::Ok().body(format!("ok"))
}

pub async fn phone_window(req: HttpRequest) -> impl Responder {
    let (_type, _is_host_admin) = get_default_template(req);
    let mut data = Context::new();
    let _template = _type + &"main/auth/phone_verification.html".to_string();
    let rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[derive(Deserialize, Debug)]
struct PhoneJson {
    status: bool,
    ucaller_id: i32,
    phone: i64,
    phone_id: String
}
pub async fn phone_send(req: HttpRequest, _phone: web::Path<String>) -> impl Responder {

    let connection = establish_connection();
    let (_type, _is_host_admin) = get_default_template(req);
    let mut data = Context::new();
    let _url = "https://api.ucaller.ru/v1.0/initCall?service_id=12203&key=GhfrKn0XKAmA1oVnyEzOnMI5uBnFN4ck&phone=".to_owned() + &_phone.to_string();
    let __request = reqwest::get(_url)
        .await
        .expect("E.");
    let new_request = __request.text().await.unwrap();
    let phone200: PhoneJson = serde_json::from_str(&new_request).unwrap();
    println!("phone - {:?}", &phone200.phone);

    let _template = _type + &"main/auth/phone_verification.html".to_string();
    let rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn phone_verify(param: web::Path<(String,i32)>) -> impl Responder {
    use crate::schema::phone_codes::dsl::phone_codes;
    use crate::models::PhoneCode;

    let connection = establish_connection();
    let _phone : String = param.0;
    let _code : i32 = param.1;
    let mut response_text : String;

    let _phone_codes = phone_codes
        .filter(schema::phone_codes::phone.eq(&_phone), schema::phone_codes::code.eq(&_code))
        .load::<PhoneCode>(&_connection)
        .expect("E");
    if _phone_codes.len() > 1 {
        diesel::delete(_phone_codes).execute(&_connection).expect("E");
        response_text = "ok".to_string();
    } else {
        response_text = "Код подтверждения неверный. Проверьте, пожалуйста, номер, с которого мы Вам звонили. Последние 4 цифры этого номера и есть код подтверждения, который нужно ввести с поле 'Последние 4 цифры'. Если не можете найти номер, нажмите на кнопку 'Перезвонить повторно.'".to_string();
    }

    HttpResponse::Ok().body(response_text)
}
