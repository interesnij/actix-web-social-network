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
    Photo,
    Video,
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
        if self.name.is_some() {
            return self.name.as_ref().unwrap().to_string();
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
            .filter(schema::chat_users::user_id.ne(user_id))
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
    pub fn get_chat_user(&self, user_id: i32) -> ChatUser {
        use crate::schema::chat_users::dsl::chat_users;

        let _connection = establish_connection();
        let chat_user = chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::user_id.ne(user_id))
            .filter(schema::chat_users::types.eq("a"))
            .load::<ChatUser>(&_connection)
            .expect("E");
        if chat_user.len() > 0 {
            return chat_user.into_iter().nth(0).unwrap();
        }
        else {
            return chat_users
                .filter(schema::chat_users::chat_id.eq(self.id))
                .filter(schema::chat_users::user_id.eq(user_id))
                .filter(schema::chat_users::types.eq("a"))
                .load::<ChatUser>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
        }
    }
    pub fn get_chat_request_user(&self, user_id: i32) -> ChatUser {
        use crate::schema::chat_users::dsl::chat_users;

        let _connection = establish_connection();
            return chat_users
                .filter(schema::chat_users::chat_id.eq(self.id))
                .filter(schema::chat_users::user_id.eq(user_id))
                .filter(schema::chat_users::types.eq("a"))
                .load::<ChatUser>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
    }
    pub fn is_not_empty(&self) -> bool {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
            return messages
                .filter(schema::messages::chat_id.eq(self.id))
                .filter(schema::messages::types.lt(10))
                .load::<Message>(&_connection)
                .expect("E")
                .len() > 0;
    }
    pub fn create_administrator(&self, user: User) -> bool {
        use crate::schema::chat_users::dsl::chat_users;
        if !user.is_member_of_chat(self.id) {
            return false;
        }
        let _connection = establish_connection();
        let member = chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::user_id.eq(user.id))
            .load::<ChatUser>(&_connection)
            .expect("E");
        let member_form = NewChatUser {
            user_id: user.id,
            chat_id: self.id,
            types: "a".to_string(),
            is_administrator: true,
            created: member[0].created,
            no_disturb: member[0].no_disturb,
        };

        diesel::update(&member[0])
            .set(member_form)
            .get_result::<ChatUser>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn delete_administrator(&self, user: User) -> bool {
        use crate::schema::chat_users::dsl::chat_users;
        if !user.is_member_of_chat(self.id) {
            return false;
        }
        let _connection = establish_connection();
        let member = chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::user_id.eq(user.id))
            .load::<ChatUser>(&_connection)
            .expect("E");

        diesel::update(&member[0])
            .set(schema::chat_users::is_administrator.eq(false))
            .get_result::<ChatUser>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn get_draft_message(&self, user_id: i32) -> Message {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        return messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::user_id.eq(user_id))
            .filter(schema::messages::types.eq(10))
            .load::<Message>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn is_have_draft_message(&self, user_id: i32) -> bool {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        return messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::user_id.eq(user_id))
            .filter(schema::messages::types.eq(10))
            .load::<Message>(&_connection)
            .expect("E").len() > 0;
    }
    pub fn is_have_draft_message_content(&self, user_id: i32) -> bool {
        // есть ли черновик сообщения, притом не пустой
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        let t_message = messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::user_id.eq(user_id))
            .filter(schema::messages::types.eq(10))
            .load::<Message>(&_connection)
            .expect("E");
        if t_message.len() > 0 {
            let message = t_message.into_iter()
                .nth(0)
                .unwrap();
            return message.content.is_some() || message.attach.is_some() || message.is_have_transfer();
        }
        return false;
    }
    pub fn get_fixed_messages(&self) -> Vec<Message> {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        return messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::types.eq_any(vec![7,8]))
            .load::<Message>(&_connection)
            .expect("E");
    }
    pub fn get_fix_message_count(&self) -> usize {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        return messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::types.eq_any(vec![7,8]))
            .load::<Message>(&_connection)
            .expect("E")
            .len();
    }
    pub fn get_fix_message_count_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru(
            self.get_fix_message_count().try_into().unwrap(),
            " сообщение".to_string(),
            " сообщения".to_string(),
            " сообщений".to_string(),
        );
    }
    pub fn get_first_fix_message(&self) -> Message {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        return messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::types.eq_any(vec![7,8]))
            .load::<Message>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn create_membership(&self, user: User, is_administrator: bool) -> ChatUser {
        use crate::schema::chat_users::dsl::chat_users;

        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::chats::members.eq(self.members + 1))
            .get_result::<Chat>(&_connection)
            .expect("Error.");

        let member_exists = chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::user_id.eq(user.id))
            .filter(schema::chat_users::types.eq("b"))
            .load::<ChatUser>(&_connection)
            .expect("E");
        if member_exists.len() > 0 {
            let curr_member = member_exists.into_iter().nth(0).unwrap();
            diesel::update(&curr_member)
                .set(schema::chat_users::types.eq("a"))
                .get_result::<ChatUser>(&_connection)
                .expect("Error.");
            return curr_member;
        }
        else {
            let new_member_form = NewChatUser {
                user_id: user.id,
                chat_id: self.id,
                types: "a".to_string(),
                is_administrator: is_administrator,
                created: chrono::Local::now().naive_utc(),
                no_disturb: None,
            };
            let new_member = diesel::insert_into(schema::chat_users::table)
                .values(&new_member_form)
                .get_result::<ChatUser>(&_connection)
                .expect("E.");
            return new_member;
        }
    }
    pub fn exit_member(&self, user: User) -> bool {
        use crate::schema::chat_users::dsl::chat_users;

        let _connection = establish_connection();

        let member_exists = chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::user_id.eq(user.id))
            .filter(schema::chat_users::types.eq("a"))
            .load::<ChatUser>(&_connection)
            .expect("E");
        if member_exists.len() > 0 {
            let curr_member = member_exists.into_iter().nth(0).unwrap();
            diesel::update(&curr_member)
                .set(schema::chat_users::types.eq("b"))
                .get_result::<ChatUser>(&_connection)
                .expect("Error.");

            diesel::update(self)
                .set(schema::chats::members.eq(self.members - 1))
                .get_result::<Chat>(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn delete_member(&self, user: User) -> bool {
        use crate::schema::chat_users::dsl::chat_users;

        let _connection = establish_connection();

        let member_exists = chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::user_id.eq(user.id))
            .filter(schema::chat_users::types.eq("a"))
            .load::<ChatUser>(&_connection)
            .expect("E");
        if member_exists.len() > 0 {
            let curr_member = member_exists.into_iter().nth(0).unwrap();
            diesel::update(&curr_member)
                .set(schema::chat_users::types.eq("c"))
                .get_result::<ChatUser>(&_connection)
                .expect("Error.");

            diesel::update(self)
                .set(schema::chats::members.eq(self.members - 1))
                .get_result::<Chat>(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn get_messages_ids(&self) -> Vec<i32> {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        let chat_messages = messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::types.lt(10))
            .load::<Message>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in chat_messages.iter() {
            stack.push(_item.id);
        };
        return stack;
    }
    pub fn get_unread_message(&self, user_id: i32 ) -> Vec<Message> {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        return messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::unread.eq(true))
            .filter(schema::messages::types.lt(10))
            .filter(schema::messages::user_id.ne(user_id))
            .load::<Message>(&_connection)
            .expect("E");
    }
    pub fn is_empty(&self, user_id: i32) -> bool {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        return messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::types.lt(10))
            .filter(schema::messages::user_id.ne(user_id))
            .load::<Message>(&_connection)
            .expect("E").len() == 0;
    }
    pub fn get_first_message(&self, user_id: i32) -> Message {
        use crate::schema::messages::dsl::messages;
        use crate::schema::message_options::dsl::message_options;

        let _connection = establish_connection();

        if message_options
            .filter(schema::message_options::user_id.eq(user_id))
            .filter(schema::message_options::is_deleted.eq(true))
            .load::<MessageOption>(&_connection)
            .expect("E")
            .len() == 0 {
                return messages
                    .filter(schema::messages::chat_id.eq(self.id))
                    .filter(schema::messages::types.lt(10))
                    .order(schema::messages::created.desc())
                    .load::<Message>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0)
                    .unwrap();
            }

        let get_message = &messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::types.lt(10))
            .order(schema::messages::created.desc())
            .load::<Message>(&_connection)
            .expect("E")[0];

        let mut stack = Vec::new();
        if message_options
            .filter(schema::message_options::user_id.eq(user_id))
            .filter(schema::message_options::message_id.eq(get_message.id))
            .filter(schema::message_options::is_deleted.eq(true))
            .limit(1)
            .load::<MessageOption>(&_connection)
            .expect("E")
            .len() == 0 {
                stack.push(get_message.id);
            }

        return messages
            .filter(schema::messages::id.eq_any(stack))
            .filter(schema::messages::types.lt(10))
            .order(schema::messages::created.desc())
            .load::<Message>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn get_preview_message(&self, user_id: i32 ) -> String {
        let first_message = self.get_first_message(user_id);
        let mut preview_text: String;
        let mut is_read = "".to_string();
        let mut creator_figure: String;
        let mut created = "".to_string();
        let mut beep_icon = "".to_string();

        if self.is_have_draft_message_content(user_id) {
            let message = self.get_draft_message(user_id);
            preview_text = "Черновик: ".to_string() + &message.get_type_text();
        }
        else if self.is_empty(user_id) {
            preview_text = "Нет сообщений".to_string();
        }
        else if first_message.is_manager() {
            created = first_message.created.format("%d-%m-%Y в %H:%M").to_string();
            if first_message.parent_id.is_some() {
                let creator = first_message.get_creator();
                let message = first_message.get_parent();
                preview_text = concat_string!(
                    creator.get_full_name(),
                    first_message.content.as_deref().unwrap(),
                    "<span class='underline'>",
                    message.get_text_60(),
                    "</span>")
            }
            else {
                preview_text = first_message.get_text_60();
            }
        }
        else {
            preview_text = first_message.get_text_60();
            if first_message.user_id == user_id {
                preview_text = "Вы: ".to_owned() + &first_message.get_type_text();
                if first_message.unread == true {
                    is_read = " bg-light-secondary".to_string();
                }
            }
            else {
                preview_text = first_message.get_type_text();
            }

        }
        let member = Some(self.get_chat_request_user(user_id));
        if member.is_some() {
            beep_icon = member.unwrap().get_beep_icon();
        }

        if self.is_group() && self.is_public() {
            let figure: String;
            let name: String;

            if self.image.is_some() {
                figure = concat_string!(
                    "<figure><img src='",
                    self.image.as_deref().unwrap(),
                    "style='border-radius:50px;width:50px;' alt='image'></figure>");
            }
            else {
                figure = "<figure><img src='/static/images/group_chat.jpg' style='border-radius:50px;width:50px;' alt='image'></figure>".to_string();
            }

            if self.name.is_some() {
                name = self.name.as_deref().unwrap().to_string();
            }
            else {
                name = "Групповой чат".to_string();
            }

            let media_body = concat_string!(
                "<div class='media-body'><h5 class='time-title mb-0'>",
                name, beep_icon,
                "<small class='float-right text-muted'>",
                created,
                "</small></h5><p class='mb-0",
                is_read,
                "' style='white-space: nowrap;'>",
                preview_text,
                "</p><span class='typed'></span></div>"
            );
            return concat_string!(
                "<div class='media'>",
                figure, media_body,
                self.get_unread_count_message(user_id),
                "</div>"
            )
        }
        else if self.is_private() {
            let member = self.get_chat_member(user_id);
            let figure: String;
            let name: String;
            let mut status = "".to_string();

            if self.image.is_some() {
                figure = concat_string!(
                    "<figure><img src='",
                    self.image.as_deref().unwrap(),
                    "style='border-radius:50px;width:50px;' alt='image'></figure>");
            }
            else if member.s_avatar.is_some() {
                figure = concat_string!(
                    "<figure><img src='",
                    member.s_avatar.as_deref().unwrap(),
                    "style='border-radius:50px;width:50px;' alt='image'></figure>");
            }
            else {
                figure = "<figure><svg fill='currentColor' class='svg_default svg_default_50' viewBox='0 0 24 24'><path d='M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z'/><path d='M0 0h24v24H0z' fill='none'/></svg></figure>".to_string();
            }

            if self.name.is_some() {
                name = self.name.as_deref().unwrap().to_string();
            }
            else {
                name = member.get_full_name();
            }

            if member.is_online() {
                status = " <span class='status bg-success'></span>".to_string();
            }

            let media_body = concat_string!(
                "<div class='media-body'><h5 class='time-title mb-0'>",
                name, beep_icon, status,
                "<small class='float-right text-muted'>",
                created,
                "</small></h5><p class='mb-0",
                is_read,
                "' style='white-space: nowrap;'>",
                preview_text,
                "</p><span class='typed'></span></div>"
            );
            return concat_string!(
                "<div class='media'>",
                figure, media_body,
                self.get_unread_count_message(user_id),
                "</div>"
            )
        }
        else if self.is_support() {
            let member = self.get_chat_member(user_id);
            let figure = "<figure><svg fill='currentColor' class='svg_default svg_default_50' viewBox='0 0 24 24'><path d='M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z'/><path d='M0 0h24v24H0z' fill='none'/></svg></figure>".to_string();
            let mut name = "".to_string();
            let mut status = "".to_string();

            if self.members == 1 {
                name = "Чат техподдержки".to_string();
            }
            else {
                 use crate::schema::support_users::dsl::support_users;
                 use crate::models::SupportUser;

                 let _connection = establish_connection();
                 for user in self.get_members() {
                     if user.id != self.user_id {
                        let supports = support_users
                             .filter(schema::support_users::manager_id.eq(user.id))
                             .load::<SupportUser>(&_connection)
                             .expect("E");
                        if supports.len() > 0 {
                            name = "Агент техподдержки №".to_string() + &supports[0].id.to_string();
                            if user.is_online() {
                                status = " <span class='status bg-success'></span>".to_string();
                            }
                        }
                     }
                 }
            }

            let media_body = concat_string!(
                "<div class='media-body'><h5 class='time-title mb-0'>",
                name, beep_icon, status,
                "<small class='float-right text-muted'>",
                created,
                "</small></h5><p class='mb-0",
                is_read,
                "' style='white-space: nowrap;'>",
                preview_text,
                "</p><span class='typed'></span></div>"
            );
            return concat_string!(
                "<div class='media'>",
                figure, media_body,
                self.get_unread_count_message(user_id),
                "</div>"
            )
        }
        else if self.is_manager() {
            let figure = "<figure><svg fill='currentColor' class='svg_default svg_default_50' viewBox='0 0 24 24'><path d='M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z'/><path d='M0 0h24v24H0z' fill='none'/></svg></figure>".to_string();
            let name = self.name.as_deref().unwrap();

            let media_body = concat_string!(
                "<div class='media-body'><h5 class='time-title mb-0'>",
                name, beep_icon,
                "<small class='float-right text-muted'>",
                created,
                "</small></h5><p class='mb-0",
                is_read,
                "' style='white-space: nowrap;'>",
                preview_text,
                "</p><span class='typed'></span></div>"
            );
            return concat_string!(
                "<div class='media'>",
                figure, media_body,
                self.get_unread_count_message(user_id),
                "</div>"
            )
        }
        else {
            return "".to_string();
        }
    }
    pub fn get_unread_count_message(&self, user_id: i32 ) -> String {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        let count = messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::unread.eq(true))
            .filter(schema::messages::types.lt(10))
            .filter(schema::messages::user_id.ne(user_id))
            .load::<Message>(&_connection)
            .expect("E")
            .len();

        if count > 0 {
            return "<span style='font-size: 80%' class='tab_badge custom_color'>".to_owned() + &count.to_string() + &"</span>".to_string();
        }
        return "".to_string()
    }
    pub fn get_messages(&self, limit: i64, offset: i64) -> Vec<Message> {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        return messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::types.lt(10))
            .order(schema::messages::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Message>(&_connection)
            .expect("E");
    }
    pub fn get_messages_for_user(&self, limit: i64, offset: i64, user_id: i32) -> Vec<Message> {
        use crate::schema::messages::dsl::messages;
        use crate::schema::message_options::dsl::message_options;

        let _connection = establish_connection();
        if message_options
            .filter(schema::message_options::user_id.eq(user_id))
            .filter(schema::message_options::is_deleted.eq(true))
            .load::<MessageOption>(&_connection)
            .expect("E")
            .len() == 0 {
                return self.get_messages(limit, offset);
            }

        let get_messages = messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::types.lt(10))
            .limit(limit)
            .offset(offset)
            .order(schema::messages::created.desc())
            .load::<Message>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in get_messages.iter() {
            if message_options
                .filter(schema::message_options::user_id.eq(user_id))
                .filter(schema::message_options::message_id.eq(_item.id))
                .filter(schema::message_options::is_deleted.eq(true))
                .load::<MessageOption>(&_connection)
                .expect("E")
                .len() == 0 {
                    stack.push(_item.id);
                }

        };
        return messages
            .filter(schema::messages::id.eq_any(stack))
            .filter(schema::messages::types.lt(10))
            .limit(limit)
            .offset(offset)
            .order(schema::messages::created.desc())
            .load::<Message>(&_connection)
            .expect("E");
    }

    pub fn get_can_add_in_chat_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_add_in_chat.eq("b"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_add_in_chat_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_add_in_chat.eq("a"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_add_in_chat_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_add_in_chat_exclude_users_ids());
    }
    pub fn get_can_add_in_chat_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_add_in_chat_include_users_ids());
    }

    pub fn get_can_add_fix_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_add_fix.eq("b"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_add_fix_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_add_fix.eq("a"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_add_fix_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_add_fix_exclude_users_ids());
    }
    pub fn get_can_add_fix_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_add_fix_include_users_ids());
    }

    pub fn get_can_send_mention_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_send_mention.eq("b"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_send_mention_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_send_mention.eq("a"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_send_mention_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_send_mention_exclude_users_ids());
    }
    pub fn get_can_send_mention_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_send_mention_include_users_ids());
    }

    pub fn get_can_add_admin_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_add_admin.eq("b"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_add_admin_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_add_admin.eq("a"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_add_admin_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_add_admin_exclude_users_ids());
    }
    pub fn get_can_add_admin_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_add_admin_include_users_ids());
    }

    pub fn get_can_add_design_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_add_design.eq("b"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_add_design_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_add_design.eq("a"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_add_design_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_add_design_exclude_users_ids());
    }
    pub fn get_can_add_design_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_add_design_include_users_ids());
    }

    pub fn get_can_see_settings_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_see_settings.eq("b"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_see_settings_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_see_settings.eq("a"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_see_settings_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_settings_exclude_users_ids());
    }
    pub fn get_can_see_settings_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_settings_include_users_ids());
    }

    pub fn get_can_see_log_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_see_log.eq("b"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_see_log_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;

        let _connection = establish_connection();
        let items = chat_ie_settings
            .filter(schema::chat_ie_settings::chat_user_id.eq_any(self.get_members_ids()))
            .filter(schema::chat_ie_settings::can_see_log.eq("a"))
            .load::<ChatIeSetting>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.chat_user_id);
        };
        return stack;
    }
    pub fn get_can_see_log_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_log_exclude_users_ids());
    }
    pub fn get_can_see_log_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_log_include_users_ids());
    }

    pub fn is_user_can_add_in_chat(&self, user_id: i32) -> bool {
        let char = &self.can_add_members;
        return match char.as_str() {
            "a" => self.get_members_ids().iter().any(|&i| i==user_id),
            "b" => self.user_id == user_id,
            "c" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "d" => !self.get_can_add_in_chat_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_add_in_chat_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_add_fix(&self, user_id: i32) -> bool {
        let char = &self.can_fix_item;
        return match char.as_str() {
            "a" => self.get_members_ids().iter().any(|&i| i==user_id),
            "b" => self.user_id == user_id,
            "c" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "d" => !self.get_can_add_fix_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_add_fix_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_send_mention(&self, user_id: i32) -> bool {
        let char = &self.can_mention;
        return match char.as_str() {
            "a" => self.get_members_ids().iter().any(|&i| i==user_id),
            "b" => self.user_id == user_id,
            "c" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "d" => !self.get_can_send_mention_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_send_mention_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_add_admin(&self, user_id: i32) -> bool {
        let char = &self.can_add_admin;
        return match char.as_str() {
            "a" => self.get_members_ids().iter().any(|&i| i==user_id),
            "b" => self.user_id == user_id,
            "c" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "d" => !self.get_can_add_admin_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_add_admin_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_add_design(&self, user_id: i32) -> bool {
        let char = &self.can_add_design;
        return match char.as_str() {
            "a" => self.get_members_ids().iter().any(|&i| i==user_id),
            "b" => self.user_id == user_id,
            "c" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "d" => !self.get_can_add_design_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_add_design_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_settings(&self, user_id: i32) -> bool {
        let char = &self.can_see_settings;
        return match char.as_str() {
            "a" => self.get_members_ids().iter().any(|&i| i==user_id),
            "b" => self.user_id == user_id,
            "c" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "d" => !self.get_can_see_settings_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_settings_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_log(&self, user_id: i32) -> bool {
        let char = &self.can_see_log;
        return match char.as_str() {
            "a" => self.get_members_ids().iter().any(|&i| i==user_id),
            "b" => self.user_id == user_id,
            "c" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "d" => !self.get_can_see_log_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_log_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn set_friends_visible_perms(&self, action: String, users_ids: Vec<i32>, types: String) -> bool {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;
        use crate::schema::chat_users::dsl::chat_users;

        let _connection = establish_connection();

        let _members = chat_users
            .filter(schema::chat_users::user_id.eq_any(&users_ids))
            .load::<ChatUser>(&_connection)
            .expect("E");
        let mut members_stack = Vec::new();
        for _item in _members.iter() {
            members_stack.push(_item.user_id);
        };
        diesel::delete(chat_ie_settings.filter(schema::chat_ie_settings::chat_user_id.eq_any(members_stack))).execute(&_connection).expect("E");

        if types == "can_add_in_chat".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewChatIeSetting {
                    chat_user_id:    *user_id,
                    can_add_in_chat:  Some(action.clone()),
                    can_add_fix:      None,
                    can_send_mention: None,
                    can_add_admin:    None,
                    can_add_design:   None,
                    can_see_settings: None,
                    can_see_log:      None,
                };
                diesel::insert_into(schema::chat_ie_settings::table)
                    .values(&_new_perm)
                    .get_result::<ChatIeSetting>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_add_fix".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewChatIeSetting {
                    chat_user_id:    *user_id,
                    can_add_in_chat:  None,
                    can_add_fix:      Some(action.clone()),
                    can_send_mention: None,
                    can_add_admin:    None,
                    can_add_design:   None,
                    can_see_settings: None,
                    can_see_log:      None,
                };
                diesel::insert_into(schema::chat_ie_settings::table)
                    .values(&_new_perm)
                    .get_result::<ChatIeSetting>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_send_mention".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewChatIeSetting {
                    chat_user_id:    *user_id,
                    can_add_in_chat:  None,
                    can_add_fix:      None,
                    can_send_mention: Some(action.clone()),
                    can_add_admin:    None,
                    can_add_design:   None,
                    can_see_settings: None,
                    can_see_log:      None,
                };
                diesel::insert_into(schema::chat_ie_settings::table)
                    .values(&_new_perm)
                    .get_result::<ChatIeSetting>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_add_admin".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewChatIeSetting {
                    chat_user_id:    *user_id,
                    can_add_in_chat:  None,
                    can_add_fix:      None,
                    can_send_mention: None,
                    can_add_admin:    Some(action.clone()),
                    can_add_design:   None,
                    can_see_settings: None,
                    can_see_log:      None,
                };
                diesel::insert_into(schema::chat_ie_settings::table)
                    .values(&_new_perm)
                    .get_result::<ChatIeSetting>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_add_design".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewChatIeSetting {
                    chat_user_id:    *user_id,
                    can_add_in_chat:  None,
                    can_add_fix:      None,
                    can_send_mention: None,
                    can_add_admin:    None,
                    can_add_design:   Some(action.clone()),
                    can_see_settings: None,
                    can_see_log:      None,
                };
                diesel::insert_into(schema::chat_ie_settings::table)
                    .values(&_new_perm)
                    .get_result::<ChatIeSetting>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_settings".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewChatIeSetting {
                    chat_user_id:    *user_id,
                    can_add_in_chat:  None,
                    can_add_fix:      None,
                    can_send_mention: None,
                    can_add_admin:    None,
                    can_add_design:   None,
                    can_see_settings: Some(action.clone()),
                    can_see_log:      None,
                };
                diesel::insert_into(schema::chat_ie_settings::table)
                    .values(&_new_perm)
                    .get_result::<ChatIeSetting>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_log".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewChatIeSetting {
                    chat_user_id:    *user_id,
                    can_add_in_chat:  None,
                    can_add_fix:      None,
                    can_send_mention: None,
                    can_add_admin:    None,
                    can_add_design:   None,
                    can_see_settings: None,
                    can_see_log:      Some(action.clone()),
                };
                diesel::insert_into(schema::chat_ie_settings::table)
                    .values(&_new_perm)
                    .get_result::<ChatIeSetting>(&_connection)
                    .expect("Error.");
            }
        }
        return true;
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
#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="chat_users"]
pub struct NewChatUser {
    pub user_id:          i32,
    pub chat_id:          i32,
    pub types:            String,
    pub is_administrator: bool,
    pub created:          chrono::NaiveDateTime,
    pub no_disturb:       Option<chrono::NaiveDateTime>,
}
impl ChatUser {
    pub fn beep(&self) -> bool {
        if self.no_disturb.is_some() {
            return self.no_disturb.as_ref().unwrap() > &chrono::Local::now().naive_utc();
        }
        else {
            return true;
        }
    }
    pub fn get_beep_icon(&self) -> String {
        if self.beep() {
            return "".to_string();
        }
        else {
            return " <svg style='width: 15px;' enable-background='new 0 0 24 24' height='15px' viewBox='0 0 24 24' width='17px' fill='currentColor'><path d='M0 0h24v24H0V0z' fill='none'/><path d='M4.34 2.93L2.93 4.34 7.29 8.7 7 9H3v6h4l5 5v-6.59l4.18 4.18c-.65.49-1.38.88-2.18 1.11v2.06c1.34-.3 2.57-.92 3.61-1.75l2.05 2.05 1.41-1.41L4.34 2.93zM10 15.17L7.83 13H5v-2h2.83l.88-.88L10 11.41v3.76zM19 12c0 .82-.15 1.61-.41 2.34l1.53 1.53c.56-1.17.88-2.48.88-3.87 0-4.28-2.99-7.86-7-8.77v2.06c2.89.86 5 3.54 5 6.71zm-7-8l-1.88 1.88L12 7.76zm4.5 8c0-1.77-1.02-3.29-2.5-4.03v1.79l2.48 2.48c.01-.08.02-.16.02-.24z'/></svg>".to_string();
        }
    }
}

/////// ChatPerm //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(ChatUser)]
pub struct ChatIeSetting {
    pub id:               i32,
    pub chat_user_id:     i32,
    pub can_add_in_chat:  Option<String>,
    pub can_add_fix:      Option<String>,
    pub can_send_mention: Option<String>,
    pub can_add_admin:    Option<String>,
    pub can_add_design:   Option<String>,
    pub can_see_settings: Option<String>,
    pub can_see_log:      Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="chat_ie_settings"]
pub struct NewChatIeSetting {
    pub chat_user_id:     i32,
    pub can_add_in_chat:  Option<String>,
    pub can_add_fix:      Option<String>,
    pub can_send_mention: Option<String>,
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
#[belongs_to(User)]
#[belongs_to(Chat)]
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
    pub post_id:       Option<i32>,
    pub created:       chrono::NaiveDateTime,
    pub content:       Option<String>,
    pub types:         i16,
    pub attach:        Option<String>,
    pub voice:         Option<String>,
}
impl Message {
    pub fn get_attach(&self, user_id: i32) -> String {
        if self.attach.is_some() {
            use crate::utils::message_elements;
            return message_elements(self.attach.as_ref().unwrap().to_string(), user_id);
        }
        else {
            return "".to_string();
        }
    }
    pub fn get_anon_attach(&self) -> String {
        if self.attach.is_some() {
            use crate::utils::anon_message_elements;
            return anon_message_elements(self.attach.as_ref().unwrap().to_string());
        }
        else {
            return "".to_string();
        }
    }
    pub fn is_have_transfer(&self) -> bool {
        use crate::schema::message_transfers::dsl::message_transfers;

        let _connection = establish_connection();
        return message_transfers
            .filter(schema::message_transfers::message_id.eq(self.id))
            .load::<MessageTransfer>(&_connection)
            .expect("E").len() > 0;
    }
    pub fn get_transfers(&self) -> Vec<MessageTransfer> {
        use crate::schema::message_transfers::dsl::message_transfers;

        let _connection = establish_connection();
        return message_transfers
            .filter(schema::message_transfers::message_id.eq(self.id))
            .load::<MessageTransfer>(&_connection)
            .expect("E");
    }
    pub fn get_draft_transfers_block(&self) -> String {
        use crate::utils::get_count_for_ru;

        let _connection = establish_connection();
        let transfers = self.get_transfers();
        let count = transfers.len();
        let text = get_count_for_ru(
            count.try_into().unwrap(),
            " сообщение".to_string(),
            " сообщения".to_string(),
            " сообщений".to_string(),
        );
        let mut text_2 = "".to_string();
        if count > 1 {
            text_2 = "Пересланные сообщения".to_string();
        }
        else {
            text_2 = "Пересланное сообщение".to_string();
        }
        let mut inputs = "".to_string();
        for i in transfers.iter() {
            inputs += &("<input type='hidden' name='transfer' value='".to_owned() + &i.id.to_string() + &"' class='transfer'>");
        }
        return "<div><p>".to_owned() + &text_2 + &"</p><div style='position:relative;padding-bottom:7px'><div><span class='pointer underline'>" + &text + &"</span><span class='remove_parent_block pointer message_form_parent_block'>x</span></div></div>" + &inputs + &"</div>";
    }
    pub fn is_edited(&self) -> bool {
        return self.types == 2 && self.types == 8;
    }
    pub fn is_manager(&self) -> bool {
        return self.types == 6;
    }
    pub fn is_fixed(&self) -> bool {
        return self.types == 7 && self.types == 8;
    }
    pub fn is_favourite(&self, user_id:i32) -> bool {
        use crate::schema::message_options::dsl::message_options;

        let _connection = establish_connection();
        return message_options
            .filter(schema::message_options::message_id.eq(self.id))
            .filter(schema::message_options::user_id.eq(user_id))
            .filter(schema::message_options::is_favourite.eq(true))
            .load::<MessageOption>(&_connection)
            .expect("E")
            .len() > 0;
    }
    pub fn get_count_attach(&self) -> usize {
        if self.attach.is_some() {
            let self_attach = self.attach.as_deref().unwrap().split(",").collect::<Vec<_>>();
            return self_attach.len();
        }
        return 0;
    }
    pub fn get_parent_message(&self) -> String {
        use crate::schema::messages::dsl::messages;
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();

        if !self.parent_id.is_some() {
            return "<div class='media p-1 pag'>Нет ответа!</div>".to_string();
        }
        let parent = messages
            .filter(schema::messages::id.eq(self.parent_id.unwrap()))
            .load::<Message>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
        let mut _preview = "".to_string();
        if parent.voice.is_some() {
            _preview = "Голосовое сообщение".to_string();
        }
        else if parent.sticker_id.is_some() {
            _preview = "Наклейка".to_string();
        }
        else if parent.attach.is_some() {
            _preview = "Вложения".to_string();
        }
        else {
            _preview = parent.content.as_deref().unwrap()[..80].to_string();
        }
        let creator = users
            .filter(schema::users::id.eq(parent.user_id))
            .load::<User>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();

        return "<div class='media p-1' data-pk=".to_owned() +
            &parent.id.to_string() +
            &"style='border-left: 1px solid rgba(0, 0, 0, 0.7)'><span style='padding-top: 6px;'><a href='" +
            &creator.get_link() +
            &"' class='ajax'>" +
            &"' class='ajax'>";
    }
    pub fn get_creator(&self) -> User {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::id.eq(self.user_id))
            .filter(schema::users::types.lt(10))
            .load::<User>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_parent(&self) -> Message {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        return messages
            .filter(schema::messages::id.eq(self.parent_id.unwrap()))
            .load::<Message>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn get_type_text(&self) -> String {
        if self.attach.is_some() && self.content.is_some() {
            return "<b class='i_link'>Текст и вложения</b>".to_string();
        }
        else if self.attach.is_some() {
            return "<b class='i_link'>Вложения</b>".to_string();
        }
        else if self.content.is_some() {
            return self.get_text_60();
        }
        else if self.voice.is_some() {
            return "<b class='i_link'>Голосовое сообщение</b>".to_string();
        }
        else if self.sticker_id.is_some() {
            return "<b class='i_link'>Наклейка</b>".to_string();
        }
        else if self.post_id.is_some() {
            return "<b class='i_link'>Репост</b>".to_string();
        }
        else if self.parent_id.is_some() {
            if self.is_manager() {
                return concat_string!(
                    "<b class='i_link'>",
                    self.get_creator().get_full_name(),
                    self.content.as_deref().unwrap(),
                    "<span class='underline'>",
                    self.get_parent().get_text_60(),
                    "</span></b>"
                );
            }
            else {
                return "<b class='i_link'>Ответ на сообщение</b>".to_string();
            }
        }
        else if self.is_have_transfer() {
            if self.get_transfers().len() > 1 {
                return "<b class='i_link'>Пересланные сообщения</b>".to_string();
            }
            else {
                return "<b class='i_link'>Пересланное сообщения</b>".to_string();
            }
        }
        else {
            return "Нет текста!".to_string()
        }
    }
    pub fn get_text_60(&self) -> String {
        if self.content.is_some() {
            use lazy_static::lazy_static;
            use regex::Regex;

            lazy_static! {
                static ref RE_IMG: Regex = Regex::new(r"<img.*?>").unwrap();
                static ref RE_A: Regex = Regex::new(r"<a.*?>").unwrap();
            }
            let text = self.content.as_deref().unwrap();
            let mut count = 60;
            let mut link_text: Option<String> = None;

            let images = RE_IMG.find_iter(text).collect::<Vec<_>>();
            for image in images.iter() {
                count += image.as_str().len();
            }

            let links = RE_A.find_iter(text).collect::<Vec<_>>();
            if links.len() > 0 {
                return "<b class='i_link'>".to_string() + &links[0].as_str() + &"</b>".to_string();
            }
            return text[count..].to_string();
        } else {
            return "".to_string();
        }
    }
}

/////// MessageOptions //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Message)]
pub struct MessageOption {
    pub id:            i32,
    pub message_id:    i32,
    pub user_id:       i32,
    pub is_deleted:    bool,
    pub is_favourite:  bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="message_options"]
pub struct NewMessageOption {
    pub message_id:    i32,
    pub user_id:       i32,
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
