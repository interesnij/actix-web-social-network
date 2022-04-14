use actix_web::HttpRequest;
use actix_session::Session;
use crate::utils::{is_signed_in, get_current_user, establish_connection};
use tera::Context;


pub fn get_default_template(req: HttpRequest)
     -> (
         String,
         bool
        )
    {

    let mut _type = "".to_string();
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

    let mut _is_host_admin : bool = false;
    let _val = format!("{:?}", Some(req.peer_addr()));
    if _val.contains(&"91.239.184.81".to_string()) {
        _is_host_admin = true;
    };
    (_type, _is_host_admin)
}

pub fn get_default_template_2(req: HttpRequest, session: Session) -> (String, tera::Context) {
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
    let mut data = Context::new();

    let _val = format!("{:?}", Some(req.peer_addr()));
    if _val.contains(&"91.239.184.81".to_string()) {
        data.insert("is_host_admin", &true);
    };

    if is_signed_in(&session) {
        let _request_user = get_current_user(&session);
        println!("{:?}", Ok(_request_user));
        match _request_user {
            Ok(s) => data.insert("request_user", &users
                .filter(schema::users::id.eq(s.id))
                .load::<User>(&_connection)
                .expect("E")[0]),
            _ => data.insert("request_user", &false),
        }
        data.insert("full_name", &false);
    } else {
        data.insert("is_authenticated", &false);
    }
    (_type, data)
}
