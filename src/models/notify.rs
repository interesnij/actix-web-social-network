use crate::schema::{
    notifications,
    wall_objects,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{User, Community};


/////// Notification //////
////////// статус уведомления
    // 'a' Не прочитано
    // 'b' Прочитано
    // 'c' Удалено
    // 'd' Закрыто

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User), foreign_key="notify_recipient_user")]
#[belongs_to(User), foreign_key="notify_creator_user")]
#[belongs_to(Community), foreign_key="notify_community")]
#[belongs_to(Community), foreign_key="notify_action_community")]
pub struct Notification {
    pub id:                  i32,
    pub recipient_id:        i32,
    pub creator_id:          i32,
    pub created:             chrono::NaiveDateTime,
    pub verb:                String,
    pub status:              char,
    pub types:               i16,  // описан в модерации тип объекта
    pub object_id:           i32,
    pub community_id:        i32,
    pub action_community_id: i32,
    pub user_set_id:         i32,
    pub object_set_id:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="notifications"]
pub struct NewNotification {
    pub recipient_id:        i32,
    pub creator_id:          i32,
    pub created:             chrono::NaiveDateTime,
    pub verb:                String,
    pub status:              char,
    pub types:               i16,  // описан в модерации тип объекта
    pub object_id:           i32,
    pub community_id:        i32,
    pub action_community_id: i32,
    pub user_set_id:         i32,
    pub object_set_id:       i32,
}

/////// Notification //////
////////// статус уведомления
    // 'a' Не прочитано
    // 'b' Прочитано
    // 'c' Удалено
    // 'd' Закрыто

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User), foreign_key="wall_creator_user")]
#[belongs_to(Community), foreign_key="wall_community")]
#[belongs_to(Community), foreign_key="wall_action_community")]
pub struct WallObject {
    pub id:                  i32,
    pub creator_id:          i32,
    pub created:             chrono::NaiveDateTime,
    pub verb:                String,
    pub status:              char,
    pub types:               i16,  // описан в модерации тип объекта
    pub object_id:           i32,
    pub community_id:        i32,
    pub action_community_id: i32,
    pub user_set_id:         i32,
    pub object_set_id:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="wall_objects"]
pub struct NewWallObject {
    pub recipient_id:        i32,
    pub created:             chrono::NaiveDateTime,
    pub verb:                String,
    pub status:              char,
    pub types:               i16,  // описан в модерации тип объекта
    pub object_id:           i32,
    pub community_id:        i32,
    pub action_community_id: i32,
    pub user_set_id:         i32,
    pub object_set_id:       i32,
}
