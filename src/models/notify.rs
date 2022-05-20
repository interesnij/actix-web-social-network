//use diesel::prelude::*;
//use crate::schema;
use crate::schema::{
    notifications,
    wall_objects,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
//use crate::utils::establish_connection;
//use crate::models::{User, Community};


/////// Notification //////
////////// статус уведомления
    // 'a' Не прочитано
    // 'b' Прочитано
    // 'c' Удалено
    // 'd' Закрыто

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
pub struct Notification {
    pub id:                  i32,
    pub recipient_id:        Option<i32>,
    pub user_id:             i32,
    pub created:             chrono::NaiveDateTime,
    pub verb:                String,
    pub status:              String,
    pub types:               i16,  // описан в модерации тип объекта
    pub object_id:           i32,
    pub community_id:        Option<i32>,
    pub action_community_id: Option<i32>,
    pub user_set_id:         Option<i32>,
    pub object_set_id:       Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="notifications"]
pub struct NewNotification {
    pub recipient_id:        Option<i32>,
    pub user_id:             i32,
    pub created:             chrono::NaiveDateTime,
    pub verb:                String,
    pub status:              String,
    pub types:               i16,  // описан в модерации тип объекта
    pub object_id:           i32,
    pub community_id:        Option<i32>,
    pub action_community_id: Option<i32>,
    pub user_set_id:         Option<i32>,
    pub object_set_id:       Option<i32>,
}

/////// Notification //////
////////// статус уведомления
    // 'a' Не прочитано
    // 'b' Прочитано
    // 'c' Удалено
    // 'd' Закрыто

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
pub struct WallObject {
    pub id:                  i32,
    pub user_id:             i32,
    pub created:             chrono::NaiveDateTime,
    pub verb:                String,
    pub status:              String,
    pub types:               i16,  // описан в модерации тип объекта
    pub object_id:           i32,
    pub community_id:        Option<i32>,
    pub action_community_id: Option<i32>,
    pub user_set_id:         Option<i32>,
    pub object_set_id:       Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="wall_objects"]
pub struct NewWallObject {
    pub user_id:             i32,
    pub created:             chrono::NaiveDateTime,
    pub verb:                String,
    pub status:              String,
    pub types:               i16,  // описан в модерации тип объекта
    pub object_id:           i32,
    pub community_id:        Option<i32>,
    pub action_community_id: Option<i32>,
    pub user_set_id:         Option<i32>,
    pub object_set_id:       Option<i32>,
}
