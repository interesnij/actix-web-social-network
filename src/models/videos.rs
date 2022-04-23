use crate::schema;
use diesel::prelude::*;
use crate::schema::{
    video_lists,
    videos,
    video_comments,
    user_video_list_collections,
    community_video_list_collections,
    video_list_perms,
    video_votes,
    video_comment_votes,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
    UserVideoListPosition,
    CommunityVideoListPosition,
    Sticker,
};


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
    // 'd' Друзья, кроме
    // 'e' Некоторые друзья
    // 'f' Подписчики
    // 'g' Только я / владелец сообщества
    // 'h' Администраторы
    // 'i' Подписчики, кроме
    // 'j' Некоторые подписчики

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
}
#[derive(Deserialize, Insertable)]
#[table_name="video_lists"]
pub struct NewVideoList {
    pub name:            String,
    pub community_id:    Option<i32>,
    pub user_id:         i32,
    pub types:           i16,
    pub description:     Option<String>,
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
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="video_lists"]
pub struct EditVideoList {
    pub name:            String,
    pub description:     Option<String>,
    pub can_see_el:      String,
    pub can_see_comment: String,
    pub create_el:       String,
    pub create_comment:  String,
    pub copy_el:         String,
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
                "f" => community.get_members_ids().iter().any(|&i| i==user_id),
                "h" => community.get_administrators_ids().iter().any(|&i| i==user_id),
                "g" => community.user_id == user_id,
                "i" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "j" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match char.as_str() {
                "b" => creator.get_friends_ids().iter().any(|&i| i==user_id),
                "c" => creator.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
                "g" => creator.id == user_id,
                "d" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "e" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
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
                "f" => community.get_members_ids().iter().any(|&i| i==user_id),
                "h" => community.get_administrators_ids().iter().any(|&i| i==user_id),
                "g" => community.user_id == user_id,
                "i" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "j" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match char.as_str() {
                "b" => creator.get_friends_ids().iter().any(|&i| i==user_id),
                "c" => creator.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
                "g" => creator.id == user_id,
                "d" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "e" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
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
                "f" => community.get_members_ids().iter().any(|&i| i==user_id),
                "h" => community.get_administrators_ids().iter().any(|&i| i==user_id),
                "g" => community.user_id == user_id,
                "i" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "j" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match char.as_str() {
                "b" => creator.get_friends_ids().iter().any(|&i| i==user_id),
                "c" => creator.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
                "g" => creator.id == user_id,
                "d" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "e" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
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
                "f" => community.get_members_ids().iter().any(|&i| i==user_id),
                "h" => community.get_administrators_ids().iter().any(|&i| i==user_id),
                "g" => community.user_id == user_id,
                "i" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "j" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match char.as_str() {
                "b" => creator.get_friends_ids().iter().any(|&i| i==user_id),
                "c" => creator.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
                "g" => creator.id == user_id,
                "d" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "e" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
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
                "f" => community.get_members_ids().iter().any(|&i| i==user_id),
                "h" => community.get_administrators_ids().iter().any(|&i| i==user_id),
                "g" => community.user_id == user_id,
                "i" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "j" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match char.as_str() {
                "b" => creator.get_friends_ids().iter().any(|&i| i==user_id),
                "c" => creator.get_friend_and_friend_of_friend_ids().iter().any(|&i| i==user_id),
                "g" => creator.id == user_id,
                "d" => !self.get_can_see_el_exclude_users_ids().iter().any(|&i| i==user_id),
                "e" => self.get_can_see_el_include_users_ids().iter().any(|&i| i==user_id),
                _ => false,
            };
        }
    }
    pub fn is_anon_user_can_see_el(&self) -> bool {
        return self.can_see_el == "a".to_string()
    }
    pub fn is_anon_user_can_see_comment(&self) -> bool {
        return self.can_see_comment == "a".to_string()
    }
    pub fn is_anon_user_can_create_item(&self) -> bool {
        return self.create_el == "a".to_string()
    }
    pub fn is_anon_user_can_create_comment(&self) -> bool {
        return self.create_comment == "a".to_string()
    }
    pub fn is_anon_user_can_copy_el(&self) -> bool {
        return self.copy_el == "a".to_string()
    }
    pub fn create_list(creator: User, name: String, description: Option<String>,
        community_id: Option<i32>, can_see_el: String, can_see_comment: String,
        create_el: String, create_comment: String, copy_el: String,
        can_see_el_users: Option<Vec<i32>>, can_see_comment_users: Option<Vec<i32>>,create_el_users: Option<Vec<i32>>,
        create_comment_users: Option<Vec<i32>>,copy_el_users: Option<Vec<i32>>) -> i32 {
        use crate::models::{
            NewCommunityVideoListPosition,
            NewUserVideoListPosition,
        };

        let _connection = establish_connection();
        let mut new_id = 1;
        if community_id.is_some() {
            use crate::schema::communitys::dsl::communitys;

            let community = communitys
                .filter(schema::communitys::id.eq(community_id.unwrap()))
                .load::<Community>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            let new_video_list = NewVideoList{
                name: name,
                community_id: Some(community.id),
                user_id: creator.id,
                types: 2,
                description: description,
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
            };
            let new_list = diesel::insert_into(schema::video_lists::table)
                .values(&new_video_list)
                .get_result::<VideoList>(&_connection)
                .expect("Error.");
            new_id = new_list.id;

            let _new_videos_list_position = NewCommunityVideoListPosition {
                community_id: community.id,
                list_id:      new_id,
                position:     community.get_video_lists_new_position(),
                types:        "a".to_string(),
            };
            let _videos_list_position = diesel::insert_into(schema::community_video_list_positions::table)
                .values(&_new_videos_list_position)
                .get_result::<CommunityVideoListPosition>(&_connection)
                .expect("Error saving video_list_position.");
        }
        else {
            let new_video_list = NewVideoList{
                name: name,
                community_id: None,
                user_id: creator.id,
                types: 2,
                description: description,
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
            };
            let new_list = diesel::insert_into(schema::video_lists::table)
                .values(&new_video_list)
                .get_result::<VideoList>(&_connection)
                .expect("Error.");
            new_id = new_list.id;

            let _new_videos_list_position = NewUserVideoListPosition {
                user_id:  creator.id,
                list_id:  new_id,
                position: creator.get_video_lists_new_position(),
                types:    "a".to_string(),
            };
            let _videos_list_position = diesel::insert_into(schema::user_video_list_positions::table)
                .values(&_new_videos_list_position)
                .get_result::<UserVideoListPosition>(&_connection)
                .expect("Error saving video_list_position.");
        }

        if can_see_el == "d".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_id,
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
        else if can_see_el == "e".to_string() && can_see_el == "j".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_id,
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

        if can_see_comment == "d".to_string() && can_see_comment == "i".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_id,
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
        else if can_see_comment == "e".to_string() && can_see_comment == "j".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_id,
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

        if create_el == "d".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_id,
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
        else if create_el == "e".to_string() && create_el == "j".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_id,
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

        if create_comment == "d".to_string() && create_comment == "i".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_id,
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
        else if create_comment == "e".to_string() && create_comment == "j".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_id,
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

        if copy_el == "d".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_id,
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
        else if copy_el == "e".to_string() && copy_el == "j".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewVideoListPerm {
                        user_id:      user_id,
                        video_list_id: new_id,
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
        return new_id;
    }
    pub fn edit_list(&self, name: String, description: Option<String>,
        can_see_el: String, can_see_comment: String,
        create_el: String, create_comment: String, copy_el: String,
        can_see_el_users: Option<Vec<i32>>, can_see_comment_users: Option<Vec<i32>>,create_el_users: Option<Vec<i32>>,
        create_comment_users: Option<Vec<i32>>,copy_el_users: Option<Vec<i32>>) -> &VideoList {

        use crate::schema::video_list_perms::dsl::video_list_perms;

        let _connection = establish_connection();

            let edit_video_list = EditVideoList{
                name: name,
                description: description,
                can_see_el: can_see_el.clone(),
                can_see_comment: can_see_comment.clone(),
                create_el: create_el.clone(),
                create_comment: create_comment.clone(),
                copy_el: copy_el.clone(),
            };
            diesel::update(self)
                .set(edit_video_list)
                .get_result::<VideoList>(&_connection)
                .expect("Error.");

        if can_see_el == "d".to_string() && can_see_el == "i".to_string() {
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
        else if can_see_el == "e".to_string() && can_see_el == "j".to_string() {
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

        if can_see_comment == "d".to_string() && can_see_comment == "i".to_string() {
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
        else if can_see_comment == "e".to_string() && can_see_comment == "j".to_string() {
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

        if create_el == "d".to_string() && create_el == "i".to_string() {
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
        else if create_el == "e".to_string() && create_el == "j".to_string() {
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

        if create_comment == "d".to_string() && create_comment == "i".to_string() {
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
        else if create_comment == "e".to_string() && create_comment == "j".to_string() {
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

        if copy_el == "d".to_string() && copy_el == "i".to_string() {
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
        else if copy_el == "e".to_string() && copy_el == "j".to_string() {
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
    pub votes_on:        bool,
    pub created:         chrono::NaiveDateTime,

    pub comment:         i32,
    pub view:            i32,
    pub liked:           i32,
    pub disliked:        i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
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
    pub votes_on:        bool,
    pub created:         chrono::NaiveDateTime,

    pub comment:         i32,
    pub view:            i32,
    pub liked:           i32,
    pub disliked:        i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
}

impl Video {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_video(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "vid".to_string() + &self.get_str_id();
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
    pub fn get_description(&self) -> String {
        if self.community_id.is_some() {
            let community = self.get_community();
            return "видеозапись сообщества <a href='".to_owned() + &community.get_link() + &"' target='_blank'>" + &community.name + &"</a>"
        }
        else {
            let creator = self.get_creator();
            return "<a href='".to_owned() + &creator.get_link() + &"' target='_blank'>" + &creator.get_full_name() + &"</a>" + &": видеозапись"
        }
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
    pub liked:      i32,
    pub disliked:   i32,
    pub repost:     i32,
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
    pub liked:      i32,
    pub disliked:   i32,
    pub repost:     i32,
}

impl VideoComment {
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
            return "запись сообщества <a href='".to_owned() + &community.get_link() + &"' target='_blank'>" + &community.name + &"</a>"
        }
        else {
            let creator = self.get_creator();
            return "<a href='".to_owned() + &creator.get_link() + &"' target='_blank'>" + &creator.get_full_name() + &"</a>" + &": запись"
        }
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
    pub id:              i32,
    pub vote:            i16,
    pub user_id:         i32,
    pub video_id:         i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="video_votes"]
pub struct NewVideoVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub video_id:         i32,
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
}
#[derive(Deserialize, Insertable)]
#[table_name="video_comment_votes"]
pub struct NewVideoCommentVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub video_comment_id: i32,
}
