use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web,
    http::header::LOCATION,
};
use serde::Deserialize;
use tera::Context;
use crate::utils::{establish_connection, get_default_template, TEMPLATES, is_signed_in};
use diesel::prelude::*;
use crate::schema;
use crate::models::{NewUser, SessionUser};
use actix_session::Session;


pub fn global_routes(config: &mut web::ServiceConfig) {
    config.route("/phone_window/", web::get().to(phone_window));
    config.route("/phone_send/{phone}/", web::get().to(phone_send));
    config.route("/phone_verify/{phone}/{code}/", web::get().to(phone_verify));
    config.route("/signup/", web::get().to(process_signup));
    config.route("/login/", web::post().to(login));
}

#[derive(Deserialize)]
pub struct NewUserForm {
    pub first_name:  String,
    pub last_name:   String,
    pub gender:      String,
    pub password:    String,
    pub birthday:    String,
    pub phone:       String,
}

fn find_user(data: web::Form<LoginUser>) -> Result<SessionUser, AuthError> {
    use crate::schema::users::dsl::users;

    let _connection = establish_connection();
    let mut items = users
        .filter(phone.eq(&data.phone))
        .load::<User>(&_connection)
        .expect("Error.");

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.password, &data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(AuthError::NotFound(String::from("User not found")))
}

fn handle_sign_in(data: AuthData,
                session: &Session,
                req: &HttpRequest) -> Result<HttpResponse, AuthError> {
    use crate::utils::{is_json_request, set_current_user};

    let _connection = establish_connection();
    let result = find_user(data, _connection);
    let is_json = is_json_request(req);

    match result {
        Ok(user) => {
            set_current_user(&session, &user);
            if is_json {
                Ok(HttpResponse::Ok().json(user))
            } else {
                Ok(to_home())
            }
        },
        Err(err) => {
            if is_json {
                Ok(HttpResponse::Unauthorized().json(err.to_string()))
            } else {
                let t = SignIn { error: Some(err.to_string()) };
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(t.call().unwrap()))
            }
        },
    }
}

pub async fn login(data: web::Form<LoginUser>, session: Session, req: HttpRequest) -> impl Responder {
    if is_signed_in(&session) {
        to_home();
    }
    handle_sign_in(data.into_inner(), &session, &req)
}
pub async fn process_signup(session: Session, req: HttpRequest) -> impl Responder {
    use crate::schema::users::dsl::users;
    use crate::utils::{hash_password, set_current_user, to_home};
    use crate::models::{User, SessionUser};
    use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

    if is_signed_in(&session) {
        to_home();
    }

    let _connection = establish_connection();
    let params = web::Query::<NewUserForm>::from_query(&req.query_string());
    println!("!!!!");
    if params.is_ok() {
        println!("params ok!");
        let params_2 = params.unwrap();
        let (_type, _is_host_admin) = get_default_template(req);
        let mut get_device = "a";
        let mut get_language = "a";
        let mut get_gender = "a";
        let mut get_perm = 1;
        if _type == "mobile/".to_string() {
            get_device = "b";
        }
        if params_2.gender.clone() == "Fem".to_string() {
            get_gender = "b";
        }
        if _is_host_admin {
            get_perm = 60;
        }

        let d = NaiveDate::from_ymd(2015, 6, 3);
        let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
        let form_user = NewUser {
            first_name: params_2.first_name.clone(),
            last_name: params_2.last_name.clone(),
            phone: params_2.phone.clone(),
            types: 1,
            gender: get_gender.to_string(),
            device: get_device.to_string(),
            language: get_language.to_string(),
            perm: get_perm,
            level: 100,
            password: hash_password(&params_2.password.clone()),
            //password: params_2.password.clone(),
            birthday: NaiveDate::parse_from_str(&params_2.birthday.clone(), "%Y-%m-%d").unwrap(),
            last_activity: NaiveDateTime::new(d, t),
        };

        let _new_user = diesel::insert_into(schema::users::table)
            .values(&form_user)
            .get_result::<User>(&_connection)
            .expect("Error saving user.");

        let _session_user = SessionUser {
            id: _new_user.id,
            phone: _new_user.phone,
        };
        set_current_user(&session, &_session_user);
        to_home();
    }
    HttpResponse::Ok().body(format!("ok"))
}

pub async fn phone_window(req: HttpRequest) -> impl Responder {
    let (_type, _is_host_admin) = get_default_template(req);
    let mut data = Context::new();
    let _template = _type + &"main/auth/phone_window.html".to_string();
    let rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[derive(Deserialize, Debug)]
struct PhoneJson {
    status: bool,
    ucaller_id: i32,
    phone: i64,
    phone_id: String,
    code: i32,
}
pub async fn phone_send(req: HttpRequest, _phone: web::Path<String>) -> impl Responder {
    let (_type, _is_host_admin) = get_default_template(req);
    let mut data = Context::new();
    let req_phone = _phone.to_string();
    if req_phone.len() > 8 {
        use crate::models::{User, PhoneCode, NewPhoneCode};
        use schema::{users::dsl::users, phone_codes::dsl::phone_codes};

        let _connection = establish_connection();
        let _some_user = users
            .filter(schema::users::phone.eq(&req_phone))
            .load::<User>(&_connection)
            .expect("E");
        if _some_user.len() > 0 {
            let rendered = "Пользователь с таким номером уже зарегистрирован. Используйте другой номер или напишите в службу поддержки, если этот номер Вы не использовали ранее.";
            HttpResponse::Ok().body(rendered)
        } else {

            let _url = "https://api.ucaller.ru/v1.0/initCall?service_id=12203&key=GhfrKn0XKAmA1oVnyEzOnMI5uBnFN4ck&phone=".to_owned() + &req_phone;
            let __request = reqwest::get(_url).await.expect("E.");
            let new_request = __request.text().await.unwrap();
            println!("{:?}", new_request);
            let phone200: PhoneJson = serde_json::from_str(&new_request).unwrap();

            let new_phone_code = NewPhoneCode {
                phone: phone200.phone.to_string(),
                code:  phone200.code,
            };
            diesel::insert_into(schema::phone_codes::table)
                .values(&new_phone_code)
                .get_result::<PhoneCode>(&_connection)
                .expect("E.");

            let rendered = "Мы Вам звоним. Последние 4 цифры нашего номера - код подтверждения, который нужно ввести в поле 'Последние 4 цифры' и нажать 'Подтвердить' <div class='row block_verify mt-5'><div class='col-md-2'></div><div class='col-md-4'><input type='number' id='code' onkeyup='code_check();' class='form-control border-0' placeholder='Последние 4 цифры'><hr class='my-0'></div><div class='mb-3 col-md-4'><button type='button' disabled='disabled' id='code_send' class='btn btn-primary pink-gradient'>Подтвердить</button></div><div class='col-md-2'></div></div>";
            HttpResponse::Ok().body(rendered)
        }
    }
    else {
        let rendered = "Введите, пожалуйста, корректное количество цифр Вашего телефона";
        HttpResponse::Ok().body(rendered)
    }
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
    if _phone_codes.len() > 0 {
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
