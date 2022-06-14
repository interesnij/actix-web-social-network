use actix_web::{HttpRequest, web};
use actix_session::Session;
use crate::utils::establish_connection;
use crate::schema;
use diesel::prelude::*;
use crate::errors::AuthError;
use crate::models::User;


pub fn is_desctop(req: HttpRequest) -> bool {
    let mut _type = true;
    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let _val = format!("{:?}", header.1);
            if _val.contains("Mobile"){
                _type = false;
            }
        }
    };
    _type
}
pub fn get_device_and_ajax(req: &HttpRequest) -> (bool, bool) {
    struct Params {
        pub is_ajax: Option<bool>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let is_ajax: bool;
    let _type: bool;

    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.is_ajax.is_some() {
            is_ajax = true;
        }
        else {
            is_ajax = false;
        }
    }
    else {
        is_ajax = false;
    }

    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let _val = format!("{:?}", header.1);
            if _val.contains("Mobile"){
                _type = false;
            }
            else {
                _type = true;
            }
        }
    };
    (_type, is_ajax)
}

pub fn get_device_and_page_and_ajax(req: &HttpRequest) -> (bool, i32, bool) {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub page:    Option<i32>,
        pub is_ajax: Option<bool>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let page: i32;
    let is_ajax: bool;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.page.is_some() {
            page = params.page.unwrap();
        }
        else {
            page = 1;
        }
        if params.is_ajax.is_some() {
            is_ajax = true;
        }
        else {
            is_ajax = false;
        }
    }
    else {
        page = 1;
        is_ajax = false;
    }

    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.is_ajax.is_some() {
            is_ajax = true;
        }
        else {
            is_ajax = false;
        }
    }
    else {
        is_ajax = false;
    }

    let _type: bool;
    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let _val = format!("{:?}", header.1);
            if _val.contains("Mobile"){
                _type = false;
            }
            else {
                _type = true;
            }
        }
    };
    (_type, page, is_ajax)
}

pub fn get_list_variables(req: HttpRequest) -> (bool, i32) {
    use crate::utils::PaginationParams;

    let params_some = web::Query::<PaginationParams>::from_query(&req.query_string());
    let page: i32;

    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.page.is_some() {
            page = params.page.unwrap();
        }
        else {
            page = 1;
        }
    }
    else {
        page = 1;
    }

    let mut is_desctop = true;
    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let _val = format!("{:?}", header.1);
            if _val.contains("Mobile"){
                is_desctop = false;
            }
        }
    };
    (is_desctop, page)
}

pub fn get_request_user_data(session: &Session) -> User {
    use crate::models::SessionUser;
    use crate::schema::users::dsl::users;

    let _connection = establish_connection();
    let mut user_id = 0;
    if let Some(user_str) = session.get::<String>("user")
        .map_err(|_| AuthError::AuthenticationError(String::from("Не удалось извлечь пользователя из сеанса")))
        .unwrap() {
            let user: SessionUser = serde_json::from_str(&user_str).expect("E.");
            user_id = user.id;
        }
    if user_id != 0 {
        users
            .filter(schema::users::id.eq(user_id))
            .load::<User>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap()
    } else {
        users
            .filter(schema::users::id.eq(1))
            .load::<User>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap()
    }
}
