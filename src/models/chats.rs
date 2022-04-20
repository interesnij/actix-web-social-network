use diesel::prelude::*;
use crate::schema;
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
    // 'f' Никто!

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Community)]
pub struct Chat {
    pub id:                 i32,
    pub name:               Option<String>,
    pub types:              i16,
    pub image:              Option<String>,
    pub description:        Option<String>,
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

#[derive(Deserialize, Insertable)]
#[table_name="chats"]
pub struct NewChat {
    pub name:               Option<String>,
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
impl Chat {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn get_name(&self, user_id: i32) -> String {
        let chat_types = self.types;
        if self.name.is_some() {
            return self.name.unwrap();
        }
        else if self.is_group() {
            return "Групповой чат".to_string();
        }
        else if self.is_public() {
            return "Публичнеый чат".to_string();
        }
        else if self.is_private() {
            return self.get_chat_member(user_id).get_full_name();
        }
        else {
            return "Без имени".to_string();
        }
    }
    pub fn is_chat(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "cha".to_string() + &self.get_str_id();
    }
    pub fn delete_support_chat(&self) -> bool {
        let _connection = establish_connection();
        let chat_types = self.types;
        let delete_case = match chat_types {
            11 => 41,
            12 => 42,
            13 => 43,
            14 => 44,
            15 => 45,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::chats::types.eq(delete_case))
            .get_result::<Chat>(&_connection)
            .expect("E");
       return true;
    }
    pub fn restore_support_chat(&self) -> bool {
        let _connection = establish_connection();
        let chat_types = self.types;
        let restore_case = match chat_types {
            41 => 11,
            42 => 12,
            43 => 13,
            44 => 14,
            45 => 15,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::chats::types.eq(restore_case))
            .get_result::<Chat>(&_connection)
            .expect("E");
       return true;
    }
    pub fn is_private(&self) -> bool {
        return self.types == 2;
    }
    pub fn is_group(&self) -> bool {
        return self.types == 4;
    }
    pub fn is_public(&self) -> bool {
        return self.types == 1;
    }
    pub fn is_manager(&self) -> bool {
        return self.types == 3;
    }
    pub fn is_open(&self) -> bool {
        return self.types < 10;
    }
    pub fn is_support(&self) -> bool {
        return self.types > 10 && self.types < 10;
    }
    pub fn is_support_1(&self) -> bool {
        return self.types == 11;
    }
    pub fn is_support_2(&self) -> bool {
        return self.types == 12;
    }
    pub fn is_support_3(&self) -> bool {
        return self.types == 13;
    }
    pub fn is_support_4(&self) -> bool {
        return self.types == 14;
    }
    pub fn is_support_5(&self) -> bool {
        return self.types == 15;
    }
    pub fn get_members_ids(&self) -> Vec<i32> {
        use crate::schema::chat_users::dsl::chat_users;

        let _connection = establish_connection();
        let items = chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::types.eq("a"))
            .load::<ChatUser>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_administrators_ids(&self) -> Vec<i32> {
        use crate::schema::chat_users::dsl::chat_users;

        let _connection = establish_connection();
        let items = chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::is_administrator.eq(true))
            .filter(schema::chat_users::types.eq("a"))
            .load::<ChatUser>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_recipients_ids(&self, user_id: i32) -> Vec<i32> {
        use crate::schema::chat_users::dsl::chat_users;

        let _connection = establish_connection();
        let items = chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::user_id.ne(user_id))
            .filter(schema::chat_users::types.eq("a"))
            .load::<ChatUser>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_recipients(&self) -> Vec<ChatUser> {
        // все объекты участников чата
        use crate::schema::chat_users::dsl::chat_users;

        let _connection = establish_connection();
        return chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::types.eq("a"))
            .load::<ChatUser>(&_connection)
            .expect("E");
    }
    pub fn get_recipients_2(&self, user_id: i32) -> Vec<ChatUser> {
        // все объекты участников чата, кроме создателя
        use crate::schema::chat_users::dsl::chat_users;

        let _connection = establish_connection();
        return chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::user_id.ne(user_id))
            .filter(schema::chat_users::types.eq("a"))
            .load::<ChatUser>(&_connection)
            .expect("E");
    }
    pub fn get_members(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_members_ids());
    }
    pub fn get_administrators(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_administrators_ids());
    }
    pub fn get_recipients_exclude_creator(&self, user_id: i32) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_recipients_ids(user_id));
    }
    pub fn get_members_count(&self) -> i32 {
        return self.members;
    }
    pub fn get_members_count_ru(&self) -> String {
        use crate::utils::get_count_for_ru;
        return get_count_for_ru(
            self.get_members_count(),
            " участник".to_string(),
            " участника".to_string(),
            " участников".to_string(),
        );
    }
    pub fn get_chat_member(&self, user_id: i32) -> User {
        use crate::schema::chat_users::dsl::chat_users;
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let chat_user = chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::user_id.eq(user_id))
            .filter(schema::chat_users::types.eq("a"))
            .load::<ChatUser>(&_connection)
            .expect("E");
        return users
            .filter(schema::users::id.eq(chat_user[0].id))
            .filter(schema::users::types.lt(10))
            .load::<User>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
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
    pub types:            String,
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
    pub chat_user_id:     i32,
    pub can_add_in_chat:  Option<String>,
    pub can_add_fix:      Option<String>,
    pub can_add_admin:    Option<String>,
    pub can_add_design:   Option<String>,
    pub can_see_settings: Option<String>,
    pub can_see_log:      Option<String>,
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
    // 6 Статусное
    // 7 Опубликовано закрепленное
    // 8 Редактировано закрепленное

    // 10 Черновик
    // 11 Удалено
    // 12 Закрыто
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
    pub post_id:       Option<i32>,
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
    pub post_id:     Option<i32>,
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
