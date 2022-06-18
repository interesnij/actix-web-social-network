use crate::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use diesel::prelude::*;
use crate::schema;
use crate::models::{
    Chat, Message, UserLocation, Smile, Sticker, Community, UserProfile, Friend,
    Post, Photo, Music, Video,
    //Survey,
    StickerCategorie,
    Doc, Good, SmileCategorie,
    PostList, PhotoList, MusicList, VideoList, SurveyList, DocList, GoodList,
    Follow, Notification, UserPrivate, UserBlock, PostCategorie,
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
    pub link:          String,
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
    pub link:          String,
    pub birthday:      chrono::NaiveDate,
    pub last_activity: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub phone:    String,
    pub password: String,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name="users"]
pub struct EditLinkUser {
    pub link:  String,
}
#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name="users"]
pub struct EditNameUser {
    pub first_name:  String,
    pub last_name:   String,
}
#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name="users"]
pub struct EditPhoneUser {
    pub phone:  String,
}
#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name="users"]
pub struct EditEmailUser {
    pub email:  String,
}
#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name="users"]
pub struct EditPasswordUser {
    pub password: String,
}
#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name="users"]
pub struct EditTypesUser {
    pub types: i16,
}

impl User {
    pub fn get_full_name(&self) -> String {
        self.first_name.clone() + &" ".to_string() + &self.last_name.clone()
    }
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn get_bb_avatar(&self) -> String {
        if self.b_avatar.is_some() {
            return self.b_avatar.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/no_img/list.jpg".to_string();
        }
    }

    pub fn message_reposts_count(&self) -> String {
        use crate::schema::user_reposts::dsl::user_reposts;
        use crate::models::UserRepost;

        let _connection = establish_connection();

        let count = user_reposts
            .filter(schema::user_reposts::user_id.eq(self.id))
            .filter(schema::user_reposts::message_id.is_not_null())
            .load::<UserRepost>(&_connection)
            .expect("E.")
            .len();

        if count == 0 {
            return "".to_string();
        }
        else {
            return ", из них в сообщениях - ".to_string() + &count.to_string();
        }
    }
    pub fn reposts(&self, limit: i64, offset: i64) -> Vec<Post> {
        use crate::schema::user_reposts::dsl::user_reposts;
        use crate::schema::posts::dsl::posts;
        use crate::models::UserRepost;

        let _connection = establish_connection();
        let item_reposts = user_reposts
            .filter(schema::user_reposts::user_id.eq(self.id))
            .filter(schema::user_reposts::post_id.is_not_null())
            .limit(limit)
            .offset(offset)
            .load::<UserRepost>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in item_reposts.iter() {
            stack.push(_item.post_id.unwrap());
        };
        return posts
            .filter(schema::posts::id.eq_any(stack))
            .limit(6)
            .load::<Post>(&_connection)
            .expect("E");
    }
    pub fn window_reposts(&self) -> Vec<Post> {
        use crate::schema::user_reposts::dsl::user_reposts;
        use crate::schema::posts::dsl::posts;
        use crate::models::UserRepost;

        let _connection = establish_connection();
        let item_reposts = user_reposts
            .filter(schema::user_reposts::user_id.eq(self.id))
            .filter(schema::user_reposts::post_id.is_not_null())
            .limit(6)
            .load::<UserRepost>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in item_reposts.iter() {
            stack.push(_item.post_id.unwrap());
        };
        return posts
            .filter(schema::posts::id.eq_any(stack))
            .limit(6)
            .load::<Post>(&_connection)
            .expect("E");
    }

    pub fn count_users() -> usize {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .load::<User>(&_connection)
            .expect("E")
            .len();
    }

    pub fn get_b_avatar(&self) -> String {
        let avatar_pk = self.get_avatar_pk();
        if avatar_pk != 0 {
            return "<img src='".to_string() + &self.b_avatar.as_ref().unwrap() + &"' class='detail_photo pointer' photo-pk='".to_string() + &avatar_pk.to_string() + &"'>".to_string();
        }
        else {
            return "<img src='/static/images/no_img/b_avatar.png' />".to_string();
        }
    }
    pub fn get_ss_avatar(&self) -> String {
        if self.s_avatar.is_some() {
            return self.s_avatar.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/no_img/list.jpg".to_string();
        }
    }
    pub fn get_s_avatar(&self) -> String {
        if self.s_avatar.is_some() {
            return "<img style='border-radius:30px;width:30px;' alt='image' src='".to_owned() + &self.s_avatar.as_deref().unwrap().to_string() +  &"' />".to_string();
        }
        else {
            return "<svg fill='currentColor' class='svg_default svg_default_30' viewBox='0 0 24 24'><path d='M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z'/><path d='M0 0h24v24H0z' fill='none'/></svg>".to_string();
        }
    }
    pub fn get_40_avatar(&self) -> String {
        if self.s_avatar.is_some() {
            return "<img style='border-radius:40px;width:40px;' alt='image' src='".to_owned() + &self.s_avatar.as_deref().unwrap().to_string() +  &"' />".to_string();
        }
        else {
            return "<svg fill='currentColor' class='svg_default svg_default_40' viewBox='0 0 24 24'><path d='M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z'/><path d='M0 0h24v24H0z' fill='none'/></svg>".to_string();
        }
    }
    pub fn get_50_avatar(&self) -> String {
        if self.s_avatar.is_some() {
            return "<img style='border-radius:50px;width:50px;' alt='image' src='".to_owned() + &self.s_avatar.as_deref().unwrap().to_string() +  &"' />".to_string();
        }
        else {
            return "<svg fill='currentColor' class='svg_default svg_default_50' viewBox='0 0 24 24'><path d='M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z'/><path d='M0 0h24v24H0z' fill='none'/></svg>".to_string();
        }
    }

