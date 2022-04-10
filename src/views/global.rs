use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web
};
use serde::Deserialize;
use tera::Context;
use crate::utils::{establish_connection, get_default_template, TEMPLATES};
use diesel::prelude::*;
use crate::schema;
use crate::models::NewUser;


pub fn global_routes(config: &mut web::ServiceConfig) {
    config.route("/phone_window/", web::get().to(phone_window));
    config.route("/phone_send/{phone}/", web::get().to(phone_send));
    config.route("/phone_verify/{phone}/{code}/", web::get().to(phone_verify));
    config.route("/signup/", web::post().to(process_signup));
}

pub async fn process_signup(req: HttpRequest, _data: web::Form<NewUser>) -> impl Responder {
    use crate::schema::users::dsl::users;
    use crate::models::{UserTypes, UserPerms, UserGender, UserDevice, UserLanguage};
    use crate::utils::hash_password;
    use chrono::NaiveDate;

    let _connection = establish_connection();
    let (_type, _is_host_admin) = get_default_template(req);
    let mut get_device = UserDevice::De;
    let mut get_gender = UserGender::Man;
    let mut get_language = UserLanguage::Ru;
    let mut get_perm = UserPerms::Standart;
    if _data.gender == "Fem".to_string() {
        get_gender = UserGender::Fem;
    }
    if _type == "mobile/".to_string() {
        get_device = UserDevice::Ph;
    }
    if _is_host_admin {
        get_perm = UserPerms::Supermanager;
    }
    let date_str = _data.date_year.clone() + "-" + &_data.date_month.clone() + "-" + &_data.date_day.clone();
    diesel::insert_into(users::table)
        .values((
            first_name.eq(_data.first_name.clone()),
            last_name.eq(_data.last_name.clone()),
            phone.eq(_data.phone.clone()),
            gender.eq(get_gender),
            device.eq(get_device),
            language.eq(get_language),
            perm.eq(get_perm),
            password.eq(hash_password(_data.password.clone())),
            birthday.eq(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap()),
            last_activity.eq(chrono::offset::Local::now()),
        ))
        .execute(&_connection)
        .expect("Insertion failed");
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
    let (_type, _is_host_admin) = get_default_template(req);
    let mut data = Context::new();
    let _url = "https://api.ucaller.ru/v1.0/initCall?service_id=12203&key=GhfrKn0XKAmA1oVnyEzOnMI5uBnFN4ck&phone=".to_owned() + &_phone.to_string();
    let __request = reqwest::get(_url).await.expect("E.");
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

    let _connection = establish_connection();
    let _phone = param.0.to_string();
    let _code = param.1;
    let mut response_text : String;

    let _phone_codes = phone_codes
        .filter(schema::phone_codes::phone.eq(&_phone))
        .filter(schema::phone_codes::code.eq(&_code))
        .load::<PhoneCode>(&_connection)
        .expect("E");
    if _phone_codes.len() > 1 {
        diesel::delete(phone_codes
                .filter(schema::phone_codes::phone.eq(&_phone))
                .filter(schema::phone_codes::code.eq(&_code))
            ).execute(&_connection)
            .expect("E");
        response_text = "ok".to_string();
    } else {
        response_text = "Код подтверждения неверный. Проверьте, пожалуйста, номер, с которого мы Вам звонили. Последние 4 цифры этого номера и есть код подтверждения, который нужно ввести с поле 'Последние 4 цифры'. Если не можете найти номер, нажмите на кнопку 'Перезвонить повторно.'".to_string();
    }

    HttpResponse::Ok().body(response_text)
}
