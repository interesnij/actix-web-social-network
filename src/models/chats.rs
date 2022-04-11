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

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Community)]
#[table_name="chats"]
pub struct Chat {
    pub id:                 i32,
    pub name:               String,
    pub types:              i16,
    pub image:              Option<String>,
    pub description:        Option<String>,
    pub community_id:       Option<i32>,
    pub creator_id:         i32,
    pub position:           i32,
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

/////// ChatUsers //////

/////// Тип участника чата //////
    // 'a' Действующий участник чата
    // 'b' Вышедший
    // 'c' Удаленный админом

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Chat)]
#[table_name="chat_users"]
pub struct ChatUsers {
    pub id:               i32,
    pub user_id:          i32,
    pub chat_id:          i32,
    pub types:            char,
    pub is_administrator: Bool,
    pub created:          chrono::NaiveDateTime,
    pub no_disturb:       chrono::NaiveDateTime,
}

/////// ChatPerm //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(ChatUsers)]
#[table_name="chat_ie_settings"]
pub struct ChatPerm {
    pub id:               i32,
    pub user_id:          i32,
    pub can_add_in_chat:  char,
    pub can_add_fix:      char,
    pub can_add_admin:    char,
    pub can_add_design:   char,
    pub can_see_settings: char,
    pub can_see_log:      char,
}

/////// Message //////

/////// Тип сообщения //////
    // 1 Опубликовано
    // 2 Редактировано
    // 3 Удалено
    // 4 Закрыто
    // 5 Черновик
    // 6 Статусное
    // 7 Опубликовано закрепленное
    // 17 Редактировано закрепленное
    // 22 Удалено редактированное
    // 24 Закрыто редактированное
    // 26 Удалено опубликованное закрепленное
    // 28 Закрыто опубликованное закрепленное
    // 30 Удалено редактированное закрепленное
    // 32 Закрыто редактированное закрепленное


#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Chat)]
#[belongs_to(User)]
#[belongs_to(Post)]
#[belongs_to(Stickers)]
#[table_name="messages"]
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
    pub typed:         i16,
    pub attach:        Option<String>,
    pub voice:         Option<String>,
}

/////// MessageOptions //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Message)]
#[table_name="message_options"]
pub struct MessageOptions {
    pub id:            i32,
    pub message_id:    i32,
    pub creator_id:    i32,
    pub is_deleted:    Bool,
    pub is_favourite:  Bool,
}

/////// MessageVersion //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Message)]
#[table_name="message_versions"]
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

/////// MessageVersion //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Message, foreign_key="message_transfers_message")]
#[belongs_to(Message, foreign_key="message_transfers_transfer")]
#[table_name="message_transfers"]
pub struct MessageTransfers {
    pub id:            i32,
    pub message_id:    i32,
    pub transfer_id:   i32,
}
