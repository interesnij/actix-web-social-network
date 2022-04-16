use actix_web::HttpRequest;
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
//&'static
pub fn get_request_user_data_2(session: Session) -> (User, String) {
    use crate::models::SessionUser;

    let _connection = establish_connection();
    //let _request_user = get_current_user(&session);
    let mut user_id = 0;
    let mut have_link = "".to_string();
    let mut s_avatar = "".to_string();
    if let Some(user_str) = session.get::<String>("user")
        .map_err(|_| AuthError::AuthenticationError(String::from("Не удалось извлечь пользователя из сеанса")))
        .unwrap() {
            let user: SessionUser = serde_json::from_str(&user_str).expect("E.");
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
            .filter(schema::users::id.eq(user_id))
            .load::<User>(&_connection)
            .expect("E")[0];
        let _design = design_settings
            .filter(schema::design_settings::user_id.eq(&user_id))
            .load::<DesignSetting>(&_connection)
            .expect("E");
        let background = &_design[0].background;
        (_user, background.to_string())
    } else {
        use crate::schema::users::dsl::users;
        let _user = users
            .filter(schema::users::id.eq(1))
            .load::<User>(&_connection)
            .expect("E")[0];
        (_user, "".to_string())
    }
}

pub fn get_request_user_data(session: Session) -> (
        i32, String, i16, String, String, i16, String, String, String
    ) {
    use crate::models::SessionUser;

    let _connection = establish_connection();
    //let _request_user = get_current_user(&session);
    let mut user_id = 0;
    let mut have_link = "".to_string();
    let mut s_avatar = "".to_string();
    if let Some(user_str) = session.get::<String>("user")
        .map_err(|_| AuthError::AuthenticationError(String::from("Не удалось извлечь пользователя из сеанса")))
        .unwrap() {
            let user: SessionUser = serde_json::from_str(&user_str).expect("E.");
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
        let _user = &users
            .filter(schema::users::id.eq(&user_id))
            .load::<User>(&_connection)
            .expect("E")[0];
        let _design = design_settings
            .filter(schema::design_settings::user_id.eq(&user_id))
            .load::<DesignSetting>(&_connection)
            .expect("E");
        let background = &_design[0].background;
        if _user.have_link.is_some() {
            have_link = _user.have_link.as_ref().expect("E.").to_string()
        }
        if _user.s_avatar.is_some() {
            s_avatar = _user.s_avatar.as_ref().expect("E.").to_string()
        }
        (
            user_id,
            _user.get_full_name(),
            _user.types.clone(),
            _user.gender.clone(),
            _user.language.clone(),
            _user.perm.clone(),
            have_link,
            s_avatar,
            background.to_string(),
        )
    } else {
        (0, "".to_string(), 0, "".to_string(), "".to_string(), 0, "".to_string(), "".to_string(), "".to_string())
    }
}
