use crate::schema;
use diesel::prelude::*;
use crate::schema::{
    video_categories,
    video_lists,
    videos,
    video_comments,
    user_video_list_collections,
    community_video_list_collections,
    video_list_perms,
    video_votes,
    video_comment_votes,
    video_list_reposts,
    video_reposts,
    video_reactions,
    video_comment_reactions,
};

use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    JsonPosition,
    JsonItemReactions
};
use crate::models::{
    User,
    Community,
    UserVideoListPosition,
    CommunityVideoListPosition,
    Sticker,
    Photo,
    Post,
    Message,
    Reaction,
};
use actix_web::web::Json;


/////// VideoCategorie //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct VideoCategorie {
    pub id:       i32,
    pub name:     String,
    pub position: i32,
}

impl VideoCategorie {
    pub fn create_category(name: String, position: i32) -> VideoCategorie {
        let _connection = establish_connection();
        let new_form = NewVideoCategorie {
            name:     name,
            position: position,
        };
        let new_cat = diesel::insert_into(schema::video_categories::table)
            .values(&new_form)
            .get_result::<VideoCategorie>(&_connection)
            .expect("Error.");
        return new_cat;
    }
    pub fn edit_category(&self, name: String, position: i32) -> &VideoCategorie {
        let _connection = establish_connection();
        let new_form = NewVideoCategorie {
            name:     name,
            position: position,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<VideoCategorie>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="video_categories"]
pub struct NewVideoCategorie {
    pub name:     String,
    pub position: i32,
}

/////// videoList //////

////////// Тип списка
    // 1 основной список
    // 2 пользовательский список
    // 3 список предложки
    // 4 Фото со страницы
    // 5 Фото со стены

    // 11 удаленный основной список
    // 12 удаленный пользовательский список
    // 13 удаленный список предложки
    // 14 удаленный Фото со страницы
    // 15 удаленный Фото со стены

    // 21 закрытый основной список
    // 22 закрытый пользовательский список
    // 23 закрытый список предложки
    // 24 закрытый Фото со страницы
    // 25 закрытый Фото со стены

    // 31 замороженный основной список
    // 32 замороженный пользовательский список
    // 33 замороженный список предложки
    // 34 замороженный Фото со страницы
    // 35 замороженный Фото со стены

//////////// Приватность списка
// 'a' Все пользователи
// 'b' Друзья
// 'c' Друзья и друзья друзей
// 'e' Друзья, кроме
// 'f' Некоторые друзья
// 'g' Подписчики
// 'o' Только я / владелец сообщества
// 'p' Администраторы
// 'h' Подписчики, кроме
// 'i' Некоторые подписчики

/////// VideoList //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(User)]
pub struct VideoList {
    pub id:              i32,
    pub name:            String,
    pub community_id:    Option<i32>,
    pub user_id:         i32,
    pub types:           i16,
    pub description:     Option<String>,
    pub image:           Option<String>,
    pub created:         chrono::NaiveDateTime,
    pub count:           i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub can_see_el:      String,
    pub can_see_comment: String,
    pub create_el:       String,
    pub create_comment:  String,
    pub copy_el:         String,
    pub reactions:       Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="video_lists"]
pub struct NewVideoList {
    pub name:            String,
    pub community_id:    Option<i32>,
    pub user_id:         i32,
    pub types:           i16,
    pub description:     Option<String>,
    pub image:           Option<String>,
    pub created:         chrono::NaiveDateTime,
    pub count:           i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub can_see_el:      String,
    pub can_see_comment: String,
    pub create_el:       String,
    pub create_comment:  String,
    pub copy_el:         String,
    pub reactions:       Option<String>,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="video_lists"]
pub struct EditVideoList {
    pub name:            String,
    pub description:     Option<String>,
    pub image:           Option<String>,
    pub can_see_el:      String,
    pub can_see_comment: String,
    pub create_el:       String,
    pub create_comment:  String,
    pub copy_el:         String,
    pub reactions:       Option<String>,
}


impl VideoList {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_video_list(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "lvi".to_string() + &self.get_str_id();
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
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(25))
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
            .filter(schema::moderateds::types.eq(25))
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

    pub fn count_copy(&self) -> String {
        if self.copy == 0 {
            return "".to_string();
        }
        else {
            return ", копировали - ".to_string() + &self.copy.to_string();
        }
    }
    pub fn message_reposts_count(&self) -> String {
        use crate::schema::video_list_reposts::dsl::video_list_reposts;

        let _connection = establish_connection();

        let count = video_list_reposts
            .filter(schema::video_list_reposts::video_list_id.eq(self.id))
            .filter(schema::video_list_reposts::message_id.is_not_null())
            .load::<VideoListRepost>(&_connection)
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
        use crate::schema::video_list_reposts::dsl::video_list_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = video_list_reposts
            .filter(schema::video_list_reposts::video_list_id.eq(self.id))
            .filter(schema::video_list_reposts::post_id.is_not_null())
            .limit(limit)
            .offset(offset)
            .load::<VideoListRepost>(&_connection)
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
        use crate::schema::video_list_reposts::dsl::video_list_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = video_list_reposts
            .filter(schema::video_list_reposts::video_list_id.eq(self.id))
            .filter(schema::video_list_reposts::post_id.is_not_null())
            .limit(6)
            .load::<VideoListRepost>(&_connection)
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

    pub fn get_description(&self) -> String {
        return "<a data-videolist='".to_string() + &self.get_str_id() + &"' class='ajax'>".to_string() + &self.name + &"</a>".to_string();
    }
    pub fn is_user_list(&self, user: User) -> bool {
        return self.user_id == user.id;
    }
    pub fn is_community_list(&self, community: Community) -> bool {
        return self.community_id.unwrap() == community.id;
    }
    pub fn get_users_ids(&self) -> Vec<i32> {
        use crate::schema::user_video_list_collections::dsl::user_video_list_collections;

        let _connection = establish_connection();
        let ids = user_video_list_collections
            .filter(schema::user_video_list_collections::video_list_id.eq(self.id))
            .load::<UserVideoListCollection>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in ids.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_communities_ids(&self) -> Vec<i32> {
        use crate::schema::community_video_list_collections::dsl::community_video_list_collections;

        let _connection = establish_connection();
        let ids = community_video_list_collections
            .filter(schema::community_video_list_collections::video_list_id.eq(self.id))
            .load::<CommunityVideoListCollection>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in ids.iter() {
            stack.push(_item.community_id);
        };
        return stack;
    }
    pub fn is_user_collection_list(&self, user_id: i32) -> bool {
        return self.get_users_ids().iter().any(|&i| i==user_id);
    }
    pub fn is_community_collection_list(&self, community_id: i32) -> bool {
        return self.get_communities_ids().iter().any(|&i| i==community_id);
    }
    pub fn count_reposts(&self) -> String {
        if self.repost > 0 {
            return self.repost.to_string()
        }
        else {
            return "".to_string()
        }
    }
    pub fn get_items(&self) -> Vec<Video> {
        use crate::schema::videos::dsl::videos;

        let _connection = establish_connection();
        return videos
            .filter(schema::videos::video_list_id.eq(self.id))
            .filter(schema::videos::types.eq("a"))
            .order(schema::videos::created.desc())
            .load::<Video>(&_connection)
            .expect("E.");
    }
    pub fn get_paginate_items(&self, limit: i64, offset: i64) -> Vec<Video> {
        use crate::schema::videos::dsl::videos;

        let _connection = establish_connection();
        return videos
            .filter(schema::videos::video_list_id.eq(self.id))
            .filter(schema::videos::types.eq("a"))
            .limit(limit)
            .offset(offset)
            .order(schema::videos::created.desc())
            .load::<Video>(&_connection)
            .expect("E.");
    }
    pub fn count_items(&self) -> String {
        if self.count > 0 {
            return self.count.to_string()
        }
        else {
            return "".to_string()
        }
    }
    pub fn count_items_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count,
            " запись".to_string(),
            " записи".to_string(),
            " записей".to_string(),
        );
    }

    pub fn get_can_see_el_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::video_list_perms::dsl::video_list_perms;

        let _connection = establish_connection();
        let items = video_list_perms
            .filter(schema::video_list_perms::video_list_id.eq(self.id))
            .filter(schema::video_list_perms::can_see_item.eq("b"))
            .load::<VideoListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::video_list_perms::dsl::video_list_perms;

        let _connection = establish_connection();
        let items = video_list_perms
            .filter(schema::video_list_perms::video_list_id.eq(self.id))
            .filter(schema::video_list_perms::can_see_item.eq("a"))
            .load::<VideoListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_el_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_el_exclude_users_ids());
    }
    pub fn get_can_see_el_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_el_include_users_ids());
    }

    pub fn get_can_see_comment_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::video_list_perms::dsl::video_list_perms;

        let _connection = establish_connection();
        let items = video_list_perms
            .filter(schema::video_list_perms::video_list_id.eq(self.id))
            .filter(schema::video_list_perms::can_see_comment.eq("b"))
            .load::<VideoListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_comment_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::video_list_perms::dsl::video_list_perms;

        let _connection = establish_connection();
        let items = video_list_perms
            .filter(schema::video_list_perms::video_list_id.eq(self.id))
            .filter(schema::video_list_perms::can_see_comment.eq("a"))
            .load::<VideoListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_comment_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_comment_exclude_users_ids());
    }
    pub fn get_can_see_comment_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_can_see_comment_include_users_ids());
    }

    pub fn get_create_el_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::video_list_perms::dsl::video_list_perms;

        let _connection = establish_connection();
        let items = video_list_perms
            .filter(schema::video_list_perms::video_list_id.eq(self.id))
            .filter(schema::video_list_perms::create_item.eq("b"))
            .load::<VideoListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_create_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::video_list_perms::dsl::video_list_perms;

        let _connection = establish_connection();
        let items = video_list_perms
            .filter(schema::video_list_perms::video_list_id.eq(self.id))
            .filter(schema::video_list_perms::create_item.eq("a"))
            .load::<VideoListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_create_el_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_create_el_exclude_users_ids());
    }
    pub fn get_create_el_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_create_el_include_users_ids());
    }

    pub fn get_create_comment_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::video_list_perms::dsl::video_list_perms;

        let _connection = establish_connection();
        let items = video_list_perms
            .filter(schema::video_list_perms::video_list_id.eq(self.id))
            .filter(schema::video_list_perms::create_comment.eq("b"))
            .load::<VideoListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_create_comment_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::video_list_perms::dsl::video_list_perms;

        let _connection = establish_connection();
        let items = video_list_perms
            .filter(schema::video_list_perms::video_list_id.eq(self.id))
            .filter(schema::video_list_perms::create_comment.eq("a"))
            .load::<VideoListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_create_comment_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_create_comment_exclude_users_ids());
    }
    pub fn get_create_comment_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_create_comment_include_users_ids());
    }

    pub fn get_copy_el_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::video_list_perms::dsl::video_list_perms;

        let _connection = establish_connection();
        let items = video_list_perms
            .filter(schema::video_list_perms::video_list_id.eq(self.id))
            .filter(schema::video_list_perms::can_copy.eq("b"))
            .load::<VideoListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_copy_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::video_list_perms::dsl::video_list_perms;

        let _connection = establish_connection();
        let items = video_list_perms
            .filter(schema::video_list_perms::video_list_id.eq(self.id))
            .filter(schema::video_list_perms::can_copy.eq("a"))
            .load::<VideoListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_copy_el_exclude_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_copy_el_exclude_users_ids());
    }
    pub fn get_copy_el_include_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_copy_el_include_users_ids());
    }
    pub fn get_community(&self) -> Community {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::id.eq(self.community_id.unwrap()))
            .filter(schema::communitys::types.lt(10))
            .load::<Community>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
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

    pub fn is_user_can_see_el(&self, user_id: i32) -> bool {
        let char = &self.can_see_el;
        if self.user_id == user_id || char == &"a".to_string() {
            return true;
        }

        if self.community_id.is_some() {
            let community = self.get_community();
            return match char.as_str() {
                "g" => community.get_members_ids().iter().any(|&i| i==user_id),
                "p" => community.get_administrators_ids().iter().any(|&i| i==user_id),
                "o" => community.user_id == user_id,
                "h" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "i" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match char.as_str() {
                "b" => creator.get_friends_ids().iter().any(|&i| i==user_id),
                "c" => creator.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
                "o" => creator.id == user_id,
                "e" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "f" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
    }

    pub fn is_user_can_see_comment(&self, user_id: i32) -> bool {
        let char = &self.can_see_comment;
        if self.user_id == user_id || char == &"a".to_string() {
            return true;
        }

        if self.community_id.is_some() {
            let community = self.get_community();
            return match char.as_str() {
                "g" => community.get_members_ids().iter().any(|&i| i==user_id),
                "p" => community.get_administrators_ids().iter().any(|&i| i==user_id),
                "o" => community.user_id == user_id,
                "h" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "i" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match char.as_str() {
                "b" => creator.get_friends_ids().iter().any(|&i| i==user_id),
                "c" => creator.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
                "o" => creator.id == user_id,
                "e" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "f" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
    }
    pub fn is_user_can_create_el(&self, user_id: i32) -> bool {
        let char = &self.create_el;
        if self.user_id == user_id || char == &"a".to_string() {
            return true;
        }

        if self.community_id.is_some() {
            let community = self.get_community();
            return match char.as_str() {
                "g" => community.get_members_ids().iter().any(|&i| i==user_id),
                "p" => community.get_administrators_ids().iter().any(|&i| i==user_id),
                "o" => community.user_id == user_id,
                "h" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "i" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match char.as_str() {
                "b" => creator.get_friends_ids().iter().any(|&i| i==user_id),
                "c" => creator.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
                "o" => creator.id == user_id,
                "e" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "f" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
    }
    pub fn is_user_can_create_comment(&self, user_id: i32) -> bool {
        let char = &self.create_comment;
        if self.user_id == user_id || char == &"a".to_string() {
            return true;
        }

        if self.community_id.is_some() {
            let community = self.get_community();
            return match char.as_str() {
                "g" => community.get_members_ids().iter().any(|&i| i==user_id),
                "p" => community.get_administrators_ids().iter().any(|&i| i==user_id),
                "o" => community.user_id == user_id,
                "h" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "i" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match char.as_str() {
                "b" => creator.get_friends_ids().iter().any(|&i| i==user_id),
                "c" => creator.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
                "o" => creator.id == user_id,
                "e" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "f" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
    }
    pub fn is_user_can_copy_el(&self, user_id: i32) -> bool {
        let char = &self.copy_el;
        if self.user_id == user_id || char == &"a".to_string() {
            return true;
        }

        if self.community_id.is_some() {
            let community = self.get_community();
            return match char.as_str() {
                "g" => community.get_members_ids().iter().any(|&i| i==user_id),
                "p" => community.get_administrators_ids().iter().any(|&i| i==user_id),
                "o" => community.user_id == user_id,
                "h" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "i" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match char.as_str() {
                "b" => creator.get_friends_ids().iter().any(|&i| i==user_id),
                "c" => creator.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
                "o" => creator.id == user_id,
                "e" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "f" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
    }
    pub fn is_anon_user_can_see_el(&self) -> bool {
        return self.can_see_el == "a";
    }
    pub fn is_anon_user_can_see_comment(&self) -> bool {
        return self.can_see_comment == "a";
    }
    pub fn is_anon_user_can_create_item(&self) -> bool {
        return self.create_el == "a";
    }
    pub fn is_anon_user_can_create_comment(&self) -> bool {
        return self.create_comment == "a";
    }
    pub fn is_anon_user_can_copy_el(&self) -> bool {
        return self.copy_el == "a";
    }
    pub fn create_list(creator: User, name: String, description: Option<String>, image: Option<String>,
        community_id: Option<i32>, can_see_el: String, can_see_comment: String,
        create_el: String, create_comment: String, copy_el: String,
        can_see_el_users: Option<Vec<i32>>, can_see_comment_users: Option<Vec<i32>>,create_el_users: Option<Vec<i32>>,
        create_comment_users: Option<Vec<i32>>,copy_el_users: Option<Vec<i32>>,
        reactions: Option<String>) -> VideoList {
        use crate::models::{
            NewCommunityVideoListPosition,
            NewUserVideoListPosition,
        };

        let _connection = establish_connection();
        let new_list_form = NewVideoList {
            name: name,
            community_id: community_id,
            user_id: creator.id,
            types: 2,
            description: description,
            image: image,
            created: chrono::Local::now().naive_utc(),
            count: 0,
            repost: 0,
            copy: 0,
            position: 0,
            can_see_el: can_see_el.clone(),
            can_see_comment: can_see_comment.clone(),
            create_el: create_el.clone(),
            create_comment: create_comment.clone(),
            copy_el: copy_el.clone(),
            reactions:       reactions,
        };
        let new_list = diesel::insert_into(schema::video_lists::table)
            .values(&new_list_form)
            .get_result::<VideoList>(&_connection)
            .expect("Error.");

        if community_id.is_some() {
            use crate::schema::communitys::dsl::communitys;

            let community = communitys
                .filter(schema::communitys::id.eq(community_id.unwrap()))
                .load::<Community>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            let _new_videos_list_position = NewCommunityVideoListPosition {
                community_id: community.id,
                list_id:      new_list.id,
                position:     community.get_video_lists_new_position(),
                types:        "a".to_string(),
            };
            let _videos_list_position = diesel::insert_into(schema::community_video_list_positions::table)
                .values(&_new_videos_list_position)
                .get_result::<CommunityVideoListPosition>(&_connection)
                .expect("Error saving video_list_position.");
        }
        else {
            let _new_videos_list_position = NewUserVideoListPosition {
                user_id:  creator.id,
                list_id:  new_list.id,
                position: creator.get_video_lists_new_position(),
                types:    "a".to_string(),
            };
            let _videos_list_position = diesel::insert_into(schema::user_video_list_positions::table)
                .values(&_new_videos_list_position)
                .get_result::<UserVideoListPosition>(&_connection)
                .expect("Error saving video_list_position.");
        }

        if can_see_el == "e".to_string() && can_see_el == "h".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_list.id,
                        can_see_item: Some("b".to_string()),
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }
        else if can_see_el == "f".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_list.id,
                        can_see_item: Some("a".to_string()),
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_include)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }

        if can_see_comment == "e".to_string() && can_see_comment == "h".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_list.id,
                        can_see_item: None,
                        can_see_comment: Some("b".to_string()),
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }
        else if can_see_comment == "f".to_string() && can_see_comment == "i".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_list.id,
                        can_see_item: None,
                        can_see_comment: Some("a".to_string()),
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_include)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }

        if create_el == "e".to_string() && create_el == "h".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_list.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: Some("b".to_string()),
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }
        else if create_el == "f".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_list.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: Some("a".to_string()),
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_include)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }

        if create_comment == "e".to_string() && create_comment == "h".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_list.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: Some("b".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }
        else if create_comment == "f".to_string() && create_comment == "i".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_list.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: Some("a".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_include)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }

        if copy_el == "e".to_string() && copy_el == "h".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_list.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: Some("b".to_string()),
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }
        else if copy_el == "f".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_list.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: Some("a".to_string()),
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_include)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }
        return new_list;
    }
    pub fn edit_list(&self, name: String, description: Option<String>,
        image: Option<String>, can_see_el: String, can_see_comment: String,
        create_el: String, create_comment: String, copy_el: String,
        can_see_el_users: Option<Vec<i32>>, can_see_comment_users: Option<Vec<i32>>,create_el_users: Option<Vec<i32>>,
        create_comment_users: Option<Vec<i32>>,copy_el_users: Option<Vec<i32>>,
        reactions: Option<String>) -> &VideoList {

        use crate::schema::video_list_perms::dsl::video_list_perms;

        let _connection = establish_connection();

        let mut descr: Option<String> = Some("".to_string());
        let mut react: Option<String> = Some("".to_string());
        if description.is_some() {
            descr = description;
        }
        if reactions.is_some() {
            react = reactions;
        }

        let edit_video_list = EditVideoList {
            name: name,
            description: descr,
            image: image,
            can_see_el: can_see_el.clone(),
            can_see_comment: can_see_comment.clone(),
            create_el: create_el.clone(),
            create_comment: create_comment.clone(),
            copy_el: copy_el.clone(),
            reactions: react,
        };
        diesel::update(self)
            .set(edit_video_list)
            .get_result::<VideoList>(&_connection)
            .expect("Error.");

        if can_see_el == "e".to_string() && can_see_el == "h".to_string() {
            if can_see_el_users.is_some() {
                diesel::delete (
                  video_list_perms
                    .filter(schema::video_list_perms::video_list_id.eq(self.id))
                    .filter(schema::video_list_perms::can_see_item.is_not_null())
                )
                  .execute(&_connection)
                  .expect("E");
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: self.id,
                        can_see_item: Some("b".to_string()),
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }
        else if can_see_el == "f".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: self.id,
                        can_see_item: Some("a".to_string()),
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_include)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }

        if can_see_comment == "e".to_string() && can_see_comment == "h".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: Some("b".to_string()),
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }
        else if can_see_comment == "f".to_string() && can_see_comment == "i".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: Some("a".to_string()),
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_include)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }

        if create_el == "e".to_string() && create_el == "h".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: Some("b".to_string()),
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }
        else if create_el == "f".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: Some("a".to_string()),
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_include)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }

        if create_comment == "e".to_string() && create_comment == "h".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: Some("b".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }
        else if create_comment == "f".to_string() && create_comment == "i".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: Some("a".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_include)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }

        if copy_el == "e".to_string() && copy_el == "h".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: Some("b".to_string()),
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }
        else if copy_el == "f".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: Some("a".to_string()),
                    };
                    diesel::insert_into(schema::video_list_perms::table)
                        .values(&_new_include)
                        .get_result::<VideoListPerm>(&_connection)
                        .expect("Error saving video_list_position.");
                }
            }
        }
        return self;
    }
    pub fn get_order(&self) -> UserVideoListPosition {
        use crate::schema::user_video_list_positions::dsl::user_video_list_positions;

        let _connection = establish_connection();
        return user_video_list_positions
            .filter(schema::user_video_list_positions::list_id.eq(self.id))
            .filter(schema::user_video_list_positions::types.eq("a"))
            .load::<UserVideoListPosition>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn add_in_community_collections(&self, community: Community) -> bool {
        use crate::models::NewCommunityVideoListPosition;

        if !self.get_communities_ids().iter().any(|&i| i==community.id) && self.community_id.is_some() && self.community_id.unwrap() == community.id {
            return false;
        }
        let _connection = establish_connection();
        let new_item = NewCommunityVideoListCollection {
            community_id: community.id,
            video_list_id: self.id,
        };
        diesel::insert_into(schema::community_video_list_collections::table)
            .values(&new_item)
            .get_result::<CommunityVideoListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewCommunityVideoListPosition {
            community_id: community.id,
            list_id:      self.id,
            position:     community.get_video_lists_new_position(),
            types:        "a".to_string(),
        };
        diesel::insert_into(schema::community_video_list_positions::table)
            .values(&new_pos)
            .get_result::<CommunityVideoListPosition>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn remove_in_community_collections(&self, community: Community) -> bool {
        use crate::schema::community_video_list_collections::dsl::community_video_list_collections;
        use crate::schema::community_video_list_positions::dsl::community_video_list_positions;

        if self.get_communities_ids().iter().any(|&i| i==community.id) {
            return false;
        }
        let _connection = establish_connection();
        diesel::delete(community_video_list_collections
            .filter(schema::community_video_list_collections::community_id.eq(community.id))
            .filter(schema::community_video_list_collections::video_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(community_video_list_positions
            .filter(schema::community_video_list_positions::community_id.eq(community.id))
            .filter(schema::community_video_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return true;
    }

    pub fn add_in_user_collections(&self, user: User) -> bool {
        use crate::models::NewUserVideoListPosition;

        if !self.get_users_ids().iter().any(|&i| i==user.id) && self.user_id == user.id {
            return false;
        }
        let _connection = establish_connection();
        let new_item = NewUserVideoListCollection {
            user_id: user.id,
            video_list_id: self.id,
        };
        diesel::insert_into(schema::user_video_list_collections::table)
            .values(&new_item)
            .get_result::<UserVideoListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewUserVideoListPosition {
            user_id:  user.id,
            list_id:  self.id,
            position: user.get_video_lists_new_position(),
            types:    "a".to_string(),
        };
        diesel::insert_into(schema::user_video_list_positions::table)
            .values(&new_pos)
            .get_result::<UserVideoListPosition>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn remove_in_user_collections(&self, user: User) -> bool {
        use crate::schema::user_video_list_collections::dsl::user_video_list_collections;
        use crate::schema::user_video_list_positions::dsl::user_video_list_positions;

        if self.get_users_ids().iter().any(|&i| i==user.id) {
            return false;
        }
        let _connection = establish_connection();
        diesel::delete(user_video_list_collections
            .filter(schema::user_video_list_collections::user_id.eq(user.id))
            .filter(schema::user_video_list_collections::video_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(user_video_list_positions
            .filter(schema::user_video_list_positions::user_id.eq(user.id))
            .filter(schema::user_video_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return true;
    }

    pub fn copy_item(pk: i32, user_or_communities: Vec<String>) -> bool {
        use crate::schema::video_lists::dsl::video_lists;
        use crate::schema::users::dsl::users;
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        let lists = video_lists
            .filter(schema::video_lists::id.eq(pk))
            .filter(schema::video_lists::types.lt(10))
            .load::<VideoList>(&_connection)
            .expect("E.");
        if lists.len() > 0 {
            let list = lists.into_iter().nth(0).unwrap();
            for item in user_or_communities.iter() {
                if item.chars().nth(0).unwrap() == 'c' {
                    let c_id: i32 = item[..1].parse().unwrap();
                    let communities = communitys
                        .filter(schema::communitys::id.eq(c_id))
                        .filter(schema::communitys::types.lt(10))
                        .load::<Community>(&_connection)
                        .expect("E.");
                    if communities.len() > 0 {
                        let com = communities.into_iter().nth(0).unwrap();
                        list.add_in_community_collections(com);
                    }
                }
                else if item.chars().nth(0).unwrap() == 'u' {
                    let u_id: i32 = item[..1].parse().unwrap();
                    let _users = users
                        .filter(schema::users::id.eq(u_id))
                        .filter(schema::users::types.lt(10))
                        .load::<User>(&_connection)
                        .expect("E.");
                    if _users.len() > 0 {
                        let us = _users.into_iter().nth(0).unwrap();
                        list.add_in_user_collections(us);
                    }
                }
            }
            return true;
        }
        else {
            return false;
        }
    }
    pub fn get_videos_ids(&self) -> Vec<i32> {
        use crate::schema::videos::dsl::videos;

        let _connection = establish_connection();
        let fix_list = videos
            .filter(schema::videos::video_list_id.eq(self.id))
            .filter(schema::videos::types.lt("b"))
            .load::<Video>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in fix_list.iter() {
            stack.push(_item.id);
        };
        return stack;
    }
    pub fn get_user_lists(user_pk: i32) -> Vec<VideoList> {
        use crate::schema::user_video_list_collections::dsl::user_video_list_collections;
        use crate::schema::user_video_list_positions::dsl::user_video_list_positions;
        use crate::schema::video_lists::dsl::video_lists;

        let _connection = establish_connection();
        let position_lists = user_video_list_positions
            .filter(schema::user_video_list_positions::user_id.eq(user_pk))
            .filter(schema::user_video_list_positions::types.eq("a"))
            .load::<UserVideoListPosition>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            let mut stack = Vec::new();
            for _item in position_lists.iter() {
                stack.push(_item.list_id);
            };
            return video_lists
                .filter(schema::video_lists::id.eq_any(stack))
                .filter(schema::video_lists::types.lt(10))
                .load::<VideoList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let user_lists = video_lists
            .filter(schema::video_lists::user_id.eq(user_pk))
            .filter(schema::video_lists::types.lt(10))
            .load::<VideoList>(&_connection)
            .expect("E.");
        for _item in user_lists.iter() {
            stack.push(_item.id);
        };
        let user_collections = user_video_list_collections
            .filter(schema::user_video_list_collections::user_id.eq(user_pk))
            .load::<UserVideoListCollection>(&_connection)
            .expect("E.");
        for _item in user_collections.iter() {
            stack.push(_item.video_list_id);
        };
        return video_lists
            .filter(schema::video_lists::id.eq_any(stack))
            .filter(schema::video_lists::types.lt(10))
            .load::<VideoList>(&_connection)
            .expect("E.");
    }
    pub fn get_community_lists(community_pk: i32) -> Vec<VideoList> {
        use crate::schema::community_video_list_collections::dsl::community_video_list_collections;
        use crate::schema::community_video_list_positions::dsl::community_video_list_positions;
        use crate::schema::video_lists::dsl::video_lists;

        let _connection = establish_connection();
        let position_lists = community_video_list_positions
            .filter(schema::community_video_list_positions::community_id.eq(community_pk))
            .filter(schema::community_video_list_positions::types.eq("a"))
            .load::<CommunityVideoListPosition>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            let mut stack = Vec::new();
            for _item in position_lists.iter() {
                stack.push(_item.list_id);
            };
            return video_lists
                .filter(schema::video_lists::id.eq_any(stack))
                .filter(schema::video_lists::types.lt(10))
                .load::<VideoList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let community_lists = video_lists
            .filter(schema::video_lists::community_id.eq(community_pk))
            .filter(schema::video_lists::types.lt(10))
            .load::<VideoList>(&_connection)
            .expect("E.");
        for _item in community_lists.iter() {
            stack.push(_item.id);
        };
        let community_collections = community_video_list_collections
            .filter(schema::community_video_list_collections::community_id.eq(community_pk))
            .load::<CommunityVideoListCollection>(&_connection)
            .expect("E.");
        for _item in community_collections.iter() {
            stack.push(_item.video_list_id);
        };
        return video_lists
            .filter(schema::video_lists::id.eq_any(stack))
            .filter(schema::video_lists::types.lt(10))
            .load::<VideoList>(&_connection)
            .expect("E.");

    }
    pub fn close_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            1 => 21,
            2 => 22,
            3 => 23,
            4 => 24,
            5 => 25,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::video_lists::types.eq(close_case))
            .get_result::<VideoList>(&_connection)
            .expect("E");
       return true;
    }
    pub fn unclose_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            21 => 1,
            22 => 2,
            23 => 3,
            24 => 4,
            25 => 5,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::video_lists::types.eq(close_case))
            .get_result::<VideoList>(&_connection)
            .expect("E");
       return true;
    }

    pub fn delete_item(&self) -> bool {
        let _connection = establish_connection();
        if self.community_id.is_some() {
            use crate::schema::community_video_list_positions::dsl::community_video_list_positions;

            let list_positions = community_video_list_positions
                .filter(schema::community_video_list_positions::community_id.eq(self.community_id.unwrap()))
                .filter(schema::community_video_list_positions::list_id.eq(self.id))
                .load::<CommunityVideoListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::community_video_list_positions::types.eq("b"))
                  .get_result::<CommunityVideoListPosition>(&_connection)
                  .expect("Error.");
            }
        } else {
            use crate::schema::user_video_list_positions::dsl::user_video_list_positions;

            let list_positions = user_video_list_positions
                .filter(schema::user_video_list_positions::user_id.eq(self.user_id))
                .filter(schema::user_video_list_positions::list_id.eq(self.id))
                .load::<UserVideoListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::user_video_list_positions::types.eq("b"))
                  .get_result::<UserVideoListPosition>(&_connection)
                  .expect("Error.");
            }
        }
        let user_types = self.types;
        let close_case = match user_types {
            1 => 11,
            2 => 12,
            3 => 13,
            4 => 14,
            5 => 15,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::video_lists::types.eq(close_case))
            .get_result::<VideoList>(&_connection)
            .expect("E");
       return true;
    }
    pub fn restore_item(&self) -> bool {
        let _connection = establish_connection();
        if self.community_id.is_some() {
            use crate::schema::community_video_list_positions::dsl::community_video_list_positions;

            let list_positions = community_video_list_positions
                .filter(schema::community_video_list_positions::community_id.eq(self.community_id.unwrap()))
                .filter(schema::community_video_list_positions::list_id.eq(self.id))
                .load::<CommunityVideoListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::community_video_list_positions::types.eq("b"))
                  .get_result::<CommunityVideoListPosition>(&_connection)
                  .expect("Error.");
            }
        } else {
            use crate::schema::user_video_list_positions::dsl::user_video_list_positions;

            let list_positions = user_video_list_positions
                .filter(schema::user_video_list_positions::user_id.eq(self.user_id))
                .filter(schema::user_video_list_positions::list_id.eq(self.id))
                .load::<UserVideoListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::user_video_list_positions::types.eq("a"))
                  .get_result::<UserVideoListPosition>(&_connection)
                  .expect("Error.");
            }
        }
        let user_types = self.types;
        let close_case = match user_types {
            11 => 1,
            12 => 2,
            13 => 3,
            14 => 4,
            15 => 5,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::video_lists::types.eq(close_case))
            .get_result::<VideoList>(&_connection)
            .expect("E");
       return true;
    }

    pub fn suspend_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            1 => 31,
            2 => 32,
            3 => 33,
            4 => 34,
            5 => 35,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::video_lists::types.eq(close_case))
            .get_result::<VideoList>(&_connection)
            .expect("E");
       return true;
    }
    pub fn unsuspend_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            31 => 1,
            32 => 2,
            33 => 3,
            34 => 4,
            35 => 5,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::video_lists::types.eq(close_case))
            .get_result::<VideoList>(&_connection)
            .expect("E");
       return true;
    }
    pub fn is_user_can_edit_delete_item(&self, user_id: i32) -> bool {
        if self.community_id.is_some() {
            let community = self.get_community();
            return community.get_staff_users_ids().iter().any(|&i| i==user_id);
        }
        else {
            return self.user_id == user_id;
        }
    }
    pub fn create_video(&self, title: String, community_id: Option<i32>, user_id: i32,
        preview: Option<String>, image: Option<String>, file: String,
        description: Option<String>, comment_enabled: bool,
        category_id: Option<i32>
    ) -> Video {

        let _connection = establish_connection();

        diesel::update(&*self)
          .set(schema::video_lists::count.eq(self.count + 1))
          .get_result::<VideoList>(&_connection)
          .expect("Error.");

        let new_video_form = NewVideo {
          title: title,
          community_id: community_id,
          user_id: user_id,
          video_list_id: self.id,
          types: "a".to_string(),
          preview: preview,
          image: image,
          file: file,
          description: description,
          comment_enabled: comment_enabled,
          created: chrono::Local::now().naive_utc(),

          comment: 0,
          view: 0,
          repost: 0,
          copy: 0,
          position: (self.count).try_into().unwrap(),
          category_id: category_id,
          reactions: 0,
        };
        let new_video = diesel::insert_into(schema::videos::table)
            .values(&new_video_form)
            .get_result::<Video>(&_connection)
            .expect("Error.");

        if community_id.is_some() {
            let community = self.get_community();
            community.plus_videos(1);
            return new_video;
        }
        else {
            use crate::utils::get_user;

            let creator = get_user(user_id);
            creator.plus_videos(1);
            return new_video;
        }
    }
}
/////// Video //////

