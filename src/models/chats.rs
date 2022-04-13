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
use crate::models::{
    User,
    Community,
    Post,
    Sticker,
};


/////// Тип чата //////
    // 1 публичный чат
    // 2 приватный
    // 3 менеджерский
    // 4 групповой
    // 11 Техподдержка 1 уровня
    // 12 Техподдержка 2 уровня
    // 13 Техподдержка 3 уровня
    // 14 Техподдержка 4 уровня
    // 15 Техподдержка 5 уровня

    // 21 удаленный публичный
    // 22 удаленный приватный
    // 23 удаленный менеджерский
    // 24 удаленный групповой
    // 31 закрытый публичный
    // 32 закрытый приватный
    // 33 закрытый менеджерский
    // 34 закрытый групповой
    // 41 удаленная техподдержка 1 уровня
    // 42 удаленная техподдержка 2 уровня
    // 43 удаленная техподдержка 3 уровня
    // 44 удаленная техподдержка 4 уровня
    // 45 удаленная техподдержка 5 уровня

/////// Приватность чата //////
    // 'a' Все участники
    // 'b' Создатель
    // 'c' Создатель и админы
    // 'd' Участники кроме
    // 'e' Некоторые участники

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Community)]
pub struct Chat {
    pub id:                 i32,
    pub name:               String,
    pub types:              i16,
    pub image:              Option<String>,
    pub description:        Option<String>,
    pub community_id:       Option<i32>,
    pub user_id:            i32,
    pub position:           i16,
    pub members:            i32,
    pub created:            chrono::NaiveDateTime,
    pub can_add_members:    char,
    pub can_fix_item:       char,
    pub can_mention:        char,
    pub can_add_admin:      char,
    pub can_add_design:     char,
    pub can_see_settings:   char,
    pub can_see_log:        char,
}

#[derive(Deserialize, Insertable)]
#[table_name="chats"]
pub struct NewChat {
    pub name:               String,
    pub types:              i16,
    pub community_id:       Option<i32>,
    pub user_id:            i32,
    pub position:           i16,
    pub members:            i32,
    pub created:            chrono::NaiveDateTime,
    pub can_add_members:    String,
    pub can_fix_item:       String,
    pub can_mention:        String,
    pub can_add_admin:      String,
    pub can_add_design:     String,
    pub can_see_settings:   String,
    pub can_see_log:        String,
}

/////// ChatUsers //////

/////// Тип участника чата //////
    // 'a' Действующий участник чата
    // 'b' Вышедший
    // 'c' Удаленный админом

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Chat)]
pub struct ChatUser {
    pub id:               i32,
    pub user_id:          i32,
    pub chat_id:          i32,
    pub types:            char,
    pub is_administrator: bool,
    pub created:          chrono::NaiveDateTime,
    pub no_disturb:       Option<chrono::NaiveDateTime>,
}
#[derive(Deserialize, Insertable)]
#[table_name="chat_users"]
pub struct NewChatUser {
    pub user_id:          i32,
    pub chat_id:          i32,
    pub types:            String,
    pub is_administrator: bool,
    pub created:          chrono::NaiveDateTime,
}

/////// ChatPerm //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(ChatUser)]
pub struct ChatIeSetting {
    pub id:               i32,
    pub chat_user_id:    i32,
    pub can_add_in_chat:  Option<char>,
    pub can_add_fix:      Option<char>,
    pub can_add_admin:    Option<char>,
    pub can_add_design:   Option<char>,
    pub can_see_settings: Option<char>,
    pub can_see_log:      Option<char>,
}
#[derive(Deserialize, Insertable)]
#[table_name="chat_ie_settings"]
pub struct NewChatIeSetting {
    pub chat_user_id:    i32,
    pub can_add_in_chat:  Option<String>,
    pub can_add_fix:      Option<String>,
    pub can_add_admin:    Option<String>,
    pub can_add_design:   Option<String>,
    pub can_see_settings: Option<String>,
    pub can_see_log:      Option<String>,
}

/////// Message //////

/////// Тип сообщения //////
    // 1 Опубликовано
    // 2 Редактировано
    // 5 Черновик
    // 6 Статусное
    // 7 Опубликовано закрепленное
    // 8 Редактировано закрепленное

    // 10 Удалено
    // 11 Закрыто
    // 22 Удалено редактированное
    // 24 Закрыто редактированное
    // 26 Удалено опубликованное закрепленное
    // 28 Закрыто опубликованное закрепленное
    // 30 Удалено редактированное закрепленное
    // 32 Закрыто редактированное закрепленное


#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Chat)]
#[belongs_to(User)]
#[belongs_to(Post)]
#[belongs_to(Sticker)]
pub struct Message {
    pub id:            i32,
    pub user_id:       i32,
    pub chat_id:       i32,
    pub parent_id:     Option<i32>,
    pub sticker_id:    Option<i32>,
    pub repost_id:     Option<i32>,
    pub created:       chrono::NaiveDateTime,
    pub content:       Option<String>,
    pub unread:        bool,
    pub types:         i16,
    pub attach:        Option<String>,
    pub voice:         Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="messages"]
pub struct NewMessage {
    pub user_id:       i32,
    pub chat_id:       i32,
    pub parent_id:     Option<i32>,
    pub sticker_id:    Option<i32>,
    pub repost_id:     Option<i32>,
    pub created:       chrono::NaiveDateTime,
    pub content:       Option<String>,
    pub types:         i16,
    pub attach:        Option<String>,
    pub voice:         Option<String>,
}

/////// MessageOptions //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Message)]
pub struct MessageOption {
    pub id:            i32,
    pub message_id:    i32,
    pub user_id:    i32,
    pub is_deleted:    bool,
    pub is_favourite:  bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="message_options"]
pub struct NewMessageOption {
    pub message_id:    i32,
    pub user_id:    i32,
    pub is_deleted:    bool,
    pub is_favourite:  bool,
}

/////// MessageVersion //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
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
#[derive(Deserialize, Insertable)]
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

/////// MessageTransfers //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct MessageTransfer {
    pub id:          i32,
    pub message_id:  i32,
    pub transfer_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="message_transfers"]
pub struct NewMessageTransfer {
    pub message_id:  i32,
    pub transfer_id: i32,
}
