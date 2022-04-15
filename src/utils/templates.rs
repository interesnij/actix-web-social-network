use actix_web::HttpRequest;
use actix_session::Session;
use crate::utils::{is_signed_in, get_current_user, establish_connection};
use crate::schema;
use diesel::prelude::*;
use std::option::Option;
use crate::errors::AuthError;


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

pub fn get_request_user_data(session: Session) -> (
        i32, String, String, i16, String, String, String, i16, String, String, String
    ) {
    use crate::models::SessionUser;

    let _connection = establish_connection();
    //let _request_user = get_current_user(&session);
    let mut user_id = 0;
    if let Some(user_str) = session.get::<String>("user")
        .map_err(|_| AuthError::AuthenticationError(String::from("Не удалось извлечь пользователя из сеанса")))
        .unwrap() {
            user: SessionUser = serde_json::from_str(&user_str);
            user_id = user.id;
        }
    if user_id != 0 {
        use crate::schema::{
            design_settings::dsl::design_settings,
            users::dsl::users,
        };
        use crate::models::{
            DesignSetting,
            User
        };
        let _user = users
            .filter(schema::users::id.eq(&user_id))
            .load::<User>(&_connection)
            .expect("E")[0];
        let _design = design_settings
            .filter(schema::design_settings::user_id.eq(&user_id))
            .load::<DesignSetting>(&_connection)
            .expect("E");
        let background = &_design[0].background;
        (
            user_id,
            _user.first_name,
            _user.last_name,
            _user.types,
            _user.gender,
            _user.device,
            _user.language,
            _user.perm,
            _user.have_link,
            _user.s_avatar,
            background.to_string(),
        )
    } else {
        (0, "".to_string(), "".to_string(), 0, "".to_string(), "".to_string(), "".to_string(), 0, None, None, "".to_string())
    }
}

pub fn get_default_template_2(req: HttpRequest, session: Session) -> String {
    use crate::schema::users::dsl::users;
    use crate::models::User;
    use diesel::prelude::*;
    use crate::schema;

    let mut _type = "".to_string();
    let _connection = establish_connection();
    let mut _request_user = "".to_string();
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

    if is_signed_in(&session) {
        let _request_user = get_current_user(&session);
        println!("request_user {:?}", _request_user);
        //match _request_user {
        //    Ok(s) => data.insert("request_user", &users
        //        .filter(schema::users::id.eq(s.id))
        //        .load::<User>(&_connection)
        //        .expect("E")[0]),
        //    _ => data.insert("request_user", &false),
        //}
        //data.insert("full_name", &false);
    //} else {
    //    data.insert("is_authenticated", &false);
    }
    _type
}
