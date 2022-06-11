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
    message_reactions,
    message_votes,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::{establish_connection,JsonItemReactions};
use crate::models::{
    User,
    Community,
    Post,
    Sticker,
    //Photo,
    //Video,
    Reaction,

};
use actix_web::web::Json;

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

#[derive(Debug, PartialEq, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Community)]
pub struct Chat {
    pub id:               i32,
    pub name:             Option<String>,
    pub types:            i16,
    pub image:            Option<String>,
    pub description:      Option<String>,
    pub community_id:     Option<i32>,
    pub user_id:          i32,
    pub position:         i16,
    pub members:          i32,
    pub created:          chrono::NaiveDateTime,
    pub can_add_members:  String,
    pub can_fix_item:     String,
    pub can_mention:      String,
    pub can_add_admin:    String,
    pub can_add_design:   String,
    pub can_see_settings: String,
    pub can_see_log:      String,
    pub reactions:        Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name="chats"]
pub struct NewChat {
    pub name:             Option<String>,
    pub types:            i16,
    pub community_id:     Option<i32>,
    pub user_id:          i32,
    pub position:         i16,
    pub members:          i32,
    pub created:          chrono::NaiveDateTime,
    pub can_add_members:  String,
    pub can_fix_item:     String,
    pub can_mention:      String,
    pub can_add_admin:    String,
    pub can_add_design:   String,
    pub can_see_settings: String,
    pub can_see_log:      String,
    pub reactions:        Option<String>,
}
#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="chats"]
pub struct EditChat {
    pub name:        Option<String>,
    pub image:       Option<String>,
    pub description: Option<String>,
    pub reactions:   Option<String>,
}
impl Chat {
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

