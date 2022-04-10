use crate::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;


pub enum UserTypes {
    Standart(String),                   // стандартный тип пользователя
    Child(String),                     // ребенок
    Identified(String),                // идентифицированный
    IdentifiedSend(String),            // пославший запрос на идентификацию
    DeletedStandart(String),           // удаленный стандартный
    DeletedChild(String),              // удаленный ребенок
    DeletedIdentified(String),         // удаленный идентифицированный
    DeletedIdentifiedSend(String),     // удаленный пославший запрос на идентификацию
    ClosedStandart(String),            // закрытый стандартный
    ClosedChild(String),               // закрытый ребенок
    ClosedIdentified(String),          // закрытый идентифицированный
    ClosedIdentifiedSend(String),      // закрытый пославший запрос на идентификацию
    SuspendedStandart(String),         // приостановленный стандартный
    SuspendedChild(String),            // приостановленный ребенок
    SuspendedIdentified(String),       // приостановленный идентифицированный
    SuspendedIdentifiedSend(String),   // приостановленный пославший запрос на идентификацию
    BannerStandart(String),            // закрытый баннером стандартный
    BannerChild(String),               // закрытый баннером ребенок
    BannerIdentified(String),          // закрытый баннером идентифицированный
    BannerIdentifiedSend(String),      // закрытый баннером пославший запрос на идентификацию
}

pub enum UserPerms {
    Standart(String),              // стандартные полномочия
    TraineeModerator(String),      // TRAINEE_MODERATOR
    Moderator(String),             // MODERATOR
    HighModerator(String),         // HIGH_MODERATOR
    TeamleadModerator(String),     // TEAMLEAD_MODERATOR
    TraineeManager(String),        // TRAINEE_MANAGER
    Manager(String),               // MANAGER
    HighManager(String),           // HIGH_MANAGER
    TeamleadManager(String),       // TEAMLEAD_MANAGER
    Advertiser(String),            // ADVERTISER
    HighAdvertiser(String),        // HIGH_ADVERTISER
    TeamleadAdvertiser(String),    // TEAMLEAD_ADVERTISER
    Administrator(String),         // ADMINISTRATOR
    HighAdministrator(String),     // HIGH_ADMINISTRATOR
    TeamleadAdministrator(String), // TEAMLEAD_ADMINISTRATOR
    Supermanager(String),          // SUPERMANAGER
}

pub enum UserGender {
    Man(String),     // Мужик
    Fem(String),     // Баба
}

pub enum UserDevice {
    De(String),      // Комп
    Ph(String),      // Телефон
}

pub enum UserLanguage {
    Ru(String),      // Русский
    En(String),      // Английский
}

#[derive(Queryable, Identifiable)]
#[table_name="users"]
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

#[derive(Deserialize,Insertable)]
pub struct NewUser {
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub types:         String,
    pub gender:        String,
    pub device:        String,
    pub language:      String,
    pub perm:          String,
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
