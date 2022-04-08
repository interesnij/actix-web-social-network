use crate::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;


enum UserTypes {
    "STA",     // стандартный тип пользователя
    "CHI",     // ребенок
    "IDE",     // идентифицированный
    "IDES",    // пославший запрос на идентификацию
    "_DELS",   // удаленный стандартный
    "_DELC",   // удаленный ребенок
    "_DELI",   // удаленный идентифицированный
    "_DELIS",  // удаленный пославший запрос на идентификацию
    "_CLOS",   // закрытый стандартный
    "_CLOC",   // закрытый ребенок
    "_CLOI",   // закрытый идентифицированный
    "_CLOIS",  // закрытый пославший запрос на идентификацию
    "_SUSS",   // приостановленный стандартный
    "_SUSC",   // приостановленный ребенок
    "_SUSI",   // приостановленный идентифицированный
    "_SUSIS",  // приостановленный пославший запрос на идентификацию
    "_BANS",   // закрытый баннером стандартный
    "_BANC",   // закрытый баннером ребенок
    "_BANI",   // закрытый баннером идентифицированный
    "_BANIS",  // закрытый баннером пославший запрос на идентификацию
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
    'Man',      // Мужик
    'Fem',     // Баба
}
enum UserDevice {
    'De',      // Комп
    'Ph',      // Телефон
}
enum UserLanguage {
    'Ru',      // Русский
    'En',      // Английский
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
