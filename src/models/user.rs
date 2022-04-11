use crate::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;

///// Типы пользоватетеля
    // 1 стандартный тип пользователя
    // 3 ребенок
    // 7 идентифицированный
    // 6 пославший запрос на идентификацию
    // 11 удаленный стандартный
    // 13 удаленный ребенок
    // 17 удаленный идентифицированный
    // 16 удаленный пославший запрос на идентификацию
    // 21 закрытый стандартный
    // 23 закрытый ребенок
    // 27 закрытый идентифицированный
    // 26 закрытый пославший запрос на идентификацию
    // 31 приостановленный стандартный
    // 33 приостановленный ребенок
    // 37 приостановленный идентифицированный
    // 36 приостановленный пославший запрос на идентификацию
    // 41 закрытый баннером стандартный
    // 43 закрытый баннером ребенок
    // 47 закрытый баннером идентифицированный
    // 46 закрытый баннером пославший запрос на идентификацию

///// Полномочия пользоватетеля
    // 1 стандартные полномочия
    // 10 TRAINEE_MODERATOR
    // 13 MODERATOR
    // 16 HIGH_MODERATOR
    // 19 TEAMLEAD_MODERATOR
    // 20 TRAINEE_MANAGER
    // 23 MANAGER
    // 26 HIGH_MANAGER
    // 29 TEAMLEAD_MANAGER
    // 30 ADVERTISER
    // 34 HIGH_ADVERTISER
    // 39 TEAMLEAD_ADVERTISER
    // 40 ADMINISTRATOR
    // 44 HIGH_ADMINISTRATOR
    // 49 TEAMLEAD_ADMINISTRATOR
    // 60 SUPERMANAGER

///// Пол пользоватетеля
    // 'a' Мужик
    // 'b' Баба

///// Оборудование пользоватетеля
    // 'a' Комп
    // 'b' Телефон

///// Язык пользоватетеля
    // 'a' Русский
    // 'b' Английский

#[derive(Queryable, Identifiable)]
#[table_name="users"]
pub struct User {
    pub id:            i32,
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub types:         i16,
    pub gender:        char,
    pub device:        char,
    pub language:      char,
    pub perm:          i16,
    pub level:         i16,
    pub password:      String,
    pub have_link:     Option<String>,
    pub city:          Option<String>,
    pub status:        Option<String>,
    pub b_avatar:      Option<String>,
    pub s_avatar:      Option<String>,
    pub email:         Option<String>,
    pub birthday:      chrono::NaiveDate,
    pub last_activity: chrono::NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub types:         i16,
    pub gender:        char,
    pub device:        char,
    pub language:      char,
    pub perm:          i16,
    pub level:         i16,
    pub password:      String,
    pub birthday:      chrono::NaiveDate,
    pub last_activity: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub phone:    String,
    pub password: String,
}

impl User {
    fn get_full_name(&self) -> String {
        self.first_name.clone() + &" ".to_string() + &self.last_name.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: i32,
    pub phone: String,
}
