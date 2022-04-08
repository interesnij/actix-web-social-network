use crate::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;


enum UserTypes {
    Standart,                   // стандартный тип пользователя
    Child,                     // ребенок
    Identifier,                // идентифицированный
    IdentifierSend,            // пославший запрос на идентификацию
    DeletedStandart,           // удаленный стандартный
    DeletedChild,              // удаленный ребенок
    DeletedIdentifier,         // удаленный идентифицированный
    DeletedIdentifierSend,     // удаленный пославший запрос на идентификацию
    ClosedStandart,            // закрытый стандартный
    ClosedChild,               // закрытый ребенок
    ClosedIdentifier,          // закрытый идентифицированный
    ClosedIdentifierSend,      // закрытый пославший запрос на идентификацию
    SuspendedStandart,         // приостановленный стандартный
    SuspendedChild,            // приостановленный ребенок
    SuspendedIdentifier,       // приостановленный идентифицированный
    SuspendedIdentifierSend,   // приостановленный пославший запрос на идентификацию
    BannerStandart,            // закрытый баннером стандартный
    BannerChild,               // закрытый баннером ребенок
    BannerIdentifier,          // закрытый баннером идентифицированный
    BannerIdentifierSend,      // закрытый баннером пославший запрос на идентификацию
}

enum UserPerms {
    Standart,                   // стандартные полномочия
    TraineeModerator,           // TRAINEE_MODERATOR
    Moderator,                  // MODERATOR
    HighModerator,              // HIGH_MODERATOR
    TeamleadModerator,          // TEAMLEAD_MODERATOR
    TraineeManager = 30,        // TRAINEE_MANAGER
    Manager = 33,               // MANAGER
    HighManager = 36,           // HIGH_MANAGER
    TeamleadManager = 39,       // TEAMLEAD_MANAGER
    Advertiser = 40,            // ADVERTISER
    HighAdvertiser = 44,        // HIGH_ADVERTISER
    TeamleadAdvertiser = 49,    // TEAMLEAD_ADVERTISER
    Administrator = 50,         // ADMINISTRATOR
    HighAdministrator = 54,     // HIGH_ADMINISTRATOR
    TeamleadAdministrator = 59, // TEAMLEAD_ADMINISTRATOR
    Supermanager = 60,          // SUPERMANAGER
}

enum UserGender {
    Man,     // Мужик
    Fem,     // Баба
}

enum UserDevice {
    De,      // Комп
    Ph,      // Телефон
}

enum UserLanguage {
    Ru,      // Русский
    En,      // Английский
}

#[derive(Queryable, Identifiable)]
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
