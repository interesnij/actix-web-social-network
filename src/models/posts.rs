use crate::schema;
use crate::schema::{
    post_categories,
    post_lists,
    posts,
    post_comments,
    user_post_list_collections,
    community_post_list_collections,
    post_list_perms,
    post_votes,
    post_comment_votes,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use diesel::prelude::*;
use crate::models::{
    User,
    Community,
    Sticker,
    UserPostListPosition,
    CommunityPostListPosition,
};
use actix_web::web::Json;


/////// CommunityCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct PostCategorie {
    pub id:       i32,
    pub name:     String,
    //pub avatar:   Option<String>,
    pub position: i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_categories"]
pub struct NewPostCategorie {
    pub name:     String,
    //pub avatar:   Option<String>,
    pub position: i16,
}

/////// PostList //////
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
        // 'o' Никто

/////// PostList //////
#[derive(Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(Community)]
#[belongs_to(User)]
pub struct PostList {
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
#[table_name="post_lists"]
pub struct NewPostList {
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
#[table_name="post_lists"]
pub struct EditPostList {
    pub name:            String,
    pub description:     Option<String>,
    pub can_see_el:      String,
    pub can_see_comment: String,
    pub create_el:       String,
    pub create_comment:  String,
    pub copy_el:         String,
}
impl PostList {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_post_list(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "lpo".to_string() + &self.get_str_id();
    }
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(20))
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
            .filter(schema::moderateds::types.eq(20))
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
        return "<a data-postlist='".to_string() + &self.get_str_id() + &"' class='ajax'>".to_string() + &self.name + &"</a>".to_string();
    }
    pub fn is_user_list(&self, user: User) -> bool {
        return self.user_id == user.id;
    }
    pub fn is_community_list(&self, community: Community) -> bool {
        return self.community_id.unwrap() == community.id;
    }
    pub fn get_users_ids(&self) -> Vec<i32> {
        use crate::schema::user_post_list_collections::dsl::user_post_list_collections;

        let _connection = establish_connection();
        let ids = user_post_list_collections
            .filter(schema::user_post_list_collections::post_list_id.eq(self.id))
            .load::<UserPostListCollection>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in ids.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_communities_ids(&self) -> Vec<i32> {
        use crate::schema::community_post_list_collections::dsl::community_post_list_collections;

        let _connection = establish_connection();
        let ids = community_post_list_collections
            .filter(schema::community_post_list_collections::post_list_id.eq(self.id))
            .load::<CommunityPostListCollection>(&_connection)
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
    pub fn get_items(&self) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::post_list_id.eq(self.id))
            .filter(schema::posts::types.eq("a"))
            .order(schema::posts::created.desc())
            .load::<Post>(&_connection)
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
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::can_see_item.eq("b"))
            .load::<PostListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::can_see_item.eq("a"))
            .load::<PostListPerm>(&_connection)
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
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::can_see_comment.eq("b"))
            .load::<PostListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_comment_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::can_see_comment.eq("a"))
            .load::<PostListPerm>(&_connection)
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
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::create_item.eq("b"))
            .load::<PostListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_create_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::create_item.eq("a"))
            .load::<PostListPerm>(&_connection)
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
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::create_comment.eq("b"))
            .load::<PostListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_create_comment_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::create_comment.eq("a"))
            .load::<PostListPerm>(&_connection)
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
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::can_copy.eq("b"))
            .load::<PostListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_copy_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::can_copy.eq("a"))
            .load::<PostListPerm>(&_connection)
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
            NewCommunityPostListPosition,
            NewUserPostListPosition,
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

            let new_post_list = NewPostList{
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
            let new_list = diesel::insert_into(schema::post_lists::table)
                .values(&new_post_list)
                .get_result::<PostList>(&_connection)
                .expect("Error.");
            new_id = new_list.id;

            let _new_posts_list_position = NewCommunityPostListPosition {
                community_id: community.id,
                list_id:      new_id,
                position:     community.get_post_lists_new_position(),
                types:        "a".to_string(),
            };
            let _posts_list_position = diesel::insert_into(schema::community_post_list_positions::table)
                .values(&_new_posts_list_position)
                .get_result::<CommunityPostListPosition>(&_connection)
                .expect("Error saving post_list_position.");
        }
        else {
            let new_post_list = NewPostList{
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
            let new_list = diesel::insert_into(schema::post_lists::table)
                .values(&new_post_list)
                .get_result::<PostList>(&_connection)
                .expect("Error.");
            new_id = new_list.id;

            let _new_posts_list_position = NewUserPostListPosition {
                user_id:  creator.id,
                list_id:  new_id,
                position: creator.get_post_lists_new_position(),
                types:    "a".to_string(),
            };
            let _posts_list_position = diesel::insert_into(schema::user_post_list_positions::table)
                .values(&_new_posts_list_position)
                .get_result::<UserPostListPosition>(&_connection)
                .expect("Error saving post_list_position.");
        }

        if can_see_el == "d".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: new_id,
                        can_see_item: Some("b".to_string()),
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        else if can_see_el == "e".to_string() && can_see_el == "j".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: new_id,
                        can_see_item: Some("a".to_string()),
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }

        if can_see_comment == "d".to_string() && can_see_comment == "i".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: Some("b".to_string()),
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        else if can_see_comment == "e".to_string() && can_see_comment == "j".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: Some("a".to_string()),
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }

        if create_el == "d".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: Some("b".to_string()),
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        else if create_el == "e".to_string() && create_el == "j".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: Some("a".to_string()),
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }

        if create_comment == "d".to_string() && create_comment == "i".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: Some("b".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        else if create_comment == "e".to_string() && create_comment == "j".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: Some("a".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }

        if copy_el == "d".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: Some("b".to_string()),
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        else if copy_el == "e".to_string() && copy_el == "j".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: Some("a".to_string()),
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        return new_id;
    }
    pub fn edit_list(&self, name: String, description: Option<String>,
        can_see_el: String, can_see_comment: String,
        create_el: String, create_comment: String, copy_el: String,
        can_see_el_users: Option<Vec<i32>>, can_see_comment_users: Option<Vec<i32>>,create_el_users: Option<Vec<i32>>,
        create_comment_users: Option<Vec<i32>>,copy_el_users: Option<Vec<i32>>) -> &PostList {

        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();

            let edit_post_list = EditPostList{
                name: name,
                description: description,
                can_see_el: can_see_el.clone(),
                can_see_comment: can_see_comment.clone(),
                create_el: create_el.clone(),
                create_comment: create_comment.clone(),
                copy_el: copy_el.clone(),
            };
            diesel::update(self)
                .set(edit_post_list)
                .get_result::<PostList>(&_connection)
                .expect("Error.");

        if can_see_el == "d".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                diesel::delete (
                  post_list_perms
                    .filter(schema::post_list_perms::post_list_id.eq(self.id))
                    .filter(schema::post_list_perms::can_see_item.is_not_null())
                )
                  .execute(&_connection)
                  .expect("E");
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: self.id,
                        can_see_item: Some("b".to_string()),
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        else if can_see_el == "e".to_string() && can_see_el == "j".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: self.id,
                        can_see_item: Some("a".to_string()),
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }

        if can_see_comment == "d".to_string() && can_see_comment == "i".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: Some("b".to_string()),
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        else if can_see_comment == "e".to_string() && can_see_comment == "j".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: Some("a".to_string()),
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }

        if create_el == "d".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: Some("b".to_string()),
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        else if create_el == "e".to_string() && create_el == "j".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: Some("a".to_string()),
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }

        if create_comment == "d".to_string() && create_comment == "i".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: Some("b".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        else if create_comment == "e".to_string() && create_comment == "j".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: Some("a".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }

        if copy_el == "d".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: Some("b".to_string()),
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        else if copy_el == "e".to_string() && copy_el == "j".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      user_id,
                        post_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: Some("a".to_string()),
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        return self;
    }
    pub fn get_order(&self) -> UserPostListPosition {
        use crate::schema::user_post_list_positions::dsl::user_post_list_positions;

        let _connection = establish_connection();
        return user_post_list_positions
            .filter(schema::user_post_list_positions::list_id.eq(self.id))
            .filter(schema::user_post_list_positions::types.eq("a"))
            .load::<UserPostListPosition>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn add_in_community_collections(&self, community: Community) -> bool {
        use crate::models::NewCommunityPostListPosition;

        if !self.get_communities_ids().iter().any(|&i| i==community.id) && self.community_id.is_some() && self.community_id.unwrap() == community.id {
            return false;
        }
        let _connection = establish_connection();
        let new_item = NewCommunityPostListCollection {
            community_id: community.id,
            post_list_id: self.id,
        };
        diesel::insert_into(schema::community_post_list_collections::table)
            .values(&new_item)
            .get_result::<CommunityPostListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewCommunityPostListPosition {
            community_id: community.id,
            list_id:      self.id,
            position:     community.get_post_lists_new_position(),
            types:        "a".to_string(),
        };
        diesel::insert_into(schema::community_post_list_positions::table)
            .values(&new_pos)
            .get_result::<CommunityPostListPosition>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn remove_in_community_collections(&self, community: Community) -> bool {
        use crate::schema::community_post_list_collections::dsl::community_post_list_collections;
        use crate::schema::community_post_list_positions::dsl::community_post_list_positions;

        if self.get_communities_ids().iter().any(|&i| i==community.id) {
            return false;
        }
        let _connection = establish_connection();
        diesel::delete(community_post_list_collections
            .filter(schema::community_post_list_collections::community_id.eq(community.id))
            .filter(schema::community_post_list_collections::post_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(community_post_list_positions
            .filter(schema::community_post_list_positions::community_id.eq(community.id))
            .filter(schema::community_post_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return true;
    }

    pub fn add_in_user_collections(&self, user: User) -> bool {
        use crate::models::NewUserPostListPosition;

        if !self.get_users_ids().iter().any(|&i| i==user.id) && self.user_id == user.id {
            return false;
        }
        let _connection = establish_connection();
        let new_item = NewUserPostListCollection {
            user_id: user.id,
            post_list_id: self.id,
        };
        diesel::insert_into(schema::user_post_list_collections::table)
            .values(&new_item)
            .get_result::<UserPostListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewUserPostListPosition {
            user_id:  user.id,
            list_id:  self.id,
            position: user.get_post_lists_new_position(),
            types:    "a".to_string(),
        };
        diesel::insert_into(schema::user_post_list_positions::table)
            .values(&new_pos)
            .get_result::<UserPostListPosition>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn remove_in_user_collections(&self, user: User) -> bool {
        use crate::schema::user_post_list_collections::dsl::user_post_list_collections;
        use crate::schema::user_post_list_positions::dsl::user_post_list_positions;

        if self.get_users_ids().iter().any(|&i| i==user.id) {
            return false;
        }
        let _connection = establish_connection();
        diesel::delete(user_post_list_collections
            .filter(schema::user_post_list_collections::user_id.eq(user.id))
            .filter(schema::user_post_list_collections::post_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(user_post_list_positions
            .filter(schema::user_post_list_positions::user_id.eq(user.id))
            .filter(schema::user_post_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return true;
    }

    pub fn copy_item(pk: i32, user_or_communities: Vec<String>) -> bool {
        use crate::schema::post_lists::dsl::post_lists;
        use crate::schema::users::dsl::users;
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        let lists = post_lists
            .filter(schema::post_lists::id.eq(pk))
            .filter(schema::post_lists::types.lt(10))
            .load::<PostList>(&_connection)
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
    pub fn get_posts_ids(&self) -> Vec<i32> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let fix_list = posts
            .filter(schema::posts::post_list_id.eq(self.id))
            .filter(schema::posts::types.lt("b"))
            .load::<Post>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in fix_list.iter() {
            stack.push(_item.id);
        };
        return stack;
    }
    pub fn get_user_lists(user_pk: i32) -> Vec<PostList> {
        use crate::schema::user_post_list_collections::dsl::user_post_list_collections;
        use crate::schema::user_post_list_positions::dsl::user_post_list_positions;
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        let position_lists = user_post_list_positions
            .filter(schema::user_post_list_positions::user_id.eq(user_pk))
            .filter(schema::user_post_list_positions::types.eq("a"))
            .load::<UserPostListPosition>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            let mut stack = Vec::new();
            for _item in position_lists.iter() {
                stack.push(_item.list_id);
            };
            return post_lists
                .filter(schema::post_lists::id.eq_any(stack))
                .filter(schema::post_lists::types.lt(10))
                .load::<PostList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let user_lists = post_lists
            .filter(schema::post_lists::user_id.eq(user_pk))
            .filter(schema::post_lists::types.lt(10))
            .load::<PostList>(&_connection)
            .expect("E.");
        for _item in user_lists.iter() {
            stack.push(_item.id);
        };
        let user_collections = user_post_list_collections
            .filter(schema::user_post_list_collections::user_id.eq(user_pk))
            .load::<UserPostListCollection>(&_connection)
            .expect("E.");
        for _item in user_collections.iter() {
            stack.push(_item.post_list_id);
        };
        return post_lists
            .filter(schema::post_lists::id.eq_any(stack))
            .filter(schema::post_lists::types.lt(10))
            .load::<PostList>(&_connection)
            .expect("E.");
    }
    pub fn get_community_lists(community_pk: i32) -> Vec<PostList> {
        use crate::schema::community_post_list_collections::dsl::community_post_list_collections;
        use crate::schema::community_post_list_positions::dsl::community_post_list_positions;
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        let position_lists = community_post_list_positions
            .filter(schema::community_post_list_positions::community_id.eq(community_pk))
            .filter(schema::community_post_list_positions::types.eq("a"))
            .load::<CommunityPostListPosition>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            let mut stack = Vec::new();
            for _item in position_lists.iter() {
                stack.push(_item.list_id);
            };
            return post_lists
                .filter(schema::post_lists::id.eq_any(stack))
                .filter(schema::post_lists::types.lt(10))
                .load::<PostList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let community_lists = post_lists
            .filter(schema::post_lists::community_id.eq(community_pk))
            .filter(schema::post_lists::types.lt(10))
            .load::<PostList>(&_connection)
            .expect("E.");
        for _item in community_lists.iter() {
            stack.push(_item.id);
        };
        let community_collections = community_post_list_collections
            .filter(schema::community_post_list_collections::community_id.eq(community_pk))
            .load::<CommunityPostListCollection>(&_connection)
            .expect("E.");
        for _item in community_collections.iter() {
            stack.push(_item.post_list_id);
        };
        return post_lists
            .filter(schema::post_lists::id.eq_any(stack))
            .filter(schema::post_lists::types.lt(10))
            .load::<PostList>(&_connection)
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
            .set(schema::post_lists::types.eq(close_case))
            .get_result::<PostList>(&_connection)
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
            .set(schema::post_lists::types.eq(close_case))
            .get_result::<PostList>(&_connection)
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
            .set(schema::post_lists::types.eq(close_case))
            .get_result::<PostList>(&_connection)
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
            .set(schema::post_lists::types.eq(close_case))
            .get_result::<PostList>(&_connection)
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
            .set(schema::post_lists::types.eq(close_case))
            .get_result::<PostList>(&_connection)
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
            .set(schema::post_lists::types.eq(close_case))
            .get_result::<PostList>(&_connection)
            .expect("E");
       return true;
    }

}

/////// Post //////

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
    // 'c' Удаленый


#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(PostCategorie)]
#[belongs_to(User)]
#[belongs_to(PostList)]
pub struct Post {
    pub id:                i32,
    pub content:           Option<String>,
    pub community_id:      Option<i32>,
    pub post_categorie_id: Option<i32>,
    pub user_id:           i32,
    pub post_list_id:      i32,
    pub types:             String,
    pub attach:            Option<String>,
    pub comment_enabled:   bool,
    pub votes_on:          bool,
    pub created:           chrono::NaiveDateTime,
    pub comment:           i32,
    pub view:              i32,
    pub liked:             i32,
    pub disliked:          i32,
    pub repost:            i32,
    pub copy:              i32,
    pub position:          i32,
    pub is_signature:      bool,
    pub parent_id:         Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub content:         Option<String>,
    pub community_id:    Option<i32>,
    pub post_categorie_id:     Option<i32>,
    pub user_id:      i32,
    pub post_list_id:         i32,
    pub types:           String,
    pub attach:          Option<String>,
    pub comment_enabled: bool,
    pub votes_on:        bool,
    pub created:         chrono::NaiveDateTime,
    pub comment:         i32,
    pub view:            i32,
    pub liked:           i32,
    pub disliked:        i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i32,
    pub is_signature:    bool,
    pub parent_id:       Option<i32>,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="posts"]
pub struct EditPost {
    pub content:         Option<String>,
    pub post_categorie_id:     Option<i32>,
    pub attach:          Option<String>,
    pub comment_enabled: bool,
    pub votes_on:        bool,
    pub is_signature:    bool,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="posts"]
pub struct EditPostPosition {
    pub position:        i32,
}

#[derive(Serialize, AsChangeset)]
#[table_name="posts"]
pub struct PostReactionsUpdate {
    pub liked:    i32,
    pub disliked: i32,
}

impl Post {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_post(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "pos".to_string() + &self.get_str_id();
    }
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(51))
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
            .filter(schema::moderateds::types.eq(51))
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
    pub fn get_list(&self) -> PostList {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        return post_lists
            .filter(schema::post_lists::id.eq(self.post_list_id))
            .filter(schema::post_lists::types.lt(10))
            .load::<PostList>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_description(&self) -> String {
        if self.community_id.is_some() {
            let community = self.get_community();
            return "запись сообщества <a href='".to_owned() + &community.get_link() + &"' target='_blank'>" + &community.name + &"</a>"
        }
        else {
            let creator = self.get_creator();
            return "<a href='".to_owned() + &creator.get_link() + &"' target='_blank'>" + &creator.get_full_name() + &"</a>" + &": запись"
        }
    }
    pub fn is_user_can_edit_delete_item(&self, user: User) -> bool {
        if self.community_id.is_some() {
            return user.is_staff_of_community(self.community_id.unwrap());
        }
        else {
            return self.user_id == user.id;
        }
    }
    pub fn create_post(creator: User, content: Option<String>, category_id: Option<i32>,
        list: PostList, attach: Option<String>, parent_id: Option<i32>,
        comment_enabled: bool, is_signature: bool, votes_on: bool,
        community_id: Option<i32>, types: Option<String>) -> Post {

        let _connection = establish_connection();
        let mut new_attach = "".to_string();
        if attach.is_some() {
            new_attach = attach.unwrap().replace("'", "").replace("[", "").replace("]", "").replace(" ", "");
        }
        diesel::update(&list)
          .set(schema::post_lists::count.eq(list.count + 1))
          .get_result::<PostList>(&_connection)
          .expect("Error.");

        let mut _types = "".to_string();

        if types.is_some() {
            _types = types.unwrap();
        }
        else {
            _types = "a".to_string();
        }

        if community_id.is_some() {
            use crate::models::CommunityInfo;

            let community = list.get_community();
            let profile = community.get_info_model();
            diesel::update(&profile)
              .set(schema::community_infos::posts.eq(profile.posts + 1))
              .get_result::<CommunityInfo>(&_connection)
              .expect("Error.");

            let new_post_form = NewPost {
              content: content,
              community_id: community_id,
              post_categorie_id: category_id,
              user_id: creator.id,
              post_list_id: list.id,
              types: _types,
              attach: Some(new_attach),
              comment_enabled: comment_enabled,
              votes_on: votes_on,
              created: chrono::Local::now().naive_utc(),
              comment: 0,
              view: 0,
              liked: 0,
              disliked: 0,
              repost: 0,
              copy: 0,
              position: list.count,
              is_signature: is_signature,
              parent_id: parent_id,
            };
            let new_post = diesel::insert_into(schema::posts::table)
                .values(&new_post_form)
                .get_result::<Post>(&_connection)
                .expect("Error.");
            return new_post;
        }
        else {
            use crate::models::UserProfile;

            let profile = creator.get_profile();
            diesel::update(&profile)
              .set(schema::user_profiles::posts.eq(profile.posts + 1))
              .get_result::<UserProfile>(&_connection)
              .expect("Error.");

            let new_post_form = NewPost {
              content: content,
              community_id: None,
              post_categorie_id: category_id,
              user_id: creator.id,
              post_list_id: list.id,
              types: _types,
              attach: Some(new_attach),
              comment_enabled: comment_enabled,
              votes_on: votes_on,
              created: chrono::Local::now().naive_utc(),
              comment: 0,
              view: 0,
              liked: 0,
              disliked: 0,
              repost: 0,
              copy: 0,
              position: list.count,
              is_signature: false,
              parent_id: parent_id,
            };
            let new_post = diesel::insert_into(schema::posts::table)
                .values(&new_post_form)
                .get_result::<Post>(&_connection)
                .expect("Error.");
            return new_post;
        }
    }
    pub fn copy_item(pk: i32, lists: Vec<i32>) -> bool {
        use crate::schema::posts::dsl::posts;
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        let item = posts
            .filter(schema::posts::id.eq(pk))
            .filter(schema::posts::types.eq("a"))
            .load::<Post>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
        let count = 0;
        for list_id in lists.iter() {
            let list = post_lists
                .filter(schema::post_lists::id.eq(list_id))
                .filter(schema::post_lists::types.lt(10))
                .load::<PostList>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            Post::create_post (
                list.get_creator(),
                item.content.clone(),
                item.post_categorie_id.clone(),
                list,
                item.attach.clone(),
                item.parent_id.clone(),
                item.comment_enabled.clone(),
                item.is_signature.clone(),
                item.votes_on.clone(),
                item.community_id.clone(),
                None,
            );
        }
        return true;
    }

    pub fn edit_post(&self, content: Option<String>, post_categorie_id: Option<i32>,
        attach: Option<String>, comment_enabled: bool, votes_on: bool,
        is_signature: bool) -> &Post {

        let _connection = establish_connection();
        let mut new_attach = "".to_string();
        if attach.is_some() {
            new_attach = attach.as_ref().unwrap().replace("'", "").replace("[", "").replace("]", "").replace(" ", "");
        }

        let edit_post = EditPost {
            content: content,
            post_categorie_id: post_categorie_id,
            attach: attach,
            comment_enabled: comment_enabled,
            votes_on: votes_on,
            is_signature: is_signature,
        };
        diesel::update(self)
            .set(edit_post)
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return self;
    }
    pub fn plus_likes(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::liked.eq(self.liked + count))
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_dislikes(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::disliked.eq(self.disliked + count))
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::comment.eq(self.comment + count))
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_likes(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::liked.eq(self.liked - count))
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_dislikes(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::disliked.eq(self.disliked - count))
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::comment.eq(self.comment - count))
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return true;
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
            //return Some(hide_text(self.content.unwrap()));
        } else { return "".to_string(); }
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
    pub fn is_fixed(&self) -> bool {
        return self.types == "b";
    }
    pub fn is_repost(&self) -> bool {
        return self.types == "r";
    }
    pub fn send_like(&self, user: User) -> Json<PostReactions> {
        let list = self.get_list();
        if self.votes_on == false && !list.is_user_can_see_el(user.pk) {
            return Json(PostReactions {
                like_count:    self.liked,
                dislike_count: self.disliked,
            });
        }
        use crate::schema::post_votes::dsl::post_votes;

        let _connection = establish_connection();

        let votes = post_votes
            .filter(schema::post_votes::user_id.eq(user.id))
            .filter(schema::post_votes::post_id.eq(self.id))
            .load::<PostVote>(&_connection)
            .expect("E.");
        if votes.len() > 0 {
            let vote = votes.into_iter().nth(0).unwrap();
            if vote.vote != 1 {
                diesel::update(&vote)
                    .set(schema::post_votes::vote.eq(1))
                    .get_result::<PostVote>(&_connection)
                    .expect("Error.");

                let reactions = PostReactionsUpdate {
                    liked:    self.liked + 1,
                    disliked: self.disliked - 1,
                };
                diesel::update(self)
                    .set(reactions)
                    .get_result::<Post>(&_connection)
                    .expect("Error.");
            }
            else {
                diesel::delete(post_votes
                    .filter(schema::post_votes::user_id.eq(user.id))
                    .filter(schema::post_votes::post_id.eq(self.id))
                    )
                    .execute(&_connection)
                    .expect("E");

                diesel::update(self)
                    .set(schema::posts::liked.eq(self.liked - 1))
                    .get_result::<Post>(&_connection)
                    .expect("Error.");
            }
        }
        else {
            let new_vote = NewPostVote {
                vote: 1,
                user_id: user.id,
                post_id: self.id,
            };
            diesel::insert_into(schema::post_votes::table)
                .values(&new_vote)
                .get_result::<PostVote>(&_connection)
                .expect("Error.");

            diesel::update(self)
                .set(schema::posts::liked.eq(self.liked + 1))
                .get_result::<Post>(&_connection)
                .expect("Error.");
        }
        let reactions = PostReactions {
            like_count:    self.liked,
            dislike_count: self.disliked,
        };
        return Json(reactions);
    }
}

#[derive(Serialize)]
pub struct PostReactions {
    pub like_count:    i32,
    pub dislike_count: i32,
}
/////// PostComment //////

// 'a' Опубликованный
// 'b' Изменённый
// 'c' Удаленый
// 'd' Изменённый Удаленый
// 'e' Закрытый модератором
// 'f' Закрытый Удаленый



#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Post)]
#[belongs_to(User)]
#[belongs_to(Sticker)]
pub struct PostComment {
    pub id:         i32,
    pub post_id:    i32,
    pub user_id: i32,
    pub sticker_id: Option<i32>,
    pub parent_id:  Option<i32>,
    pub content:    Option<String>,
    pub attach:     Option<String>,
    pub types:      String,
    pub created:    chrono::NaiveDateTime,
    pub liked:      i32,
    pub disliked:   i32,
    pub repost:     i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_comments"]
pub struct NewPostComment {
    pub post_id:    i32,
    pub user_id: i32,
    pub sticker_id: Option<i32>,
    pub parent_id:  Option<i32>,
    pub content:    Option<String>,
    pub attach:     Option<String>,
    pub types:      String,
    pub created:    chrono::NaiveDateTime,
    pub liked:      i32,
    pub disliked:   i32,
    pub repost:     i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="post_comments"]
pub struct EditPostComment {
    pub content:    Option<String>,
    pub attach:     Option<String>,
}

#[derive(Serialize, AsChangeset)]
#[table_name="post_comments"]
pub struct PostCommentReactionsUpdate {
    pub liked:    i32,
    pub disliked: i32,
}

/////// UserPostListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(PostList)]
pub struct UserPostListCollection {
    pub id:       i32,
    pub user_id:  i32,
    pub post_list_id:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_post_list_collections"]
pub struct NewUserPostListCollection {
    pub user_id:  i32,
    pub post_list_id:  i32,
}

/////// CommunityPostListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(PostList)]
pub struct CommunityPostListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub post_list_id:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_post_list_collections"]
pub struct NewCommunityPostListCollection {
    pub community_id:  i32,
    pub post_list_id:       i32,
}

/////// PostListPerm //////
// 'a' Активно
// 'b' Не активно
// 'c' Нет значения
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(PostList)]
pub struct PostListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub post_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub can_see_comment: Option<String>,
    pub create_item:     Option<String>,
    pub create_comment:  Option<String>,
    pub can_copy:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_list_perms"]
pub struct NewPostListPerm {
    pub user_id:         i32,
    pub post_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub can_see_comment: Option<String>,
    pub create_item:     Option<String>,
    pub create_comment:  Option<String>,
    pub can_copy:        Option<String>,
}


/////// PostVote//////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Post)]
pub struct PostVote {
    pub id:              i32,
    pub vote:            i16,
    pub user_id:         i32,
    pub post_id:         i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_votes"]
pub struct NewPostVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub post_id:         i32,
}


/////// PostCommentVote //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(PostComment)]
pub struct PostCommentVote {
    pub id:              i32,
    pub vote:            i16,
    pub user_id:         i32,
    pub post_comment_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_comment_votes"]
pub struct NewPostCommentVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub post_comment_id: i32,
}
