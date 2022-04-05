use crate::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;

#[derive(Debug ,Queryable, Serialize, Identifiable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub _type: String,
    pub gender: String,
    pub device: String,
    pub language: String,
    pub perm: String,
    pub level: i32,
    pub password: String,
    pub have_link: Option<String>,
    pub city: Option<String>,
    pub status: Option<String>,
    pub b_avatar: Option<String>,
    pub s_avatar: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub gender: String,
    pub device: String,
    pub _type: String,
    pub password: String,
    pub level: i32,
    pub language: String,
}
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub phone: String,
    pub password: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserNameChange {
    pub first_name: String,
    pub last_name: String,
}
#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserPhoneChange {
    pub phone: String,
}
#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserAvatarChange {
    pub b_avatar: String,
    pub s_avatar: String,
}
#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserLinkChange {
    pub have_link: String,
}
#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserCityChange {
    pub city: String,
}
#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserStatusChange {
    pub status: String,
}
#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserPermChange {
    pub perm: String,
}
#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserLevelChange {
    pub level: i32,
}
#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserLanguageChange {
    pub language: String,
}
#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserTypeChange {
    pub _type: String,
}

/// Методы модели User...
pub(crate) struct UserOperation;

impl UserOperation {
    pub fn get_full_name(user: &User) -> String {
        user.first_name.clone() + &" ".to_string() + &user.last_name.clone()
    }
}
