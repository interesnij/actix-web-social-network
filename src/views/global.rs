use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web
};
use serde::Deserialize;
use tera::Context;
use crate::utils::{establish_connection, get_default_template, TEMPLATES};
use crate::diesel::RunQueryDsl;
use crate::models::NewUser;


pub fn global_routes(config: &mut web::ServiceConfig) {
    config.route("/phone_send/{phone}/", web::post().to(phone_send));
    config.route("/phone_verify/{phone}/{code}/", web::post().to(phone_verify));
    config.route("/signup/", web::post().to(signup));
}

pub async fn process_signup(data: web::Form<NewUser>) -> impl Responder {
    use crate::schema::users;
    use crate::models::User;

    let connection = establish_connection();

    diesel::insert_into(users::table)
        .values(&*data)
        .get_result::<User>(&connection)
        .expect("Error registering user.");

    println!("{:?}", data);
    HttpResponse::Ok().body(format!("ok"))
}