//////////// тип
// 'a' Опубликовано
// 'b' Закрепленый
// 'c' Удаленый
// 'd' Черновик владельца
// 'e' Черновик предложки
// 'f' Предложка сообщества
// 'g' Предложка пользователя
// 'h' Закрыто модератором
// 'i' Удаленый предложенный в сообщество
// 'y' Удаленый предложенный у пользователя

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(User)]
#[belongs_to(VideoList)]
pub struct Video {
    pub id:              i32,
    pub title:           String,
    pub community_id:    Option<i32>,
    pub user_id:         i32,
    pub video_list_id:   i32,
    pub types:           String,
    pub preview:         Option<String>,
    pub image:           Option<String>,
    pub file:            String,
    pub description:     Option<String>,
    pub comment_enabled: bool,
    pub created:         chrono::NaiveDateTime,

    pub comment:         i32,
    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub category_id:     Option<i32>,
    pub reactions:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="videos"]
pub struct NewVideo {
    pub title:           String,
    pub community_id:    Option<i32>,
    pub user_id:         i32,
    pub video_list_id:   i32,
    pub types:           String,
    pub preview:         Option<String>,
    pub image:           Option<String>,
    pub file:            String,
    pub description:     Option<String>,
    pub comment_enabled: bool,
    pub created:         chrono::NaiveDateTime,