    pub fn get_slug(&self) -> String {
        return "@".to_string() + &self.link.replace("/", "").to_string();
    }
    pub fn get_description(&self) -> String {
        return "<a href='".to_string() + &self.link.to_string() + &"' target='_blank'>".to_string() + &self.get_full_name() + &"</a>".to_string();
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
    pub fn delete_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            1 => 11,
            3 => 13,
            7 => 17,
            6 => 16,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::users::types.eq(close_case))
            .get_result::<User>(&_connection)
            .expect("E");
       return true;
    }
    pub fn restore_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            11 => 1,
            13 => 3,
            17 => 7,
            16 => 6,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::users::types.eq(close_case))
            .get_result::<User>(&_connection)
            .expect("E");
       return true;
    }

    pub fn get_plus_or_create_populate_smile(&self, smile_id: i32) {
        use crate::schema::user_populate_smiles::dsl::user_populate_smiles;
        use crate::models::{UserPopulateSmile, NewUserPopulateSmile};

        let _connection = establish_connection();

        let populate_smiles = user_populate_smiles
            .filter(schema::user_populate_smiles::user_id.eq(self.id))
            .filter(schema::user_populate_smiles::smile_id.eq(smile_id))
            .load::<UserPopulateSmile>(&_connection)
            .expect("E");
        if populate_smiles.len() > 0 {
            let populate_smile = populate_smiles.into_iter().nth(0).unwrap();
            diesel::update(&populate_smile)
                .set(schema::user_populate_smiles::count.eq(populate_smile.count + 1))
                .get_result::<UserPopulateSmile>(&_connection)
                .expect("Error.");
        } else {
            let new_smile = NewUserPopulateSmile {
                user_id:  self.id,
                smile_id: smile_id,
                count:    1,
            };
            diesel::insert_into(schema::user_populate_smiles::table)
                .values(&new_smile)
                .get_result::<UserPopulateSmile>(&_connection)
                .expect("Error.");
        }
    }
    pub fn get_plus_or_create_populate_sticker(&self, sticker_id: i32) {
        use crate::schema::user_populate_stickers::dsl::user_populate_stickers;
        use crate::models::{UserPopulateSticker, NewUserPopulateSticker};

        let _connection = establish_connection();

        let populate_stickers = user_populate_stickers
            .filter(schema::user_populate_stickers::user_id.eq(self.id))
            .filter(schema::user_populate_stickers::sticker_id.eq(sticker_id))
            .load::<UserPopulateSticker>(&_connection)
            .expect("E");
        if populate_stickers.len() > 0 {
            let populate_sticker = populate_stickers.into_iter().nth(0).unwrap();
            diesel::update(&populate_sticker)
                .set(schema::user_populate_stickers::count.eq(populate_sticker.count + 1))
                .get_result::<UserPopulateSticker>(&_connection)
                .expect("Error.");
        } else {
            let new_sticker = NewUserPopulateSticker {
                user_id:    self.id,
                sticker_id: sticker_id,
                count:      1,
            };
            diesel::insert_into(schema::user_populate_stickers::table)
                .values(&new_sticker)
                .get_result::<UserPopulateSticker>(&_connection)
                .expect("Error.");
        }
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

            let new_manager_chat = NewChat{
                name:             Some("Рассылка новостей".to_string()),
                types:            3,
                community_id:     None,
                user_id:          self.id,
                position:         10,
                members:          1,
                created:          chrono::Local::now().naive_utc(),
                can_add_members:  "f".to_string(),
                can_fix_item:     "b".to_string(),
                can_mention:      "f".to_string(),
                can_add_admin:    "f".to_string(),
                can_add_design:   "f".to_string(),
                can_see_settings: "f".to_string(),
                can_see_log:      "f".to_string(),
                reactions:         None,
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
                created: chrono::Local::now().naive_utc(),
                no_disturb: None,
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

            let new_manager_chat = NewChat{
                name:             Some("Рассылка служународу.рус".to_string()),
                types:            11,
                community_id:     None,
                user_id:          self.id,
                position:         10,
                members:          1,
                created:          chrono::Local::now().naive_utc(),
                can_add_members:  "f".to_string(),
                can_fix_item:     "b".to_string(),
                can_mention:      "f".to_string(),
                can_add_admin:    "f".to_string(),
                can_add_design:   "f".to_string(),
                can_see_settings: "f".to_string(),
                can_see_log:      "f".to_string(),
                reactions:         None,
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
                created: chrono::Local::now().naive_utc(),
                no_disturb: None,
            };
            diesel::insert_into(schema::chat_users::table)
                .values(&new_chat_user)
                .get_result::<ChatUser>(&_connection)
                .expect("E.");
            return manager_chat.id;
        }
    }
    pub fn get_deleted_support_chats(&self, limit: i64, offset: i64) -> Vec<Chat> {
        use crate::schema::chats::dsl::chats;

        let _connection = establish_connection();
        return chats
            .filter(schema::chats::user_id.eq(self.id))
            .filter(schema::chats::types.between(40,46))
            .order(schema::chats::id.desc())
            .limit(limit)
            .offset(offset)
            .load::<Chat>(&_connection)
            .expect("E");
    }
    pub fn count_deleted_support_chats(&self) -> usize {
        use crate::schema::chats::dsl::chats;

        let _connection = establish_connection();
        return chats
            .filter(schema::chats::user_id.eq(self.id))
            .filter(schema::chats::types.between(40,46))
            .order(schema::chats::id.desc())
            .load::<Chat>(&_connection)
            .expect("E")
            .len();
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
    pub fn get_favourite_messages(&self, limit: i64, offset: i64) -> Vec<Message> {
        use crate::schema::messages::dsl::messages;
        use crate::schema::message_options::dsl::message_options;
        use crate::models::MessageOption;

        let _connection = establish_connection();
        let all_option_messages = message_options
            .filter(schema::message_options::user_id.eq(self.id))
            .filter(schema::message_options::is_favourite.eq(true))
            .order(schema::message_options::id.desc())
            .limit(limit)
            .offset(offset)
            .load::<MessageOption>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in all_option_messages.iter() {
            stack.push(_item.message_id);
        };
        return messages
            .filter(schema::messages::id.eq_any(stack))
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

    pub fn is_list_in_collection(&self, types: &String) -> bool {
        let pk: i32 = types[3..].parse().unwrap();
        let code = &types[..3];
        if code == &"lpo".to_string() {
            use crate::utils::get_post_list;
            let list = get_post_list(pk);
            return list.get_users_ids().iter().any(|&i| i==self.id);
        }
        else if code == &"lgo".to_string() {
            use crate::utils::get_good_list;
            let list = get_good_list(pk);
            return list.get_users_ids().iter().any(|&i| i==self.id);
        }
        else if code == &"lph".to_string() {
            use crate::utils::get_photo_list;
            let list = get_photo_list(pk);
            return list.get_users_ids().iter().any(|&i| i==self.id);
        }
        else if code == &"lvi".to_string() {
            use crate::utils::get_video_list;
            let list = get_video_list(pk);
            return list.get_users_ids().iter().any(|&i| i==self.id);
        }
        else if code == &"lsu".to_string() {
            use crate::utils::get_survey_list;
            let list = get_survey_list(pk);
            return list.get_users_ids().iter().any(|&i| i==self.id);
        }
        else if code == &"ldo".to_string() {
            use crate::utils::get_doc_list;
            let list = get_doc_list(pk);
            return list.get_users_ids().iter().any(|&i| i==self.id);
        }
        else if code == &"lmu".to_string() {
            use crate::utils::get_music_list;
            let list = get_music_list(pk);
            return list.get_users_ids().iter().any(|&i| i==self.id);
        }
        return false;
    }

    pub fn get_fixed_posts(&self) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::user_id.eq(self.id))
            .filter(schema::posts::types.eq("b"))
            .filter(schema::posts::community_id.is_null())
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

        let _connection = establish_connection();
        let all_populate_smiles = user_populate_smiles
            .filter(schema::user_populate_smiles::user_id.eq(self.id))
            .order(schema::user_populate_smiles::count.desc())
            .load::<UserPopulateSmile>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in all_populate_smiles.iter() {
            stack.push(_item.smile_id);
        };
        return smiles
            .filter(schema::smiles::id.eq_any(stack))
            .load::<Smile>(&_connection)
            .expect("E.");
    }
    pub fn get_smiles(limit: i64, offset: i64) -> Vec<Smile> {
        use crate::schema::smiles::dsl::smiles;

        let _connection = establish_connection();
        return smiles
            .order(schema::smiles::position.asc())
            .limit(limit)
            .offset(offset)
            .load::<Smile>(&_connection)
            .expect("E.");
    }
    pub fn count_smiles() -> usize {
        use crate::schema::smiles::dsl::smiles;

        let _connection = establish_connection();
        return smiles
            .load::<Smile>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn get_populate_stickers(&self) -> Vec<Sticker> {
        use crate::schema::stickers::dsl::stickers;
        use crate::schema::user_populate_stickers::dsl::user_populate_stickers;
        use crate::models::UserPopulateSticker;

        let _connection = establish_connection();
        let all_populate_stickers = user_populate_stickers
            .filter(schema::user_populate_stickers::user_id.eq(self.id))
            .order(schema::user_populate_stickers::count.desc())
            .load::<UserPopulateSticker>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in all_populate_stickers.iter() {
            stack.push(_item.sticker_id);
        };
        return stickers
            .filter(schema::stickers::id.eq_any(stack))
            .load::<Sticker>(&_connection)
            .expect("E.");
    }
    pub fn get_sticker_categories(limit: i64, offset: i64) -> Vec<StickerCategorie> {
        use crate::schema::sticker_categories::dsl::sticker_categories;

        let _connection = establish_connection();
        return sticker_categories
            .order(schema::sticker_categories::position.asc())
            .limit(limit)
            .offset(offset)
            .load::<StickerCategorie>(&_connection)
            .expect("E.");
    }
    pub fn get_smilies_categories() -> Vec<SmileCategorie> {
        use crate::schema::smile_categories::dsl::smile_categories;

        let _connection = establish_connection();
        return smile_categories
            .order(schema::smile_categories::position.asc())
            .load::<SmileCategorie>(&_connection)
            .expect("E.");
    }
    pub fn get_stickers_for_category(cat_id:i32, limit: i64, offset: i64) -> Vec<Sticker> {
        use crate::schema::stickers::dsl::stickers;

        let _connection = establish_connection();
        return stickers
            .filter(schema::stickers::sticker_categorie_id.eq(cat_id))
            .order(schema::stickers::position.asc())
            .limit(limit)
            .offset(offset)
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
        return support_users
            .filter(schema::support_users::manager_id.eq(&self.id))
            .load::<SupportUser>(&_connection)
            .expect("E").len() > 0;
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
        use chrono::Duration;
        return (self.last_activity + Duration::seconds(300)) > chrono::Local::now().naive_utc();
    }
    pub fn get_online_display(&self) -> String {
        let device = match self.is_desctop() {
            true => "&nbsp;<svg style='width: 17px;' class='svg_default' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M20 18c1.1 0 1.99-.9 1.99-2L22 6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2H0v2h24v-2h-4zM4 6h16v10H4V6z'/></svg>",
            false => "&nbsp;<svg style='width: 17px;' class='svg_default' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M17 1.01L7 1c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z'/></svg>",
        };
        let gender = match self.is_men() {
            true => "<i>Был </i>",
            false => "<i>Была </i>",
        };
        if self.is_online() == true {
            return "<i>Онлайн</i>".to_string() + &device;
        }
        else {
            return gender.to_owned() + &self.last_activity.format("%d/%m/%Y").to_string() + &device;
        }
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

    pub fn get_staffed_communities_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;

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
        return community_ids;
    }
    pub fn get_staffed_communities(&self, limit: i64, offset: i64) -> Vec<Community> {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::id.eq_any(self.get_staffed_communities_ids()))
            .limit(limit)
            .offset(offset)
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
            .filter(schema::featured_user_communities::community_id.is_null())
            .order(schema::featured_user_communities::id.desc())
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
            .filter(schema::featured_user_communities::community_id.is_null())
            .order(schema::featured_user_communities::id.desc())
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

        let _connection = establish_connection();
        return users
            .filter(schema::users::id.eq_any(self.get_featured_friends_ids()))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_6_featured_friends(&self) -> Vec<User> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::id.eq_any(self.get_6_featured_friends_ids()))
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
            .filter(schema::featured_user_communities::user_id.is_null())
            .order(schema::featured_user_communities::id.desc())
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
            .filter(schema::featured_user_communities::user_id.is_null())
            .order(schema::featured_user_communities::id.desc())
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
    pub fn is_user_in_block(&self, user_id: i32) -> bool {
        // user_id заблокирован у self
        use crate::schema::user_blocks::dsl::user_blocks;

        let _connection = establish_connection();
        return user_blocks
            .filter(schema::user_blocks::blocked_user_id.eq(user_id))
            .filter(schema::user_blocks::user_block_i.eq(self.id))
            .load::<UserBlock>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_self_user_in_block(&self, user_id: i32) -> bool {
        use crate::schema::user_blocks::dsl::user_blocks;

        let _connection = establish_connection();
        return user_blocks
            .filter(schema::user_blocks::user_block_i.eq(user_id))
            .filter(schema::user_blocks::blocked_user_id.eq(self.id))
            .load::<UserBlock>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_connected_with_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        return friends
            .filter(schema::friends::user_id.eq(user_id))
            .filter(schema::friends::target_user_id.eq(self.id))
            .load::<Friend>(&_connection)
            .expect("E.")
            .len() > 0;
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
        return communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_follow_from_community(&self, community_id: i32) -> bool {
        use crate::schema::community_follows::dsl::community_follows;
        use crate::models::CommunityFollow;

        let _connection = establish_connection();
        return community_follows
            .filter(schema::community_follows::user_id.eq(self.id))
            .filter(schema::community_follows::community_id.eq(community_id))
            .load::<CommunityFollow>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_creator_community(&self, community_id: i32) -> bool {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        let com = communitys
            .filter(schema::communitys::id.eq(community_id))
            .load::<Community>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
        return com.user_id == self.id;
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

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::user_id.eq(self.id))
            .filter(schema::follows::followed_user.eq(user_id))
            .load::<Follow>(&_connection)
            .expect("E.").len() > 0;
    }
    pub fn is_followers_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::followed_user.eq(self.id))
            .filter(schema::follows::user_id.eq(user_id))
            .load::<Follow>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_followers_user_view(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::followed_user.eq(self.id))
            .filter(schema::follows::user_id.eq(user_id))
            .filter(schema::follows::view.eq(true))
            .load::<Follow>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn get_buttons_profile(&self, user_id: i32) -> String {
        let mut suffix: String = "".to_string();
        if self.perm > 19 {
            suffix = "staff_".to_string();
        }
        if self.is_user_in_block(user_id) {
            return "desctop/users/button/".to_owned() + &suffix + &"blocked_user.stpl".to_string();
        }
        else if self.is_self_user_in_block(user_id) {
            return "desctop/users/button/".to_owned() + &suffix + &"blocker_user.stpl".to_string();
        }
        else if self.is_connected_with_user_with_id(user_id){
            return "desctop/users/button/".to_owned() + &suffix + &"frend_user.stpl".to_string();
        }
        else if self.is_followers_user_view(user_id){
            return "desctop/users/button/".to_owned() + &suffix + &"follow_user.stpl".to_string();
        }
        else if self.is_following_user_with_id(user_id){
            return "desctop/users/button/".to_owned() + &suffix + &"following_user.stpl".to_string();
        }
        else if self.is_followers_user_with_id(user_id){
            return "desctop/users/button/".to_owned() + &suffix + &"follow_view_user.stpl".to_string();
        }
        else {
            return "desctop/users/button/".to_owned() + &suffix + &"default_user.stpl".to_string();
        }
    }
    pub fn get_profile(&self) -> UserProfile {
        use crate::schema::user_profiles::dsl::user_profiles;

        let _connection = establish_connection();
        let profile = user_profiles
            .filter(schema::user_profiles::id.eq(self.id))
            .load::<UserProfile>(&_connection)
            .expect("E.");

        if profile.len() > 0 {
            return profile.into_iter().nth(0).unwrap();
        }
        else {
            use crate::models::NewUserProfile;

            let _new_profile = NewUserProfile {
                user_id: self.id,
                posts: 0,
                views_post: 0,
                friends: 0,
                follows: 0,
                communities: 0,
                photos: 0,
                goods: 0,
                docs: 0,
                tracks: 0,
                videos: 0,
                articles: 0,
                planners: 0,
                avatar_id: None,
                activity: None,
                interests: None,
                favorite_music: None,
                favorite_films: None,
                favorite_books: None,
                favorite_game: None,
                about: None,
                survey: 0,
                playlist: 0,
            };
            let profile = diesel::insert_into(schema::user_profiles::table)
                .values(&_new_profile)
                .get_result::<UserProfile>(&_connection)
                .expect("Error saving user_profile.");
            return profile;
        }
    }
    pub fn is_have_followers(&self) -> bool {
        return self.get_profile().follows > 0
    }
    pub fn is_have_followings(&self) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::user_id.eq(self.id))
            .load::<Follow>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_have_blacklist(&self) -> bool {
        use crate::schema::user_blocks::dsl::user_blocks;

        let _connection = establish_connection();
        return user_blocks
            .filter(schema::user_blocks::user_block_i.eq(self.id))
            .load::<UserBlock>(&_connection)
            .expect("E.")
            .len() > 0;
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
    pub fn is_have_good(&self) -> bool {
        return self.get_profile().goods > 0;
    }
    pub fn is_have_post(&self) -> bool {
        return self.get_profile().posts > 0;
    }

    pub fn count_no_view_followers(&self) -> usize {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::followed_user.eq(self.id))
            .filter(schema::follows::view.eq(false))
            .load::<Follow>(&_connection)
            .expect("E.")
            .len();
    }
    pub fn count_following(&self) -> usize {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::user_id.eq(self.id))
            .load::<Follow>(&_connection)
            .expect("E.")
            .len();
    }
    pub fn count_followers(&self) -> i32 {
        return self.get_profile().follows;
    }
    pub fn count_followers_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_followers(),
            " подписчик".to_string(),
            " подписчика".to_string(),
            " подписчиков".to_string(),
        );
    }
    pub fn count_followers_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_followers(),
            " подписчик".to_string(),
            " подписчика".to_string(),
            " подписчиков".to_string(),
        );
    }

    pub fn count_blacklist(&self) -> usize {
        use crate::schema::user_blocks::dsl::user_blocks;

        let _connection = establish_connection();
        return user_blocks
            .filter(schema::user_blocks::user_block_i.eq(self.id))
            .load::<UserBlock>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn count_goods(&self) -> i32 {
        return self.get_profile().goods;
    }
    pub fn count_goods_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_goods(),
            " товар".to_string(),
            " товара".to_string(),
            " товаров".to_string(),
        );
    }
    pub fn count_goods_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_goods(),
            " товар".to_string(),
            " товара".to_string(),
            " товаров".to_string(),
        );
    }

    pub fn count_tracks(&self) -> i32 {
        return self.get_profile().tracks;
    }
    pub fn count_tracks_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_tracks(),
            " трек".to_string(),
            " трека".to_string(),
            " треков".to_string(),
        );
    }
    pub fn count_tracks_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_tracks(),
            " трек".to_string(),
            " трека".to_string(),
            " треков".to_string(),
        );
    }

    pub fn count_photos(&self) -> i32 {
        return self.get_profile().photos;
    }
    pub fn count_photos_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru(
            self.count_photos(),
            " фотография".to_string(),
            " фотографии".to_string(),
            " фотографий".to_string(),
        );
    }
    pub fn count_photos_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt(
            self.count_photos(),
            " фотография".to_string(),
            " фотографии".to_string(),
            " фотографий".to_string(),
        );
    }

    pub fn count_docs(&self) -> i32 {
        return self.get_profile().docs;
    }
    pub fn count_docs_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru(
            self.count_docs(),
            " документ".to_string(),
            " документа".to_string(),
            " документов".to_string(),
        );
    }
    pub fn count_docs_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt(
            self.count_docs(),
            " документ".to_string(),
            " документа".to_string(),
            " документов".to_string(),
        );
    }

    pub fn count_posts(&self) -> i32 {
        return self.get_profile().posts;
    }
    pub fn count_posts_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru(
            self.count_posts(),
            " запись".to_string(),
            " записи".to_string(),
            " записей".to_string(),
        );
    }
    pub fn count_posts_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt(
            self.count_posts(),
            " запись".to_string(),
            " записи".to_string(),
            " записей".to_string(),
        );
    }

    pub fn count_articles(&self) -> i32 {
        return self.get_profile().articles;
    }
    pub fn count_articles_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru(
            self.count_articles(),
            " статья".to_string(),
            " статьи".to_string(),
            " статей".to_string(),
        );
    }
    pub fn count_articles_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt(
            self.count_articles(),
            " статья".to_string(),
            " статьи".to_string(),
            " статей".to_string(),
        );
    }

    pub fn count_communities(&self) -> i32 {
        return self.get_profile().communities;
    }
    pub fn count_communities_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_communities(),
            " сообщество".to_string(),
            " сообщества".to_string(),
            " сообществ".to_string(),
        );
    }
    pub fn count_communities_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_communities(),
            " сообщество".to_string(),
            " сообщества".to_string(),
            " сообществ".to_string(),
        );
    }

    pub fn count_videos(&self) -> i32 {
        return self.get_profile().videos;
    }
    pub fn count_videos_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_videos(),
            " ролик".to_string(),
            " ролика".to_string(),
            " роликов".to_string(),
        );
    }
    pub fn count_videos_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_videos(),
            " ролик".to_string(),
            " ролика".to_string(),
            " роликов".to_string(),
        );
    }

    pub fn get_blocked_users(&self, limit: i64, offset: i64) -> Vec<User> {
        use crate::schema::user_blocks::dsl::user_blocks;
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let all_user_blocks = user_blocks
            .filter(schema::user_blocks::user_block_i.eq(self.id))
            .order(schema::user_blocks::id.desc())
            .limit(limit)
            .offset(offset)
            .load::<UserBlock>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in all_user_blocks.iter() {
            stack.push(_item.blocked_user_id);
        };
        return users
            .filter(schema::users::id.eq_any(stack))
            .load::<User>(&_connection)
            .expect("E.");
    }

    pub fn count_friends(&self) -> i32 {
        return self.get_profile().friends;
    }
    pub fn count_friends_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_friends(),
            " друг".to_string(),
            " друга".to_string(),
            " друзей".to_string(),
        );
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
    pub fn plus_surveys(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::survey.eq(profile.survey + count))
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
    pub fn minus_surveys(&self, count: i32) -> bool {
        let profile = self.get_profile();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_profiles::survey.eq(profile.survey - count))
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
    pub fn get_6_friends_ids(&self) -> Vec<i32> {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        let mut stack = Vec::new();
        let _friends = friends
            .filter(schema::friends::user_id.eq(self.id))
            .order(schema::friends::visited.desc())
            .limit(6)
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
        for friend in self.get_friends(500, 0).iter() {
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

    pub fn get_friends(&self, limit: i64, offset: i64) -> Vec<User> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::id.eq_any(self.get_friends_ids()))
            .limit(limit)
            .offset(offset)
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_6_friends(&self) -> Vec<User> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::id.eq_any(self.get_6_friends_ids()))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_6_communities_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let _user_communities = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .order(schema::communities_memberships::visited.desc())
            .limit(6)
            .load::<CommunitiesMembership>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for _item in _user_communities.iter() {
            stack.push(_item.community_id);
        };
        return stack;
    }
    pub fn get_communities(&self, limit: i64, offset: i64) -> Vec<Community> {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::schema::communitys::dsl::communitys;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let _user_communities = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .order(schema::communities_memberships::visited.desc())
            .limit(limit)
            .offset(offset)
            .load::<CommunitiesMembership>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for _item in _user_communities.iter() {
            stack.push(_item.community_id);
        };
        return communitys
            .filter(schema::communitys::id.eq_any(stack))
            .load::<Community>(&_connection)
            .expect("E.");
    }
    pub fn get_6_communities(&self) -> Vec<Community> {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::schema::communitys::dsl::communitys;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let _user_communities = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .order(schema::communities_memberships::visited.desc())
            .limit(6)
            .load::<CommunitiesMembership>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for _item in _user_communities.iter() {
            stack.push(_item.community_id);
        };
        return communitys
            .filter(schema::communitys::id.eq_any(stack))
            .load::<Community>(&_connection)
            .expect("E.");
    }
    pub fn get_online_friends(&self, limit: i64, offset: i64) -> Vec<User> {
        use crate::schema::users::dsl::users;
        use chrono::Duration;

        let _connection = establish_connection();

        return users
            .filter(schema::users::id.eq_any(self.get_friends_ids()))
            .filter(schema::users::last_activity.gt(chrono::Local::now().naive_utc() - Duration::seconds(300)))
            .limit(limit)
            .offset(offset)
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_online_friends_count(&self) -> usize {
        return self.get_online_friends(500, 0).len();
    }
    pub fn get_6_online_friends(&self) -> Vec<User> {
        use crate::schema::users::dsl::users;
        use chrono::Duration;

        let _connection = establish_connection();

        return users
            .filter(schema::users::id.eq_any(self.get_friends_ids()))
            .filter(schema::users::last_activity.gt(chrono::Local::now().naive_utc() - Duration::seconds(300)))
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
            .filter(schema::good_lists::types.eq(1))
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

            let new_list = NewGoodList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          1,
                    description:    None,
                    image:          None,
                    created:         chrono::Local::now().naive_utc(),
                    count:           0,
                    repost:          0,
                    copy:            0,
                    position:        0,
                    can_see_el:      "a".to_string(),
                    can_see_comment: "a".to_string(),
                    create_el:       "g".to_string(),
                    create_comment:  "a".to_string(),
                    copy_el:         "g".to_string(),
                    reactions:       Some("1, 2".to_string()),
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
            .filter(schema::music_lists::types.eq(1))
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

            let new_list = NewMusicList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          1,
                    description:     None,
                    image:           None,
                    created:         chrono::Local::now().naive_utc(),
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
    pub fn get_saved_playlist(&self) -> MusicList {
        private = self.get_profile();
        if private.playlist == 0 {
            return self.get_music_list();
        }
        else {
            use crate::utils::get_music_list;
            return get_music_list(private.playlist);
        }
    }
    pub fn get_video_list(&self) -> VideoList {
        use crate::schema::video_lists::dsl::video_lists;

        let _connection = establish_connection();
        let _video_lists  = video_lists
            .filter(schema::video_lists::user_id.eq(self.id))
            .filter(schema::video_lists::types.eq(1))
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

            let new_list = NewVideoList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          1,
                    description:    None,
                    image:          None,
                    created:         chrono::Local::now().naive_utc(),
                    count:           0,
                    repost:          0,
                    copy:            0,
                    position:        0,
                    can_see_el:      "a".to_string(),
                    can_see_comment: "a".to_string(),
                    create_el:       "g".to_string(),
                    create_comment:  "a".to_string(),
                    copy_el:         "g".to_string(),
                    reactions:       Some("1, 2".to_string()),
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
            .filter(schema::photo_lists::types.eq(1))
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

            let new_list = NewPhotoList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          1,
                    description:     None,
                    cover_photo:     None,
                    created:         chrono::Local::now().naive_utc(),
                    count:           0,
                    repost:          0,
                    copy:            0,
                    position:        0,
                    can_see_el:      "a".to_string(),
                    can_see_comment: "a".to_string(),
                    create_el:       "g".to_string(),
                    create_comment:  "a".to_string(),
                    copy_el:         "g".to_string(),
                    reactions:       Some("1, 2".to_string()),
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
            .filter(schema::photo_lists::types.eq(4))
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
            .filter(schema::post_lists::types.eq(1))
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

            let new_list = NewPostList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          1,
                    description:    None,
                    image:          None,
                    created:         chrono::Local::now().naive_utc(),
                    count:           0,
                    repost:          0,
                    copy:            0,
                    position:        0,
                    can_see_el:      "a".to_string(),
                    can_see_comment: "a".to_string(),
                    create_el:       "g".to_string(),
                    create_comment:  "a".to_string(),
                    copy_el:         "g".to_string(),
                    reactions:       Some("1, 2".to_string()),
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
            .filter(schema::doc_lists::types.eq(1))
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

            let new_list = NewDocList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          1,
                    description:    None,
                    image:          None,
                    created:         chrono::Local::now().naive_utc(),
                    count:           0,
                    repost:          0,
                    copy:            0,
                    position:        0,
                    can_see_el:      "a".to_string(),
                    create_el:       "g".to_string(),
                    copy_el:         "g".to_string(),
                    //reactions:       Some("1, 2".to_string()),
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
            .filter(schema::survey_lists::types.eq(1))
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

            let new_list = NewSurveyList{
                    name:          "Основной список".to_string(),
                    community_id:   None,
                    user_id:        self.id,
                    types:          1,
                    description:    None,
                    image:          None,
                    created:         chrono::Local::now().naive_utc(),
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
            .order(schema::user_post_list_positions::position.desc())
            .limit(1)
            .load::<UserPostListPosition>(&_connection)
            .expect("E.");
        if _post_list_positions.len() > 0 {
            return _post_list_positions
            .into_iter()
            .nth(0)
            .unwrap()
            .list_id;
        }
        else {
            return self.get_post_list().id;
        }
    }
    pub fn get_selected_photo_list_pk(&self) -> i32 {
        use crate::schema::user_photo_list_positions::dsl::user_photo_list_positions;
        use crate::models::UserPhotoListPosition;

        let _connection = establish_connection();
        let _photo_list_positions  = user_photo_list_positions
            .filter(schema::user_photo_list_positions::user_id.eq(self.id))
            .filter(schema::user_photo_list_positions::types.eq("a"))
            .limit(1)
            .load::<UserPhotoListPosition>(&_connection)
            .expect("E.");
        if _photo_list_positions.len() > 0 {
            return _photo_list_positions
            .into_iter()
            .nth(0)
            .unwrap()
            .list_id;
        }
        else {
            return self.get_photo_list().id;
        }
    }
    pub fn get_selected_doc_list_pk(&self) -> i32 {
        use crate::schema::user_doc_list_positions::dsl::user_doc_list_positions;
        use crate::models::UserDocListPosition;

        let _connection = establish_connection();
        let _doc_list_positions  = user_doc_list_positions
            .filter(schema::user_doc_list_positions::user_id.eq(self.id))
            .filter(schema::user_doc_list_positions::types.eq("a"))
            .limit(1)
            .load::<UserDocListPosition>(&_connection)
            .expect("E.");
        if _doc_list_positions.len() > 0 {
            return _doc_list_positions
            .into_iter()
            .nth(0)
            .unwrap()
            .list_id;
        }
        else {
            return self.get_doc_list().id;
        }
    }
    pub fn get_selected_good_list_pk(&self) -> i32 {
        use crate::schema::user_good_list_positions::dsl::user_good_list_positions;
        use crate::models::UserGoodListPosition;

        let _connection = establish_connection();
        let _good_list_positions  = user_good_list_positions
            .filter(schema::user_good_list_positions::user_id.eq(self.id))
            .filter(schema::user_good_list_positions::types.eq("a"))
            .limit(1)
            .load::<UserGoodListPosition>(&_connection)
            .expect("E.");
        if _good_list_positions.len() > 0 {
            return _good_list_positions
            .into_iter()
            .nth(0)
            .unwrap()
            .list_id;
        }
        else {
            return self.get_good_list().id;
        }
    }
    pub fn get_selected_music_list_pk(&self) -> i32 {
        use crate::schema::user_music_list_positions::dsl::user_music_list_positions;
        use crate::models::UserMusicListPosition;

        let _connection = establish_connection();
        let _music_list_positions  = user_music_list_positions
            .filter(schema::user_music_list_positions::user_id.eq(self.id))
            .filter(schema::user_music_list_positions::types.eq("a"))
            .limit(1)
            .load::<UserMusicListPosition>(&_connection)
            .expect("E.");
        if _music_list_positions.len() > 0 {
            return _music_list_positions
            .into_iter()
            .nth(0)
            .unwrap()
            .list_id;
        }
        else {
            return self.get_music_list().id;
        }
    }
    pub fn get_selected_video_list_pk(&self) -> i32 {
        use crate::schema::user_video_list_positions::dsl::user_video_list_positions;
        use crate::models::UserVideoListPosition;

        let _connection = establish_connection();
        let _video_list_positions  = user_video_list_positions
            .filter(schema::user_video_list_positions::user_id.eq(self.id))
            .filter(schema::user_video_list_positions::types.eq("a"))
            .limit(1)
            .load::<UserVideoListPosition>(&_connection)
            .expect("E.");
        if _video_list_positions.len() > 0 {
            return _video_list_positions
            .into_iter()
            .nth(0)
            .unwrap()
            .list_id;
        }
        else {
            return self.get_video_list().id;
        }
    }
    pub fn get_selected_survey_list_pk(&self) -> i32 {
        use crate::schema::user_survey_list_positions::dsl::user_survey_list_positions;
        use crate::models::UserSurveyListPosition;

        let _connection = establish_connection();
        let _survey_list_positions  = user_survey_list_positions
            .filter(schema::user_survey_list_positions::user_id.eq(self.id))
            .filter(schema::user_survey_list_positions::types.eq("a"))
            .limit(1)
            .load::<UserSurveyListPosition>(&_connection)
            .expect("E.");
        if _survey_list_positions.len() > 0 {
            return _survey_list_positions
            .into_iter()
            .nth(0)
            .unwrap()
            .list_id;
        }
        else {
            return self.get_survey_list().id;
        }
    }
    pub fn get_post_lists(&self) -> Vec<PostList> {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        return post_lists
            .filter(schema::post_lists::user_id.eq(self.id))
            .filter(schema::post_lists::types.lt(10))
            .filter(schema::post_lists::community_id.is_null())
            .order(schema::post_lists::created.desc())
            .load::<PostList>(&_connection)
            .expect("E.");
    }
    pub fn get_post_lists_new_position(&self) -> i16 {
        return (self.get_post_lists().iter().count() + 1).try_into().unwrap();
    }
    pub fn get_post_lists_from_staffed_comunities(&self) -> Vec<PostList> {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        return post_lists
            .filter(schema::post_lists::community_id.eq_any(self.get_staffed_communities_ids()))
            .filter(schema::post_lists::types.lt(10))
            .order(schema::post_lists::created.desc())
            .order(schema::post_lists::created.desc())
            .load::<PostList>(&_connection)
            .expect("E.");
    }
    pub fn get_survey_lists(&self) -> Vec<SurveyList> {
        use crate::schema::survey_lists::dsl::survey_lists;

        let _connection = establish_connection();
        return survey_lists
            .filter(schema::survey_lists::user_id.eq(self.id))
            .filter(schema::survey_lists::types.lt(10))
            .filter(schema::survey_lists::community_id.is_null())
            .order(schema::survey_lists::created.desc())
            .load::<SurveyList>(&_connection)
            .expect("E.");
    }
    pub fn get_survey_lists_new_position(&self) -> i16 {
        return (self.get_survey_lists().iter().count() + 1).try_into().unwrap();
    }
    pub fn get_survey_lists_from_staffed_comunities(&self) -> Vec<SurveyList> {
        use crate::schema::survey_lists::dsl::survey_lists;

        let _connection = establish_connection();
        return survey_lists
            .filter(schema::survey_lists::community_id.eq_any(self.get_staffed_communities_ids()))
            .filter(schema::survey_lists::types.lt(10))
            .order(schema::survey_lists::created.desc())
            .load::<SurveyList>(&_connection)
            .expect("E.");
    }
    pub fn get_photo_lists(&self) -> Vec<PhotoList> {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        return photo_lists
            .filter(schema::photo_lists::user_id.eq(self.id))
            .filter(schema::photo_lists::types.lt(10))
            .filter(schema::photo_lists::community_id.is_null())
            .order(schema::photo_lists::created.desc())
            .load::<PhotoList>(&_connection)
            .expect("E.");
    }
    pub fn get_photo_lists_new_position(&self) -> i16 {
        return (self.get_photo_lists().iter().count() + 1).try_into().unwrap();
    }
    pub fn get_photo_lists_from_staffed_comunities(&self) -> Vec<PhotoList> {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        return photo_lists
            .filter(schema::photo_lists::community_id.eq_any(self.get_staffed_communities_ids()))
            .filter(schema::photo_lists::types.lt(10))
            .order(schema::photo_lists::created.desc())
            .load::<PhotoList>(&_connection)
            .expect("E.");
    }
    pub fn get_video_lists(&self) -> Vec<VideoList> {
        use crate::schema::video_lists::dsl::video_lists;

        let _connection = establish_connection();
        return video_lists
            .filter(schema::video_lists::user_id.eq(self.id))
            .filter(schema::video_lists::types.lt(10))
            .filter(schema::video_lists::community_id.is_null())
            .order(schema::video_lists::created.desc())
            .load::<VideoList>(&_connection)
            .expect("E.");
    }
    pub fn get_video_lists_new_position(&self) -> i16 {
        return (self.get_video_lists().iter().count() + 1).try_into().unwrap();
    }
    pub fn get_video_lists_from_staffed_comunities(&self) -> Vec<VideoList> {
        use crate::schema::video_lists::dsl::video_lists;

        let _connection = establish_connection();
        return video_lists
            .filter(schema::video_lists::community_id.eq_any(self.get_staffed_communities_ids()))
            .filter(schema::video_lists::types.lt(10))
            .load::<VideoList>(&_connection)
            .expect("E.");
    }
    pub fn get_music_lists(&self) -> Vec<MusicList> {
        use crate::schema::music_lists::dsl::music_lists;

        let _connection = establish_connection();
        return music_lists
            .filter(schema::music_lists::user_id.eq(self.id))
            .filter(schema::music_lists::types.lt(10))
            .filter(schema::music_lists::community_id.is_null())
            .load::<MusicList>(&_connection)
            .expect("E.");
    }
    pub fn get_music_lists_new_position(&self) -> i16 {
        return (self.get_music_lists().iter().count() + 1).try_into().unwrap();
    }
    pub fn get_music_lists_from_staffed_comunities(&self) -> Vec<MusicList> {
        use crate::schema::music_lists::dsl::music_lists;

        let _connection = establish_connection();
        return music_lists
            .filter(schema::music_lists::community_id.eq_any(self.get_staffed_communities_ids()))
            .filter(schema::music_lists::types.lt(10))
            .load::<MusicList>(&_connection)
            .expect("E.");
    }
    pub fn get_good_lists(&self) -> Vec<GoodList> {
        use crate::schema::good_lists::dsl::good_lists;

        let _connection = establish_connection();
        return good_lists
            .filter(schema::good_lists::user_id.eq(self.id))
            .filter(schema::good_lists::types.lt(10))
            .filter(schema::good_lists::community_id.is_null())
            .load::<GoodList>(&_connection)
            .expect("E.");
    }
    pub fn get_good_lists_new_position(&self) -> i16 {
        return (self.get_good_lists().iter().count() + 1).try_into().unwrap();
    }
    pub fn get_good_lists_from_staffed_comunities(&self) -> Vec<GoodList> {
        use crate::schema::good_lists::dsl::good_lists;

        let _connection = establish_connection();
        return good_lists
            .filter(schema::good_lists::community_id.eq_any(self.get_staffed_communities_ids()))
            .filter(schema::good_lists::types.lt(10))
            .load::<GoodList>(&_connection)
            .expect("E.");
    }

    pub fn get_doc_lists(&self) -> Vec<DocList> {
        use crate::schema::doc_lists::dsl::doc_lists;

        let _connection = establish_connection();
        return doc_lists
            .filter(schema::doc_lists::user_id.eq(self.id))
            .filter(schema::doc_lists::types.lt(10))
            .filter(schema::doc_lists::community_id.is_null())
            .load::<DocList>(&_connection)
            .expect("E.");
    }
    pub fn get_doc_lists_new_position(&self) -> i16 {
        return (self.get_doc_lists().iter().count() + 1).try_into().unwrap();
    }
    pub fn get_doc_lists_from_staffed_comunities(&self) -> Vec<DocList> {
        use crate::schema::doc_lists::dsl::doc_lists;

        let _connection = establish_connection();
        return doc_lists
            .filter(schema::doc_lists::community_id.eq_any(self.get_staffed_communities_ids()))
            .filter(schema::doc_lists::types.lt(10))
            .load::<DocList>(&_connection)
            .expect("E.");
    }
    pub fn get_6_photos(&self) -> Vec<Photo> {
        use crate::schema::photos::dsl::photos;

        let _connection = establish_connection();
        return photos
            .filter(schema::photos::user_id.eq(self.id))
            .filter(schema::photos::types.eq("a"))
            .filter(schema::photos::community_id.is_null())
            .order(schema::photos::created.desc())
            .limit(6)
            .load::<Photo>(&_connection)
            .expect("E.");
    }
    pub fn get_6_docs(&self) -> Vec<Doc> {
        use crate::schema::docs::dsl::docs;

        let _connection = establish_connection();
        return docs
            .filter(schema::docs::user_id.eq(self.id))
            .filter(schema::docs::types.eq("a"))
            .filter(schema::docs::community_id.is_null())
            .order(schema::docs::created.desc())
            .limit(6)
            .load::<Doc>(&_connection)
            .expect("E.");
    }
    pub fn get_6_tracks(&self) -> Vec<Music> {
        use crate::schema::musics::dsl::musics;

        let _connection = establish_connection();
        return musics
            .filter(schema::musics::user_id.eq(self.id))
            .filter(schema::musics::types.eq("a"))
            .filter(schema::musics::community_id.is_null())
            .order(schema::musics::created.desc())
            .limit(6)
            .load::<Music>(&_connection)
            .expect("E.");
    }
    pub fn get_2_videos(&self) -> Vec<Video> {
        use crate::schema::videos::dsl::videos;

        let _connection = establish_connection();
        return videos
            .filter(schema::videos::user_id.eq(self.id))
            .filter(schema::videos::types.eq("a"))
            .filter(schema::videos::community_id.is_null())
            .order(schema::videos::created.desc())
            .limit(2)
            .load::<Video>(&_connection)
            .expect("E.");
    }
    pub fn get_3_goods(&self) -> Vec<Good> {
        use crate::schema::goods::dsl::goods;

        let _connection = establish_connection();
        return goods
            .filter(schema::goods::user_id.eq(self.id))
            .filter(schema::goods::types.eq("a"))
            .filter(schema::goods::community_id.is_null())
            .order(schema::goods::created.desc())
            .limit(2)
            .load::<Good>(&_connection)
            .expect("E.");
    }
    pub fn get_followers(&self, limit: i64, offset: i64) -> Vec<User> {
        use crate::schema::follows::dsl::follows;
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let followers =  follows
            .filter(schema::follows::followed_user.eq(self.id))
            .order(schema::follows::visited.desc())
            .limit(limit)
            .offset(offset)
            .load::<Follow>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in followers.iter() {
            stack.push(_item.user_id);
        };
        return users
            .filter(schema::users::id.eq_any(stack))
            .filter(schema::users::types.lt(11))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_6_followers(&self) -> Vec<User> {
        use crate::schema::follows::dsl::follows;
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let followers =  follows
            .filter(schema::follows::followed_user.eq(self.id))
            .order(schema::follows::visited.desc())
            .limit(6)
            .load::<Follow>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in followers.iter() {
            stack.push(_item.user_id);
        };
        return users
            .filter(schema::users::id.eq_any(stack))
            .filter(schema::users::types.lt(11))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_all_users_count(&self) -> usize {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        if self.types == 3 {
            return users
                .filter(schema::users::types.eq(7))
                .or_filter(schema::users::perm.lt(9))
                .load::<User>(&_connection)
                .expect("E.").len();
        }
        else {
            return users
                .filter(schema::users::types.lt(10))
                .load::<User>(&_connection)
                .expect("E.").len();
        }
    }
    pub fn get_all_communities_count() -> usize {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::types.lt(10))
            .load::<Community>(&_connection)
            .expect("E.").len();
    }
    pub fn get_users(&self, limit: i64, offset: i64) -> Vec<User> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        if self.types == 3 {
            return users
                .filter(schema::users::types.eq(7))
                .or_filter(schema::users::perm.lt(9))
                .limit(limit)
                .offset(offset)
                .load::<User>(&_connection)
                .expect("E.");
        }
        else {
            return users
                .filter(schema::users::types.lt(10))
                .load::<User>(&_connection)
                .expect("E.");
        }
    }
    pub fn get_anon_users(limit: i64, offset: i64) -> Vec<User> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::types.lt(10))
            .limit(limit)
            .offset(offset)
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_all_communities(limit: i64, offset: i64) -> Vec<Community> {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::types.lt(10))
            .limit(limit)
            .offset(offset)
            .load::<Community>(&_connection)
            .expect("E.");
    }
    pub fn get_anon_users_count() -> usize {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::types.lt(10))
            .load::<User>(&_connection)
            .expect("E.")
            .len();
    }
    pub fn get_followings(&self, limit: i64, offset: i64) -> Vec<User> {
        use crate::schema::follows::dsl::follows;
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let followers =  follows
            .filter(schema::follows::user_id.eq(self.id))
            .order(schema::follows::visited.desc())
            .limit(limit)
            .offset(offset)
            .load::<Follow>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in followers.iter() {
            stack.push(_item.followed_user);
        };
        return users
            .filter(schema::users::id.eq_any(stack))
            .filter(schema::users::types.lt(11))
            .load::<User>(&_connection)
            .expect("E.");
    }

    pub fn get_common_friends_of_user(&self, user: &User, limit: i64, offset: i64) -> Vec<User> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let self_friends = self.get_friends_ids();
        let user_friends = user.get_friends_ids();
        let mut stack = Vec::new();
        for int in self_friends.iter() {
            if user_friends.iter().any(|i| i==int) {
                stack.push(int);
            }
        }
        return users
            .filter(schema::users::id.eq_any(stack))
            .filter(schema::users::types.lt(11))
            .limit(limit)
            .offset(offset)
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn count_common_friends_of_user(&self, user: &User) -> usize {
        let _connection = establish_connection();
        let self_friends = self.get_friends_ids();
        let user_friends = user.get_friends_ids();
        let mut stack = Vec::new();
        for int in self_friends.iter() {
            if user_friends.iter().any(|i| i==int) {
                stack.push(int);
            }
        }
        return stack.len();
    }

    pub fn is_have_common_friends_of_user(&self, user: &User) -> bool {
        let _connection = establish_connection();
        let self_friends = self.get_friends_ids();
        let user_friends = user.get_friends_ids();
        for int in self_friends.iter() {
            if user_friends.iter().any(|i| i==int) {
                return true;
            }
        }
        return false;
    }
    pub fn get_6_common_friends_of_user(&self, user: &User) -> Vec<User> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let self_friends = self.get_friends_ids();
        let user_friends = user.get_friends_ids();
        let mut stack = Vec::new();
        for (i, int) in self_friends.iter().enumerate() {
            if i < 7 && user_friends.iter().any(|i| i==int) {
                stack.push(int);
            }
        }
        return users
            .filter(schema::users::id.eq_any(stack))
            .filter(schema::users::types.lt(11))
            .limit(6)
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_common_friends_of_community(&self, community_id: i32, limit: i64, offset: i64) -> Vec<User> {
        use crate::schema::users::dsl::users;
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let self_friends = self.get_friends_ids();
        let members_of_community = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .limit(limit)
            .offset(offset)
            .load::<CommunitiesMembership>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for member in members_of_community.iter() {
            if self_friends.iter().any(|i| i==&member.user_id) {
                stack.push(member.user_id);
            }
        }
        return users
            .filter(schema::users::id.eq_any(stack))
            .filter(schema::users::types.lt(11))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_common_friends_of_community_count_ru(&self, community_id: i32) -> String  {
        use crate::utils::get_count_for_ru;
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let self_friends = self.get_friends_ids();
        let members_of_community = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E.");
        let mut count = 0;
        for member in members_of_community.iter() {
            if self_friends.iter().any(|i| i==&member.user_id) {
                count += 1;
            }
        }

        return get_count_for_ru (
            count,
            " друг".to_string(),
            " друга".to_string(),
            " друзей".to_string(),
        );
    }
    pub fn get_users_ids_for_main_news(&self) -> Vec<i32> {
        use crate::schema::news_user_communities::dsl::news_user_communities;
        use crate::models::NewsUserCommunitie;

        let _connection = establish_connection();

        let news_users = news_user_communities
            .filter(schema::news_user_communities::owner.eq(self.id))
            .filter(schema::news_user_communities::community_id.is_null())
            .filter(schema::news_user_communities::mute.eq(false))
            .filter(schema::news_user_communities::sleep.lt(chrono::Local::now().naive_utc()))
            .load::<NewsUserCommunitie>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for member in news_users.iter() {
            stack.push(member.user_id.unwrap());
        }
        return stack;
    }
    pub fn get_communities_ids_for_main_news(&self) -> Vec<i32> {
        //use crate::schema::users::dsl::users;
        use crate::schema::news_user_communities::dsl::news_user_communities;
        use crate::models::NewsUserCommunitie;

        let _connection = establish_connection();

        let news_users = news_user_communities
            .filter(schema::news_user_communities::owner.eq(self.id))
            .filter(schema::news_user_communities::user_id.is_null())
            .filter(schema::news_user_communities::mute.eq(false))
            .filter(schema::news_user_communities::sleep.lt(chrono::Local::now().naive_utc()))
            .load::<NewsUserCommunitie>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for member in news_users.iter() {
            stack.push(member.community_id.unwrap());
        }
        return stack;
    }
    pub fn get_users_ids_for_main_notifications(&self) -> Vec<i32> {
        use crate::schema::notify_user_communities::dsl::notify_user_communities;
        use crate::models::NotifyUserCommunitie;

        let _connection = establish_connection();

        let news_users = notify_user_communities
            .filter(schema::notify_user_communities::owner.eq(self.id))
            .filter(schema::notify_user_communities::community_id.is_null())
            .filter(schema::notify_user_communities::mute.eq(false))
            .filter(schema::notify_user_communities::sleep.lt(chrono::Local::now().naive_utc()))
            .load::<NotifyUserCommunitie>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for member in news_users.iter() {
            stack.push(member.user_id.unwrap());
        }
        return stack;
    }
    pub fn get_communities_ids_for_main_notifications(&self) -> Vec<i32> {
        use crate::schema::notify_user_communities::dsl::notify_user_communities;
        use crate::models::NotifyUserCommunitie;

        let _connection = establish_connection();

        let news_users = notify_user_communities
            .filter(schema::notify_user_communities::owner.eq(self.id))
            .filter(schema::notify_user_communities::user_id.is_null())
            .filter(schema::notify_user_communities::mute.eq(false))
            .filter(schema::notify_user_communities::sleep.lt(chrono::Local::now().naive_utc()))
            .load::<NotifyUserCommunitie>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for member in news_users.iter() {
            stack.push(member.community_id.unwrap());
        }
        return stack;
    }
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(1))
            .load::<ModeratedPenaltie>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
        return penaltie.expiration.unwrap().format("%d/%m/%Y").to_string();
    }
    pub fn get_moderated_description(&self) -> String {
        use crate::schema::moderateds::dsl::moderateds;
        use crate::models::Moderated;

        let _connection = establish_connection();

        let moder = moderateds
            .filter(schema::moderateds::object_id.eq(self.id))
            .filter(schema::moderateds::types.eq(1))
            .load::<Moderated>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
        if moder.description.is_some() {
            return moder.description.unwrap().to_string();
        }
        else {
            return "Предупреждение за нарушение правил соцсети трезвый.рус".to_string();
        }
    }
    pub fn get_all_chats(&self, limit: i64, offset: i64) -> Vec<Chat> {
        use crate::schema::chat_users::dsl::chat_users;
        use crate::schema::chats::dsl::chats;
        use crate::models::ChatUser;

        let _connection = establish_connection();
        let members_of_chats = chat_users
            .filter(schema::chat_users::user_id.eq(self.id))
            .filter(schema::chat_users::types.eq("a"))
            .limit(limit)
            .offset(offset)
            .load::<ChatUser>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for member in members_of_chats.iter() {
            stack.push(member.chat_id);
        }
        let chat_list = chats
            .filter(schema::chats::id.eq_any(&stack))
            .filter(schema::chats::types.lt(20))
            .load::<Chat>(&_connection)
            .expect("E.");

        for chat in chat_list.iter() {
            if !chat.is_group() && !chat.is_public() && !chat.is_not_empty() {
                stack.retain(|&x| x != chat.id);
            }
        }

        return chats
            .filter(schema::chats::id.eq_any(stack))
            .filter(schema::chats::types.lt(20))
            .order(schema::chats::created.desc())
            .load::<Chat>(&_connection)
            .expect("E.");
    }
    pub fn get_all_chats_count(&self) -> usize {
        use crate::schema::chat_users::dsl::chat_users;
        use crate::schema::chats::dsl::chats;
        use crate::models::ChatUser;

        let _connection = establish_connection();
        let members_of_chats = chat_users
            .filter(schema::chat_users::user_id.eq(self.id))
            .filter(schema::chat_users::types.eq("a"))
            .load::<ChatUser>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for member in members_of_chats.iter() {
            stack.push(member.chat_id);
        }
        return chats
            .filter(schema::chats::id.eq_any(stack))
            .filter(schema::chats::types.lt(20))
            .order(schema::chats::created.desc())
            .load::<Chat>(&_connection)
            .expect("E.")
            .len();
    }
    pub fn is_administrator_of_chat(&self, chat_id: i32) -> bool {
        use crate::schema::chat_users::dsl::chat_users;
        use crate::models::ChatUser;

        let _connection = establish_connection();
        let all_chats = chat_users
            .filter(schema::chat_users::user_id.eq(self.id))
            .filter(schema::chat_users::chat_id.eq(chat_id))
            .filter(schema::chat_users::types.eq("a"))
            .load::<ChatUser>(&_connection)
            .expect("E");

        for _item in all_chats.iter() {
            if _item.is_administrator {
                return true;
            }
        };
        return false;
    }
    pub fn is_member_of_chat(&self, chat_id: i32) -> bool {
        use crate::schema::chat_users::dsl::chat_users;
        use crate::models::ChatUser;

        let _connection = establish_connection();
        return chat_users
            .filter(schema::chat_users::user_id.eq(self.id))
            .filter(schema::chat_users::chat_id.eq(chat_id))
            .filter(schema::chat_users::types.eq("a"))
            .load::<ChatUser>(&_connection)
            .expect("E").len() > 0;
    }
    pub fn count_unread_chats(&self) -> i32 {
        use crate::schema::messages::dsl::messages;

        let _connection = establish_connection();
        let chats = self.get_all_chats(100, 0);
        let mut count = 0;
        for chat in chats {
            if messages
                .filter(schema::messages::chat_id.eq(chat.id))
                .filter(schema::messages::unread.eq(true))
                .filter(schema::messages::user_id.ne(self.id))
                .filter(schema::messages::types.lt(10))
                .load::<Message>(&_connection)
                .expect("E").len() > 0 {
                    count += 1;
                }
        }
        return count
    }
    pub fn get_user_notifications(&self) -> Vec<Notification> {
        use crate::schema::notifications::dsl::notifications;

        let _connection = establish_connection();
        return notifications
            .filter(schema::notifications::user_id.eq_any(self.get_users_ids_for_main_notifications()))
            .or_filter(schema::notifications::community_id.eq_any(self.get_communities_ids_for_main_notifications()))
            .filter(schema::notifications::user_set_id.is_null())
            .filter(schema::notifications::object_set_id.is_null())
            .load::<Notification>(&_connection)
            .expect("E");
    }
    pub fn count_user_notifications(&self) -> usize {
        use crate::schema::notifications::dsl::notifications;

        let _connection = establish_connection();
        return notifications
            .filter(schema::notifications::recipient_id.eq(self.id))
            .filter(schema::notifications::community_id.is_null())
            .filter(schema::notifications::status.eq("a"))
            .load::<Notification>(&_connection)
            .expect("E").len();
    }
    pub fn unread_notify_count(&self) -> String {
        let mut count = self.count_user_notifications();
        if self.is_staffed_user() {
            for _community in self.get_staffed_communities(50, 0).iter() {
                count += 1;
            }
        }
        if count > 0 {
            return count.to_string();
        }
        else {
            return "".to_string();
        }
    }

    pub fn get_can_see_all_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_all.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_all_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_all.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_all_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_all_exclude_users_ids());
    }
    pub fn get_can_see_all_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_all_include_users_ids());
    }

    pub fn get_can_see_community_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_community.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_community_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_community.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_community_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_community_exclude_users_ids());
    }
    pub fn get_can_see_community_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_community_include_users_ids());
    }
    pub fn get_can_see_info_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_info.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_info_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_info.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_info_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_info_exclude_users_ids());
    }
    pub fn get_can_see_info_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_info_include_users_ids());
    }

    pub fn get_can_see_friend_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_friend.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_friend_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_friend.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_friend_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_friend_exclude_users_ids());
    }
    pub fn get_can_see_friend_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_friend_include_users_ids());
    }
    pub fn get_can_send_message_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_send_message.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_send_message_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_send_message.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_send_message_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_send_message_exclude_users_ids());
    }
    pub fn get_can_send_message_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_send_message_include_users_ids());
    }

    pub fn get_can_add_in_chat_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_add_in_chat.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_add_in_chat_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_add_in_chat.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
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
    pub fn get_can_see_post_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_post.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_post_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_post.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_post_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_post_exclude_users_ids());
    }
    pub fn get_can_see_post_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_post_include_users_ids());
    }

    pub fn get_can_see_photo_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_photo.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_photo_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_photo.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_photo_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_photo_exclude_users_ids());
    }
    pub fn get_can_see_photo_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_photo_include_users_ids());
    }

    pub fn get_can_see_good_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_good.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_good_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_good.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_good_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_good_exclude_users_ids());
    }
    pub fn get_can_see_good_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_good_include_users_ids());
    }

    pub fn get_can_see_video_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_video.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_video_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_video.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_video_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_video_exclude_users_ids());
    }
    pub fn get_can_see_video_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_video_include_users_ids());
    }

    pub fn get_can_see_music_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_music.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_music_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_music.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_music_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_music_exclude_users_ids());
    }
    pub fn get_can_see_music_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_music_include_users_ids());
    }

    pub fn get_can_see_planner_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_planner.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_planner_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_planner.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_planner_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_planner_exclude_users_ids());
    }
    pub fn get_can_see_planner_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_planner_include_users_ids());
    }

    pub fn get_can_see_doc_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_doc.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_doc_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_doc.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_doc_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_doc_exclude_users_ids());
    }
    pub fn get_can_see_doc_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_doc_include_users_ids());
    }

    pub fn get_can_see_survey_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_survey.eq("b"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_survey_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::models::FriendsVisiblePerm;

        let _connection = establish_connection();
        let items = friends_visible_perms
            .filter(schema::friends_visible_perms::user_id.eq_any(self.get_friends_ids()))
            .filter(schema::friends_visible_perms::can_see_survey.eq("a"))
            .load::<FriendsVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_survey_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_survey_exclude_users_ids());
    }
    pub fn get_can_see_survey_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_survey_include_users_ids());
    }

    pub fn get_private_model(&self) -> UserPrivate {
        use crate::schema::user_privates::dsl::user_privates;

        let _connection = establish_connection();
        return user_privates
            .filter(schema::user_privates::user_id.eq(self.id))
            .load::<UserPrivate>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn is_user_can_see_info(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        let char = private.can_see_info;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_add_in_chat(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return false;
        }
        let private = self.get_private_model();
        let char = private.can_add_in_chat;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_post(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        let char = private.can_see_post;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_community(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        let char = private.can_see_community;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_photo(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        let char = private.can_see_photo;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_video(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        let char = private.can_see_video;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_music(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        let char = private.can_see_music;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_doc(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        let char = private.can_see_doc;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_all(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        let char = private.can_see_all;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_friend(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        let char = private.can_see_friend;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_good(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        let char = private.can_see_good;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_survey(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        let char = private.can_see_survey;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_send_message(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        let char = private.can_send_message;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn get_profile_all_can_see(&self, user_id: i32) -> Vec<bool> {
        if self.id == user_id {
            return vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true];
        }
        let private = self.get_private_model();

        let can_see_all = private.can_see_all;
        let bool_can_see_all = match can_see_all.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_all_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_all_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        if bool_can_see_all == false {
            return vec![false, false, false, false, self.is_user_can_send_message(user_id), self.is_user_can_add_in_chat(user_id), false, false, false, false, false, false, false, false];
        }

        let mut bool_stack = Vec::new();
        bool_stack.push(true);

        let can_see_community = private.can_see_community;
        let bool_can_see_community = match can_see_community.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_community_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_community_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_community);

        let can_see_info = private.can_see_info;
        let bool_can_see_info = match can_see_info.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_info);

        let can_see_friend = private.can_see_friend;
        let bool_can_see_friend = match can_see_friend.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_friend_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_friend_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_friend);

        let can_send_message = private.can_send_message;
        let bool_can_send_message = match can_send_message.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_send_message_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_send_message_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_send_message);

        let can_add_in_chat = private.can_add_in_chat;
        let bool_can_add_in_chat = match can_add_in_chat.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_add_in_chat_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_add_in_chat_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_add_in_chat);

        let can_see_post = private.can_see_post;
        let bool_can_see_post = match can_see_post.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_post_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_post_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_post);

        let can_see_photo = private.can_see_photo;
        let bool_can_see_photo = match can_see_photo.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_photo_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_photo_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_photo);

        let can_see_good = private.can_see_good;
        let bool_can_see_good = match can_see_good.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_good_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_good_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_good);

        let can_see_video = private.can_see_video;
        let bool_can_see_video = match can_see_video.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_video_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_video_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_video);

        let can_see_music = private.can_see_music;
        let bool_can_see_music = match can_see_music.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_music_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_music_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_music);

        let can_see_planner = private.can_see_planner;
        let bool_can_see_planner = match can_see_planner.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_planner_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_planner_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_planner);

        let can_see_doc = private.can_see_doc;
        let bool_can_see_doc = match can_see_doc.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_doc_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_doc_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_doc);

        let can_see_survey = private.can_see_survey;
        let bool_can_see_survey = match can_see_survey.as_str() {
            "a" => true,
            "b" => self.get_friends_ids().iter().any(|&i| i==user_id),
            "c" => self.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
            "d" => false,
            "e" => !self.get_can_see_survey_exclude_users_ids().iter().any(|&i| i==user_id),
            "f" => self.get_can_see_survey_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_survey);

        return bool_stack;
    }
    pub fn is_anon_user_can_see_post(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_post == "a";
    }
    pub fn is_anon_user_can_see_all(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_all == "a";
    }
    pub fn is_anon_user_can_see_photo(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_photo == "a";
    }
    pub fn is_anon_user_can_see_community(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_community == "a";
    }
    pub fn is_anon_user_can_see_friend(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_friend == "a";
    }
    pub fn is_anon_user_can_see_doc(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_doc == "a";
    }
    pub fn is_anon_user_can_see_music(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_music == "a";
    }
    pub fn is_anon_user_can_see_video(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_video == "a";
    }
    pub fn is_anon_user_can_see_good(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_good == "a";
    }
    pub fn is_anon_user_can_see_planner(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_planner == "a";
    }
    pub fn is_anon_user_can_see_survey(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_survey == "a";
    }

    pub fn get_anon_profile_all_can_see(&self) -> Vec<bool> {
        let private = self.get_private_model();

        let can_see_all = private.can_see_all;
        let bool_can_see_all = match can_see_all.as_str() {
            "a" => true,
            _ => false,
        };
        if bool_can_see_all == false {
            return vec![false, false, false, false, false, false, false, false, false, false, false, false];
        }

        let mut bool_stack = Vec::new();
        bool_stack.push(true);

        let can_see_community = private.can_see_community;
        let bool_can_see_community = match can_see_community.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_community);

        let can_see_info = private.can_see_info;
        let bool_can_see_info = match can_see_info.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_info);

        let can_see_friend = private.can_see_friend;
        let bool_can_see_friend = match can_see_friend.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_friend);

        let can_see_post = private.can_see_post;
        let bool_can_see_post = match can_see_post.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_post);

        let can_see_photo = private.can_see_photo;
        let bool_can_see_photo = match can_see_photo.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_photo);

        let can_see_good = private.can_see_good;
        let bool_can_see_good = match can_see_good.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_good);

        let can_see_video = private.can_see_video;
        let bool_can_see_video = match can_see_video.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_video);

        let can_see_music = private.can_see_music;
        let bool_can_see_music = match can_see_music.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_music);

        let can_see_planner = private.can_see_planner;
        let bool_can_see_planner = match can_see_planner.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_planner);

        let can_see_doc = private.can_see_doc;
        let bool_can_see_doc = match can_see_doc.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_doc);

        let can_see_survey = private.can_see_survey;
        let bool_can_see_survey = match can_see_survey.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_survey);
        return bool_stack;
    }
    pub fn set_friends_visible_perms(&self, action: String, users: String, types: String) -> bool {
        use crate::models::{FriendsVisiblePerm, NewFriendsVisiblePerm};
        use crate::schema::friends_visible_perms::dsl::friends_visible_perms;
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        let mut users_ids = Vec::new();
        let v: Vec<&str> = users.split(", ").collect();
        for item in v.iter() {
            if !item.is_empty() {
                let pk: i32 = item.parse().unwrap();
                users_ids.push(pk);
            }
        }
        let _friends = friends
            .filter(schema::friends::target_user_id.eq_any(&users_ids))
            .load::<Friend>(&_connection)
            .expect("E");
        let mut friends_stack = Vec::new();
        for _item in _friends.iter() {
            friends_stack.push(_item.target_user_id);
        };
        diesel::delete(friends_visible_perms.filter(schema::friends_visible_perms::user_id.eq_any(friends_stack))).execute(&_connection).expect("E");

        if types == "can_see_community".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_see_community(*user_id, action.clone());
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_info".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_see_info(
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_friend".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_see_friend (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_send_message".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_send_message (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_add_in_chat".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_add_in_chat (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_post".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_see_post (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_photo".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_see_photo (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_good".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_see_good (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_video".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_see_video (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_music".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_see_music (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_planner".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_see_planner (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_doc".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_see_doc (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_survey".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_see_survey (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_all".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewFriendsVisiblePerm::add_can_see_all (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::friends_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<FriendsVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        return true;
    }
    pub fn add_new_subscriber(&self, user_id: i32) -> bool {
        use crate::models::{NewsUserCommunitie, NewNewsUserCommunitie};
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        if news_user_communities
            .filter(schema::news_user_communities::owner.eq(self.id))
            .filter(schema::news_user_communities::user_id.eq(user_id))
            .load::<NewsUserCommunitie>(&_connection)
            .expect("E").len() == 0 {
                let _new = NewNewsUserCommunitie {
                    owner: self.id,
                    list_id: None,
                    user_id: Some(user_id),
                    community_id: None,
                    mute: false,
                    sleep: None,
                };
            diesel::insert_into(schema::news_user_communities::table)
                .values(&_new)
                .get_result::<NewsUserCommunitie>(&_connection)
                .expect("Error.");
        }
        return true;
    }
    pub fn add_new_subscriber_in_list(&self, new_id: i32, list_id: i32) -> bool {
        use crate::models::{NewsUserCommunitie, ListUserCommunitiesKey};
        use crate::schema::news_user_communities::dsl::news_user_communities;
        use crate::schema::list_user_communities_keys::dsl::list_user_communities_keys;

        let _connection = establish_connection();
        let _new = news_user_communities
            .filter(schema::news_user_communities::id.eq(new_id))
            .load::<NewsUserCommunitie>(&_connection)
            .expect("E");
        let _list = list_user_communities_keys
            .filter(schema::list_user_communities_keys::id.eq(list_id))
            .load::<ListUserCommunitiesKey>(&_connection)
            .expect("E");

        if _new.len() > 0 && _new[0].owner == self.id && _list.len() > 0 && _list[0].owner == self.id {
            diesel::update(news_user_communities.filter(schema::news_user_communities::id.eq(new_id)))
                .set(schema::news_user_communities::list_id.eq(list_id))
                .get_result::<NewsUserCommunitie>(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn delete_new_subscriber(&self, user_id: i32) -> bool {
        use crate::models::NewsUserCommunitie;
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        let _new = news_user_communities
            .filter(schema::news_user_communities::owner.eq(self.id))
            .filter(schema::news_user_communities::user_id.eq(user_id))
            .load::<NewsUserCommunitie>(&_connection)
            .expect("E");
        if _new.len() > 0 && _new[0].owner == self.id {
            diesel::delete(
                news_user_communities
                    .filter(schema::news_user_communities::owner.eq(self.id))
                    .filter(schema::news_user_communities::user_id.eq(user_id))
                )
                .execute(&_connection)
                .expect("E");
            return true;
        }
        return false;
    }
    pub fn delete_new_subscriber_from_list(&self, new_id: i32) -> bool {
        use crate::models::{NewsUserCommunitie, NewNewsUserCommunitie};
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        let _new = news_user_communities.filter(schema::news_user_communities::id.eq(new_id)).load::<NewsUserCommunitie>(&_connection).expect("E");
        if _new.len() > 0 && _new[0].owner == self.id {
            let _new = NewNewsUserCommunitie {
                owner: self.id,
                list_id: None,
                user_id: _new[0].user_id,
                community_id: None,
                mute: _new[0].mute,
                sleep: _new[0].sleep,
            };
            diesel::update(news_user_communities.filter(schema::news_user_communities::id.eq(new_id)))
                .set(_new)
                .get_result::<NewsUserCommunitie>(&_connection)
                .expect("Error.");
                return true;
            }
        return false;
    }

    pub fn add_notification_subscriber(&self, user_id: i32) -> bool {
        use crate::models::{NotifyUserCommunitie, NewNotifyUserCommunitie};
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        if notify_user_communities
            .filter(schema::notify_user_communities::owner.eq(self.id))
            .filter(schema::notify_user_communities::user_id.eq(user_id))
            .load::<NotifyUserCommunitie>(&_connection)
            .expect("E").len() == 0 {
                let _new = NewNotifyUserCommunitie {
                    owner: self.id,
                    list_id: None,
                    user_id: Some(user_id),
                    community_id: None,
                    mute: false,
                    sleep: None,
                };
                diesel::insert_into(schema::notify_user_communities::table)
                    .values(&_new)
                    .get_result::<NotifyUserCommunitie>(&_connection)
                    .expect("Error.");
        }
        return true;
    }
    pub fn add_notification_subscriber_in_list(&self, notify_id: i32, list_id: i32) -> bool {
        use crate::models::{NotifyUserCommunitie, ListUserCommunitiesKey};
        use crate::schema::notify_user_communities::dsl::notify_user_communities;
        use crate::schema::list_user_communities_keys::dsl::list_user_communities_keys;

        let _connection = establish_connection();
        let _notify = notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)).load::<NotifyUserCommunitie>(&_connection).expect("E");
        let _list = list_user_communities_keys.filter(schema::list_user_communities_keys::id.eq(list_id)).load::<ListUserCommunitiesKey>(&_connection).expect("E");

        if _notify.len() > 0 && _notify[0].owner == self.id && _list.len() > 0 && _list[0].owner == self.id {
            diesel::update(notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)))
                .set(schema::notify_user_communities::list_id.eq(_list[0].id))
                .get_result::<NotifyUserCommunitie>(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn delete_notification_subscriber(&self, user_id: i32) -> bool {
        use crate::models::NotifyUserCommunitie;
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        let _notify = notify_user_communities
            .filter(schema::notify_user_communities::owner.eq(self.id))
            .filter(schema::notify_user_communities::user_id.eq(user_id))
            .load::<NotifyUserCommunitie>(&_connection)
            .expect("E");
        if _notify.len() > 0 && _notify[0].owner == self.id {
            diesel::delete(
                notify_user_communities
                    .filter(schema::notify_user_communities::owner.eq(self.id))
                    .filter(schema::notify_user_communities::user_id.eq(user_id))
                )
                .execute(&_connection)
                .expect("E");
            return true;
        }
        return false;
    }
    pub fn delete_notification_subscriber_from_list(&self, notify_id: i32) -> bool {
        use crate::models::{NotifyUserCommunitie, NewNotifyUserCommunitie};
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        let _notify = notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)).load::<NotifyUserCommunitie>(&_connection).expect("E");
        if _notify.len() > 0 && _notify[0].owner == self.id {
            let _new = NewNotifyUserCommunitie {
                owner: self.id,
                list_id: None,
                user_id: _notify[0].user_id,
                community_id: None,
                mute: _notify[0].mute,
                sleep: _notify[0].sleep,
            };
            diesel::update(notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)))
                .set(_new)
                .get_result::<NotifyUserCommunitie>(&_connection)
                .expect("Error.");
                return true;
            }
        return false;
    }
    pub fn get_or_create_featured_objects(&self, user: User) -> bool {
        use crate::models::{NewFeaturedUserCommunitie, FeaturedUserCommunitie};
        use crate::schema::featured_user_communities::dsl::featured_user_communities;

        let _connection = establish_connection();
        for friend_id in user.get_6_friends_ids().iter() {
            if !self.is_connected_with_user_with_id(*friend_id) && featured_user_communities
                .filter(schema::featured_user_communities::owner.eq(self.id))
                .filter(schema::featured_user_communities::user_id.eq(friend_id))
                .load::<FeaturedUserCommunitie>(&_connection)
                .expect("E")
                .len() == 0 {
                    let new_featured = NewFeaturedUserCommunitie {
                            owner: self.id,
                            list_id: None,
                            user_id: Some(*friend_id),
                            community_id: None,
                            mute: false,
                            sleep: None,
                        };
                        diesel::insert_into(schema::featured_user_communities::table)
                            .values(&new_featured)
                            .get_result::<FeaturedUserCommunitie>(&_connection)
                            .expect("Error.");
                }
            }
            for community_id in user.get_6_communities_ids().iter() {
                if !self.is_member_of_community(*community_id) && featured_user_communities
                    .filter(schema::featured_user_communities::owner.eq(self.id))
                    .filter(schema::featured_user_communities::community_id.eq(community_id))
                    .load::<FeaturedUserCommunitie>(&_connection)
                    .expect("E").len() == 0 {
                        let new_featured = NewFeaturedUserCommunitie {
                                owner: self.id,
                                list_id: None,
                                user_id: None,
                                community_id: Some(*community_id),
                                mute: false,
                                sleep: None,
                            };
                            diesel::insert_into(schema::featured_user_communities::table)
                                .values(&new_featured)
                                .get_result::<FeaturedUserCommunitie>(&_connection)
                                .expect("Error.");
                }
            }
            return true;
    }

    pub fn follow_user(&self, user: User) -> bool {
        if self.id == user.id || self.is_self_user_in_block(user.id) || self.is_followers_user_with_id(user.id) || self.is_following_user_with_id(user.id) {
            return false;
        }
        use crate::models::NewFollow;
        //use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        let _new_follow = NewFollow {
            user_id: self.id,
            followed_user: user.id,
            view: false,
            visited: 0,
        };
        diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .get_result::<Follow>(&_connection)
            .expect("Error.");
        user.plus_follows(1);
        if user.is_user_can_see_all(self.id) == true {
            self.add_new_subscriber(user.id);
            self.get_or_create_featured_objects(user);
        }
        return true;
    }
    pub fn follow_view_user(&self, user: User) -> bool {
        if self.id == user.id || !self.is_followers_user_with_id(user.id) {
            return false;
        }
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();

        let _follow = follows
            .filter(schema::follows::user_id.eq(self.id))
            .filter(schema::follows::followed_user.eq(user.id))
            .load::<Follow>(&_connection)
            .expect("E");
        diesel::update(&_follow[0])
            .set(schema::follows::view.eq(true))
            .get_result::<Follow>(&_connection)
            .expect("Error.");
        return true;
    }

    pub fn unfollow_user(&self, user: User) -> bool {
        if self.id == user.id || !self.is_following_user_with_id(user.id) {
            return false;
        }
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        let _follow = follows
            .filter(schema::follows::user_id.eq(self.id))
            .filter(schema::follows::followed_user.eq(user.id))
            .load::<Follow>(&_connection)
            .expect("E");
        if _follow.len() > 0 {
            diesel::delete(
                    follows
                        .filter(schema::follows::followed_user.eq(user.id))
                        .filter(schema::follows::user_id.eq(self.id))
                )
                .execute(&_connection)
                .expect("E");
            self.delete_new_subscriber(user.id);
            user.minus_follows(1);
            return true;
        }
        return false;
    }

    pub fn frend_user(&self, user: User) -> bool {
        if self.id == user.id || !self.is_followers_user_with_id(user.id) {
            return false;
        }
        use crate::models::NewFriend;
        use crate::schema::follows::dsl::follows;
        use crate::schema::featured_user_communities::dsl::featured_user_communities;

        let _connection = establish_connection();
        let _new_friend = NewFriend {
            user_id: self.id,
            target_user_id: user.id,
            visited: 0,
        };
        diesel::insert_into(schema::friends::table)
            .values(&_new_friend)
            .get_result::<Friend>(&_connection)
            .expect("Error.");

        let _new_friend_2 = NewFriend {
            user_id: user.id,
            target_user_id: self.id,
            visited: 0,
        };
        diesel::insert_into(schema::friends::table)
            .values(&_new_friend_2)
            .get_result::<Friend>(&_connection)
            .expect("Error.");

        diesel::delete(
            follows
                .filter(schema::follows::user_id.eq(user.id))
                .filter(schema::follows::followed_user.eq(self.id)))
                .execute(&_connection)
                .expect("E");
        diesel::delete(
            featured_user_communities
                .filter(schema::featured_user_communities::owner.eq(self.id))
                .filter(schema::featured_user_communities::user_id.eq(user.id)))
                .execute(&_connection)
                .expect("E");

        user.plus_friends(1);
        self.plus_friends(1);
        self.minus_follows(1);
        if user.is_user_can_see_all(self.id) == false {
            self.add_new_subscriber(user.id);
            self.get_or_create_featured_objects(user);
        }
        return true;
    }
    pub fn unfrend_user(&self, user: User) -> bool {
        if self.id == user.id || !self.is_connected_with_user_with_id(user.id) {
            return false;
        }
        use crate::models::NewFollow;
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();

        diesel::delete(
            friends
                .filter(schema::friends::user_id.eq(self.id))
                .filter(schema::friends::target_user_id.eq(user.id)))
                .execute(&_connection)
                .expect("E");
        diesel::delete(
            friends
                .filter(schema::friends::target_user_id.eq(self.id))
                .filter(schema::friends::user_id.eq(user.id)))
                .execute(&_connection)
                .expect("E");

        let _new_follow = NewFollow {
            user_id: user.id,
            followed_user: self.id,
            view: true,
            visited: 0,
        };
        diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .get_result::<Follow>(&_connection)
            .expect("Error.");

        user.minus_friends(1);
        self.minus_friends(1);
        self.plus_follows(1);
        if user.is_user_can_see_all(self.id) == false {
            self.delete_new_subscriber(user.id);
        }
        return true;
    }

    pub fn block_user(&self, user: User) -> bool {
        if self.id == user.id || self.is_user_in_block(user.id) {
            return false;
        }
        //use crate::schema::user_blocks::dsl::user_blocks;
        use crate::models::NewUserBlock;

        let _connection = establish_connection();

        if self.is_connected_with_user_with_id(user.id) {
            use crate::schema::friends::dsl::friends;
            diesel::delete(
                friends
                    .filter(schema::friends::user_id.eq(self.id))
                    .filter(schema::friends::target_user_id.eq(user.id)))
                    .execute(&_connection)
                    .expect("E");
            diesel::delete(
                friends
                    .filter(schema::friends::target_user_id.eq(self.id))
                    .filter(schema::friends::user_id.eq(user.id)))
                    .execute(&_connection)
                    .expect("E");
            user.minus_friends(1);
            self.minus_friends(1);
        }
        else if self.is_followers_user_with_id(user.id) {
            use crate::schema::follows::dsl::follows;
            diesel::delete(
                follows
                    .filter(schema::follows::followed_user.eq(self.id))
                    .filter(schema::follows::user_id.eq(user.id)))
                    .execute(&_connection)
                    .expect("E");
            user.minus_follows(1);
        }
        else if self.is_following_user_with_id(user.id) {
            use crate::schema::follows::dsl::follows;
            diesel::delete(
                follows
                    .filter(schema::follows::user_id.eq(self.id))
                    .filter(schema::follows::followed_user.eq(user.id)))
                    .execute(&_connection)
                    .expect("E");
            self.minus_follows(1);
        }

        let _user_block = NewUserBlock {
            user_block_i: self.id,
            blocked_user_id: user.id,
        };
        diesel::insert_into(schema::user_blocks::table)
            .values(&_user_block)
            .get_result::<UserBlock>(&_connection)
            .expect("Error.");
        self.delete_new_subscriber(user.id);
        self.delete_notification_subscriber(user.id);
        return true;
    }
    pub fn unblock_user(&self, user: User) -> bool {
        if self.id == user.id || !self.is_user_in_block(user.id) {
            return false;
        }
        use crate::schema::user_blocks::dsl::user_blocks;

        let _connection = establish_connection();
        diesel::delete(
            user_blocks
                .filter(schema::user_blocks::user_block_i.eq(self.id))
                .filter(schema::user_blocks::blocked_user_id.eq(user.id)))
                .execute(&_connection)
                .expect("E");
        return true;
    }
    pub fn plus_friend_visited(&self, user_id: i32) -> bool {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        let _connect = friends
            .filter(schema::friends::user_id.eq(self.id))
            .filter(schema::friends::target_user_id.eq(user_id))
            .load::<Friend>(&_connection)
            .expect("E");
        diesel::update(&_connect[0])
                .set(schema::friends::visited.eq(_connect[0].visited + 1))
                .get_result::<Friend>(&_connection)
                .expect("Error.");
        return true;
    }
    pub fn plus_community_visited(&self, community_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let _member = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");
        diesel::update(&_member[0])
                .set(schema::communities_memberships::visited.eq(_member[0].visited + 1))
                .get_result::<CommunitiesMembership>(&_connection)
                .expect("Error.");
        return true;
    }
    pub fn is_banned_from_community(&self, community_id: i32) -> bool {
        use crate::schema::community_banner_users::dsl::community_banner_users;
        use crate::models::CommunityBannerUser;

        let _connection = establish_connection();

        return community_banner_users
            .filter(schema::community_banner_users::community_id.eq(community_id))
            .filter(schema::community_banner_users::user_id.eq(self.id))
            .load::<CommunityBannerUser>(&_connection)
            .expect("E")
            .len() > 0;
    }
    pub fn get_members_for_notify_ids(&self) -> Vec<i32> {
        use crate::schema::notify_user_communities::dsl::notify_user_communities;
        use crate::models::NotifyUserCommunitie;

        let _connection = establish_connection();
        let items = notify_user_communities
            .filter(schema::notify_user_communities::user_id.eq(self.id))
            .filter(schema::notify_user_communities::mute.eq(false))
            .filter(schema::notify_user_communities::sleep.lt(chrono::Local::now().naive_utc()))
            .load::<NotifyUserCommunitie>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.owner);
        };
        return stack;
    }
    pub fn follow_community(&self, community:Community) -> bool {
        use crate::models::{CommunityFollow, NewCommunityFollow};

        if self.is_banned_from_community(community.id) || self.is_member_of_community(community.id) || self.is_follow_from_community(community.id) {
            return false;
        }
        community.add_notify_subscriber(self.id);
        if community.is_public() {
            community.add_new_subscriber(self.id);
        }

        let _connection = establish_connection();
        let new_follow = NewCommunityFollow {
                user_id: self.id,
                community_id: community.id,
                view: false,
                visited: 0,
            };
        diesel::insert_into(schema::community_follows::table)
            .values(&new_follow)
            .get_result::<CommunityFollow>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn unfollow_community(&self, community:Community) -> bool {
        use crate::schema::community_follows::dsl::community_follows;

        if self.is_member_of_community(community.id) || !self.is_follow_from_community(community.id) {
            return false;
        }
        community.delete_notify_subscriber(self.id);
        community.delete_new_subscriber(self.id);

        let _connection = establish_connection();
        diesel::delete(
            community_follows
                .filter(schema::community_follows::community_id.eq(community.id))
                .filter(schema::community_follows::user_id.eq(self.id))
           )
          .execute(&_connection)
          .expect("E");
        return true;
    }
    pub fn join_community(&self, community:Community) -> bool {
        use crate::models::{CommunitiesMembership, NewCommunitiesMembership};

        if self.is_member_of_community(community.id) || self.is_banned_from_community(community.id) {
            return false;
        }
        let _connection = establish_connection();

        if community.is_private() {
            use crate::schema::community_invites::dsl::community_invites;
            use crate::models::CommunityInvite;

            let invites = community_invites
                .filter(schema::community_invites::user_id.eq(self.id))
                .filter(schema::community_invites::community_id.eq(community.id))
                .load::<CommunityInvite>(&_connection)
                .expect("E");
            if invites.len() == 0 {
                return false;
            }
            diesel::delete(
                community_invites
                    .filter(schema::community_invites::user_id.eq(self.id))
                    .filter(schema::community_invites::community_id.eq(community.id)))
              .execute(&_connection)
              .expect("E");
        }
        else if community.is_close() {
            use crate::schema::community_follows::dsl::community_follows;
            use crate::models::CommunityFollow;

            let follows = community_follows
                .filter(schema::community_follows::user_id.eq(self.id))
                .filter(schema::community_follows::community_id.eq(community.id))
                .load::<CommunityFollow>(&_connection)
                .expect("E");
            if follows.len() == 0 {
                return false;
            }
            diesel::delete(
                community_follows
                    .filter(schema::community_follows::user_id.eq(self.id))
                    .filter(schema::community_follows::community_id.eq(community.id)))
              .execute(&_connection)
              .expect("E");
        }

        community.add_new_subscriber(self.id);

        let new_member = NewCommunitiesMembership{
            user_id: self.id,
            community_id: community.id,
            is_administrator: false,
            is_moderator: false,
            is_editor: false,
            is_advertiser: false,
            created: chrono::Local::now().naive_utc(),
            visited: 1,
        };
        diesel::insert_into(schema::communities_memberships::table)
            .values(&new_member)
            .get_result::<CommunitiesMembership>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn leave_community(&self, community:Community) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        if !self.is_member_of_community(community.id) {
            return false;
        }
        let _connection = establish_connection();
        community.delete_new_subscriber(self.id);
        diesel::delete(
            communities_memberships
                .filter(schema::communities_memberships::user_id.eq(self.id))
                .filter(schema::communities_memberships::community_id.eq(community.id)))
          .execute(&_connection)
          .expect("E");
        return true;
    }

    pub fn get_post_categories(&self) -> Vec<PostCategorie> {
        let _connection = establish_connection();
        return schema::post_categories::table
            .load::<PostCategorie>(&_connection)
            .expect("could not load tags");
    }
    pub fn get_gender_a(&self) -> String {
        if self.gender == "b" {
            return "a".to_string();
        }
        return "".to_string();
    }
    pub fn get_gender(&self) -> String {
        if self.gender == "b" {
            return "Женский".to_string();
        }
        return "Мужской".to_string();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: i32,
    pub phone: String,
}
