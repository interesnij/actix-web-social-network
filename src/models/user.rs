use crate::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use diesel::prelude::*;
use crate::schema;
use crate::models::{
    Chat, Message, UserLocation, Post, Smile, Sticker, Community,
};

///// Типы пользоватетеля
    // 1 стандартный тип пользователя
    // 3 ребенок
    // 7 идентифицированный
    // 6 пославший запрос на идентификацию
    // 11 удаленный стандартный
    // 13 удаленный ребенок
    // 17 удаленный идентифицированный
    // 16 удаленный пославший запрос на идентификацию
    // 21 закрытый стандартный
    // 23 закрытый ребенок
    // 27 закрытый идентифицированный
    // 26 закрытый пославший запрос на идентификацию
    // 31 приостановленный стандартный
    // 33 приостановленный ребенок
    // 37 приостановленный идентифицированный
    // 36 приостановленный пославший запрос на идентификацию
    // 41 закрытый баннером стандартный
    // 43 закрытый баннером ребенок
    // 47 закрытый баннером идентифицированный
    // 46 закрытый баннером пославший запрос на идентификацию

///// Полномочия пользоватетеля
    // 1 стандартные полномочия
    // 10 TRAINEE_MODERATOR
    // 13 MODERATOR
    // 16 HIGH_MODERATOR
    // 19 TEAMLEAD_MODERATOR
    // 20 TRAINEE_MANAGER
    // 23 MANAGER
    // 26 HIGH_MANAGER
    // 29 TEAMLEAD_MANAGER
    // 30 ADVERTISER
    // 34 HIGH_ADVERTISER
    // 39 TEAMLEAD_ADVERTISER
    // 40 ADMINISTRATOR
    // 44 HIGH_ADMINISTRATOR
    // 49 TEAMLEAD_ADMINISTRATOR
    // 60 SUPERMANAGER

///// Пол пользоватетеля
    // 'a' Мужик
    // 'b' Баба

///// Оборудование пользоватетеля
    // 'a' Комп
    // 'b' Телефон

///// Язык пользоватетеля
    // 'a' Русский
    // 'b' Английский

#[derive(Serialize, Identifiable, Queryable)]
pub struct User {
    pub id:            i32,
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub types:         i16,
    pub gender:        String,
    pub device:        String,
    pub language:      String,
    pub perm:          i16,
    pub level:         i16,
    pub password:      String,
    pub have_link:     Option<String>,
    pub city:          Option<String>,
    pub status:        Option<String>,
    pub b_avatar:      Option<String>,
    pub s_avatar:      Option<String>,
    pub email:         Option<String>,
    pub birthday:      chrono::NaiveDate,
    pub last_activity: chrono::NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub types:         i16,
    pub gender:        String,
    pub device:        String,
    pub language:      String,
    pub perm:          i16,
    pub level:         i16,
    pub password:      String,
    pub birthday:      chrono::NaiveDate,
    pub last_activity: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub phone:    String,
    pub password: String,
}