    pub comment:         i32,
    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub category_id:     Option<i32>,
    pub reactions:       i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="videos"]
pub struct EditVideo {
    pub title:           String,
    pub preview:         Option<String>,
    pub image:           Option<String>,
    pub description:     Option<String>,
    pub comment_enabled: bool,
    pub category_id:     Option<i32>,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="videos"]
pub struct EditVideoPosition {
    pub position:        i16,
}

impl Video {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_video(&self) -> bool {
        return true;
    }
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/no_img/list.jpg".to_string();
        }
    }
    pub fn get_code(&self) -> String {
        return "vid".to_string() + &self.get_str_id();
    }
    pub fn get_folder(&self) -> String {
        return "video".to_string();
    }
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(56))
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
            .filter(schema::moderateds::types.eq(56))
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

    pub fn get_or_create_react_model(&self) -> VideoReaction {
        use crate::schema::video_reactions::dsl::video_reactions;

        let _connection = establish_connection();
        let _react_model = video_reactions
            .filter(schema::video_reactions::video_id.eq(self.id))
            .load::<VideoReaction>(&_connection)
            .expect("E.");
        if _react_model.len() > 0 {
            return _react_model.into_iter().nth(0).unwrap();
        }
        else {
            let new_react_model = NewVideoReaction {
                video_id: self.id,
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
            let _react_model = diesel::insert_into(schema::video_reactions::table)
                .values(&new_react_model)
                .get_result::<VideoReaction>(&_connection)
                .expect("Error.");

            return _react_model;
        }
    }

    pub fn send_reaction(&self, user_id: i32, types: i16) -> Json<JsonItemReactions> {
        use crate::schema::video_votes::dsl::video_votes;

        let _connection = establish_connection();
        let list = self.get_list();
        let reactions_of_list = list.get_reactions_list();
        let react_model = self.get_or_create_react_model();
        let mut new_plus = false;
        let mut old_type = 0;

        if reactions_of_list.iter().any(|&i| i==types) && list.is_user_can_see_el(user_id) {

            let votes = video_votes
                .filter(schema::video_votes::user_id.eq(user_id))
                .filter(schema::video_votes::video_id.eq(self.id))
                .load::<VideoVote>(&_connection)
                .expect("E.");

            // если пользователь уже реагировал на товар
            if votes.len() > 0 {
                let vote = votes.into_iter().nth(0).unwrap();

                // если пользователь уже реагировал этой реакцией на этот товар
                if vote.reaction == types {
                    diesel::delete(video_votes
                        .filter(schema::video_votes::user_id.eq(user_id))
                        .filter(schema::video_votes::video_id.eq(self.id))
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
                        .set(schema::video_votes::reaction.eq(types))
                        .get_result::<VideoVote>(&_connection)
                        .expect("Error.");

                    react_model.update_model(types, Some(old_type), false);
                }
            }

            // если пользователь не реагировал на этот товар
            else {
                let new_vote = NewVideoVote {
                    vote: 1,
                    user_id: user_id,
                    video_id: self.id,
                    reaction: types,
                };
                diesel::insert_into(schema::video_votes::table)
                    .values(&new_vote)
                    .get_result::<VideoVote>(&_connection)
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

    pub fn get_community(&self) -> Community {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::id.eq(self.community_id.unwrap()))
            .filter(schema::communitys::types.lt(10))
            .load::<Community>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
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
    pub fn is_user_can_edit_delete_item(&self, user_id: i32) -> bool {
        if self.community_id.is_some() {
            let community = self.get_community();
            return community.get_staff_users_ids().iter().any(|&i| i==user_id);
        }
        else {
            return self.user_id == user_id;
        }
    }
    pub fn get_description(&self) -> String {
        if self.community_id.is_some() {
            let community = self.get_community();
            return "видеозапись сообщества <a href='".to_owned() + &community.link.to_string() + &"' target='_blank'>" + &community.name + &"</a>"
        }
        else {
            let creator = self.get_creator();
            return "<a href='".to_owned() + &creator.link.to_string() + &"' target='_blank'>" + &creator.get_full_name() + &"</a>" + &": видеозапись"
        }
    }

    pub fn get_list(&self) -> VideoList {
        use crate::schema::video_lists::dsl::video_lists;

        let _connection = establish_connection();
        return video_lists
            .filter(schema::video_lists::id.eq(self.video_list_id))
            .filter(schema::video_lists::types.lt(10))
            .load::<VideoList>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn copy_item(pk: i32, lists: Vec<i32>) -> bool {
        use crate::schema::videos::dsl::videos;
        use crate::schema::video_lists::dsl::video_lists;

        let _connection = establish_connection();
        let item = videos
            .filter(schema::videos::id.eq(pk))
            .filter(schema::videos::types.eq("a"))
            .load::<Video>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
        let mut count = 0;
        for list_id in lists.iter() {
            count += 1;
            let list = video_lists
                .filter(schema::video_lists::id.eq(list_id))
                .filter(schema::video_lists::types.lt(10))
                .load::<VideoList>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            list.create_video (
                item.title.clone(),
                item.community_id,
                item.user_id,
                item.preview.clone(),
                item.image.clone(),
                item.file.clone(),
                None,
                true,
                item.category_id,
            );
        }
        diesel::update(&item)
          .set(schema::videos::copy.eq(item.copy + count))
          .get_result::<Video>(&_connection)
          .expect("Error.");

        if item.community_id.is_some() {
            let community = item.get_community();
            community.plus_videos(count);
        }
        else {
            let creator = item.get_creator();
            creator.plus_videos(count);
          }
        return true;
    }

    pub fn edit_video(self, title: String, preview: Option<String>,
        image: Option<String>, description: Option<String>,
        comment_enabled: bool, category_id: Option<i32>) -> Video {

        let _connection = establish_connection();

        let edit_video = EditVideo {
            title: title,
            preview: preview,
            image: image,
            description: description,
            comment_enabled: comment_enabled,
            category_id: category_id,
        };
        diesel::update(&self)
            .set(edit_video)
            .get_result::<Video>(&_connection)
            .expect("Error.");
        return self;
    }

    pub fn plus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::videos::comment.eq(self.comment + count))
            .get_result::<Video>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_reactions(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::videos::reactions.eq(self.reactions + count))
            .get_result::<Video>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_reactions(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::videos::reactions.eq(self.reactions - count))
            .get_result::<Video>(&_connection)
            .expect("Error.");
        return true;
    }

    pub fn minus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::videos::comment.eq(self.comment - count))
            .get_result::<Video>(&_connection)
            .expect("Error.");
        return true;
    }

    pub fn is_open(&self) -> bool {
        return self.types == "a" && self.types == "b";
    }
    pub fn is_deleted(&self) -> bool {
        return self.types == "c";
    }
    pub fn is_closed(&self) -> bool {
        return self.types == "h";
    }

    pub fn delete_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "a" => "c",
            "b" => "m",
            "f" => "i",
            "g" => "y",
            _ => "c",
        };
        diesel::update(self)
            .set(schema::videos::types.eq(close_case))
            .get_result::<Video>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::video_lists::count.eq(list.count - 1))
            .get_result::<VideoList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.minus_videos(1);
        }
        else {
            let creator = self.get_creator();
            creator.minus_videos(1);
         }
      return true;
    }
    pub fn restore_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "c" => "a",
            "m" => "b",
            "i" => "f",
            "y" => "g",
            _ => "a",
        };
        diesel::update(self)
            .set(schema::videos::types.eq(close_case))
            .get_result::<Video>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::video_lists::count.eq(list.count + 1))
            .get_result::<VideoList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.plus_videos(1);
        }
        else {
            let creator = self.get_creator();
            creator.plus_videos(1);
         }
       return true;
    }

    pub fn close_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "a" => "h",
            "b" => "n",
            _ => "h",
        };
        diesel::update(self)
            .set(schema::videos::types.eq(close_case))
            .get_result::<Video>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::video_lists::count.eq(list.count - 1))
            .get_result::<VideoList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.minus_videos(1);
        }
        else {
            let creator = self.get_creator();
            creator.minus_videos(1);
        }
       return true;
    }
    pub fn unclose_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "h" => "a",
            "n" => "b",
            _ => "a",
        };
        diesel::update(self)
            .set(schema::videos::types.eq(close_case))
            .get_result::<Video>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::video_lists::count.eq(list.count + 1))
            .get_result::<VideoList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.plus_videos(1);
        }
        else {
            let creator = self.get_creator();
            creator.plus_videos(1);
         }
       return true;
    }
    pub fn get_format_text(&self) -> String {
        if self.description.is_some() {
            let unwrap = self.description.as_ref().unwrap();
            if unwrap.len() <= 101 {
                return self.description.as_ref().unwrap().to_string();
            }
            else {
                let new_str = unwrap[..100].to_owned() + &"<br><a class='pointer show_post_text'>Показать полностью...</a><br><span style='display:none'>" + &unwrap[101..] + &"</span>";
                return new_str;
            }
        } else { return "".to_string(); }
    }

    pub fn count_comments(&self) -> String {
        if self.comment == 0 {
            return "".to_string();
        }
        else {
            return self.comment.to_string();
        }
    }

    pub fn count_reposts(&self) -> String {
        if self.repost == 0 {
            return "".to_string();
        }
        else {
            return self.repost.to_string();
        }
    }

    pub fn reposts_count_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.repost,
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }

    pub fn is_have_reposts(&self) -> bool {
        return self.repost > 0;
    }
    pub fn count_reactions(&self) -> String {
        if self.reactions == 0 {
            return "".to_string();
        }
        else {
            return self.reactions.to_string();
        }
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
        use crate::schema::video_votes::dsl::video_votes;

        let _connection = establish_connection();
        let votes = video_votes
            .filter(schema::video_votes::video_id.eq(self.id))
            .load::<VideoVote>(&_connection)
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
        use crate::schema::video_votes::dsl::video_votes;
        // "/static/images/reactions/" + get_user_reaction + ".jpg"
        let _connection = establish_connection();
        let vote = video_votes
            .filter(schema::video_votes::user_id.eq(user_id))
            .filter(schema::video_votes::video_id.eq(self.id))
            .load::<VideoVote>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();

        return vote.reaction;
    }

    pub fn get_reactions_users_of_types(&self, limit: i64, offset: i64, types: i16) -> Vec<User> {
        use crate::schema::video_votes::dsl::video_votes;
        use crate::utils::get_users_from_ids;

        let _connection = establish_connection();
        let votes = video_votes
            .filter(schema::video_votes::video_id.eq(self.id))
            .filter(schema::video_votes::reaction.eq(types))
            .limit(limit)
            .offset(offset)
            .load::<VideoVote>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in votes.iter() {
            stack.push(_item.user_id);
        };
        return get_users_from_ids(stack);
    }

    pub fn get_6_reactions_users_of_types(&self, types: i16) -> Vec<User> {
        use crate::schema::video_votes::dsl::video_votes;
        use crate::utils::get_users_from_ids;

        let _connection = establish_connection();
        let votes = video_votes
            .filter(schema::video_votes::video_id.eq(self.id))
            .filter(schema::video_votes::reaction.eq(types))
            .limit(6)
            .load::<VideoVote>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in votes.iter() {
            stack.push(_item.user_id);
        };
        return get_users_from_ids(stack);
    }

    pub fn change_position(query: Json<Vec<JsonPosition>>) -> bool {
        use crate::schema::videos::dsl::videos;

        let _connection = establish_connection();
        for i in query.iter() {
            let item = videos
                .filter(schema::videos::id.eq(i.key))
                .filter(schema::videos::types.eq("a"))
                .limit(1)
                .load::<Video>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            diesel::update(&item)
                .set(schema::videos::position.eq(i.value))
                .get_result::<Video>(&_connection)
                .expect("Error.");
        }
        return true;
    }

    pub fn create_comment(&self, user: &User, attach: Option<String>,
        parent_id: Option<i32>, content: Option<String>, sticker_id: Option<i32>) -> VideoComment {

        let _connection = establish_connection();
        diesel::update(self)
          .set(schema::videos::comment.eq(self.comment + 1))
          .get_result::<Video>(&_connection)
          .expect("Error.");

        let new_comment_form = NewVideoComment {
            video_id:   self.id,
            user_id:    user.id,
            sticker_id: sticker_id,
            parent_id:  parent_id,
            content:    content,
            attach:     attach,
            types:      "a".to_string(),
            created:    chrono::Local::now().naive_utc(),
            repost:     0,
            reactions:  0,
        };
        let new_comment = diesel::insert_into(schema::video_comments::table)
            .values(&new_comment_form)
            .get_result::<VideoComment>(&_connection)
            .expect("Error.");

        return new_comment;
    }
    pub fn get_comments(&self, limit: i64, offset: i64) -> Vec<VideoComment> {
        use crate::schema::video_comments::dsl::video_comments;

        let _connection = establish_connection();

        return video_comments
            .filter(schema::video_comments::video_id.eq(self.id))
            .filter(schema::video_comments::types.eq_any(vec!["a","b"]))
            .limit(limit)
            .offset(offset)
            .load::<VideoComment>(&_connection)
            .expect("E.");
    }

    pub fn count_copy(&self) -> String {
        if self.copy == 0 {
            return "".to_string();
        }
        else {
            return ", копировали - ".to_string() + &self.copy.to_string();
        }
    }
    pub fn message_reposts_count(&self) -> String {
        use crate::schema::video_reposts::dsl::video_reposts;

        let _connection = establish_connection();

        let count = video_reposts
            .filter(schema::video_reposts::video_id.eq(self.id))
            .filter(schema::video_reposts::message_id.is_not_null())
            .load::<VideoRepost>(&_connection)
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
        use crate::schema::video_reposts::dsl::video_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = video_reposts
            .filter(schema::video_reposts::video_id.eq(self.id))
            .filter(schema::video_reposts::post_id.is_not_null())
            .limit(limit)
            .offset(offset)
            .load::<VideoRepost>(&_connection)
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
        use crate::schema::video_reposts::dsl::video_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = video_reposts
            .filter(schema::video_reposts::video_id.eq(self.id))
            .filter(schema::video_reposts::post_id.is_not_null())
            .limit(6)
            .load::<VideoRepost>(&_connection)
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
}
/////// VideoComment //////

    // 'a' Опубликованный
    // 'b' Изменённый
    // 'c' Удаленый
    // 'd' Изменённый Удаленый
    // 'e' Закрытый модератором
    // 'f' Закрытый Удаленый

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Video)]
#[belongs_to(User)]
#[belongs_to(Sticker)]
pub struct VideoComment {
    pub id:         i32,
    pub video_id:   i32,
    pub user_id:    i32,
    pub sticker_id: Option<i32>,
    pub parent_id:  Option<i32>,
    pub content:    Option<String>,
    pub types:      String,
    pub attach:     Option<String>,
    pub created:    chrono::NaiveDateTime,
    pub repost:     i32,
    pub reactions:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="video_comments"]
