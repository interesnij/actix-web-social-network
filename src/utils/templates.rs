use actix_web::{HttpRequest, web};
use actix_session::Session;
use crate::utils::establish_connection;
use crate::schema;
use diesel::prelude::*;
use crate::errors::AuthError;
use crate::models::User;


pub fn get_folder(req: HttpRequest) -> String {
    let mut _type = "".to_string();
    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let _val = format!("{:?}", header.1);
            if _val.contains("Mobile"){
                _type = "mobile/".to_string();
            } else {
                _type = "desctop/".to_string();
            };
        }
    };
    _type
}
pub fn get_list_variables(req: HttpRequest) -> (bool, i32) {
    use crate::utils::PaginationParams;

    let params_some = web::Query::<PaginationParams>::from_query(&req.query_string());
    let mut page: i32 = 0;

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

    let is_desctop: bool;
    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let _val = format!("{:?}", header.1);
            if _val.contains("Mobile"){
                is_desctop = true;
            } else {
                is_desctop = false;
            };
        }
    };
    (is_desctop, page)
}

pub fn get_request_user_data(session: Session) -> User {
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