    // 41 удаленная техподдержка 1 уровня
    // 42 удаленная техподдержка 2 уровня
    // 43 удаленная техподдержка 3 уровня
    // 44 удаленная техподдержка 4 уровня
    // 45 удаленная техподдержка 5 уровня
    pub fn delete_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            1 => 21,
            2 => 22,
            3 => 23,
            4 => 24,
            11 => 41,
            12 => 42,
            13 => 43,
            14 => 44,
            15 => 45,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::chats::types.eq(close_case))
            .get_result::<Chat>(&_connection)
            .expect("E");
       return true;
    }
    pub fn restore_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            21 => 1,
            22 => 2,
            23 => 3,
            24 => 4,
            41 => 11,
            42 => 12,
            43 => 13,
            44 => 14,
            45 => 15,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::chats::types.eq(close_case))
            .get_result::<Chat>(&_connection)
            .expect("E");
       return true;
    }

    pub fn close_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            1 => 31,
            2 => 32,
            3 => 33,
            4 => 34,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::chats::types.eq(close_case))
            .get_result::<Chat>(&_connection)
            .expect("E");
       return true;
    }
    pub fn unclose_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            31 => 1,
            32 => 2,
            33 => 3,
            34 => 4,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::chats::types.eq(close_case))
            .get_result::<Chat>(&_connection)
            .expect("E");
       return true;
    }
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn create_group_chat(creator: &User, name: Option<String>,
        community_id: Option<i32>, types: i16,
        users_ids: Option<String>) -> Chat {

        let _connection = establish_connection();
        let new_chat_form = NewChat {
            name: name,
            types: types,
            community_id: community_id,
            user_id: creator.id,
            position: 0,
            members: 0,
            created: chrono::Local::now().naive_utc(),
            can_add_members: "a".to_string(),
            can_fix_item: "c".to_string(),
            can_mention: "a".to_string(),
            can_add_admin: "b".to_string(),
            can_add_design: "b".to_string(),
            can_see_settings: "b".to_string(),
            can_see_log: "b".to_string(),
            reactions: None,
        };
        let new_chat = diesel::insert_into(schema::chats::table)
            .values(&new_chat_form)
            .get_result::<Chat>(&_connection)
            .expect("Error.");

        new_chat.create_membership(&creator, true);
        let _messages = new_chat.invite_users_in_chat(&creator, users_ids);
        return new_chat;
    }
    pub fn create_private_chat(creator: &User, recipient: &User, community_id: Option<i32>) -> Chat {
        let _connection = establish_connection();
        let new_chat_form = NewChat {
            name: None,
            types: 2,
            community_id: community_id,
            user_id: creator.id,
            position: 0,
            members: 0,
            created: chrono::Local::now().naive_utc(),
            can_add_members: "a".to_string(),
            can_fix_item: "c".to_string(),
            can_mention: "a".to_string(),
            can_add_admin: "b".to_string(),
            can_add_design: "b".to_string(),
            can_see_settings: "b".to_string(),
            can_see_log: "b".to_string(),
            reactions: None,
        };
        let new_chat = diesel::insert_into(schema::chats::table)
            .values(&new_chat_form)
            .get_result::<Chat>(&_connection)
            .expect("Error.");

        new_chat.create_membership(&creator, false);
        new_chat.create_membership(&recipient, false);
        return new_chat;
    }
    pub fn edit_chat(&self, name: Option<String>, image: Option<String>,
        description: Option<String>, reactions: Option<String>) -> &Chat {
        let _connection = establish_connection();

        let edit_chat_form = EditChat {
            name: name,
            image: image,
            description: description,
            reactions: reactions,
        };
        diesel::update(self)
            .set(edit_chat_form)
            .get_result::<Chat>(&_connection)
            .expect("Error.");
        return self;
    }
    pub fn invite_users_in_chat(&self, creator: &User, users_ids: Option<String>) ->
        Vec<Message> {
        let _connection = establish_connection();
        let mut info_messages: Vec<Message> = Vec::new();

        if users_ids.is_some() {
            use crate::schema::users::dsl::users;
            use crate::schema::chat_users::dsl::chat_users;

            let mut stack = Vec::new();
            let unwrap_users_ids = users_ids.as_ref().unwrap().to_string();
            if !unwrap_users_ids.is_empty() {
                let v: Vec<&str> = unwrap_users_ids.split(", ").collect();
                for item in v.iter() {
                    if !item.is_empty() {
                        let pk: i32 = item.parse().unwrap();
                        stack.push(pk);
                    }
                }
            }

            let mut m_word = "пригласил".to_string();
            if creator.gender == "b".to_string() {
                m_word = "пригласила".to_string();
            }

            let users_list = users
                .filter(schema::users::id.eq_any(stack))
                .filter(schema::users::types.lt(10))
                .load::<User>(&_connection)
                .expect("E.");
            for user in users_list.iter() {
                if chat_users
                    .filter(schema::chat_users::user_id.eq(user.id))
                    .filter(schema::chat_users::chat_id.eq(self.id))
                    .filter(schema::chat_users::types.ne("c"))
                    .load::<ChatUser>(&_connection)
                    .expect("E.")
                    .len() == 0 {

                    self.create_membership(user, false);
                    let text = concat_string!(
                        "<a target='_blank' href='",
                        creator.link, "'>",
                        creator.get_full_name(),
                        "</a>&nbsp;", m_word,
                        "&nbsp; пользователя&nbsp;<a target='_blank' href='",
                        user.link, "'>",
                        user.get_full_name(), "</a>");

                    let new_message_form = NewMessage {
                        user_id:    creator.id,
                        chat_id:    self.id,
                        parent_id:  None,
                        sticker_id: None,
                        post_id:    None,
                        created:    chrono::Local::now().naive_utc(),
                        content:    Some(text),
                        unread:     true,
                        types:      6,
                        attach:     None,
                        voice:      None,
                        reactions:  0,
                    };
                    let new_message = diesel::insert_into(schema::messages::table)
                        .values(&new_message_form)
                        .get_result::<Message>(&_connection)
                        .expect("Error.");
                    for _recipient in self.get_recipients_2(creator.id).iter() {
                        println!("Socket!!");
                    }
                    info_messages.push(new_message);
                }
            }
        }
        return info_messages;
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
    pub fn get_reactions_list(&self) -> Vec<i16> {
        let mut stack = Vec::new();
        if self.reactions.is_some() {
            let react_scring = self.reactions.as_ref().unwrap().to_string();
            if !react_scring.is_empty() {
                let v: Vec<&str> = react_scring.split(", ").collect();
                for item in v.iter() {
                    if !item.is_empty() {
                        let pk: i16 = item.parse().unwrap();
                        stack.push(pk);
                    }
                }
            }
        }
        return stack;
    }
    pub fn count_reactions_list(&self) -> usize {
        return self.get_reactions_list().len();
    }
    pub fn count_reactions_list_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt(
            self.count_reactions_list().try_into().unwrap(),
            " реакция".to_string(),
            " реакции".to_string(),
            " реакций".to_string(),
        );
    }
    pub fn liked_manager(&self, user_id: i32) -> bool {
        use crate::schema::support_user_votes::dsl::support_user_votes;
        use crate::models::SupportUserVote;

        let _connection = establish_connection();
        return support_user_votes
            .filter(schema::support_user_votes::user_id.eq(user_id))
            .filter(schema::support_user_votes::manager_id.eq(self.get_chat_member(user_id).id))
            .filter(schema::support_user_votes::vote.eq(1))
            .load::<SupportUserVote>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn disliked_manager(&self, user_id: i32) -> bool {
        use crate::schema::support_user_votes::dsl::support_user_votes;
        use crate::models::SupportUserVote;

        let _connection = establish_connection();
        return support_user_votes
            .filter(schema::support_user_votes::user_id.eq(user_id))
            .filter(schema::support_user_votes::manager_id.eq(self.get_chat_member(user_id).id))
            .filter(schema::support_user_votes::vote.eq(-1))
            .load::<SupportUserVote>(&_connection)
            .expect("E.")
            .len() > 0;
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
        return self.types > 10 && self.types < 20;
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
    pub fn get_members(&self, limit: i64, offset: i64) -> Vec<User> {
        use crate::schema::chat_users::dsl::chat_users;
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let items = chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::types.eq("a"))
            .limit(limit)
            .offset(offset)
            .load::<ChatUser>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return users
            .filter(schema::users::id.eq_any(stack))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn is_muted(&self, user_id: i32) -> bool {
        use crate::schema::chat_users::dsl::chat_users;

        let _connection = establish_connection();
        let users = chat_users
            .filter(schema::chat_users::chat_id.eq(self.id))
            .filter(schema::chat_users::user_id.ne(user_id))
            .filter(schema::chat_users::types.eq("a"))
            .load::<ChatUser>(&_connection)
            .expect("E");
        if users.len() > 0 {
            let user = users.into_iter().nth(0).unwrap();
            return user.beep();
        }
        return false;
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
    pub fn get_draft_message(&self, user_id: i32) -> Option<Message> {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        return messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::user_id.eq(user_id))
            .filter(schema::messages::types.eq(10))
            .load::<Message>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0);
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
    pub fn get_fixed_messages(&self, limit: i64, offset: i64) -> Vec<Message> {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        return messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::types.eq_any(vec![7,8]))
            .limit(limit)
            .offset(offset)
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

    pub fn get_first_fix_message(&self) -> Option<Message> {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        return messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::types.eq_any(vec![7,8]))
            .load::<Message>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0);
    }

    pub fn create_membership(&self, user: &User, is_administrator: bool) -> ChatUser {
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
    pub fn read_messages(&self, user_id: &i32 ) -> bool {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        let unread_messages = messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::unread.eq(true))
            .filter(schema::messages::user_id.ne(user_id))
            .load::<Message>(&_connection)
            .expect("E");

        for message in unread_messages.iter() {
            diesel::update(message)
                .set(schema::messages::unread.eq(false))
                .get_result::<Message>(&_connection)
                .expect("Error.");
        }
        return true;
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

    pub fn get_header_chat(&self, user_id: i32 ) -> String {
        let mut beep_icon = "".to_string();
        let mut muted_drop = "".to_string();
        let mut fix_btn = "".to_string();

        let mut chat_name = "".to_string();
        let mut dop_drops = "".to_string();
        let mut target_display = "".to_string();
        let mut u_chat_info = "".to_string();
        let member = Some(self.get_chat_request_user(user_id));
        if member.is_some() {
            beep_icon = member.unwrap().get_beep_icon();
            if self.is_muted(user_id) {
                muted_drop = "<span><a class='dropdown-item off_full_chat_notify pointer'>Откл. уведомления</a></span>".to_string();
            }
            else {
                muted_drop = "<span><a class='dropdown-item on_full_chat_notify pointer'>Вкл. уведомления</a></span>".to_string();
            }
        }
        if self.is_user_can_add_fix(user_id) {
            fix_btn = "<span tooltip='Закрепить' flow='up'><svg class='svg_default_30 mr-1 pointer u_message_fixed' fill='currentColor' viewBox='0 0 24 24'><g><rect fill='none' height='24' width='24'/></g><g><path d='M16,9V4l1,0c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H7C6.45,2,6,2.45,6,3v0 c0,0.55,0.45,1,1,1l1,0v5c0,1.66-1.34,3-3,3h0v2h5.97v7l1,1l1-1v-7H19v-2h0C17.34,12,16,10.66,16,9z' fill-rule='evenodd'/></g></svg></span>".to_string();
        }
        let buttons = "<span class='console_btn_other btn_default' style='display:none;padding-top:5px'><span class='one_message'><span tooltip='Ответить' flow='up'><svg class='svg_default_30 mr-1 pointer u_message_reply' viewBox='0 0 24 24' fill='currentColor'><path d='M0 0h24v24H0V0z' fill='none'/><path d='M10 9V5l-7 7 7 7v-4.1c5 0 8.5 1.6 11 5.1-1-5-4-10-11-11z'/></svg></span><span tooltip='Пожаловаться' flow='up'><svg class='svg_default_30 mr-1 pointer u_message_claim' viewBox='0 0 24 24' fill='currentColor'><path d='M11 15h2v2h-2v-2zm0-8h2v6h-2V7zm.99-5C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z'/></svg></span><span tooltip='Редактировать' flow='up'><svg class='svg_default_30 mr-1 pointer u_message_edit' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z'/></svg></span>".to_string() + &fix_btn + &"</span><span tooltip='Удалить' flow='up'><svg class='svg_default_30 mr-1 pointer u_message_delete' viewBox='0 0 24 24' fill='currentColor'><path d='M0 0h24v24H0V0z' fill='none'/><path d='M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM8 9h8v10H8V9zm7.5-5l-1-1h-5l-1 1H5v2h14V4h-3.5z'/></svg></span><span tooltip='Переслать' flow='up'><svg class='svg_default_30 pointer u_message_transfer' viewBox='0 0 24 24' fill='currentColor'><path d='m0 0h24v24h-24z' fill='none'/><path fill='currentColor' d='m12.1 7.87v-3.47a1.32 1.32 0 0 1 2.17-1l8.94 7.6a1.32 1.32 0 0 1 .15 1.86l-.15.15-8.94 7.6a1.32 1.32 0 0 1 -2.17-1v-3.45c-4.68.11-8 1.09-9.89 2.87a1.15 1.15 0 0 1 -1.9-1.11c1.53-6.36 5.51-9.76 11.79-10.05zm1.8-2.42v4.2h-.9c-5.3 0-8.72 2.25-10.39 6.86 2.45-1.45 5.92-2.16 10.39-2.16h.9v4.2l7.71-6.55z' /></svg></span><span flow='up'><svg class='svg_default_30 ml-1 toggle_messages_favourite pointer' fill='currentColor' enable-background='new 0 0 24 24' viewBox='0 0 24 24'></svg></span></span>".to_string();

        if self.is_public() {
            chat_name = "Групповой чат".to_string();
            target_display = "<span class='u_chat_info pointer type_display small' style='position:absolute;top: 21px;'>".to_owned() + &self.get_members_count_ru() + &"</span>".to_string();
            u_chat_info = "u_chat_info".to_string();
            if self.is_user_can_add_in_chat(user_id) {
                dop_drops = dop_drops + &"<a class='dropdown-item u_add_members_in_chat pointer'>Добавить друзей</a>".to_string();
            }
            dop_drops = dop_drops + &"<a class='dropdown-item user_exit_in_user_chat pointer'>Выйти из чата</a>".to_string();
        }
        else if self.is_group() {
            chat_name = "Публичный чат".to_string();
            target_display = "<span class='u_chat_info pointer type_display small' style='position:absolute;top: 21px;'></span>".to_string();
            dop_drops = "<a class='dropdown-item add_member_in_chat pointer'>Добавить в чат</a>".to_string();
        }
        else if self.is_private() {
            let member = self.get_chat_member(user_id);
            chat_name = "<a href='".to_owned() + &member.link.to_string() + &"' target='_blank'>".to_string() + &member.get_full_name() + &"</a>".to_string();
            target_display = "<span class='u_chat_info pointer type_display small' style='position:absolute;top: 21px;'>".to_owned() + &self.get_members_count_ru() + &"</span>".to_string();
            dop_drops = "<a class='dropdown-item add_member_in_chat pointer'>Добавить в чат</a>".to_string();
        }
        else if self.is_manager() {
            chat_name = "Служебный чат".to_string();
            target_display = "<span class='type_display small style='position:absolute;top: 21px;'>Категория такая-то</span>".to_string();
        }
        else if self.is_support() {
            dop_drops = "<a class='dropdown-item close_support_chat pointer'>Закрыть заявку</a>".to_string();
            if self.members == 1 {
                chat_name = "Чат техподдержки".to_string();
            }
            else {
                 use crate::schema::support_users::dsl::support_users;
                 use crate::models::SupportUser;

                 let _connection = establish_connection();
                 for user in self.get_members(10, 0) {
                     if user.id != self.user_id {
                        let supports = support_users
                             .filter(schema::support_users::manager_id.eq(user.id))
                             .load::<SupportUser>(&_connection)
                             .expect("E");
                        if supports.len() > 0 {
                            chat_name = "Агент техподдержки №".to_string() + &supports[0].id.to_string();
                            target_display = "<span class='type_display small' style='position:absolute;top: 21px;'>".to_owned() + &user.get_online_status() + &"</span>".to_string();
                        }
                     }
                 }
            }
        }
        let media_body = concat_string!(
            "<div class='media-body' style='overflow: inherit;padding-top: 3px;'><h5 class='time-title mb-1'><span class='",
            u_chat_info, "pointer'>", chat_name,
            "</span><span class='notify_box'>", beep_icon,
            "</h5><span class='mt-1 mb-2 target_display'>",
            target_display, buttons,
            "</span></div>"
        );
        let dropdown = concat_string!(
            "<div class='dropdown d-inline-block'><a style='cursor:pointer' class='icon-circle icon-30 btn_default drop'><svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z'/></svg></a><div class='dropdown-menu dropdown-menu-right' style='top: 29px; width: 100%;'><a class='dropdown-item chat_search pointer'>Поиск сообщений</a><a class='dropdown-item show_attach_files pointer'>Показать вложения</a>",
            muted_drop, dop_drops,
            "<a class='dropdown-item u_clean_chat_messages pointer'>Очистить историю</a></div></div>"
        );
        return concat_string!(media_body, dropdown);
    }

    pub fn get_preview_message(&self, user_id: i32 ) -> String {
        let first_message = self.get_first_message(user_id);
        let preview_text: String;
        let mut is_read = "".to_string();
        //let mut creator_figure: String;
        let mut created = "".to_string();
        let mut beep_icon = "".to_string();

        if self.is_have_draft_message_content(user_id) {
            let message = self.get_draft_message(user_id).unwrap();
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
            //preview_text = first_message.get_text_60();
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
            //let member = self.get_chat_member(user_id);
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
                 for user in self.get_members(10, 0) {
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
    pub fn get_search_list(&self, q: &str, limit: i64, offset: i64) -> Vec<Message> {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        let _q_standalone = "%".to_owned() + q + &"%".to_string();

        return messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::types.lt(10))
            .filter(schema::messages::content.ilike(&_q_standalone))
            .order(schema::messages::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Message>(&_connection)
            .expect("E");
    }
    pub fn count_search_list(&self, q: &str) -> usize {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        let _q_standalone = "%".to_owned() + q + &"%".to_string();

        return messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::types.lt(10))
            .filter(schema::messages::content.ilike(&_q_standalone))
            .order(schema::messages::created.desc())
            .load::<Message>(&_connection)
            .expect("E")
            .len();
    }
    pub fn count_messages_for_user(&self, user_id: i32) -> usize {
        use crate::schema::messages::dsl::messages;
        use crate::schema::message_options::dsl::message_options;

        let _connection = establish_connection();
        if message_options
            .filter(schema::message_options::user_id.eq(user_id))
            .filter(schema::message_options::is_deleted.eq(true))
            .load::<MessageOption>(&_connection)
            .expect("E")
            .len() == 0 {
                return self.get_messages(500, 0).len();
            }

        let get_messages = messages
            .filter(schema::messages::chat_id.eq(self.id))
            .filter(schema::messages::types.lt(10))
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
            .order(schema::messages::created.desc())
            .load::<Message>(&_connection)
            .expect("E")
            .len();
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
            "e" => self.get_can_add_in_chat_include_users_ids().iter().any(|&i| i==user_id),
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
            "e" => self.get_can_add_fix_include_users_ids().iter().any(|&i| i==user_id),
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
            "e" => self.get_can_send_mention_include_users_ids().iter().any(|&i| i==user_id),
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
            "e" => self.get_can_add_admin_include_users_ids().iter().any(|&i| i==user_id),
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
            "e" => self.get_can_add_design_include_users_ids().iter().any(|&i| i==user_id),
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
            "e" => self.get_can_see_settings_include_users_ids().iter().any(|&i| i==user_id),
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
            "e" => self.get_can_see_log_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn set_friends_visible_perms(&self, action: String, users: String, types: String) -> bool {
        use crate::schema::chat_ie_settings::dsl::chat_ie_settings;
        use crate::schema::chat_users::dsl::chat_users;

        let _connection = establish_connection();
        let mut users_ids = Vec::new();
        let v: Vec<&str> = users.split(", ").collect();
        for item in v.iter() {
            if !item.is_empty() {
                let pk: i32 = item.parse().unwrap();
                users_ids.push(pk);
            }
        }

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

// 'a' Все участники
// 'b' Создатель
// 'c' Создатель и админы
// 'd' Участники, кроме
// 'e' Некоторые участники

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
    pub id:         i32,
    pub user_id:    i32,
    pub chat_id:    i32,
    pub parent_id:  Option<i32>,
    pub sticker_id: Option<i32>,
    pub post_id:    Option<i32>,
    pub created:    chrono::NaiveDateTime,
    pub content:    Option<String>,
    pub unread:     bool,
    pub types:      i16,
    pub attach:     Option<String>,
    pub voice:      Option<String>,
    pub reactions:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="messages"]
pub struct NewMessage {
    pub user_id:    i32,
    pub chat_id:    i32,
    pub parent_id:  Option<i32>,
    pub sticker_id: Option<i32>,
    pub post_id:    Option<i32>,
    pub created:    chrono::NaiveDateTime,
    pub content:    Option<String>,
    pub unread:     bool,
    pub types:      i16,
    pub attach:     Option<String>,
    pub voice:      Option<String>,
    pub reactions:  i32,
}

impl Message {
    pub fn get_or_create_chat_and_send_message(&self, creator: User,
        user: &User, repost_id: Option<i32>, content: Option<String>,
        attach: Option<String>, voice: Option<String>,
        sticker_id: Option<i32>) -> bool {

        let _connection = establish_connection();
        let chat_list = creator.get_all_chats(200, 0);
        let mut chat_exists = false;
        for chat in chat_list.iter() {
            if user.is_member_of_chat(chat.id) {
                let message_form = NewMessage {
                    user_id:    creator.id,
                    chat_id:    chat.id,
                    parent_id:  None,
                    sticker_id: sticker_id,
                    post_id:    repost_id,
                    created:    chrono::Local::now().naive_utc(),
                    content:    content.clone(),
                    unread:     true,
                    types:      1,
                    attach:     attach.clone(),
                    voice:      voice.clone(),
                    reactions:  0,
                };
                diesel::insert_into(schema::messages::table)
                    .values(&message_form)
                    .get_result::<Message>(&_connection)
                    .expect("Error.");

                diesel::update(chat)
                    .set(schema::chats::created.eq(chrono::Local::now().naive_utc()))
                    .get_result::<Chat>(&_connection)
                    .expect("Error.");
                chat_exists = true;
            }
        }
        if chat_exists == false {
            let chat = Chat::create_private_chat(&creator, user, None);
            let message_form = NewMessage {
                user_id:    creator.id,
                chat_id:    chat.id,
                parent_id:  None,
                sticker_id: sticker_id,
                post_id:    repost_id,
                created:    chrono::Local::now().naive_utc(),
                content:    content.clone(),
                unread:     true,
                types:      1,
                attach:     attach.clone(),
                voice:      voice.clone(),
                reactions:  0,
            };
            diesel::insert_into(schema::messages::table)
                .values(&message_form)
                .get_result::<Message>(&_connection)
                .expect("Error.");
        }
        return true;
    }
    pub fn get_attach(&self, user_id: i32) -> String {
        if self.attach.is_some() {
            use crate::utils::message_elements;
            return message_elements(self.attach.as_ref().unwrap().to_string(), user_id);
        }
        else {
            return "".to_string();
        }
    }
    pub fn get_sticker(&self) -> Sticker {
        use crate::schema::stickers::dsl::stickers;

        let _connection = establish_connection();
        return stickers
            .filter(schema::stickers::id.eq(self.sticker_id.unwrap()))
            .load::<Sticker>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
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
    pub fn get_preview_text(&self) -> String {
        if self.is_manager() {
            let creator = self.get_creator();
            let message = self.get_parent();
            return creator.get_full_name() + &self.content.as_deref().unwrap() + &"<span class='underline'>".to_string() + &message.get_text_60() + &"</span>".to_string();
        } else{
            return self.get_type_text();
        }
    }
    pub fn plus_reactions(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::messages::reactions.eq(self.reactions + count))
            .get_result::<Message>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_reactions(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::messages::reactions.eq(self.reactions - count))
            .get_result::<Message>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn get_manager_text(&self) -> String {
        if self.parent_id.is_some() {
            let message = self.get_parent();
            let text = message.get_type_text();
            let creator = self.get_creator();
            return "<i><a target='_blank' href='".to_owned() + &creator.link.to_string() + &"</a><span>".to_string() + &self.content.as_deref().unwrap() + &"</span><a class='pointer show_selected_fix_message underline'>".to_string() + &text + &"</a></i>".to_string();
        } else{
            return (&self.content.as_deref().unwrap()).to_string();
        }
    }
    pub fn get_edit_attach(&self, _user_id: i32) -> String {
        if self.attach.is_some() {
            use crate::utils::edit_message_elements;
            return edit_message_elements(self.attach.as_ref().unwrap().to_string());
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
    pub fn get_transfers(&self) -> Vec<Message> {
        use crate::schema::message_transfers::dsl::message_transfers;
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        let transfers = message_transfers
            .filter(schema::message_transfers::message_id.eq(self.id))
            .load::<MessageTransfer>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in transfers.iter() {
            stack.push(_item.transfer_id);
        };
        return messages
            .filter(schema::messages::id.eq_any(stack))
            .load::<Message>(&_connection)
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
        let text_2: String;
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
            &creator.link.to_string() +
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
    pub fn get_chat(&self) -> Chat {
        use crate::schema::chats::dsl::chats;

        let _connection = establish_connection();
        return chats
            .filter(schema::chats::id.eq(self.chat_id))
            .load::<Chat>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_repost(&self) -> Post {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::id.eq(self.post_id.unwrap()))
            .load::<Post>(&_connection)
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

    pub fn count_reactions(&self) -> String {
        if self.reactions == 0 {
            return "".to_string();
        }
        else {
            return self.reactions.to_string();
        }
    }

    pub fn get_or_create_react_model(&self) -> MessageReaction {
        use crate::schema::message_reactions::dsl::message_reactions;

        let _connection = establish_connection();
        let _react_model = message_reactions
            .filter(schema::message_reactions::message_id.eq(self.id))
            .load::<MessageReaction>(&_connection)
            .expect("E.");
        if _react_model.len() > 0 {
            return _react_model.into_iter().nth(0).unwrap();
        }
        else {
            let new_react_model = NewMessageReaction {
                message_id:  self.id,
                field_1:  0,
                field_2:  0,
                field_3:  0,
                field_4:  0,
                field_5:  0,
                field_6:  0,
                field_7:  0,
                field_8:  0,
                field_9:  0,
                field_10: 0,
                field_11: 0,
                field_12: 0,
                field_13: 0,
                field_14: 0,
                field_15: 0,
                field_16: 0,
            };
            let _react_model = diesel::insert_into(schema::message_reactions::table)
                .values(&new_react_model)
                .get_result::<MessageReaction>(&_connection)
                .expect("Error.");

            return _react_model;
        }
    }

    pub fn send_reaction(&self, user_id: i32, types: i16) -> Json<JsonItemReactions> {
        use crate::schema::message_votes::dsl::message_votes;

        let _connection = establish_connection();
        let list = self.get_chat();
        let reactions_of_list = list.get_reactions_list();
        let react_model = self.get_or_create_react_model();
        let mut new_plus = false;
        let mut old_type = 0;

        if reactions_of_list.iter().any(|&i| i==types) && list.get_members_ids().iter().any(|&i| i==user_id) {

            let votes = message_votes
                .filter(schema::message_votes::user_id.eq(user_id))
                .filter(schema::message_votes::message_id.eq(self.id))
                .load::<MessageVote>(&_connection)
                .expect("E.");

            // если пользователь уже реагировал на товар
            if votes.len() > 0 {
                let vote = votes.into_iter().nth(0).unwrap();

                // если пользователь уже реагировал этой реакцией на этот товар
                if vote.reaction == types {
                    diesel::delete(message_votes
                        .filter(schema::message_votes::user_id.eq(user_id))
                        .filter(schema::message_votes::message_id.eq(self.id))
                        )
                        .execute(&_connection)
                        .expect("E");
                    react_model.update_model(types, None, false);
                    self.minus_reactions(1);
                }
                // если пользователь уже реагировал другой реакцией на этот товар
                else {
                    old_type = vote.reaction;
                    diesel::update(&vote)
                        .set(schema::message_votes::reaction.eq(types))
                        .get_result::<MessageVote>(&_connection)
                        .expect("Error.");

                    react_model.update_model(types, Some(old_type), false);
                }
            }

            // если пользователь не реагировал на этот товар
            else {
                let new_vote = NewMessageVote {
                    vote:       1,
                    user_id:    user_id,
                    message_id: self.id,
                    reaction:   types,
                };
                diesel::insert_into(schema::message_votes::table)
                    .values(&new_vote)
                    .get_result::<MessageVote>(&_connection)
                    .expect("Error.");

                react_model.update_model(types, None, true);
                self.plus_reactions(1);
                new_plus = true;
            }
        }
        let mut data: Vec<i32> = Vec::new();
        data.push(self.reactions);
        data.push(react_model.field_1);
        data.push(react_model.field_2);
        data.push(react_model.field_3);
        data.push(react_model.field_4);
        data.push(react_model.field_5);
        data.push(react_model.field_6);
        data.push(react_model.field_7);
        data.push(react_model.field_8);
        data.push(react_model.field_9);
        data.push(react_model.field_10);
        data.push(react_model.field_11);
        data.push(react_model.field_12);
        data.push(react_model.field_13);
        data.push(react_model.field_14);
        data.push(react_model.field_15);
        data.push(react_model.field_16);

        let types_usize: usize = types as usize;
        if old_type != 0 {
            let old_type_usize: usize = old_type as usize;
            data[types_usize] = data[types_usize] + 1;
            data[old_type_usize] = data[old_type_usize] - 1;
        }
        else if new_plus {
            data[types_usize] = data[types_usize] + 1;
            data[0] = data[0] + 1;
        }
        else {
            data[types_usize] = data[types_usize] - 1;
            data[0] = data[0] - 1;
        }

        return Json(JsonItemReactions {data});
    }

    pub fn count_reactions_of_types(&self, types: i16) -> i32 {
        let react_model = self.get_or_create_react_model();
        let format_types: i32 = types.into();
        let count = match format_types {
            1 => react_model.field_1,
            2 => react_model.field_2,
            3 => react_model.field_3,
            4 => react_model.field_4,
            5 => react_model.field_5,
            6 => react_model.field_6,
            7 => react_model.field_7,
            8 => react_model.field_8,
            9 => react_model.field_9,
            10 => react_model.field_10,
            11 => react_model.field_11,
            12 => react_model.field_12,
            13 => react_model.field_13,
            14 => react_model.field_14,
            15 => react_model.field_15,
            16 => react_model.field_16,
            _ => 0,
        };
        return count;
    }
    pub fn count_reactions_of_types_ru(&self, types: i16) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_reactions_of_types(types),
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }

    pub fn count_reactions_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.reactions,
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }

    pub fn is_have_reactions(&self) -> bool {
        return self.reactions > 0;
    }

    pub fn reactions_ids(&self) -> Vec<i32> {
        use crate::schema::message_votes::dsl::message_votes;

        let _connection = establish_connection();
        let votes = message_votes
            .filter(schema::message_votes::message_id.eq(self.id))
            .load::<MessageVote>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in votes.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }

    pub fn is_have_user_reaction(&self, user_id: i32) -> bool {
        return self.reactions_ids().iter().any(|&i| i==user_id);
    }

    pub fn get_user_reaction(&self, user_id: i32) -> i16 {
        use crate::schema::message_votes::dsl::message_votes;
        // "/static/images/reactions/" + get_user_reaction + ".jpg"
        let _connection = establish_connection();
        let vote = message_votes
            .filter(schema::message_votes::user_id.eq(user_id))
            .filter(schema::message_votes::message_id.eq(self.id))
            .load::<MessageVote>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();

        return vote.reaction;
    }

    pub fn get_reactions_users_of_types(&self, limit: i64, offset: i64, types: i16) -> Vec<User> {
        use crate::schema::message_votes::dsl::message_votes;
        use crate::utils::get_users_from_ids;

        let _connection = establish_connection();
        let votes = message_votes
            .filter(schema::message_votes::message_id.eq(self.id))
            .filter(schema::message_votes::reaction.eq(types))
            .limit(limit)
            .offset(offset)
            .load::<MessageVote>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in votes.iter() {
            stack.push(_item.user_id);
        };
        return get_users_from_ids(stack);
    }

    pub fn get_6_reactions_users_of_types(&self, types: i16) -> Vec<User> {
        use crate::schema::message_votes::dsl::message_votes;
        use crate::utils::get_users_from_ids;

        let _connection = establish_connection();
        let votes = message_votes
            .filter(schema::message_votes::message_id.eq(self.id))
            .filter(schema::message_votes::reaction.eq(types))
            .limit(6)
            .load::<MessageVote>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in votes.iter() {
            stack.push(_item.user_id);
        };
        return get_users_from_ids(stack);
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

/////// MessageReaction //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Message)]
pub struct MessageReaction {
    pub id:       i32,
    pub message_id: i32,
    pub field_1:  i32,
    pub field_2:  i32,
    pub field_3:  i32,
    pub field_4:  i32,
    pub field_5:  i32,
    pub field_6:  i32,
    pub field_7:  i32,
    pub field_8:  i32,
    pub field_9:  i32,
    pub field_10: i32,
    pub field_11: i32,
    pub field_12: i32,
    pub field_13: i32,
    pub field_14: i32,
    pub field_15: i32,
    pub field_16: i32,
}

impl MessageReaction {
    pub fn count_reactions_of_types(&self, types: i16) -> i32 {
        let format_types: i32 = types.into();
        let count = match format_types {
            1 => self.field_1,
            2 => self.field_2,
            3 => self.field_3,
            4 => self.field_4,
            5 => self.field_5,
            6 => self.field_6,
            7 => self.field_7,
            8 => self.field_8,
            9 => self.field_9,
            10 => self.field_10,
            11 => self.field_11,
            12 => self.field_12,
            13 => self.field_13,
            14 => self.field_14,
            15 => self.field_15,
            16 => self.field_16,
            _ => 0,
        };
        return count;
    }
    pub fn count_reactions_of_types_ru(&self, types: i16) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_reactions_of_types(types),
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }

    pub fn update_model(
        &self,
        new_types: i16,
        old_types_option: Option<i16>,
        plus: bool,
    ) -> &MessageReaction {
        let _connection = establish_connection();
        if old_types_option.is_some() {
            let old_types = old_types_option.unwrap();
            match new_types {
                1 => diesel::update(self)
                    .set(schema::message_reactions::field_1.eq(self.field_1 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::message_reactions::field_2.eq(self.field_2 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::message_reactions::field_3.eq(self.field_3 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::message_reactions::field_4.eq(self.field_4 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::message_reactions::field_5.eq(self.field_5 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::message_reactions::field_6.eq(self.field_6 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::message_reactions::field_7.eq(self.field_7 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::message_reactions::field_8.eq(self.field_8 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::message_reactions::field_9.eq(self.field_9 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::message_reactions::field_10.eq(self.field_10 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::message_reactions::field_11.eq(self.field_11 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::message_reactions::field_12.eq(self.field_12 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::message_reactions::field_13.eq(self.field_13 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::message_reactions::field_14.eq(self.field_14 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::message_reactions::field_15.eq(self.field_15 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::message_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::message_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
            };

            match old_types {
                1 => diesel::update(self)
                    .set(schema::message_reactions::field_1.eq(self.field_1 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::message_reactions::field_2.eq(self.field_2 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::message_reactions::field_3.eq(self.field_3 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::message_reactions::field_4.eq(self.field_4 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::message_reactions::field_5.eq(self.field_5 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::message_reactions::field_6.eq(self.field_6 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::message_reactions::field_7.eq(self.field_7 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::message_reactions::field_8.eq(self.field_8 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::message_reactions::field_9.eq(self.field_9 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::message_reactions::field_10.eq(self.field_10 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::message_reactions::field_11.eq(self.field_11 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::message_reactions::field_12.eq(self.field_12 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::message_reactions::field_13.eq(self.field_13 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::message_reactions::field_14.eq(self.field_14 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::message_reactions::field_15.eq(self.field_15 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::message_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::message_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<MessageReaction>(&_connection)
                    .expect("Error."),
            };
            return &self;
        }
        else {
            if plus {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::message_reactions::field_1.eq(self.field_1 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::message_reactions::field_2.eq(self.field_2 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::message_reactions::field_3.eq(self.field_3 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::message_reactions::field_4.eq(self.field_4 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::message_reactions::field_5.eq(self.field_5 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::message_reactions::field_6.eq(self.field_6 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::message_reactions::field_7.eq(self.field_7 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::message_reactions::field_8.eq(self.field_8 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::message_reactions::field_9.eq(self.field_9 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::message_reactions::field_10.eq(self.field_10 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::message_reactions::field_11.eq(self.field_11 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::message_reactions::field_12.eq(self.field_12 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::message_reactions::field_13.eq(self.field_13 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::message_reactions::field_14.eq(self.field_14 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::message_reactions::field_15.eq(self.field_15 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::message_reactions::field_16.eq(self.field_16 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::message_reactions::field_16.eq(self.field_16 + 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                };
            }
            else {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::message_reactions::field_1.eq(self.field_1 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::message_reactions::field_2.eq(self.field_2 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::message_reactions::field_3.eq(self.field_3 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::message_reactions::field_4.eq(self.field_4 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::message_reactions::field_5.eq(self.field_5 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::message_reactions::field_6.eq(self.field_6 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::message_reactions::field_7.eq(self.field_7 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::message_reactions::field_8.eq(self.field_8 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::message_reactions::field_9.eq(self.field_9 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::message_reactions::field_10.eq(self.field_10 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::message_reactions::field_11.eq(self.field_11 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::message_reactions::field_12.eq(self.field_12 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::message_reactions::field_13.eq(self.field_13 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::message_reactions::field_14.eq(self.field_14 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::message_reactions::field_15.eq(self.field_15 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::message_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::message_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<MessageReaction>(&_connection)
                        .expect("Error."),
                };
            }
            return &self;
        }
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="message_reactions"]
pub struct NewMessageReaction {
    pub message_id:  i32,
    pub field_1:  i32,
    pub field_2:  i32,
    pub field_3:  i32,
    pub field_4:  i32,
    pub field_5:  i32,
    pub field_6:  i32,
    pub field_7:  i32,
    pub field_8:  i32,
    pub field_9:  i32,
    pub field_10: i32,
    pub field_11: i32,
    pub field_12: i32,
    pub field_13: i32,
    pub field_14: i32,
    pub field_15: i32,
    pub field_16: i32,
}

/////// MessageVote//////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Message)]
pub struct MessageVote {
    pub id:         i32,
    pub vote:       i16,
    pub user_id:    i32,
    pub message_id: i32,
    pub reaction:   i16,
}
impl MessageVote {
    pub fn get_reaction(&self) -> Reaction {
        use crate::schema::reactions::dsl::reactions;

        let _connection = establish_connection();
        return reactions
            .filter(schema::reactions::types.eq(self.reaction))
            .load::<Reaction>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
}
#[derive(Deserialize, Insertable)]
#[table_name="message_votes"]
pub struct NewMessageVote {
    pub vote:       i16,
    pub user_id:    i32,
    pub message_id: i32,
    pub reaction:   i16,
}
