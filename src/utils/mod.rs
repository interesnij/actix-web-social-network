mod templates;
pub use self::{
    templates::*
};
use argonautica::{Hasher, Verifier};
use actix_session::Session;
use diesel::prelude::*;
use actix_web::{
  http::header::{CONTENT_TYPE, LOCATION},
  HttpRequest,
  HttpResponse,
};
use crate::schema;
use crate::{errors::AuthError, vars, models::SessionUser};
use crate::models::User;

pub fn establish_connection() -> PgConnection {
    use dotenv::dotenv;

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// Auth

pub fn hash_password(password: &str) -> String {
  Hasher::default()
      .with_password(password)
      .with_secret_key(vars::secret_key().as_str())
      .hash()
      .expect("E.")
      //.map_err(|_| AuthError::AuthenticationError(String::from("Не удалось хэшировать пароль")))
}

pub fn to_home() -> HttpResponse {
  HttpResponse::Found().header(LOCATION, "/").finish()
}

pub fn verify(hash: &str, password: &str) -> Result<bool, AuthError> {
  Verifier::default()
      .with_hash(hash)
      .with_password(password)
      .with_secret_key(vars::secret_key().as_str())
      .verify()
      .map_err(|_| AuthError::AuthenticationError(String::from("Не удалось подтвердить пароль")))
}

pub fn is_json_request(req: &HttpRequest) -> bool {
    req
      .headers()
      .get(CONTENT_TYPE)
      .map_or(
        false,
        |header| header.to_str().map_or(false, |content_type| "application/json" == content_type)
      )
}

pub fn is_signed_in(session: &Session) -> bool {
  match get_current_user(session) {
      Ok(_) => true,
      _ => false,
  }
}

pub fn set_current_user(session: &Session, user: &SessionUser) -> () {
    // сериализация в строку подходит для этого случая,
    // но двоичный код был бы предпочтительнее в производственных вариантах использования.
    session.insert("user", serde_json::to_string(user).unwrap()).unwrap();
}

pub fn get_current_user(session: &Session) -> Result<SessionUser, AuthError> {
    let msg = "Не удалось извлечь пользователя из сеанса";

    session.get::<String>("user")
        .map_err(|_| AuthError::AuthenticationError(String::from(msg)))
        .unwrap()
        .map_or(
          Err(AuthError::AuthenticationError(String::from(msg))),
          |user| serde_json::from_str(&user).or_else(|_| Err(AuthError::AuthenticationError(String::from(msg))))
        )
}
//pub fn get_request_user(session: &Session) -> i8  {
//    session.get::<String>("user")
//        .map_err(|_| AuthError::AuthenticationError(String::from(msg)))
//        .unwrap()
//        .map_or(
//          Err(AuthError::AuthenticationError(String::from(msg))),
//          |user| serde_json::from_str(&user).or_else(|_| Err(AuthError::AuthenticationError(String::from(msg))))
//        )
//}

pub fn get_count_for_ru(count:i32, word1: String, word2: String, word3: String) -> String {
    let a = count % 10;
    let b = count % 100;
    let count_str: String = count.to_string().parse().unwrap();
    if a == 1 && b != 11 {
        return count_str + &word1;
    }
    else if a >= 2 && a <= 4 && (b < 10 || b >= 20) {
        return count_str + &word2;
    }
    else {
        return count_str + &word3;
    }
}

pub fn get_users_from_ids(ids:Vec<i32>) -> Vec<User> {
    use crate::schema::users::dsl::users;

    let _connection = establish_connection();
    return users
        .filter(schema::users::id.eq_any(ids))
        .filter(schema::users::types.lt(10))
        .load::<User>(&_connection)
        .expect("E");
}

pub fn hide_text(text:Option <String>) -> Option <String> {
    if text.is_some() {
        let words = text.split(" ").collect::<Vec<_>>();
        if words.len() <= 30 {
            return text;
        }
        else {
            return text[..100].to_owned() + &"<br><a class='pointer show_post_text'>Показать полностью...</a><br><span style='display:none'>" + &text[101:] + &"</span>";
        }
    } else { return None; }
}
