use crate::schema::{
    chats,
    chat_users,
    chat_ie_settings,
    messages,
    message_versions,
    message_options,
    message_transfers,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{User, Community, Post};


/////// Chat //////
enum ChatTypes {
    "PUB",     // публичный чат
    "PRI",     // приватный
    "MAN",     // менеджерский
    "GRO",     //  групповой
    "_DPUB",   // удаленный публичный
    "_DPRI",   // удаленный приватный
    "_DMAN",   // удаленный менеджерский
    "_DGRO",   // удаленный групповой
    "_CPUB",   // закрытый публичный
    "_CPRI",   // закрытый приватный
    "_CMAN",   // закрытый менеджерский
    "_CGRO",   // закрытый групповой
    "SUP1",    // Техподдержка 1 уровня
    "SUP2",    // Техподдержка 2 уровня
    "SUP3",    // Техподдержка 3 уровня
    "SUP4",    // Техподдержка 4 уровня
    "SUP5",    // Техподдержка 5 уровня
    "_SU1",    // удаленная техподдержка 1 уровня
    "_SU2",    // удаленная техподдержка 2 уровня
    "_SU3",    // удаленная техподдержка 3 уровня
    "_SU4",    // удаленная техподдержка 4 уровня
    "_SU5",    // удаленная техподдержка 5 уровня
}

enum ChatPerms {
    1,     // Все участники
    2,     // Создатель
    3,     // Создатель и админы
    4,     // Участники кроме
    5,     // Некоторые участники
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Community)]
pub struct Chat {
    pub id:                 i32,
    pub name:               String,
    pub types:              ChatTypes,
    pub image:              Option<String>,
    pub description:        Option<String>,
    pub community_id:       Option<i32>,
    pub creator_id:         i32,
    pub position:           i32,
    pub members:            i32,
    pub created:            chrono::NaiveDateTime,
    pub can_add_members:    i32,
    pub can_fix_item:       i32,
    pub can_mention:        i32,
    pub can_add_admin:      i32,
    pub can_add_design:     i32,
    pub can_see_settings:   i32,
    pub can_see_log:        i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="chats"]
pub struct NewChat {
    pub name:             String,
    pub types:            ChatTypes,
    pub image:            Option<String>,
    pub description:      Option<String>,
    pub community_id:     Option<i32>,
    pub creator_id:       i32,
    pub position:         i32,
    pub members:          i32,
    pub created:          chrono::NaiveDateTime,
    pub can_add_members:  i32,
    pub can_fix_item:     i32,
    pub can_mention:      i32,
    pub can_add_admin:    i32,
    pub can_add_design:   i32,
    pub can_see_settings: i32,
    pub can_see_log:      i32,
}

/////// ChatUsers //////
enum ChatUsersTypes {
    "ACT",      // Все участники
    "EXI",      // Создатель
    "DEL",      // Создатель и админы
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Chat)]
pub struct ChatUsers {
    pub id:               i32,
    pub user_id:          i32,
    pub chat_id:          i32,
    pub types:            ChatUsersTypes,
    pub is_administrator: Bool,
    pub created:          chrono::NaiveDateTime,
    pub no_disturb:       chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="chat_users"]
pub struct NewChatUsers {
    pub user_id:          i32,
    pub chat_id:          i32,
    pub types:            ChatUsersTypes,
    pub is_administrator: Bool,
    pub created:          chrono::NaiveDateTime,
    pub no_disturb:       chrono::NaiveDateTime,
}

/////// ChatPerm //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(ChatUsers)]
pub struct ChatPerm {
    pub id:               i32,
    pub user_id:          i32,
    pub can_add_in_chat:  i32,
    pub can_add_fix:      i32,
    pub can_add_admin:    i32,
    pub can_add_design:   i32,
    pub can_see_settings: i32,
    pub can_see_log:      i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="chat_ie_settings"]
pub struct NewChatPerm {
    pub user_id:          i32,
    pub can_add_in_chat:  i32,
    pub can_add_fix:      i32,
    pub can_add_admin:    i32,
    pub can_add_design:   i32,
    pub can_see_settings: i32,
    pub can_see_log:      i32,
}

/////// Message //////
enum MessageTypes {
    "PUB",      // Опубликовано
    "EDI",      // Редактировано
    "_DEL",     // Удалено
    "_CLO",     // Закрыто
    "_DRA",     // Черновик
    "MAN",      // Статусное
    "PFIX",     // Опубликовано закрепленное
    "EFIX",     // Редактировано закрепленное
    "_DELE",    // Удалено редактированное
    "_CLOE",    // Закрыто редактированное
    "_DELPF",   // Удалено опубликованное закрепленное
    "_CLOPF",   // Закрыто опубликованное закрепленное
    "_DELEF",   // Удалено редактированное закрепленное
    "_CLOEF",   // Закрыто редактированное закрепленное
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Chat)]
#[belongs_to(User)]
#[belongs_to(Post)]
pub struct Message {
    pub id:            i32,
    pub creator_id:    i32,
    pub chat_id:       i32,
    pub parent_id:     Option<i32>,
    pub sticker_id:    Option<i32>,
    pub repost_id:     Option<i32>,
    pub created:       chrono::NaiveDateTime,
    pub content:       Option<String>,
    pub unread:        Bool,
    pub typed:         MessageTypes,
    pub attach:        Option<String>,
    pub voice:         Option<String>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="messages"]
pub struct NewMessage {
    pub creator_id:    i32,
    pub chat_id:       i32,
    pub parent_id:     Option<i32>,
    pub sticker_id:    Option<i32>,
    pub repost_id:     Option<i32>,
    pub created:       chrono::NaiveDateTime,
    pub content:       Option<String>,
    pub unread:        Bool,
    pub typed:         MessageTypes,
    pub attach:        Option<String>,
    pub voice:         Option<String>,
}

/////// MessageOptions //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Message)]
pub struct MessageOptions {
    pub id:            i32,
    pub message_id:    i32,
    pub creator_id:    i32,
    pub is_deleted:    Bool,
    pub is_favourite:  Bool,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="message_options"]
pub struct NewMessageOptions {
    pub message_id:    i32,
    pub creator_id:    i32,
    pub is_deleted:    Bool,
    pub is_favourite:  Bool,
}

/////// MessageVersion //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Message)]
pub struct MessageVersion {
    pub id:            i32,
    pub message_id:    i32,
    pub sticker_id:    Option<i32>,
    pub repost_id:     Option<i32>,
    pub parent_id:     Option<i32>,
    pub created:       chrono::NaiveDateTime,
    pub content:       Option<String>,
    pub attach:        Option<String>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="message_versions"]
pub struct NewMessageVersion {
    pub message_id:    i32,
    pub sticker_id:    Option<i32>,
    pub repost_id:     Option<i32>,
    pub parent_id:     Option<i32>,
    pub created:       chrono::NaiveDateTime,
    pub content:       Option<String>,
    pub attach:        Option<String>,
}

/////// MessageVersion //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Message, foreign_key="message_transfers_message")]
#[belongs_to(Message, foreign_key="message_transfers_transfer")]
pub struct MessageTransfers {
    pub id:            i32,
    pub message_id:    i32,
    pub transfer_id:   i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="message_transfers"]
pub struct NewMessageTransfers {
    pub message_id:    i32,
    pub transfer_id:   i32,
}