impl User {
    pub fn get_full_name(&self) -> String {
        self.first_name.clone() + &" ".to_string() + &self.last_name.clone()
    }
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn get_link(&self) -> String {
        if self.have_link.is_some() {
            return self.have_link.as_deref().unwrap().to_string();
        }
        else {
            return "/id".to_string() + &self.get_str_id() + &"/".to_string();
        }
    }
    pub fn get_s_avatar(&self) -> String {
            if self.s_avatar.is_some() {
                return self.s_avatar.as_deref().unwrap().to_string();
            }
            else {
                return "/static/images/icons/avatar30.svg".to_string();
            }
    }
    pub fn get_description(&self) -> String {
        return "<a href='".to_string() + &self.get_link() + &"' target='_blank'>".to_string() + &self.get_full_name() + &"</a>".to_string();
    }
    pub fn is_user(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "use".to_string() + &self.get_str_id();
    }
    pub fn close_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            1 => 21,
            3 => 23,
            7 => 27,
            6 => 26,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::users::types.eq(close_case))
            .get_result::<User>(&_connection)
            .expect("E");
       return true;
    }
    pub fn unclose_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            21 => 1,
            23 => 3,
            27 => 7,
            26 => 6,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::users::types.eq(close_case))
            .get_result::<User>(&_connection)
            .expect("E");
       return true;
    }
    pub fn suspend_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            1 => 31,
            3 => 33,
            7 => 37,
            6 => 36,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::users::types.eq(close_case))
            .get_result::<User>(&_connection)
            .expect("E");
       return true;
    }
    pub fn unsuspend_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            31 => 1,
            33 => 3,
            37 => 7,
            36 => 6,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::users::types.eq(close_case))
            .get_result::<User>(&_connection)
            .expect("E");
       return true;
    }
    pub fn get_or_create_manager_chat_pk(&self) -> i32 {
        use crate::schema::chats::dsl::chats;

        let _connection = establish_connection();

        let manager_chats = chats
            .filter(schema::chats::user_id.eq(self.id))
            .filter(schema::chats::types.eq(3))
            .load::<Chat>(&_connection)
            .expect("E");
        if manager_chats.len() > 0 {
            return manager_chats[0].id
        } else {
            use crate::schema::chat_users::dsl::chat_users;
            use crate::models::{NewChat, ChatUser, NewChatUser};
            use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

            let d = NaiveDate::from_ymd(2015, 6, 3);
            let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
            let new_manager_chat = NewChat{
                name: Some("Рассылка служународу.рус".to_string()),
                types: 3,
                community_id: None,
                user_id: self.id,
                position: 10,
                members: 1,
                created: NaiveDateTime::new(d, t),
                can_add_members:  "f".to_string(),
                can_fix_item:     "b".to_string(),
                can_mention:      "f".to_string(),
                can_add_admin:    "f".to_string(),
                can_add_design:   "f".to_string(),
                can_see_settings: "f".to_string(),
                can_see_log:      "f".to_string(),
            };
            let manager_chat = diesel::insert_into(schema::chats::table)
                .values(&new_manager_chat)
                .get_result::<Chat>(&_connection)
                .expect("E.");

            let new_chat_user = NewChatUser{
                user_id: self.id,
                chat_id: manager_chat.id,
                types: "a".to_string(),
                is_administrator: false,
                created: NaiveDateTime::new(d, t),
            };
            diesel::insert_into(schema::chat_users::table)
                .values(&new_chat_user)
                .get_result::<ChatUser>(&_connection)
                .expect("E.");
            return manager_chat.id;
        }
    }
    pub fn get_or_create_support_chat_pk(&self) -> i32 {
        use crate::schema::chats::dsl::chats;

        let _connection = establish_connection();

        let manager_chats = chats
            .filter(schema::chats::user_id.eq(self.id))
            .filter(schema::chats::types.between(10,16))
            .load::<Chat>(&_connection)
            .expect("E");
        if manager_chats.len() > 0 {
            return manager_chats[0].id
        } else {
            use crate::schema::chat_users::dsl::chat_users;
            use crate::models::{NewChat, ChatUser, NewChatUser};
            use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

            let d = NaiveDate::from_ymd(2015, 6, 3);
            let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
            let new_manager_chat = NewChat{
                name: Some("Рассылка служународу.рус".to_string()),
                types: 11,
                community_id: None,
                user_id: self.id,
                position: 10,
                members: 1,
                created: NaiveDateTime::new(d, t),
                can_add_members:  "f".to_string(),
                can_fix_item:     "b".to_string(),
                can_mention:      "f".to_string(),
                can_add_admin:    "f".to_string(),
                can_add_design:   "f".to_string(),
                can_see_settings: "f".to_string(),
                can_see_log:      "f".to_string(),
            };
            let manager_chat = diesel::insert_into(schema::chats::table)
                .values(&new_manager_chat)
                .get_result::<Chat>(&_connection)
                .expect("E.");

            let new_chat_user = NewChatUser{
                user_id: self.id,
                chat_id: manager_chat.id,
                types: "a".to_string(),
                is_administrator: false,
                created: NaiveDateTime::new(d, t),
            };
            diesel::insert_into(schema::chat_users::table)
                .values(&new_chat_user)
                .get_result::<ChatUser>(&_connection)
                .expect("E.");
            return manager_chat.id;
        }
    }
    pub fn get_deleted_support_chats(&self) -> Vec<Chat> {
        use crate::schema::chats::dsl::chats;

        let _connection = establish_connection();
        return chats
            .filter(schema::chats::user_id.eq(self.id))
            .filter(schema::chats::types.between(40,46))
            .order(schema::chats::id.desc())
            .load::<Chat>(&_connection)
            .expect("E");
    }
    pub fn get_last_location(&self) -> UserLocation {
        use crate::schema::user_locations::dsl::user_locations;

        let _connection = establish_connection();
        return user_locations
            .filter(schema::user_locations::user_id.eq(self.id))
            .order(schema::user_locations::id.desc())
            .load::<UserLocation>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_favourite_messages(&self) -> Vec<Message> {
        use crate::schema::messages::dsl::messages;
        use crate::schema::message_options::dsl::message_options;
        use crate::models::MessageOption;
        use diesel::dsl::any;

        let _connection = establish_connection();
        let all_option_messages = message_options
            .filter(schema::message_options::user_id.eq(self.id))
            .filter(schema::message_options::is_favourite.eq(true))
            .order(schema::message_options::id.desc())
            .load::<MessageOption>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in all_option_messages.iter() {
            stack.push(_item.message_id);
        };
        return messages
            .filter(schema::messages::id.eq(any(stack)))
            .load::<Message>(&_connection)
            .expect("E.");
    }
    pub fn favourite_messages_count(&self) -> usize {
        use crate::schema::message_options::dsl::message_options;
        use crate::models::MessageOption;

        let _connection = establish_connection();
        return message_options
            .filter(schema::message_options::user_id.eq(self.id))
            .filter(schema::message_options::is_favourite.eq(true))
            .load::<MessageOption>(&_connection)
            .expect("E")
            .len();
    }
    pub fn get_fixed_posts(&self) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::user_id.eq(self.id))
            .filter(schema::posts::types.eq("b"))
            .order(schema::posts::created.desc())
            .load::<Post>(&_connection)
            .expect("E");
    }
    pub fn get_fixed_posts_ids(&self) -> Vec<i32> {
        let user_fixed_posts = self.get_fixed_posts();
        let mut stack = Vec::new();
        for _item in user_fixed_posts.iter() {
            stack.push(_item.id);
        };
        return stack;
    }
    pub fn count_fix_items(&self) -> usize {
        return self.get_fixed_posts().len();
    }
    pub fn is_can_fixed_post(&self) -> bool {
        return self.count_fix_items() < 10;
    }
    pub fn get_verb_gender(&self, str: String) -> String {
        if self.gender == "b" {
            return "W".to_string() + &str;
        }
        else {
            return str;
        }
    }
    pub fn get_populate_smiles(&self) -> Vec<Smile> {
        use crate::schema::smiles::dsl::smiles;
        use crate::schema::user_populate_smiles::dsl::user_populate_smiles;
        use crate::models::UserPopulateSmile;
        use diesel::dsl::any;

        let _connection = establish_connection();
        let all_populate_smiles = user_populate_smiles
            .filter(schema::user_populate_smiles::user_id.eq(self.id))
            .order(schema::user_populate_smiles::id.desc())
            .load::<UserPopulateSmile>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in all_populate_smiles.iter() {
            stack.push(_item.smile_id);
        };
        return smiles
            .filter(schema::smiles::id.eq(any(stack)))
            .load::<Smile>(&_connection)
            .expect("E.");
    }
    pub fn get_populate_stickers(&self) -> Vec<Sticker> {
        use crate::schema::stickers::dsl::stickers;
        use crate::schema::user_populate_stickers::dsl::user_populate_stickers;
        use crate::models::UserPopulateSticker;
        use diesel::dsl::any;

        let _connection = establish_connection();
        let all_populate_stickers = user_populate_stickers
            .filter(schema::user_populate_stickers::user_id.eq(self.id))
            .order(schema::user_populate_stickers::id.desc())
            .load::<UserPopulateSticker>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in all_populate_stickers.iter() {
            stack.push(_item.sticker_id);
        };
        return stickers
            .filter(schema::stickers::id.eq(any(stack)))
            .load::<Sticker>(&_connection)
            .expect("E.");
    }
    pub fn get_color_background(&self) -> String {
        use crate::schema::design_settings::dsl::design_settings;
        use crate::models::DesignSetting;

        let _connection = establish_connection();
        let _designs = design_settings
            .filter(schema::design_settings::user_id.eq(&self.id))
            .load::<DesignSetting>(&_connection)
            .expect("E");
        if _designs.len() > 0 {
            return _designs[0].background.to_string();
        } else {
            return "white".to_string();
        }
    }
    pub fn get_email_status(&self) -> String {
        if self.email.is_some() {
            return self.email.as_deref().unwrap().to_string();
        } else {
            return "Почта не указана".to_string();
        }
    }
    pub fn calculate_age(&self) -> i32 {
        use chrono::{NaiveDate, Datelike};
        let birthday = self.birthday;
        let d = NaiveDate::from_ymd(2015, 6, 3);
        return d.year() - birthday.year();
    }
    pub fn is_women(&self) -> bool {
        return self.gender == "b";
    }
    pub fn is_men(&self) -> bool {
        return self.gender == "a";
    }
    pub fn is_supermanager(&self) -> bool {
        return self.perm == 60;
    }
    pub fn is_administrator(&self) -> bool {
        return self.perm > 39;
    }
    pub fn is_advertiser(&self) -> bool {
        return self.perm > 29;
    }
    pub fn is_manager(&self) -> bool {
        return self.perm > 19;
    }
    pub fn is_support(&self) -> bool {
        use crate::schema::support_users::dsl::support_users;
        use crate::models::SupportUser;

        let _connection = establish_connection();
        let _supp_users = support_users
            .filter(schema::support_users::manager_id.eq(&self.id))
            .load::<SupportUser>(&_connection)
            .expect("E");
        if _supp_users.len() > 0 {
            return true;
        } else {
            return false;
        }
    }
    pub fn is_moderator(&self) -> bool {
        return self.perm > 9;
    }
    pub fn is_suspended(&self) -> bool {
        return 40 > self.types && self.types > 30;
    }
    pub fn is_have_warning_banner(&self) -> bool {
        return 50 > self.types && self.types > 40;
    }
    pub fn is_deleted(&self) -> bool {
        return 20 > self.types && self.types > 10;
    }
    pub fn is_closed(&self) -> bool {
        return 30 > self.types && self.types > 20;
    }
    pub fn is_identified_send(&self) -> bool {
        return self.types == 6;
    }
    pub fn is_identified(&self) -> bool {
        return self.types == 7;
    }
    pub fn is_child(&self) -> bool {
        return self.types == 3;
    }
    pub fn is_child_safety(&self) -> bool {
        return self.perm > 9 || self.types == 7;
    }
    pub fn is_online(&self) -> bool {
        use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Duration};

        let d = NaiveDate::from_ymd(2015, 6, 3);
        let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
        return self.last_activity.checked_add_signed(Duration::seconds(301)) > NaiveDateTime::new(d, t).checked_add_signed(Duration::seconds(1));
    }
    pub fn is_desctop(&self) -> bool {
        return self.device == "a";
    }
    pub fn is_mobile(&self) -> bool {
        return self.device == "b";
    }
    pub fn get_online_status(&self) -> String {
        if self.is_online() {
            return "Онлайн".to_string();
        }
        else {
            if self.is_women() {
                return "Была ".to_string() + &self.last_activity.to_string();
            } else {
                return "Был ".to_string() + &self.last_activity.to_string();
            }
        }
    }
    pub fn get_blocked_users(&self) -> Vec<User> {
        use crate::schema::user_blocks::dsl::user_blocks;
        use crate::schema::users::dsl::users;
        use crate::models::UserBlock;
        use diesel::dsl::any;

        let _connection = establish_connection();
        let all_user_blocks = user_blocks
            .filter(schema::user_blocks::user_block_i.eq(self.id))
            .order(schema::user_blocks::id.desc())
            .load::<UserBlock>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in all_user_blocks.iter() {
            stack.push(_item.blocked_user_id);
        };
        return users
            .filter(schema::users::id.eq(any(stack)))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_staffed_communities(&self) -> Vec<Community> {
        use crate::schema::communitys::dsl::communitys;
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;
        use diesel::dsl::any;

        let _connection = establish_connection();
        let all_memberships = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .order(schema::communities_memberships::visited.desc())
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        let mut community_ids = Vec::new();
        for _item in all_memberships.iter() {
            if _item.is_administrator || _item.is_moderator || _item.is_editor || _item.is_advertiser {
                community_ids.push(_item.community_id);
            }
        };
        return communitys
            .filter(schema::communitys::id.eq(any(community_ids)))
            .load::<Community>(&_connection)
            .expect("E.");
    }
    pub fn get_featured_friends_ids(&self) -> Vec<i32> {
        use crate::schema::featured_user_communities::dsl::featured_user_communities;
        use crate::models::FeaturedUserCommunitie;

        let _connection = establish_connection();
        let mut stack = Vec::new();
        let featured_friends = featured_user_communities
            .filter(schema::featured_user_communities::owner.eq(self.id))
            .load::<FeaturedUserCommunitie>(&_connection)
            .expect("E.");
        for _item in featured_friends.iter() {
            stack.push(_item.user_id.unwrap());
        };
        return stack;
    }
    pub fn get_6_featured_friends_ids(&self) -> Vec<i32> {
        use crate::schema::featured_user_communities::dsl::featured_user_communities;
        use crate::models::FeaturedUserCommunitie;

        let _connection = establish_connection();
        let mut stack = Vec::new();
        let featured_friends = &featured_user_communities
            .filter(schema::featured_user_communities::owner.eq(self.id))
            .load::<FeaturedUserCommunitie>(&_connection)
            .expect("E.")[..6];
        for _item in featured_friends.iter() {
            stack.push(_item.user_id.unwrap());
        };
        return stack;
    }
    pub fn get_featured_friends(&self) -> Vec<User> {
        use crate::schema::users::dsl::users;
        use diesel::dsl::any;

        let _connection = establish_connection();
        return users
            .filter(schema::users::id.eq(any(self.get_featured_friends_ids())))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_6_featured_friends(&self) -> Vec<User> {
        use crate::schema::users::dsl::users;
        use diesel::dsl::any;

        let _connection = establish_connection();
        return users
            .filter(schema::users::id.eq(any(self.get_6_featured_friends_ids())))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_featured_friends_count(&self) -> usize {
        return self.get_featured_friends_ids().len();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: i32,
    pub phone: String,
}
