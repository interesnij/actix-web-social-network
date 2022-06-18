use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::utils::{
    establish_connection,
    is_signed_in,
    verify,
    get_ajax,
};
use diesel::prelude::*;
use crate::schema;
use crate::models::{User, NewUser, SessionUser};
use actix_session::Session;
use crate::errors::AuthError;
use actix_multipart::{Field, Multipart};
use std::borrow::BorrowMut;
use futures_util::stream::StreamExt as _;
use sailfish::TemplateOnce;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.route("/phone_send/{phone}/", web::get().to(phone_send));
    config.route("/phone_verify/{phone}/{code}/", web::get().to(phone_verify));
    config.route("/signup/", web::get().to(process_signup));
    config.route("/mob_register/", web::get().to(mobile_signup));
    config.route("/login/", web::post().to(login));
    config.route("/logout/", web::get().to(logout));
}

#[derive(TemplateOnce)]
#[template(path = "mobile/main/auth/signup.stpl")]
struct NobileSignupTemplate {
    is_ajax: bool,
}

pub async fn mobile_signup(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let is_ajax = get_ajax(&req);
        let body = NobileSignupTemplate {
            is_ajax: is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn logout(session: Session) -> HttpResponse {
    session.clear();
    HttpResponse::Ok().body("ok")
}

fn find_user(data: LoginUser2) -> Result<SessionUser, AuthError> {
    use crate::schema::users::dsl::users;

    let _connection = establish_connection();
    let mut items = users
        .filter(schema::users::phone.eq(&data.phone))
        .load::<User>(&_connection)
        .expect("Error.");

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.password, &data.password) {
            if matching {
                let __user = SessionUser {
                    id: user.id,
                    phone: user.phone,
                };
                return Ok(__user.into());
            }
        }
    }
    Err(AuthError::NotFound(String::from("User not found")))
}

fn handle_sign_in(data: LoginUser2,
                session: &Session,
                req: &HttpRequest) -> Result<HttpResponse, AuthError> {
    use crate::utils::{is_json_request, set_current_user};

    let _connection = establish_connection();
    let result = find_user(data);
    let is_json = is_json_request(req);

    match result {
        Ok(user) => {
            set_current_user(&session, &user);
            if is_json {
                Ok(HttpResponse::Ok().json(user))
            } else {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
            }
        },
        Err(err) => {
            if is_json {
                Ok(HttpResponse::Unauthorized().json(err.to_string()))
            } else {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
            }
        },
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser2 {
    pub phone:    String,
    pub password: String,
}
pub async fn login_form(payload: &mut Multipart) -> LoginUser2 {
    let mut form: LoginUser2 = LoginUser2 {
        phone: "".to_string(),
        password: "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = std::str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "phone" {
                    form.phone = data_string
                } else if field.name() == "password" {
                    form.password = data_string
                }
            }
        }
    }
    form
}

pub async fn login(mut payload: Multipart, session: Session, req: HttpRequest) -> impl Responder {
    if is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let form = login_form(payload.borrow_mut()).await;
        println!("{:?}", form.phone.clone());
        println!("{:?}", form.password.clone());
        handle_sign_in(form, &session, &req)
    }
}