pub struct NewVideoComment {
    pub video_id:   i32,
    pub user_id:    i32,
    pub sticker_id: Option<i32>,
    pub parent_id:  Option<i32>,
    pub content:    Option<String>,
    pub types:      String,
    pub attach:     Option<String>,
    pub created:    chrono::NaiveDateTime,
    pub repost:     i32,
    pub reactions:  i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="video_comments"]
pub struct EditVideoComment {
    pub content:    Option<String>,
    pub attach:     Option<String>,
}

impl VideoComment {
    pub fn get_attach(&self, user_id: i32) -> String {
        if self.attach.is_some() {
            use crate::utils::comment_elements;
            return comment_elements(self.attach.as_ref().unwrap().to_string(), user_id);
        }
        else {
            return "".to_string();
        }
    }
    pub fn is_deleted(&self) -> bool {
        return self.types == "c" && self.types == "d";
    }
    pub fn is_closed(&self) -> bool {
        return self.types == "e" && self.types == "f";
    }
    pub fn get_anon_attach(&self) -> String {
        if self.attach.is_some() {
            use crate::utils::anon_comment_elements;
            return anon_comment_elements(self.attach.as_ref().unwrap().to_string());
        }
        else {
            return "".to_string();
        }
    }
    pub fn get_edit_attach(&self) -> String {
        if self.attach.is_some() {
            use crate::utils::edit_comment_elements;
            return edit_comment_elements(self.attach.as_ref().unwrap().to_string());
        }
        else {
            return "".to_string();
        }
    }
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_video_comment(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "cvi".to_string() + &self.get_str_id();
    }
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(83))
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
            .filter(schema::moderateds::types.eq(83))
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
    pub fn get_community(&self) -> Community {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::id.eq(self.get_item().community_id.unwrap()))
            .filter(schema::communitys::types.lt(10))
            .load::<Community>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_creator(&self) -> User {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::id.eq(self.get_item().user_id))
            .filter(schema::users::types.lt(10))
            .load::<User>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
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
    pub fn get_commenter(&self) -> User {
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
    pub fn get_item(&self) -> Video {
        use crate::schema::videos::dsl::videos;

        let _connection = establish_connection();
        return videos
            .filter(schema::videos::id.eq(self.video_id))
            .filter(schema::videos::types.eq("a"))
            .load::<Video>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_list(&self) -> VideoList {
        return self.get_item().get_list();
    }
    pub fn get_parent(&self) -> VideoComment {
        use crate::schema::video_comments::dsl::video_comments;

        let _connection = establish_connection();
        return video_comments
            .filter(schema::video_comments::id.eq(self.parent_id.unwrap()))
            .filter(schema::video_comments::types.eq_any(vec!["a", "b"]))
            .load::<VideoComment>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_description(&self) -> String {
        if self.get_item().community_id.is_some() {
            let community = self.get_community();
            return "запись сообщества <a href='".to_owned() + &community.link.to_string() + &"' target='_blank'>" + &community.name + &"</a>"
        }
        else {
            let creator = self.get_creator();
            return "<a href='".to_owned() + &creator.link.to_string() + &"' target='_blank'>" + &creator.get_full_name() + &"</a>" + &": запись"
        }
    }

    pub fn get_attach_photos(&self) -> Vec<Photo> {
        use crate::schema::photos::dsl::photos;

        let _connection = establish_connection();
        let attach = self.attach.as_ref().unwrap().to_string();
        let v: Vec<&str> = attach.split(",").collect();
        let mut stack = Vec::new();
        for item in v.iter() {
            let pk: i32 = item[3..].parse().unwrap();
            let code = &item[..3];
            if code == "pho".to_string() {
                stack.push(pk);
            }
        }

        return photos
            .filter(schema::photos::id.eq_any(stack))
            .load::<Photo>(&_connection)
            .expect("E");
    }
    pub fn get_attach_videos(&self) -> Vec<Video> {
        use crate::schema::videos::dsl::videos;

        let _connection = establish_connection();
        let attach = self.attach.as_ref().unwrap().to_string();
        let v: Vec<&str> = attach.split(",").collect();
        let mut stack = Vec::new();
        for item in v.iter() {
            let pk: i32 = item[3..].parse().unwrap();
            let code = &item[..3];
            if code == "vid".to_string() {
                stack.push(pk);
            }
        }

        return videos
            .filter(schema::videos::id.eq_any(stack))
            .load::<Video>(&_connection)
            .expect("E");
    }



    pub fn get_replies(&self) -> Vec<VideoComment> {
        use crate::schema::video_comments::dsl::video_comments;

        let _connection = establish_connection();
        return video_comments
            .filter(schema::video_comments::video_id.eq(self.id))
            .filter(schema::video_comments::types.eq_any(vec!["a", "b"]))
            .load::<VideoComment>(&_connection)
            .expect("E");
    }
    pub fn count_replies(&self) -> usize {
        return self.get_replies().len();
    }
    pub fn count_replies_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        let count_usize: usize = self.count_replies() as usize;
        return get_count_for_ru (
            count_usize.try_into().unwrap(),
            " ответ".to_string(),
            " ответа".to_string(),
            " ответов".to_string(),
        );
    }
    pub fn close_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "a" => "e".to_string(),
            "b" => "f".to_string(),
            _ => "e".to_string(),
        };
        let item = self.get_item();
        diesel::update(&item)
            .set(schema::videos::comment.eq(item.comment - 1))
            .get_result::<Video>(&_connection)
            .expect("E");

        diesel::update(self)
            .set(schema::video_comments::types.eq(close_case))
            .get_result::<VideoComment>(&_connection)
            .expect("E");
       return true;
    }
    pub fn unclose_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "e" => "a".to_string(),
            "f" => "b".to_string(),
            _ => "a".to_string(),
        };
        let item = self.get_item();
        diesel::update(&item)
            .set(schema::videos::comment.eq(item.comment + 1))
            .get_result::<Video>(&_connection)
            .expect("E");

        diesel::update(self)
            .set(schema::video_comments::types.eq(close_case))
            .get_result::<VideoComment>(&_connection)
            .expect("E");
       return true;
    }

    pub fn delete_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "a" => "c".to_string(),
            "b" => "d".to_string(),
            _ => "c".to_string(),
        };
        let item = self.get_item();
        diesel::update(&item)
            .set(schema::videos::comment.eq(item.comment - 1))
            .get_result::<Video>(&_connection)
            .expect("E");

        diesel::update(self)
            .set(schema::video_comments::types.eq(close_case))
            .get_result::<VideoComment>(&_connection)
            .expect("E");
       return true;
    }
    pub fn restore_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "c" => "a".to_string(),
            "d" => "b".to_string(),
            _ => "a".to_string(),
        };
        let item = self.get_item();
        diesel::update(&item)
            .set(schema::videos::comment.eq(item.comment + 1))
            .get_result::<Video>(&_connection)
            .expect("E");

        diesel::update(self)
            .set(schema::video_comments::types.eq(close_case))
            .get_result::<VideoComment>(&_connection)
            .expect("E");
       return true;
    }
    pub fn get_count_attach(&self) -> String {
        if self.attach.is_some() {
            let length = self.attach.as_deref().unwrap().split(",").collect::<Vec<_>>().len();
            if length == 1 {
                return "files_one".to_string();
            }
            else if length == 2 {
                return "files_two".to_string();
            }
        }
        return "files_null".to_string();
    }
    pub fn get_format_text(&self) -> String {
        if self.content.is_some() {
            let unwrap = self.content.as_ref().unwrap();
            if unwrap.len() <= 101 {
                return self.content.as_ref().unwrap().to_string();
            }
            else {
                let new_str = unwrap[..100].to_owned() + &"<br><a class='pointer show_post_text'>Показать полностью...</a><br><span style='display:none'>" + &unwrap[101..] + &"</span>";
                return new_str;
            }
        } else { return "".to_string(); }
    }

    pub fn count_reactions(&self) -> String {
        if self.reactions == 0 {
            return "".to_string();
        }
        else {
            return self.reactions.to_string();
        }
    }

    pub fn get_or_create_react_model(&self) -> VideoCommentReaction {
        use crate::schema::video_comment_reactions::dsl::video_comment_reactions;

        let _connection = establish_connection();
        let _react_model = video_comment_reactions
            .filter(schema::video_comment_reactions::video_comment_id.eq(self.id))
            .load::<VideoCommentReaction>(&_connection)
            .expect("E.");
        if _react_model.len() > 0 {
            return _react_model.into_iter().nth(0).unwrap();
        }
        else {
            let new_react_model = NewVideoCommentReaction {
                video_comment_id: self.id,
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
            let _react_model = diesel::insert_into(schema::video_comment_reactions::table)
                .values(&new_react_model)
                .get_result::<VideoCommentReaction>(&_connection)
                .expect("Error.");

            return _react_model;
        }
    }

    pub fn send_reaction(&self, user_id: i32, types: i16) -> Json<JsonItemReactions> {
        use crate::schema::video_comment_votes::dsl::video_comment_votes;

        let _connection = establish_connection();
        let list = self.get_list();
        let reactions_of_list = list.get_reactions_list();
        let react_model = self.get_or_create_react_model();
        let mut new_plus = false;
        let mut old_type = 0;

        if reactions_of_list.iter().any(|&i| i==types) && list.is_user_can_see_el(user_id) && list.is_user_can_see_comment(user_id) {

            let votes = video_comment_votes
                .filter(schema::video_comment_votes::user_id.eq(user_id))
                .filter(schema::video_comment_votes::video_comment_id.eq(self.id))
                .load::<VideoCommentVote>(&_connection)
                .expect("E.");

            // если пользователь уже реагировал на товар
            if votes.len() > 0 {
                let vote = votes.into_iter().nth(0).unwrap();

                // если пользователь уже реагировал этой реакцией на этот товар
                if vote.reaction == types {
                    diesel::delete(video_comment_votes
                        .filter(schema::video_comment_votes::user_id.eq(user_id))
                        .filter(schema::video_comment_votes::video_comment_id.eq(self.id))
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
                        .set(schema::video_comment_votes::reaction.eq(types))
                        .get_result::<VideoCommentVote>(&_connection)
                        .expect("Error.");

                    react_model.update_model(types, Some(old_type), false);
                }
            }

            // если пользователь не реагировал на этот товар
            else {
                let new_vote = NewVideoCommentVote {
                    vote:            1,
                    user_id:         user_id,
                    video_comment_id: self.id,
                    reaction:        types,
                };
                diesel::insert_into(schema::video_comment_votes::table)
                    .values(&new_vote)
                    .get_result::<VideoCommentVote>(&_connection)
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
        use crate::schema::video_comment_votes::dsl::video_comment_votes;

        let _connection = establish_connection();
        let votes = video_comment_votes
            .filter(schema::video_comment_votes::video_comment_id.eq(self.id))
            .load::<VideoCommentVote>(&_connection)
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
        use crate::schema::video_comment_votes::dsl::video_comment_votes;
        // "/static/images/reactions/" + get_user_reaction + ".jpg"
        let _connection = establish_connection();
        let vote = video_comment_votes
            .filter(schema::video_comment_votes::user_id.eq(user_id))
            .filter(schema::video_comment_votes::video_comment_id.eq(self.id))
            .load::<VideoCommentVote>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();

        return vote.reaction;
    }

    pub fn get_reactions_users_of_types(&self, limit: i64, offset: i64, types: i16) -> Vec<User> {
        use crate::schema::video_comment_votes::dsl::video_comment_votes;
        use crate::utils::get_users_from_ids;

        let _connection = establish_connection();
        let votes = video_comment_votes
            .filter(schema::video_comment_votes::video_comment_id.eq(self.id))
            .filter(schema::video_comment_votes::reaction.eq(types))
            .limit(limit)
            .offset(offset)
            .load::<VideoCommentVote>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in votes.iter() {
            stack.push(_item.user_id);
        };
        return get_users_from_ids(stack);
    }

    pub fn get_6_reactions_users_of_types(&self, types: i16) -> Vec<User> {
        use crate::schema::video_comment_votes::dsl::video_comment_votes;
        use crate::utils::get_users_from_ids;

        let _connection = establish_connection();
        let votes = video_comment_votes
            .filter(schema::video_comment_votes::video_comment_id.eq(self.id))
            .filter(schema::video_comment_votes::reaction.eq(types))
            .limit(6)
            .load::<VideoCommentVote>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in votes.iter() {
            stack.push(_item.user_id);
        };
        return get_users_from_ids(stack);
    }

    pub fn plus_reactions(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::video_comments::reactions.eq(self.reactions + count))
            .get_result::<VideoComment>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_reactions(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::video_comments::reactions.eq(self.reactions - count))
            .get_result::<VideoComment>(&_connection)
            .expect("Error.");
        return true;
    }
}

/////// UserVideoListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(VideoList)]
pub struct UserVideoListCollection {
    pub id:       i32,
    pub user_id:  i32,
    pub video_list_id:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_video_list_collections"]
pub struct NewUserVideoListCollection {
    pub user_id:  i32,
    pub video_list_id:  i32,
}

/////// CommunityVideoListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(VideoList)]
pub struct CommunityVideoListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub video_list_id:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_video_list_collections"]
pub struct NewCommunityVideoListCollection {
    pub community_id:  i32,
    pub video_list_id:       i32,
}

/////// VideoListPerm //////
    // 'a' Активно
    // 'b' Не активно
    // 'c' Нет значения

#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(VideoList)]
pub struct VideoListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub video_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub can_see_comment: Option<String>,
    pub create_item:     Option<String>,
    pub create_comment:  Option<String>,
    pub can_copy:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="video_list_perms"]
pub struct NewVideoListPerm {
    pub user_id:         i32,
    pub video_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub can_see_comment: Option<String>,
    pub create_item:     Option<String>,
    pub create_comment:  Option<String>,
    pub can_copy:        Option<String>,
}


/////// VideoVote//////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Video)]
pub struct VideoVote {
    pub id:       i32,
    pub vote:     i16,
    pub user_id:  i32,
    pub video_id: i32,
    pub reaction: i16,
}
impl VideoVote {
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
#[table_name="video_votes"]
pub struct NewVideoVote {
    pub vote:     i16,
    pub user_id:  i32,
    pub video_id: i32,
    pub reaction: i16,
}
/////// VideoCommentVote //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(VideoComment)]
pub struct VideoCommentVote {
    pub id:              i32,
    pub vote:            i16,
    pub user_id:         i32,
    pub video_comment_id: i32,
    pub reaction:        i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="video_comment_votes"]
pub struct NewVideoCommentVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub video_comment_id: i32,
    pub reaction:        i16,
}

/////// VideoListRepost //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(VideoList)]
#[belongs_to(Post)]
#[belongs_to(Message)]
pub struct VideoListRepost {
    pub id:            i32,
    pub video_list_id: i32,
    pub post_id:       Option<i32>,
    pub message_id:    Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="video_list_reposts"]
pub struct NewVideoListRepost {
    pub video_list_id: i32,
    pub post_id:       Option<i32>,
    pub message_id:    Option<i32>,
}

/////// VideoRepost //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Video)]
#[belongs_to(Post)]
#[belongs_to(Message)]
pub struct VideoRepost {
    pub id:         i32,
    pub video_id:   i32,
    pub post_id:    Option<i32>,
    pub message_id: Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="video_reposts"]
pub struct NewVideoRepost {
    pub video_id:   i32,
    pub post_id:    Option<i32>,
    pub message_id: Option<i32>,
}

/////// VideoReaction //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Video)]
pub struct VideoReaction {
    pub id:       i32,
    pub video_id: i32,
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

impl VideoReaction {
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
    ) -> &VideoReaction {
        let _connection = establish_connection();
        if old_types_option.is_some() {
            let old_types = old_types_option.unwrap();
            match new_types {
                1 => diesel::update(self)
                    .set(schema::video_reactions::field_1.eq(self.field_1 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::video_reactions::field_2.eq(self.field_2 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::video_reactions::field_3.eq(self.field_3 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::video_reactions::field_4.eq(self.field_4 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::video_reactions::field_5.eq(self.field_5 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::video_reactions::field_6.eq(self.field_6 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::video_reactions::field_7.eq(self.field_7 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::video_reactions::field_8.eq(self.field_8 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::video_reactions::field_9.eq(self.field_9 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::video_reactions::field_10.eq(self.field_10 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::video_reactions::field_11.eq(self.field_11 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::video_reactions::field_12.eq(self.field_12 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::video_reactions::field_13.eq(self.field_13 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::video_reactions::field_14.eq(self.field_14 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::video_reactions::field_15.eq(self.field_15 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::video_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::video_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
            };

            match old_types {
                1 => diesel::update(self)
                    .set(schema::video_reactions::field_1.eq(self.field_1 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::video_reactions::field_2.eq(self.field_2 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::video_reactions::field_3.eq(self.field_3 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::video_reactions::field_4.eq(self.field_4 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::video_reactions::field_5.eq(self.field_5 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::video_reactions::field_6.eq(self.field_6 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::video_reactions::field_7.eq(self.field_7 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::video_reactions::field_8.eq(self.field_8 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::video_reactions::field_9.eq(self.field_9 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::video_reactions::field_10.eq(self.field_10 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::video_reactions::field_11.eq(self.field_11 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::video_reactions::field_12.eq(self.field_12 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::video_reactions::field_13.eq(self.field_13 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::video_reactions::field_14.eq(self.field_14 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::video_reactions::field_15.eq(self.field_15 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::video_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::video_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<VideoReaction>(&_connection)
                    .expect("Error."),
            };
            return &self;
        }
        else {
            if plus {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::video_reactions::field_1.eq(self.field_1 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::video_reactions::field_2.eq(self.field_2 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::video_reactions::field_3.eq(self.field_3 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::video_reactions::field_4.eq(self.field_4 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::video_reactions::field_5.eq(self.field_5 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::video_reactions::field_6.eq(self.field_6 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::video_reactions::field_7.eq(self.field_7 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::video_reactions::field_8.eq(self.field_8 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::video_reactions::field_9.eq(self.field_9 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::video_reactions::field_10.eq(self.field_10 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::video_reactions::field_11.eq(self.field_11 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::video_reactions::field_12.eq(self.field_12 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::video_reactions::field_13.eq(self.field_13 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::video_reactions::field_14.eq(self.field_14 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::video_reactions::field_15.eq(self.field_15 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::video_reactions::field_16.eq(self.field_16 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::video_reactions::field_16.eq(self.field_16 + 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                };
            }
            else {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::video_reactions::field_1.eq(self.field_1 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::video_reactions::field_2.eq(self.field_2 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::video_reactions::field_3.eq(self.field_3 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::video_reactions::field_4.eq(self.field_4 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::video_reactions::field_5.eq(self.field_5 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::video_reactions::field_6.eq(self.field_6 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::video_reactions::field_7.eq(self.field_7 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::video_reactions::field_8.eq(self.field_8 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::video_reactions::field_9.eq(self.field_9 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::video_reactions::field_10.eq(self.field_10 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::video_reactions::field_11.eq(self.field_11 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::video_reactions::field_12.eq(self.field_12 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::video_reactions::field_13.eq(self.field_13 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::video_reactions::field_14.eq(self.field_14 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::video_reactions::field_15.eq(self.field_15 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::video_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::video_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<VideoReaction>(&_connection)
                        .expect("Error."),
                };
            }
            return &self;
        }
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="video_reactions"]
pub struct NewVideoReaction {
    pub video_id:    i32,
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


/////// VideoCommentReaction //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(VideoComment)]
pub struct VideoCommentReaction {
    pub id:              i32,
    pub video_comment_id:i32,
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
impl VideoCommentReaction {
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
    ) -> &VideoCommentReaction {
        let _connection = establish_connection();
        if old_types_option.is_some() {
            let old_types = old_types_option.unwrap();
            match new_types {
                1 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_1.eq(self.field_1 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::video_comment_reactions::field_2.eq(self.field_2 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_3.eq(self.field_3 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_4.eq(self.field_4 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_5.eq(self.field_5 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_6.eq(self.field_6 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_7.eq(self.field_7 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_8.eq(self.field_8 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_9.eq(self.field_9 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_10.eq(self.field_10 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_11.eq(self.field_11 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_12.eq(self.field_12 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_13.eq(self.field_13 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_14.eq(self.field_14 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_15.eq(self.field_15 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::video_comment_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
            };

            match old_types {
                1 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_1.eq(self.field_1 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::video_comment_reactions::field_2.eq(self.field_2 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_3.eq(self.field_3 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_4.eq(self.field_4 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_5.eq(self.field_5 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_6.eq(self.field_6 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_7.eq(self.field_7 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_8.eq(self.field_8 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_9.eq(self.field_9 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_10.eq(self.field_10 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_11.eq(self.field_11 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_12.eq(self.field_12 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_13.eq(self.field_13 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_14.eq(self.field_14 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_15.eq(self.field_15 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::video_comment_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::video_comment_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<VideoCommentReaction>(&_connection)
                    .expect("Error."),
            };
            return &self;
        }
        else {
            if plus {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_1.eq(self.field_1 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::video_comment_reactions::field_2.eq(self.field_2 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_3.eq(self.field_3 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_4.eq(self.field_4 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_5.eq(self.field_5 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_6.eq(self.field_6 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_7.eq(self.field_7 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_8.eq(self.field_8 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_9.eq(self.field_9 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_10.eq(self.field_10 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_11.eq(self.field_11 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_12.eq(self.field_12 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_13.eq(self.field_13 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_14.eq(self.field_14 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_15.eq(self.field_15 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_16.eq(self.field_16 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::video_comment_reactions::field_2.eq(self.field_2 + 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                };
            }
            else {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_1.eq(self.field_1 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::video_comment_reactions::field_2.eq(self.field_2 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_3.eq(self.field_3 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_4.eq(self.field_4 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_5.eq(self.field_5 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_6.eq(self.field_6 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_7.eq(self.field_7 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_8.eq(self.field_8 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_9.eq(self.field_9 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_10.eq(self.field_10 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_11.eq(self.field_11 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_12.eq(self.field_12 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_13.eq(self.field_13 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_14.eq(self.field_14 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_15.eq(self.field_15 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::video_comment_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::video_comment_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<VideoCommentReaction>(&_connection)
                        .expect("Error."),
                };
            }
            return &self;
        }
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="video_comment_reactions"]
pub struct NewVideoCommentReaction {
    pub video_comment_id:i32,
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
