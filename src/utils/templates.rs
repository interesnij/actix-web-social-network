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

pub fn get_request_user_data(session: Session) -> (Vec<User>, String) {
    use crate::models::SessionUser;
    use crate::schema::{ 
        design_settings::dsl::design_settings,
        users::dsl::users,
    };

    let _connection = establish_connection();
    let mut user_id = 0;
    if let Some(user_str) = session.get::<String>("user")
        .map_err(|_| AuthError::AuthenticationError(String::from("Не удалось извлечь пользователя из сеанса")))
        .unwrap() {
            let user: SessionUser = serde_json::from_str(&user_str).expect("E.");
            user_id = user.id;
        }
    if user_id != 0 {
        use crate::models::DesignSetting;
        let _design = design_settings
            .filter(schema::design_settings::user_id.eq(&user_id))
            .load::<DesignSetting>(&_connection)
            .expect("E");
        let background = &_design[0].background;
        (users
            .filter(schema::users::id.eq(user_id))
            .load::<User>(&_connection)
            .expect("E"), background.to_string())
    } else {
        (users
            .filter(schema::users::id.eq(1))
            .load::<User>(&_connection)
            .expect("E"), "".to_string())
    }
}
