use crate::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;

static mut numba : i32;
enum UserTypes {
    numba = 1,     // стандартный тип пользователя
    numba = 1,     // ребенок
    3,     // идентифицированный
    4,    // пославший запрос на идентификацию
    11,   // удаленный стандартный
    12,   // удаленный ребенок
    13,   // удаленный идентифицированный
    14,  // удаленный пославший запрос на идентификацию
    21,   // закрытый стандартный
    22,   // закрытый ребенок
    23,   // закрытый идентифицированный
    24,  // закрытый пославший запрос на идентификацию
    31,   // приостановленный стандартный
    32,   // приостановленный ребенок
    33,   // приостановленный идентифицированный
    34,  // приостановленный пославший запрос на идентификацию
    41,   // закрытый баннером стандартный
    42,   // закрытый баннером ребенок
    43,   // закрытый баннером идентифицированный
    44,  // закрытый баннером пославший запрос на идентификацию
}

enum UserPerms {
    1,      // стандартные полномочия
    10,     // TRAINEE_MODERATOR
    13,     // MODERATOR
    16,     // HIGH_MODERATOR
    19,     // TEAMLEAD_MODERATOR
    30,     // TRAINEE_MANAGER
    33,     // MANAGER
    36,     // HIGH_MANAGER
    39,     // TEAMLEAD_MANAGER
    40,     // ADVERTISER
    44,     // HIGH_ADVERTISER
    49,     // TEAMLEAD_ADVERTISER
    50,     // ADMINISTRATOR
    54,     // HIGH_ADMINISTRATOR
    59,     // TEAMLEAD_ADMINISTRATOR
    60,     // SUPERMANAGER
}

enum UserGender {
    1,      // Мужик
    2,     // Баба
}
enum UserDevice {
    1,      // Комп
    2,      // Телефон
}
enum UserLanguage {
    1,      // Русский
    2,      // Английский
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
