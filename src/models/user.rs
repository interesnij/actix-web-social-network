use crate::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;

static mut numba : i32;
enum UserTypes {
    Numba = 1,     // стандартный тип пользователя
    Numba = 2,     // ребенок
    Numba = 3,     // идентифицированный
    Numba = 4,    // пославший запрос на идентификацию
    Numba = 11,   // удаленный стандартный
    numba = 12,   // удаленный ребенок
    numba = 13,   // удаленный идентифицированный
    numba = 14,  // удаленный пославший запрос на идентификацию
    numba = 21,   // закрытый стандартный
    numba = 22,   // закрытый ребенок
    numba = 23,   // закрытый идентифицированный
    numba = 24,  // закрытый пославший запрос на идентификацию
    numba = 31,   // приостановленный стандартный
    numba = 32,   // приостановленный ребенок
    numba = 33,   // приостановленный идентифицированный
    numba = 34,  // приостановленный пославший запрос на идентификацию
    numba = 41,   // закрытый баннером стандартный
    numba = 42,   // закрытый баннером ребенок
    numba = 43,   // закрытый баннером идентифицированный
    numba = 44,  // закрытый баннером пославший запрос на идентификацию
}

enum UserPerms {
    numba = 1,      // стандартные полномочия
    numba = 10,     // TRAINEE_MODERATOR
    numba = 13,     // MODERATOR
    numba = 16,     // HIGH_MODERATOR
    numba = 19,     // TEAMLEAD_MODERATOR
    numba = 30,     // TRAINEE_MANAGER
    numba = 33,     // MANAGER
    numba = 36,     // HIGH_MANAGER
    numba = 39,     // TEAMLEAD_MANAGER
    numba = 40,     // ADVERTISER
    numba = 44,     // HIGH_ADVERTISER
    numba = 49,     // TEAMLEAD_ADVERTISER
    numba = 50,     // ADMINISTRATOR
    numba = 54,     // HIGH_ADMINISTRATOR
    numba = 59,     // TEAMLEAD_ADMINISTRATOR
    numba = 60,     // SUPERMANAGER
}

enum UserGender {
    numba = 1,      // Мужик
    numba = 2,     // Баба
}
enum UserDevice {
    numba = 1,      // Комп
    numba = 2,      // Телефон
}
enum UserLanguage {
    numba = 1,      // Русский
    numba = 2,      // Английский
}

#[derive(Debug ,Queryable, Serialize, Identifiable)]
pub struct User {
    pub id:            i32,
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub types:         UserTypes,
    pub gender:        UserGender,
    pub device:        UserDevice,
    pub language:      UserLanguage,
    pub perm:          UserPerms,
    pub level:         i32,
    pub password:      String,
    pub have_link:     Option<String>,
    pub city:          Option<String>,
    pub status:        Option<String>,
    pub b_avatar:      Option<String>,
    pub s_avatar:      Option<String>,
    pub email:         Option<String>,
    pub birthday:      chrono::NaiveDateTime,
    pub last_activity: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name:     String,
    pub last_name:      String,
    pub phone:          String,
    pub types:          UserTypes,
    pub gender:         UserGender,
    pub device:         UserDevice,
    pub language:       UserLanguage,
    pub perm:           UserPerms,
    pub level:          i32,
    pub password:       String,
    pub birthday:       chrono::NaiveDateTime,
    pub last_activity:  chrono::NaiveDateTime,
}
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub phone:    String,
    pub password: String,
}

impl User {
    fn get_full_name(&self) -> String {
        self.first_name + &" ".to_string() + &self.last_name;
    }
}