#[derive(Debug, Deserialize)]
pub struct UserLoc {
    pub city:      CityLoc,
    pub region:    RegionLoc,
    pub country:   CountryLoc,
}
#[derive(Debug, Deserialize)]
pub struct CityLoc {
    pub name_ru:    String,
    pub name_en:    String,
}
#[derive(Debug, Deserialize)]
pub struct RegionLoc {
    pub name_ru:    String,
    pub name_en:    String,
}
#[derive(Debug, Deserialize)]
pub struct CountryLoc {
    pub name_ru:    String,
    pub name_en:    String,
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

pub async fn process_signup(session: Session, req: HttpRequest) -> impl Responder {
    use crate::utils::{hash_password, set_current_user};
    use chrono::NaiveDate;
    use crate::models::{
        UserLocation, NewUserLocation,
        UserProfile, NewUserProfile,
        IpUser, NewIpUser,
        DesignSetting, NewDesignSetting,
        UserPrivate, NewUserPrivate,

        UserPhotoNotification, NewUserPhotoNotification,
        UserGoodNotification, NewUserGoodNotification,
        UserVideoNotification, NewUserVideoNotification,
        UserMusicNotification, NewUserMusicNotification,
        UserPostNotification, NewUserPostNotification,
        UserSurveyNotification, NewUserSurveyNotification,
        UserNotification, NewUserNotification,
    };

    let params = web::Query::<NewUserForm>::from_query(&req.query_string());
     // Если пользователь не аноним, то отправляем его на страницу новостей
    if is_signed_in(&session) {
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body("")
    }
    else if params.is_err() {
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body("")
    }
    else {

    let _connection = establish_connection();
        let params_2 = params.unwrap();
        let mut get_perm = 1;
        let mut ipaddr: String = String::new();

        if let Some(val) = &req.peer_addr() {
            ipaddr = val.ip().to_string();
            if ipaddr.contains(&"91.239.184.81".to_string()) {
                get_perm = 60;
            };
            //println!("{:?}", location200.city.name_ru);
        };

        let mut get_device = "a";
        for header in req.headers().into_iter() {
            if header.0 == "user-agent" {
                let _val = format!("{:?}", header.1);
                if _val.contains("Mobile"){
                    get_device = "b";
                };
            }
        };

        let get_language = "a";
        let mut get_gender = "a";
        if params_2.gender.clone() == "Fem".to_string() {
            get_gender = "b";
        }
        let count = User::count_users() + 1;
        let link = "/id".to_string() + &count.to_string() + &"/".to_string();
        let form_user = NewUser {
            first_name:    params_2.first_name.clone(),
            last_name:     params_2.last_name.clone(),
            phone:         params_2.phone.clone(),
            types:         1,
            gender:        get_gender.to_string(),
            device:        get_device.to_string(),
            language:      get_language.to_string(),
            perm:          get_perm,
            level:         100,
            password:      hash_password(&params_2.password.clone()),
            link:          link,
            birthday:      NaiveDate::parse_from_str(&params_2.birthday.clone(), "%Y-%m-%d").unwrap(),
            last_activity: chrono::Local::now().naive_utc(),
        };

        let _new_user = diesel::insert_into(schema::users::table)
            .values(&form_user)
            .get_result::<User>(&_connection)
            .expect("Error saving user.");

        let _session_user = SessionUser {
            id: _new_user.id,
            phone: _new_user.phone,
        };

        // записываем местоположение нового пользователя
        let _geo_url = "http://api.sypexgeo.net/J5O6d/json/".to_owned() + &ipaddr;
        let _geo_request = reqwest::get(_geo_url).await.expect("E.");
        let new_request = _geo_request.text().await.unwrap();
        let location200: UserLoc = serde_json::from_str(&new_request).unwrap();
        let _user_location = NewUserLocation {
            user_id: _new_user.id,
            city_ru: Some(location200.city.name_ru),
            city_en: Some(location200.city.name_en),
            region_ru: Some(location200.region.name_ru),
            region_en: Some(location200.region.name_en),
            country_ru: Some(location200.country.name_ru),
            country_en: Some(location200.country.name_en),
        };
        diesel::insert_into(schema::user_locations::table)
            .values(&_user_location)
            .get_result::<UserLocation>(&_connection)
            .expect("Error saving user_location.");

        // записываем IP нового пользователя
        let _user_ip = NewIpUser {
            user_id: _new_user.id,
            ip: ipaddr,
        };
        diesel::insert_into(schema::ip_users::table)
            .values(&_user_ip)
            .get_result::<IpUser>(&_connection)
            .expect("Error saving user_ip.");

        // записываем профиль нового пользователя
        let _user_profile = NewUserProfile {
            user_id: _new_user.id,
            posts: 0,
            views_post: 0,
            friends: 0,
            follows: 0,
            communities: 0,
            photos: 0,
            goods: 0,
            docs: 0,
            tracks: 0,
            videos: 0,
            articles: 0,
            planners: 0,
            avatar_id: None,
            activity: None,
            interests: None,
            favorite_music: None,
            favorite_films: None,
            favorite_books: None,
            favorite_game: None,
            about: None,
            survey: 0,
            playlist: 0,
        };
        diesel::insert_into(schema::user_profiles::table)
            .values(&_user_profile)
            .get_result::<UserProfile>(&_connection)
            .expect("Error saving user_profile.");

        // записываем приватность нового пользователя
        let _user_private = NewUserPrivate {
            user_id:            _new_user.id,
            can_see_all:        "a".to_string(),
            can_see_community:  "a".to_string(),
            can_see_info:       "a".to_string(),
            can_see_friend:     "a".to_string(),
            can_send_message:   "a".to_string(),
            can_add_in_chat:    "a".to_string(),
            can_see_post:       "a".to_string(),
            can_see_photo:      "a".to_string(),
            can_see_good:       "a".to_string(),
            can_see_video:      "a".to_string(),
            can_see_music:      "a".to_string(),
            can_see_planner:    "a".to_string(),
            can_see_doc:        "a".to_string(),
            can_see_survey:     "a".to_string(),
        };
        diesel::insert_into(schema::user_privates::table)
            .values(&_user_private)
            .get_result::<UserPrivate>(&_connection)
            .expect("Error saving user_private.");

        // записываем уведомления профиля нового пользователя
        let _user_notification = NewUserNotification {
            user_id:              _new_user.id,
            connection_request:   true,
            connection_confirmed: true,
            user_invite:          true,
        };
        diesel::insert_into(schema::user_notifications::table)
            .values(&_user_notification)
            .get_result::<UserNotification>(&_connection)
            .expect("Error saving user_notification.");

        // записываем уведомления записей нового пользователя
        let _user_post_notification = NewUserPostNotification {
            user_id:                _new_user.id,
            comment:                true,
            comment_reply:          true,
            mention:                true,
            comment_mention:        true,
            repost:                 true,
            reactions:              true,
        };
        diesel::insert_into(schema::user_post_notifications::table)
            .values(&_user_post_notification)
            .get_result::<UserPostNotification>(&_connection)
            .expect("Error saving user_photo_notification.");

        // записываем уведомления фотографий нового пользователя
        let _user_photo_notification = NewUserPhotoNotification {
            user_id:                _new_user.id,
            comment:                true,
            comment_reply:          true,
            mention:                true,
            comment_mention:        true,
            repost:                 true,
            reactions:              true,
        };
        diesel::insert_into(schema::user_photo_notifications::table)
            .values(&_user_photo_notification)
            .get_result::<UserPhotoNotification>(&_connection)
            .expect("Error saving user_photo_notification.");

        // записываем уведомления товаров нового пользователя
        let _user_good_notification = NewUserGoodNotification {
            user_id:                _new_user.id,
            comment:                true,
            comment_reply:          true,
            mention:                true,
            comment_mention:        true,
            repost:                 true,
            reactions:              true,
        };
        diesel::insert_into(schema::user_good_notifications::table)
            .values(&_user_good_notification)
            .get_result::<UserGoodNotification>(&_connection)
            .expect("Error saving user_good_notification.");

        // записываем уведомления роликов нового пользователя
        let _user_video_notification = NewUserVideoNotification {
            user_id:                _new_user.id,
            comment:                true,
            comment_reply:          true,
            mention:                true,
            comment_mention:        true,
            repost:                 true,
            reactions:              true,
        };
        diesel::insert_into(schema::user_video_notifications::table)
            .values(&_user_video_notification)
            .get_result::<UserVideoNotification>(&_connection)
            .expect("Error saving user_video_notification.");

        // записываем уведомления роликов нового пользователя
        let _user_music_notification = NewUserMusicNotification {
            user_id:                _new_user.id,
            repost:                 true,
        };
        diesel::insert_into(schema::user_music_notifications::table)
            .values(&_user_music_notification)
            .get_result::<UserMusicNotification>(&_connection)
            .expect("Error saving user_music_notification.");

        // записываем уведомления роликов нового пользователя
        let _user_survey_notification = NewUserSurveyNotification {
            user_id:  _new_user.id,
            vote:     true,
            repost:   true,
        };
        diesel::insert_into(schema::user_survey_notifications::table)
            .values(&_user_survey_notification)
            .get_result::<UserSurveyNotification>(&_connection)
            .expect("Error saving user_survey_notification.");

        // записываем уведомления роликов нового пользователя
        let _design_settings = NewDesignSetting {
            user_id:    _new_user.id,
            background: "a".to_string(),
        };
        diesel::insert_into(schema::design_settings::table)
            .values(&_design_settings)
            .get_result::<DesignSetting>(&_connection)
            .expect("Error saving design_settings.");

        set_current_user(&session, &_session_user);
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok")
    }
}

pub async fn phone_send(_phone: web::Path<String>) -> impl Responder {
    use crate::utils::PhoneJson;
    let req_phone = _phone.to_string();
    if req_phone.len() > 8 {
        use crate::models::{PhoneCode, NewPhoneCode};
        use crate::schema::users::dsl::users;

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
            let code_i32: i32 = phone200.code.parse().unwrap();
            let new_phone_code = NewPhoneCode {
                phone: _phone.to_string(),
                code:  code_i32,
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
    let response_text : String;

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
