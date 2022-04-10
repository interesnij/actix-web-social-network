use crate::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use diesel_derive_enum::DbEnum;

#[derive(DbEnum, DB)]
pub enum UserTypesEnum {
    Standart,                   // стандартный тип пользователя
    Child,                     // ребенок
    Identified,                // идентифицированный
    IdentifiedSend,            // пославший запрос на идентификацию
    DeletedStandart,           // удаленный стандартный
    DeletedChild,              // удаленный ребенок
    DeletedIdentified,         // удаленный идентифицированный
    DeletedIdentifiedSend,     // удаленный пославший запрос на идентификацию
    ClosedStandart,            // закрытый стандартный
    ClosedChild,               // закрытый ребенок
    ClosedIdentified,          // закрытый идентифицированный
    ClosedIdentifiedSend,      // закрытый пославший запрос на идентификацию
    SuspendedStandart,         // приостановленный стандартный
    SuspendedChild,            // приостановленный ребенок
    SuspendedIdentified,       // приостановленный идентифицированный
    SuspendedIdentifiedSend,   // приостановленный пославший запрос на идентификацию
    BannerStandart,            // закрытый баннером стандартный
    BannerChild,               // закрытый баннером ребенок
    BannerIdentified,          // закрытый баннером идентифицированный
    BannerIdentifiedSend,      // закрытый баннером пославший запрос на идентификацию
}

#[derive(DbEnum, DB)]
pub enum UserPermEnum {
    Standart,              // стандартные полномочия
    TraineeModerator,      // TRAINEE_MODERATOR
    Moderator,             // MODERATOR
    HighModerator,         // HIGH_MODERATOR
    TeamleadModerator,     // TEAMLEAD_MODERATOR
    TraineeManager,        // TRAINEE_MANAGER
    Manager,               // MANAGER
    HighManager,           // HIGH_MANAGER
    TeamleadManager,       // TEAMLEAD_MANAGER
    Advertiser,            // ADVERTISER
    HighAdvertiser,        // HIGH_ADVERTISER
    TeamleadAdvertiser,    // TEAMLEAD_ADVERTISER
    Administrator,         // ADMINISTRATOR
    HighAdministrator,     // HIGH_ADMINISTRATOR
    TeamleadAdministrator, // TEAMLEAD_ADMINISTRATOR
    Supermanager,          // SUPERMANAGER
}

#[derive(DbEnum, DB)]
pub enum UserGenderEnum {
    Man,     // Мужик
    Fem,     // Баба
}

#[derive(DbEnum, DB)]
pub enum UserDeviceEnum {
    De,      // Комп
    Ph,      // Телефон
}

#[derive(DbEnum, DB)]
pub enum UserLanguageEnum {
    Ru,      // Русский
    En,      // Английский
}

#[derive(Queryable, Identifiable)]
#[table_name="users"]
pub struct User {
    pub id:            i32,
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub types:         UserTypesEnum,
    pub gender:        UserGenderEnum,
    pub device:        UserDeviceEnum,
    pub language:      UserLanguageEnum,
    pub perm:          UserPermEnum,
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

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub types:         UserTypesEnum,
    pub gender:        UserGenderEnum,
    pub device:        UserDeviceEnum,
    pub language:      UserLanguageEnum,
    pub perm:          UserPermEnum,
    pub level:         i32,
    pub password:      String,
    pub birthday:      chrono::NaiveDateTime,
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
