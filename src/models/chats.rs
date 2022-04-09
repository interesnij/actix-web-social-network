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


/////// Chat //////
enum ChatTypes {
    Public,             // публичный чат
    Private,            // приватный
    Manager,            // менеджерский
    Group,              //  групповой
    DeletedPublic,      // удаленный публичный
    DeletedPrivate,     // удаленный приватный
    DeletedManager,     // удаленный менеджерский
    DeletedGroup,       // удаленный групповой
    ClosedPublic,       // закрытый публичный
    ClosedPrivate,      // закрытый приватный
    ClosedManager,      // закрытый менеджерский
    ClosedGroup,        // закрытый групповой
    Support1,           // Техподдержка 1 уровня
    Support2,           // Техподдержка 2 уровня
    Support3,           // Техподдержка 3 уровня
    Support4,           // Техподдержка 4 уровня
    Support5,           // Техподдержка 5 уровня
    DeletedSupport1,    // удаленная техподдержка 1 уровня
    DeletedSupport2,    // удаленная техподдержка 2 уровня
    DeletedSupport3,    // удаленная техподдержка 3 уровня
    DeletedSupport4,    // удаленная техподдержка 4 уровня
    DeletedSupport5,    // удаленная техподдержка 5 уровня
}

enum ChatPerms {
    AllCan,           // Все участники
    Creator,          // Создатель
    CreatorAndAdmins, // Создатель и админы
    MembersBut,       // Участники кроме
    SomeMembersBut,   // Некоторые участники
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Community)]
#[table_name="chats"]
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

/////// ChatUsers //////
enum ChatUsersTypes {
    Active,    // Действующий участник чата
    Left,      // Вышедший
    Deleted,   // Удаленный админом
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Chat)]
#[table_name="chat_users"]
pub struct ChatUsers {
    pub id:               i32,
    pub user_id:          i32,
    pub chat_id:          i32,
    pub types:            ChatUsersTypes,
    pub is_administrator: Bool,
    pub created:          chrono::NaiveDateTime,
    pub no_disturb:       chrono::NaiveDateTime,
}

/////// ChatPerm //////
pub enum ChatPermTypes{
    NoValue,    // Нет значения
    Enable,     // Активно
    Disable,    // Не активно
}
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(ChatUsers)]
#[table_name="chat_ie_settings"]
pub struct ChatPerm {
    pub id:               i32,
    pub user_id:          i32,
    pub can_add_in_chat:  ChatPermTypes,
    pub can_add_fix:      ChatPermTypes,
    pub can_add_admin:    ChatPermTypes,
    pub can_add_design:   ChatPermTypes,
    pub can_see_settings: ChatPermTypes,
    pub can_see_log:      ChatPermTypes,
}

/////// Message //////
enum MessageTypes {
    Published,               // Опубликовано
    Edited,                 // Редактировано
    Deleted,                // Удалено
    Closed,                 // Закрыто
    Draft,                  // Черновик
    Manager,                // Статусное
    FixedPublished,         // Опубликовано закрепленное
    FixedEdited,            // Редактировано закрепленное
    DeletedEdited,          // Удалено редактированное
    ClosedEdited,           // Закрыто редактированное
    DeletedFixedPublished,  // Удалено опубликованное закрепленное
    ClosedFixedPublished,   // Закрыто опубликованное закрепленное
    DeletedFixedEdited,     // Удалено редактированное закрепленное
    ClosedFixedEdited,      // Закрыто редактированное закрепленное
}

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
    pub typed:         MessageTypes,
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
