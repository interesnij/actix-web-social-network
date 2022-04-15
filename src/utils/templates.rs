use actix_web::HttpRequest;
use actix_session::Session;
use crate::utils::{is_signed_in, get_current_user, establish_connection};
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

pub fn get_request_user(session: Session) -> User {
    let _request_user = get_current_user(&session);
    match _request_user {
        Ok(s) => &users
            .filter(schema::users::id.eq(s.id))
            .load::<User>(&_connection)
            .expect("E")[0],
        _ => Null
    }
}

pub fn get_default_template_2(req: HttpRequest, session: Session) -> (String) {
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
