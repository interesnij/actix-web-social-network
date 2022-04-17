use crate::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use diesel::prelude::*;
use crate::schema;
use crate::models::{
    Chat, Message, UserLocation, Smile, Sticker, Community, UserProfile, Friend,
    Post, Photo, Music, Video, Survey, Doc, Good,
    PostList, PhotoList, MusicList, VideoList, SurveyList, DocList, GoodList,
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
            .limit(6)
            .load::<FeaturedUserCommunitie>(&_connection)
            .expect("E.");
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
    pub fn get_featured_communities_ids(&self) -> Vec<i32> {
        use crate::schema::featured_user_communities::dsl::featured_user_communities;
        use crate::models::FeaturedUserCommunitie;

        let _connection = establish_connection();
        let mut stack = Vec::new();
        let featured_communities = featured_user_communities
            .filter(schema::featured_user_communities::owner.eq(self.id))
            .load::<FeaturedUserCommunitie>(&_connection)
            .expect("E.");
        for _item in featured_communities.iter() {
            stack.push(_item.community_id.unwrap());
        };
        return stack;
    }
    pub fn get_6_featured_communities_ids(&self) -> Vec<i32> {
        use crate::schema::featured_user_communities::dsl::featured_user_communities;
        use crate::models::FeaturedUserCommunitie;

        let _connection = establish_connection();
        let mut stack = Vec::new();
        let featured_communities = &featured_user_communities
            .filter(schema::featured_user_communities::owner.eq(self.id))
            .limit(6)
            .load::<FeaturedUserCommunitie>(&_connection)
            .expect("E.");
        for _item in featured_communities.iter() {
            stack.push(_item.community_id.unwrap());
        };
        return stack;
    }
    pub fn get_featured_communities(&self) -> Vec<Community> {
        use crate::schema::communitys::dsl::communitys;
        use diesel::dsl::any;

        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::id.eq(any(self.get_featured_communities_ids())))
            .load::<Community>(&_connection)
            .expect("E.");
    }
    pub fn get_6_featured_communities(&self) -> Vec<Community> {
        use crate::schema::communitys::dsl::communitys;
        use diesel::dsl::any;

        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::id.eq(any(self.get_6_featured_communities_ids())))
            .load::<Community>(&_connection)
            .expect("E.");
    }
    pub fn get_featured_communities_count(&self) -> usize {
        return self.get_featured_communities_ids().len();
    }
    pub fn is_blocked_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::user_blocks::dsl::user_blocks;
        use crate::models::UserBlock;

        let _connection = establish_connection();
        let all_blocks = user_blocks
            .filter(schema::user_blocks::blocked_user_id.eq(user_id))
            .filter(schema::user_blocks::user_block_i.eq(self.id))
            .load::<UserBlock>(&_connection)
            .expect("E.");
        return all_blocks.len() > 0;
    }
    pub fn is_connected_with_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        let all_friends = friends
            .filter(schema::friends::user_id.eq(user_id))
            .filter(schema::friends::target_user_id.eq(self.id))
            .load::<Friend>(&_connection)
            .expect("E.");
        return all_friends.len() > 0;
    }
    pub fn is_staff_of_community(&self, community_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let _member = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
        if _member.is_administrator || _member.is_moderator || _member.is_editor ||_member.is_advertiser {
            return true;
        } else {
            return false;
        }
    }
    pub fn is_member_of_community(&self, community_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let _members = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E.");
        if _members.len() > 0 {
            return true;
        } else {
            return false;
        }
    }
    pub fn is_follow_from_community(&self, community_id: i32) -> bool {
        use crate::schema::community_follows::dsl::community_follows;
        use crate::models::CommunityFollow;

        let _connection = establish_connection();
        let follows = community_follows
            .filter(schema::community_follows::user_id.eq(self.id))
            .filter(schema::community_follows::community_id.eq(community_id))
            .load::<CommunityFollow>(&_connection)
            .expect("E.");
        return follows.len() > 0;
    }
    pub fn is_creator_community(&self, community_id: i32) -> bool {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        let community = communitys
            .filter(schema::communitys::id.eq(community_id))
            .load::<Community>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
        return community.user_id == self.id;
    }
    pub fn is_staffed_user(&self) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let all_memberships = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        for _item in all_memberships.iter() {
            if _item.is_administrator || _item.is_moderator || _item.is_editor || _item.is_advertiser {
                return true;
            }
        };
        return false;
    }
    pub fn is_administrator_of_community(&self, community_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let all_memberships = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        for _item in all_memberships.iter() {
            if _item.is_administrator {
                return true;
            }
        };
        return false;
    }
    pub fn is_editor_of_community(&self, community_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let all_memberships = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        for _item in all_memberships.iter() {
            if _item.is_editor {
                return true;
            }
        };
        return false;
    }
    pub fn is_moderator_of_community(&self, community_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let all_memberships = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        for _item in all_memberships.iter() {
            if _item.is_moderator {
                return true;
            }
        };
        return false;
    }
    pub fn is_advertiser_of_community(&self, community_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let all_memberships = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        for _item in all_memberships.iter() {
            if _item.is_advertiser {
                return true;
            }
        };
        return false;
    }
    pub fn is_following_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;
        use crate::models::Follow;

        let _connection = establish_connection();
        let all_follows = follows
            .filter(schema::follows::user_id.eq(self.id))
            .filter(schema::follows::followed_user.eq(user_id))
            .load::<Follow>(&_connection)
            .expect("E.");
        return all_follows.len() > 0;
    }
    pub fn is_followers_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;
        use crate::models::Follow;

        let _connection = establish_connection();
        let all_follows = follows
            .filter(schema::follows::followed_user.eq(self.id))
            .filter(schema::follows::user_id.eq(user_id))
            .load::<Follow>(&_connection)
            .expect("E.");
        return all_follows.len() > 0;
    }
    pub fn is_followers_user_view(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;
        use crate::models::Follow;

        let _connection = establish_connection();
        let all_follows = follows
            .filter(schema::follows::followed_user.eq(self.id))
            .filter(schema::follows::user_id.eq(user_id))
            .filter(schema::follows::view.eq(true))
            .load::<Follow>(&_connection)
            .expect("E.");
        return all_follows.len() > 0;
    }
    pub fn get_buttons_profile(&self, user_id: i32) -> String {
        let mut suffix: String = "".to_string();
        if self.perm > 19 {
            suffix = "staff_".to_string();
        }
        if self.is_blocked_user_with_id(user_id) {
            return "desctop/users/button/".to_owned() + &suffix + &"blocked_user.html".to_string();
        }
        else if self.is_connected_with_user_with_id(user_id){
            return "desctop/users/button/".to_owned() + &suffix + &"frend_user.html".to_string();
        }
        else if self.is_followers_user_view(user_id){
            return "desctop/users/button/".to_owned() + &suffix + &"follow_user.html".to_string();
        }
        else if self.is_following_user_with_id(user_id){
            return "desctop/users/button/".to_owned() + &suffix + &"following_user.html".to_string();
        }
        else if self.is_followers_user_with_id(user_id){
            return "desctop/users/button/".to_owned() + &suffix + &"follow_view_user.html".to_string();
        }
        else {
            return "desctop/users/button/".to_owned() + &suffix + &"default_user.html".to_string();
        }
    }
    pub fn get_profile(&self) -> UserProfile {
        use crate::schema::user_profiles::dsl::user_profiles;

        let _connection = establish_connection();
        return user_profiles
            .filter(schema::user_profiles::id.eq(self.id))
            .load::<UserProfile>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn is_have_followers(&self) -> bool {
        return self.get_profile().follows > 0
    }
    pub fn is_have_followings(&self) -> bool {
        use crate::schema::follows::dsl::follows;
        use crate::models::Follow;

        let _connection = establish_connection();
        let all_follows = follows
            .filter(schema::follows::user_id.eq(self.id))
            .load::<Follow>(&_connection)
            .expect("E.");
        return all_follows.len() > 0;
    }
    pub fn is_have_blacklist(&self) -> bool {
        use crate::schema::user_blocks::dsl::user_blocks;
        use crate::models::UserBlock;

        let _connection = establish_connection();
        let all_user_blocks = user_blocks
            .filter(schema::user_blocks::user_block_i.eq(self.id))
            .load::<UserBlock>(&_connection)
            .expect("E.");
        return all_user_blocks.len() > 0;
    }
    pub fn is_have_friends(&self) -> bool {
        return self.get_profile().friends > 0;
    }
    pub fn is_have_communities(&self) -> bool {
        return self.get_profile().communities > 0;
    }
    pub fn is_have_music(&self) -> bool {
        return self.get_profile().tracks > 0;
    }
    pub fn is_have_photo(&self) -> bool {
        return self.get_profile().photos > 0;
    }
    pub fn is_have_video(&self) -> bool {
        return self.get_profile().videos > 0;
    }
    pub fn is_have_doc(&self) -> bool {
        return self.get_profile().docs > 0;
    }
    pub fn is_have_post(&self) -> bool {
        return self.get_profile().posts > 0;
    }

    pub fn count_no_view_followers(&self) -> usize {
        use crate::schema::follows::dsl::follows;
        use crate::models::Follow;

        let _connection = establish_connection();
        let all_follows = follows
            .filter(schema::follows::followed_user.eq(self.id))
            .filter(schema::follows::view.eq(false))
            .load::<Follow>(&_connection)
            .expect("E.");
        return all_follows.len();
    }
    pub fn count_following(&self) -> usize {
        use crate::schema::follows::dsl::follows;
        use crate::models::Follow;

        let _connection = establish_connection();
        let all_follows = follows
            .filter(schema::follows::user_id.eq(self.id))
            .load::<Follow>(&_connection)
            .expect("E.");
        return all_follows.len();
    }
    pub fn count_followers(&self) -> i32 {
        return self.get_profile().follows;
    }
    pub fn count_blacklist(&self) -> usize {
        use crate::schema::user_blocks::dsl::user_blocks;
        use crate::models::UserBlock;

        let _connection = establish_connection();
        let all_user_blocks = user_blocks
            .filter(schema::user_blocks::user_block_i.eq(self.id))
            .load::<UserBlock>(&_connection)
            .expect("E.");
        return all_user_blocks.len();
    }
    pub fn count_goods(&self) -> i32 {
        return self.get_profile().goods;
    }
    pub fn count_photos(&self) -> i32 {
        return self.get_profile().photos;
    }
    pub fn count_docs(&self) -> i32 {
        return self.get_profile().docs;
    }
    pub fn count_posts(&self) -> i32 {
        return self.get_profile().posts;
    }
    pub fn count_articles(&self) -> i32 {
        return self.get_profile().articles;
    }
    pub fn count_communities(&self) -> i32 {
        return self.get_profile().communities;
    }
    pub fn count_communities_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru(
            self.count_communities(),
            " сообщество".to_string(),
            " сообщества".to_string(),
            " сообществ".to_string(),
        );
    }
    pub fn count_tracks(&self) -> i32 {
        return self.get_profile().tracks;
    }
    pub fn count_videos(&self) -> i32 {
        return self.get_profile().videos;
    }
    pub fn count_friends(&self) -> i32 {
        return self.get_profile().friends;
    }
    pub fn plus_photos(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::photos.eq(profile.photos + count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_goods(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::goods.eq(profile.goods + count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_posts(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::posts.eq(profile.posts + count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_videos(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::videos.eq(profile.videos + count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_docs(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::docs.eq(profile.docs + count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_tracks(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::tracks.eq(profile.tracks + count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_communities(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::communities.eq(profile.communities + count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_articles(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::articles.eq(profile.articles + count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_follows(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::follows.eq(profile.follows + count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_friends(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::friends.eq(profile.friends + count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_photos(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::photos.eq(profile.photos - count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_goods(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::goods.eq(profile.goods - count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_posts(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::posts.eq(profile.posts - count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_videos(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::videos.eq(profile.videos - count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_docs(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::docs.eq(profile.docs - count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_tracks(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::tracks.eq(profile.tracks - count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_communities(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::communities.eq(profile.communities - count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_articles(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::articles.eq(profile.articles - count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_follows(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::follows.eq(profile.follows - count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_friends(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::friends.eq(profile.friends - count))
            .get_result::<UserProfile>(&_connection)
            .expect("Error.");
        return true;
    }

    pub fn get_friends_ids(&self) -> Vec<i32> {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        let mut stack = Vec::new();
        let _friends = friends
            .filter(schema::friends::user_id.eq(self.id))
            .load::<Friend>(&_connection)
            .expect("E.");
        for _item in _friends.iter() {
            stack.push(_item.target_user_id);
        };
        return stack;
    }
    pub fn get_friend_and_friend_of_friend_ids(&self) -> Vec<i32> {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        let mut stack = Vec::new();

        let user_friends = friends
            .filter(schema::friends::user_id.eq(self.id))
            .load::<Friend>(&_connection)
            .expect("E.");

        for _item in user_friends.iter() {
            stack.push(_item.target_user_id);
        };
        for friend in self.get_friends().iter() {
            let user_friend_friends = friends
                .filter(schema::friends::user_id.eq(friend.id))
                .load::<Friend>(&_connection)
                .expect("E.");
            for f in user_friend_friends.iter() {
                if stack.iter().any(|&i| i!=f.target_user_id) {
                    stack.push(f.target_user_id);
                }
            }
        }
        return stack;
    }

    pub fn get_friends(&self) -> Vec<User> {
        use crate::schema::users::dsl::users;
        use diesel::dsl::any;

        let _connection = establish_connection();
        return users
            .filter(schema::users::id.eq(any(self.get_friends_ids())))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_communities(&self) -> Vec<Community> {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::schema::communitys::dsl::communitys;
        use crate::models::CommunitiesMembership;
        use diesel::dsl::any;

        let _connection = establish_connection();
        let _user_communities = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .order(schema::communities_memberships::visited.desc())
            .load::<CommunitiesMembership>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for _item in _user_communities.iter() {
            stack.push(_item.community_id);
        };
        return communitys
            .filter(schema::communitys::id.eq(any(stack)))
            .load::<Community>(&_connection)
            .expect("E.");
    }
    pub fn get_online_friends(&self) -> Vec<User> {
        use crate::schema::users::dsl::users;
        use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Duration};
        use diesel::dsl::any;

        let _connection = establish_connection();
        let d = NaiveDate::from_ymd(2015, 6, 3);
        let t = NaiveTime::from_hms_milli(12, 34, 56, 789) - Duration::seconds(300);

        return users
            .filter(schema::users::id.eq(any(self.get_friends_ids())))
            .filter(schema::users::last_activity.gt(NaiveDateTime::new(d, t)))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_online_friends_count(&self) -> usize {
        return self.get_online_friends().len();
    }
    pub fn get_6_online_friends(&self) -> Vec<User> {
        use crate::schema::users::dsl::users;
        use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Duration};
        use diesel::dsl::any;

        let _connection = establish_connection();
        let d = NaiveDate::from_ymd(2015, 6, 3);
        let t = NaiveTime::from_hms_milli(12, 34, 56, 789) - Duration::seconds(300);

        return users
            .filter(schema::users::id.eq(any(self.get_friends_ids())))
            .filter(schema::users::last_activity.gt(NaiveDateTime::new(d, t)))
            .limit(6)
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_draft_posts(&self) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::user_id.eq(self.id))
            .filter(schema::posts::types.eq("f"))
            .filter(schema::posts::community_id.is_null())
            .order(schema::posts::created.desc())
            .load::<Post>(&_connection)
            .expect("E.");
    }
    pub fn get_draft_posts_of_community_with_pk(&self, community_id: i32) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::user_id.eq(self.id))
            .filter(schema::posts::types.eq("f"))
            .filter(schema::posts::community_id.eq(community_id))
            .order(schema::posts::created.desc())
            .load::<Post>(&_connection)
            .expect("E.");
    }
    pub fn get_good_list(&self) -> GoodList {
        use crate::schema::good_lists::dsl::good_lists;

        let _connection = establish_connection();
        let _good_lists  = good_lists
            .filter(schema::good_lists::user_id.eq(self.id))
            .filter(schema::good_lists::types.eq("a"))
            .filter(schema::good_lists::community_id.is_null())
            .limit(1)
            .load::<GoodList>(&_connection)
            .expect("E.");
        if _good_lists.len() > 0 {
            return _good_lists
            .into_iter()
            .nth(0)
            .unwrap();
        }
        else {
            use crate::models::{NewGoodList, UserGoodListPosition, NewUserGoodListPosition};
            let d = NaiveDate::from_ymd(2015, 6, 3);
            let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
            use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

            let new_list = NewGoodList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          "a".to_string(),
                    description:     None,
                    created:         NaiveDateTime::new(d, t),
                    count:           0,
                    repost:          0,
                    copy:            0,
                    position:        0,
                    can_see_el:      "a".to_string(),
                    can_see_comment: "a".to_string(),
                    create_el:       "g".to_string(),
                    create_comment:  "a".to_string(),
                    copy_el:         "g".to_string(),
                };
            let _goods_list = diesel::insert_into(schema::good_lists::table)
                .values(&new_list)
                .get_result::<GoodList>(&_connection)
                .expect("Error saving good_list.");

            let _new_goods_list_position = NewUserGoodListPosition {
                user_id:  self.id,
                list_id:  _goods_list.id,
                position: 1,
                types:    "a".to_string(),
            };
            let _goods_list_position = diesel::insert_into(schema::user_good_list_positions::table)
                .values(&_new_goods_list_position)
                .get_result::<UserGoodListPosition>(&_connection)
                .expect("Error saving good_list_position.");
            return _goods_list;
        }
    }
    pub fn get_music_list(&self) -> MusicList {
        use crate::schema::music_lists::dsl::music_lists;

        let _connection = establish_connection();
        let _music_lists  = music_lists
            .filter(schema::music_lists::user_id.eq(self.id))
            .filter(schema::music_lists::types.eq("a"))
            .filter(schema::music_lists::community_id.is_null())
            .limit(1)
            .load::<MusicList>(&_connection)
            .expect("E.");
        if _music_lists.len() > 0 {
            return _music_lists
            .into_iter()
            .nth(0)
            .unwrap();
        }
        else {
            use crate::models::{NewMusicList, UserMusicListPosition, NewUserMusicListPosition};
            let d = NaiveDate::from_ymd(2015, 6, 3);
            let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
            use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

            let new_list = NewMusicList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          "a".to_string(),
                    description:     None,
                    image:           None,
                    created:         NaiveDateTime::new(d, t),
                    count:           0,
                    repost:          0,
                    copy:            0,
                    position:        0,
                    can_see_el:      "a".to_string(),
                    create_el:       "g".to_string(),
                    copy_el:         "g".to_string(),
                };
            let _musics_list = diesel::insert_into(schema::music_lists::table)
                .values(&new_list)
                .get_result::<MusicList>(&_connection)
                .expect("Error saving music_list.");

            let _new_musics_list_position = NewUserMusicListPosition {
                user_id:  self.id,
                list_id:  _musics_list.id,
                position: 1,
                types:    "a".to_string(),
            };
            let _musics_list_position = diesel::insert_into(schema::user_music_list_positions::table)
                .values(&_new_musics_list_position)
                .get_result::<UserMusicListPosition>(&_connection)
                .expect("Error saving music_list_position.");
            return _musics_list;
        }
    }
    pub fn get_video_list(&self) -> VideoList {
        use crate::schema::video_lists::dsl::video_lists;

        let _connection = establish_connection();
        let _video_lists  = video_lists
            .filter(schema::video_lists::user_id.eq(self.id))
            .filter(schema::video_lists::types.eq("a"))
            .filter(schema::video_lists::community_id.is_null())
            .limit(1)
            .load::<VideoList>(&_connection)
            .expect("E.");
        if _video_lists.len() > 0 {
            return _video_lists
            .into_iter()
            .nth(0)
            .unwrap();
        }
        else {
            use crate::models::{NewVideoList, UserVideoListPosition, NewUserVideoListPosition};
            let d = NaiveDate::from_ymd(2015, 6, 3);
            let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
            use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

            let new_list = NewVideoList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          "a".to_string(),
                    description:     None,
                    created:         NaiveDateTime::new(d, t),
                    count:           0,
                    repost:          0,
                    copy:            0,
                    position:        0,
                    can_see_el:      "a".to_string(),
                    can_see_comment: "a".to_string(),
                    create_el:       "g".to_string(),
                    create_comment:  "a".to_string(),
                    copy_el:         "g".to_string(),
                };
            let _videos_list = diesel::insert_into(schema::video_lists::table)
                .values(&new_list)
                .get_result::<VideoList>(&_connection)
                .expect("Error saving video_list.");

            let _new_videos_list_position = NewUserVideoListPosition {
                user_id:  self.id,
                list_id:  _videos_list.id,
                position: 1,
                types:    "a".to_string(),
            };
            let _videos_list_position = diesel::insert_into(schema::user_video_list_positions::table)
                .values(&_new_videos_list_position)
                .get_result::<UserVideoListPosition>(&_connection)
                .expect("Error saving video_list_position.");
            return _videos_list;
        }
    }
    pub fn get_photo_list(&self) -> PhotoList {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        let _photo_lists  = photo_lists
            .filter(schema::photo_lists::user_id.eq(self.id))
            .filter(schema::photo_lists::types.eq("a"))
            .filter(schema::photo_lists::community_id.is_null())
            .limit(1)
            .load::<PhotoList>(&_connection)
            .expect("E.");
        if _photo_lists.len() > 0 {
            return _photo_lists
            .into_iter()
            .nth(0)
            .unwrap();
        }
        else {
            use crate::models::{NewPhotoList, UserPhotoListPosition, NewUserPhotoListPosition};
            let d = NaiveDate::from_ymd(2015, 6, 3);
            let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
            use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

            let new_list = NewPhotoList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          "a".to_string(),
                    description:     None,
                    cover_photo:     None,
                    created:         NaiveDateTime::new(d, t),
                    count:           0,
                    repost:          0,
                    copy:            0,
                    position:        0,
                    can_see_el:      "a".to_string(),
                    can_see_comment: "a".to_string(),
                    create_el:       "g".to_string(),
                    create_comment:  "a".to_string(),
                    copy_el:         "g".to_string(),
                };
            let _photos_list = diesel::insert_into(schema::photo_lists::table)
                .values(&new_list)
                .get_result::<PhotoList>(&_connection)
                .expect("Error saving photo_list.");

            let _new_photos_list_position = NewUserPhotoListPosition {
                user_id:  self.id,
                list_id:  _photos_list.id,
                position: 1,
                types:    "a".to_string(),
            };
            let _photos_list_position = diesel::insert_into(schema::user_photo_list_positions::table)
                .values(&_new_photos_list_position)
                .get_result::<UserPhotoListPosition>(&_connection)
                .expect("Error saving photo_list_position.");
            return _photos_list;
        }
    }
    pub fn get_avatar_pk(&self) -> i32 {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        let _photo_lists  = photo_lists
            .filter(schema::photo_lists::user_id.eq(self.id))
            .filter(schema::photo_lists::types.eq("d"))
            .filter(schema::photo_lists::community_id.is_null())
            .limit(1)
            .load::<PhotoList>(&_connection)
            .expect("E.");
        if _photo_lists.len() > 0 {
            use crate::schema::photos::dsl::photos;
            let list = _photo_lists.into_iter().nth(0).unwrap();
            let _photos  = photos
                .filter(schema::photos::photo_list_id.eq(list.id))
                .filter(schema::photos::types.eq("a"))
                .limit(1)
                .load::<Photo>(&_connection)
                .expect("E.");
            if _photos.len() > 0 {
                return _photos.into_iter().nth(0).unwrap().id;
            }
        }
        return 0;
    }
    pub fn get_post_list(&self) -> PostList {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        let _post_lists  = post_lists
            .filter(schema::post_lists::user_id.eq(self.id))
            .filter(schema::post_lists::types.eq("a"))
            .filter(schema::post_lists::community_id.is_null())
            .limit(1)
            .load::<PostList>(&_connection)
            .expect("E.");
        if _post_lists.len() > 0 {
            return _post_lists
            .into_iter()
            .nth(0)
            .unwrap();
        }
        else {
            use crate::models::{NewPostList, UserPostListPosition, NewUserPostListPosition};
            let d = NaiveDate::from_ymd(2015, 6, 3);
            let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
            use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

            let new_list = NewPostList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          "a".to_string(),
                    description:     None,
                    created:         NaiveDateTime::new(d, t),
                    count:           0,
                    repost:          0,
                    copy:            0,
                    position:        0,
                    can_see_el:      "a".to_string(),
                    can_see_comment: "a".to_string(),
                    create_el:       "g".to_string(),
                    create_comment:  "a".to_string(),
                    copy_el:         "g".to_string(),
                };
            let _posts_list = diesel::insert_into(schema::post_lists::table)
                .values(&new_list)
                .get_result::<PostList>(&_connection)
                .expect("Error saving post_list.");

            let _new_posts_list_position = NewUserPostListPosition {
                user_id:  self.id,
                list_id:  _posts_list.id,
                position: 1,
                types:    "a".to_string(),
            };
            let _posts_list_position = diesel::insert_into(schema::user_post_list_positions::table)
                .values(&_new_posts_list_position)
                .get_result::<UserPostListPosition>(&_connection)
                .expect("Error saving post_list_position.");
            return _posts_list;
        }
    }
    pub fn get_doc_list(&self) -> DocList {
        use crate::schema::doc_lists::dsl::doc_lists;

        let _connection = establish_connection();
        let _doc_lists  = doc_lists
            .filter(schema::doc_lists::user_id.eq(self.id))
            .filter(schema::doc_lists::types.eq("a"))
            .filter(schema::doc_lists::community_id.is_null())
            .limit(1)
            .load::<DocList>(&_connection)
            .expect("E.");
        if _doc_lists.len() > 0 {
            return _doc_lists
            .into_iter()
            .nth(0)
            .unwrap();
        }
        else {
            use crate::models::{NewDocList, UserDocListPosition, NewUserDocListPosition};
            let d = NaiveDate::from_ymd(2015, 6, 3);
            let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
            use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

            let new_list = NewDocList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          "a".to_string(),
                    description:     None,
                    created:         NaiveDateTime::new(d, t),
                    count:           0,
                    repost:          0,
                    copy:            0,
                    position:        0,
                    can_see_el:      "a".to_string(),
                    create_el:       "g".to_string(),
                    copy_el:         "g".to_string(),
                };
            let _docs_list = diesel::insert_into(schema::doc_lists::table)
                .values(&new_list)
                .get_result::<DocList>(&_connection)
                .expect("Error saving doc_list.");

            let _new_docs_list_position = NewUserDocListPosition {
                user_id:  self.id,
                list_id:  _docs_list.id,
                position: 1,
                types:    "a".to_string(),
            };
            let _docs_list_position = diesel::insert_into(schema::user_doc_list_positions::table)
                .values(&_new_docs_list_position)
                .get_result::<UserDocListPosition>(&_connection)
                .expect("Error saving doc_list_position.");
            return _docs_list;
        }
    }
    pub fn get_survey_list(&self) -> SurveyList {
        use crate::schema::survey_lists::dsl::survey_lists;

        let _connection = establish_connection();
        let _survey_lists  = survey_lists
            .filter(schema::survey_lists::user_id.eq(self.id))
            .filter(schema::survey_lists::types.eq("a"))
            .filter(schema::survey_lists::community_id.is_null())
            .limit(1)
            .load::<SurveyList>(&_connection)
            .expect("E.");
        if _survey_lists.len() > 0 {
            return _survey_lists
            .into_iter()
            .nth(0)
            .unwrap();
        }
        else {
            use crate::models::{NewSurveyList, UserSurveyListPosition, NewUserSurveyListPosition};
            let d = NaiveDate::from_ymd(2015, 6, 3);
            let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
            use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

            let new_list = NewSurveyList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          "a".to_string(),
                    description:     None,
                    created:         NaiveDateTime::new(d, t),
                    count:           0,
                    repost:          0,
                    copy:            0,
                    position:        0,
                    can_see_el:      "a".to_string(),
                    create_el:       "g".to_string(),
                    copy_el:         "g".to_string(),
                };
            let _surveys_list = diesel::insert_into(schema::survey_lists::table)
                .values(&new_list)
                .get_result::<SurveyList>(&_connection)
                .expect("Error saving survey_list.");

            let _new_surveys_list_position = NewUserSurveyListPosition {
                user_id:  self.id,
                list_id:  _surveys_list.id,
                position: 1,
                types:    "a".to_string(),
            };
            let _surveys_list_position = diesel::insert_into(schema::user_survey_list_positions::table)
                .values(&_new_surveys_list_position)
                .get_result::<UserSurveyListPosition>(&_connection)
                .expect("Error saving survey_list_position.");
            return _surveys_list;
        }
    }
    pub fn get_selected_post_list_pk(&self) -> i32 {
        use crate::schema::user_post_list_positions::dsl::user_post_list_positions;
        use crate::models::UserPostListPosition;

        let _connection = establish_connection();
        let _post_list_positions  = user_post_list_positions
            .filter(schema::user_post_list_positions::user_id.eq(self.id))
            .filter(schema::user_post_list_positions::types.eq("a"))
            .limit(1)
            .load::<UserPostListPosition>(&_connection)
            .expect("E.");
        if _post_list_positions.len() > 0 {
            return _survey_lists
            .into_iter()
            .nth(0)
            .unwrap()
            .list;
        }
        else {
            return self.get_post_list().id;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: i32,
    pub phone: String,
}
