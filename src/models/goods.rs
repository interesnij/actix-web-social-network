use diesel::prelude::*;
use crate::schema;
use crate::schema::{
    good_lists,
    goods,
    good_comments,
    user_good_list_collections,
    community_good_list_collections,
    good_list_perms,
    good_votes,
    good_comment_votes,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::{establish_connection, JsonReactions, JsonPosition};
use crate::models::{
    User,
    Community,
    UserGoodListPosition,
    CommunityGoodListPosition,
    Sticker,
    Photo,
    Video,
};
use actix_web::web::Json;

/////// GoodList //////

/////////// Тип списка
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

/////// GoodList //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(User)]
pub struct GoodList {
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
#[table_name="good_lists"]
pub struct NewGoodList {
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
#[table_name="good_lists"]
pub struct EditGoodList {
    pub name:            String,
    pub description:     Option<String>,
    pub can_see_el:      String,
    pub can_see_comment: String,
    pub create_el:       String,
    pub create_comment:  String,
    pub copy_el:         String,
}


impl GoodList {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_good_list(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "lgo".to_string() + &self.get_str_id();
    }
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(26))
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
            .filter(schema::moderateds::types.eq(26))
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
        return "<a data-goodlist='".to_string() + &self.get_str_id() + &"' class='ajax'>".to_string() + &self.name + &"</a>".to_string();
    }
    pub fn is_user_list(&self, user: User) -> bool {
        return self.user_id == user.id;
    }
    pub fn is_community_list(&self, community: Community) -> bool {
        return self.community_id.unwrap() == community.id;
    }
    pub fn get_users_ids(&self) -> Vec<i32> {
        use crate::schema::user_good_list_collections::dsl::user_good_list_collections;

        let _connection = establish_connection();
        let ids = user_good_list_collections
            .filter(schema::user_good_list_collections::good_list_id.eq(self.id))
            .load::<UserGoodListCollection>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in ids.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_communities_ids(&self) -> Vec<i32> {
        use crate::schema::community_good_list_collections::dsl::community_good_list_collections;

        let _connection = establish_connection();
        let ids = community_good_list_collections
            .filter(schema::community_good_list_collections::good_list_id.eq(self.id))
            .load::<CommunityGoodListCollection>(&_connection)
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
    pub fn get_items(&self) -> Vec<Good> {
        use crate::schema::goods::dsl::goods;

        let _connection = establish_connection();
        return goods
            .filter(schema::goods::good_list_id.eq(self.id))
            .filter(schema::goods::types.eq("a"))
            .order(schema::goods::created.desc())
            .load::<Good>(&_connection)
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
        use crate::schema::good_list_perms::dsl::good_list_perms;

        let _connection = establish_connection();
        let items = good_list_perms
            .filter(schema::good_list_perms::good_list_id.eq(self.id))
            .filter(schema::good_list_perms::can_see_item.eq("b"))
            .load::<GoodListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::good_list_perms::dsl::good_list_perms;

        let _connection = establish_connection();
        let items = good_list_perms
            .filter(schema::good_list_perms::good_list_id.eq(self.id))
            .filter(schema::good_list_perms::can_see_item.eq("a"))
            .load::<GoodListPerm>(&_connection)
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
        use crate::schema::good_list_perms::dsl::good_list_perms;

        let _connection = establish_connection();
        let items = good_list_perms
            .filter(schema::good_list_perms::good_list_id.eq(self.id))
            .filter(schema::good_list_perms::can_see_comment.eq("b"))
            .load::<GoodListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_comment_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::good_list_perms::dsl::good_list_perms;

        let _connection = establish_connection();
        let items = good_list_perms
            .filter(schema::good_list_perms::good_list_id.eq(self.id))
            .filter(schema::good_list_perms::can_see_comment.eq("a"))
            .load::<GoodListPerm>(&_connection)
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
        use crate::schema::good_list_perms::dsl::good_list_perms;

        let _connection = establish_connection();
        let items = good_list_perms
            .filter(schema::good_list_perms::good_list_id.eq(self.id))
            .filter(schema::good_list_perms::create_item.eq("b"))
            .load::<GoodListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_create_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::good_list_perms::dsl::good_list_perms;

        let _connection = establish_connection();
        let items = good_list_perms
            .filter(schema::good_list_perms::good_list_id.eq(self.id))
            .filter(schema::good_list_perms::create_item.eq("a"))
            .load::<GoodListPerm>(&_connection)
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
        use crate::schema::good_list_perms::dsl::good_list_perms;

        let _connection = establish_connection();
        let items = good_list_perms
            .filter(schema::good_list_perms::good_list_id.eq(self.id))
            .filter(schema::good_list_perms::create_comment.eq("b"))
            .load::<GoodListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_create_comment_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::good_list_perms::dsl::good_list_perms;

        let _connection = establish_connection();
        let items = good_list_perms
            .filter(schema::good_list_perms::good_list_id.eq(self.id))
            .filter(schema::good_list_perms::create_comment.eq("a"))
            .load::<GoodListPerm>(&_connection)
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
        use crate::schema::good_list_perms::dsl::good_list_perms;

        let _connection = establish_connection();
        let items = good_list_perms
            .filter(schema::good_list_perms::good_list_id.eq(self.id))
            .filter(schema::good_list_perms::can_copy.eq("b"))
            .load::<GoodListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_copy_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::good_list_perms::dsl::good_list_perms;

        let _connection = establish_connection();
        let items = good_list_perms
            .filter(schema::good_list_perms::good_list_id.eq(self.id))
            .filter(schema::good_list_perms::can_copy.eq("a"))
            .load::<GoodListPerm>(&_connection)
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
            NewCommunityGoodListPosition,
            NewUserGoodListPosition,
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

            let new_good_list = NewGoodList{
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
            let new_list = diesel::insert_into(schema::good_lists::table)
                .values(&new_good_list)
                .get_result::<GoodList>(&_connection)
                .expect("Error.");
            new_id = new_list.id;

            let _new_goods_list_position = NewCommunityGoodListPosition {
                community_id: community.id,
                list_id:      new_id,
                position:     community.get_good_lists_new_position(),
                types:        "a".to_string(),
            };
            let _goods_list_position = diesel::insert_into(schema::community_good_list_positions::table)
                .values(&_new_goods_list_position)
                .get_result::<CommunityGoodListPosition>(&_connection)
                .expect("Error saving good_list_position.");
        }
        else {
            let new_good_list = NewGoodList{
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
            let new_list = diesel::insert_into(schema::good_lists::table)
                .values(&new_good_list)
                .get_result::<GoodList>(&_connection)
                .expect("Error.");
            new_id = new_list.id;

            let _new_goods_list_position = NewUserGoodListPosition {
                user_id:  creator.id,
                list_id:  new_id,
                position: creator.get_good_lists_new_position(),
                types:    "a".to_string(),
            };
            let _goods_list_position = diesel::insert_into(schema::user_good_list_positions::table)
                .values(&_new_goods_list_position)
                .get_result::<UserGoodListPosition>(&_connection)
                .expect("Error saving good_list_position.");
        }

        if can_see_el == "d".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_id,
                        can_see_item: Some("b".to_string()),
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }
        else if can_see_el == "e".to_string() && can_see_el == "j".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_id,
                        can_see_item: Some("a".to_string()),
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_include)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }

        if can_see_comment == "d".to_string() && can_see_comment == "i".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: Some("b".to_string()),
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }
        else if can_see_comment == "e".to_string() && can_see_comment == "j".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: Some("a".to_string()),
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_include)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }

        if create_el == "d".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: Some("b".to_string()),
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }
        else if create_el == "e".to_string() && create_el == "j".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: Some("a".to_string()),
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_include)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }

        if create_comment == "d".to_string() && create_comment == "i".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: Some("b".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }
        else if create_comment == "e".to_string() && create_comment == "j".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: Some("a".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_include)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }

        if copy_el == "d".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: Some("b".to_string()),
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }
        else if copy_el == "e".to_string() && copy_el == "j".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: Some("a".to_string()),
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_include)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }
        return new_id;
    }
    pub fn edit_list(&self, name: String, description: Option<String>,
        can_see_el: String, can_see_comment: String,
        create_el: String, create_comment: String, copy_el: String,
        can_see_el_users: Option<Vec<i32>>, can_see_comment_users: Option<Vec<i32>>,create_el_users: Option<Vec<i32>>,
        create_comment_users: Option<Vec<i32>>,copy_el_users: Option<Vec<i32>>) -> &GoodList {

        use crate::schema::good_list_perms::dsl::good_list_perms;

        let _connection = establish_connection();

            let edit_good_list = EditGoodList{
                name: name,
                description: description,
                can_see_el: can_see_el.clone(),
                can_see_comment: can_see_comment.clone(),
                create_el: create_el.clone(),
                create_comment: create_comment.clone(),
                copy_el: copy_el.clone(),
            };
            diesel::update(self)
                .set(edit_good_list)
                .get_result::<GoodList>(&_connection)
                .expect("Error.");

        if can_see_el == "d".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                diesel::delete (
                  good_list_perms
                    .filter(schema::good_list_perms::good_list_id.eq(self.id))
                    .filter(schema::good_list_perms::can_see_item.is_not_null())
                )
                  .execute(&_connection)
                  .expect("E");
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: self.id,
                        can_see_item: Some("b".to_string()),
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }
        else if can_see_el == "e".to_string() && can_see_el == "j".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: self.id,
                        can_see_item: Some("a".to_string()),
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_include)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }

        if can_see_comment == "d".to_string() && can_see_comment == "i".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: Some("b".to_string()),
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }
        else if can_see_comment == "e".to_string() && can_see_comment == "j".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: Some("a".to_string()),
                        create_item: None,
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_include)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }

        if create_el == "d".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: Some("b".to_string()),
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }
        else if create_el == "e".to_string() && create_el == "j".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: Some("a".to_string()),
                        create_comment: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_include)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }

        if create_comment == "d".to_string() && create_comment == "i".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: Some("b".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }
        else if create_comment == "e".to_string() && create_comment == "j".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: Some("a".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_include)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }

        if copy_el == "d".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: Some("b".to_string()),
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }
        else if copy_el == "e".to_string() && copy_el == "j".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: self.id,
                        can_see_item: None,
                        can_see_comment: None,
                        create_item: None,
                        create_comment: None,
                        can_copy: Some("a".to_string()),
                    };
                    diesel::insert_into(schema::good_list_perms::table)
                        .values(&_new_include)
                        .get_result::<GoodListPerm>(&_connection)
                        .expect("Error saving good_list_position.");
                }
            }
        }
        return self;
    }
    pub fn get_order(&self) -> UserGoodListPosition {
        use crate::schema::user_good_list_positions::dsl::user_good_list_positions;

        let _connection = establish_connection();
        return user_good_list_positions
            .filter(schema::user_good_list_positions::list_id.eq(self.id))
            .filter(schema::user_good_list_positions::types.eq("a"))
            .load::<UserGoodListPosition>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn add_in_community_collections(&self, community: Community) -> bool {
        use crate::models::NewCommunityGoodListPosition;

        if !self.get_communities_ids().iter().any(|&i| i==community.id) && self.community_id.is_some() && self.community_id.unwrap() == community.id {
            return false;
        }
        let _connection = establish_connection();
        let new_item = NewCommunityGoodListCollection {
            community_id: community.id,
            good_list_id: self.id,
        };
        diesel::insert_into(schema::community_good_list_collections::table)
            .values(&new_item)
            .get_result::<CommunityGoodListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewCommunityGoodListPosition {
            community_id: community.id,
            list_id:      self.id,
            position:     community.get_good_lists_new_position(),
            types:        "a".to_string(),
        };
        diesel::insert_into(schema::community_good_list_positions::table)
            .values(&new_pos)
            .get_result::<CommunityGoodListPosition>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn remove_in_community_collections(&self, community: Community) -> bool {
        use crate::schema::community_good_list_collections::dsl::community_good_list_collections;
        use crate::schema::community_good_list_positions::dsl::community_good_list_positions;

        if self.get_communities_ids().iter().any(|&i| i==community.id) {
            return false;
        }
        let _connection = establish_connection();
        diesel::delete(community_good_list_collections
            .filter(schema::community_good_list_collections::community_id.eq(community.id))
            .filter(schema::community_good_list_collections::good_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(community_good_list_positions
            .filter(schema::community_good_list_positions::community_id.eq(community.id))
            .filter(schema::community_good_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return true;
    }

    pub fn add_in_user_collections(&self, user: User) -> bool {
        use crate::models::NewUserGoodListPosition;

        if !self.get_users_ids().iter().any(|&i| i==user.id) && self.user_id == user.id {
            return false;
        }
        let _connection = establish_connection();
        let new_item = NewUserGoodListCollection {
            user_id: user.id,
            good_list_id: self.id,
        };
        diesel::insert_into(schema::user_good_list_collections::table)
            .values(&new_item)
            .get_result::<UserGoodListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewUserGoodListPosition {
            user_id:  user.id,
            list_id:  self.id,
            position: user.get_good_lists_new_position(),
            types:    "a".to_string(),
        };
        diesel::insert_into(schema::user_good_list_positions::table)
            .values(&new_pos)
            .get_result::<UserGoodListPosition>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn remove_in_user_collections(&self, user: User) -> bool {
        use crate::schema::user_good_list_collections::dsl::user_good_list_collections;
        use crate::schema::user_good_list_positions::dsl::user_good_list_positions;

        if self.get_users_ids().iter().any(|&i| i==user.id) {
            return false;
        }
        let _connection = establish_connection();
        diesel::delete(user_good_list_collections
            .filter(schema::user_good_list_collections::user_id.eq(user.id))
            .filter(schema::user_good_list_collections::good_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(user_good_list_positions
            .filter(schema::user_good_list_positions::user_id.eq(user.id))
            .filter(schema::user_good_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return true;
    }

    pub fn copy_item(pk: i32, user_or_communities: Vec<String>) -> bool {
        use crate::schema::good_lists::dsl::good_lists;
        use crate::schema::users::dsl::users;
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        let lists = good_lists
            .filter(schema::good_lists::id.eq(pk))
            .filter(schema::good_lists::types.lt(10))
            .load::<GoodList>(&_connection)
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
    pub fn get_goods_ids(&self) -> Vec<i32> {
        use crate::schema::goods::dsl::goods;

        let _connection = establish_connection();
        let fix_list = goods
            .filter(schema::goods::good_list_id.eq(self.id))
            .filter(schema::goods::types.lt("b"))
            .load::<Good>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in fix_list.iter() {
            stack.push(_item.id);
        };
        return stack;
    }
    pub fn get_user_lists(user_pk: i32) -> Vec<GoodList> {
        use crate::schema::user_good_list_collections::dsl::user_good_list_collections;
        use crate::schema::user_good_list_positions::dsl::user_good_list_positions;
        use crate::schema::good_lists::dsl::good_lists;

        let _connection = establish_connection();
        let position_lists = user_good_list_positions
            .filter(schema::user_good_list_positions::user_id.eq(user_pk))
            .filter(schema::user_good_list_positions::types.eq("a"))
            .load::<UserGoodListPosition>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            let mut stack = Vec::new();
            for _item in position_lists.iter() {
                stack.push(_item.list_id);
            };
            return good_lists
                .filter(schema::good_lists::id.eq_any(stack))
                .filter(schema::good_lists::types.lt(10))
                .load::<GoodList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let user_lists = good_lists
            .filter(schema::good_lists::user_id.eq(user_pk))
            .filter(schema::good_lists::types.lt(10))
            .load::<GoodList>(&_connection)
            .expect("E.");
        for _item in user_lists.iter() {
            stack.push(_item.id);
        };
        let user_collections = user_good_list_collections
            .filter(schema::user_good_list_collections::user_id.eq(user_pk))
            .load::<UserGoodListCollection>(&_connection)
            .expect("E.");
        for _item in user_collections.iter() {
            stack.push(_item.good_list_id);
        };
        return good_lists
            .filter(schema::good_lists::id.eq_any(stack))
            .filter(schema::good_lists::types.lt(10))
            .load::<GoodList>(&_connection)
            .expect("E.");
    }
    pub fn get_community_lists(community_pk: i32) -> Vec<GoodList> {
        use crate::schema::community_good_list_collections::dsl::community_good_list_collections;
        use crate::schema::community_good_list_positions::dsl::community_good_list_positions;
        use crate::schema::good_lists::dsl::good_lists;

        let _connection = establish_connection();
        let position_lists = community_good_list_positions
            .filter(schema::community_good_list_positions::community_id.eq(community_pk))
            .filter(schema::community_good_list_positions::types.eq("a"))
            .load::<CommunityGoodListPosition>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            let mut stack = Vec::new();
            for _item in position_lists.iter() {
                stack.push(_item.list_id);
            };
            return good_lists
                .filter(schema::good_lists::id.eq_any(stack))
                .filter(schema::good_lists::types.lt(10))
                .load::<GoodList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let community_lists = good_lists
            .filter(schema::good_lists::community_id.eq(community_pk))
            .filter(schema::good_lists::types.lt(10))
            .load::<GoodList>(&_connection)
            .expect("E.");
        for _item in community_lists.iter() {
            stack.push(_item.id);
        };
        let community_collections = community_good_list_collections
            .filter(schema::community_good_list_collections::community_id.eq(community_pk))
            .load::<CommunityGoodListCollection>(&_connection)
            .expect("E.");
        for _item in community_collections.iter() {
            stack.push(_item.good_list_id);
        };
        return good_lists
            .filter(schema::good_lists::id.eq_any(stack))
            .filter(schema::good_lists::types.lt(10))
            .load::<GoodList>(&_connection)
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
            .set(schema::good_lists::types.eq(close_case))
            .get_result::<GoodList>(&_connection)
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
            .set(schema::good_lists::types.eq(close_case))
            .get_result::<GoodList>(&_connection)
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
            .set(schema::good_lists::types.eq(close_case))
            .get_result::<GoodList>(&_connection)
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
            .set(schema::good_lists::types.eq(close_case))
            .get_result::<GoodList>(&_connection)
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
            .set(schema::good_lists::types.eq(close_case))
            .get_result::<GoodList>(&_connection)
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
            .set(schema::good_lists::types.eq(close_case))
            .get_result::<GoodList>(&_connection)
            .expect("E");
       return true;
    }

}
/////// Good //////

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
// 'm' Удаленый закрепленный
// 'n' Закрытый закрепленный
// 'r' Репост

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(User)]
#[belongs_to(GoodList)]
pub struct Good {
    pub id:              i32,
    pub title:           String,
    pub community_id:    Option<i32>,
    pub category_id:     Option<i32>,
    pub user_id:         i32,
    pub good_list_id:    i32,
    pub price:           Option<i32>,
    pub types:           String,
    pub description:     Option<String>,
    pub image:           Option<String>,
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
#[table_name="goods"]
pub struct NewGood {
    pub title:           String,
    pub community_id:    Option<i32>,
    pub category_id:     Option<i32>,
    pub user_id:         i32,
    pub good_list_id:    i32,
    pub price:           Option<i32>,
    pub types:           String,
    pub description:     Option<String>,
    pub image:           Option<String>,
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
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="goods"]
pub struct EditGood {
    pub title:           String,
    pub price:           Option<i32>,
    pub description:     Option<String>,
    pub image:           Option<String>,
    pub comment_enabled: bool,
    pub votes_on:        bool,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="goods"]
pub struct EditGoodPosition {
    pub position: i16,
}

#[derive(Serialize, AsChangeset)]
#[table_name="goods"]
pub struct GoodReactionsUpdate {
    pub liked:    i32,
    pub disliked: i32,
}

impl Good {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_good(&self) -> bool {
        return true;
    }
    pub fn is_user_can_edit_delete_item(&self, user: User) -> bool {
        if self.community_id.is_some() {
            return user.is_staff_of_community(self.community_id.unwrap());
        }
        else {
            return self.user_id == user.id;
        }
    }
    pub fn get_code(&self) -> String {
        return "goo".to_string() + &self.get_str_id();
    }
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return "<img class='image_fit opacity-100' src='".to_string() +  &self.image.as_deref().unwrap().to_string() + &"' alt='img' />".to_string();
        }
        else {
            return "<svg class='image_fit svg_default opacity-100' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none' /><path d='M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z' /></svg>".to_string();
        }
    }
    pub fn get_price(&self) -> String {
        if self.price.is_some() {
            return self.price.unwrap().to_string() + &" ₽".to_string();
        }
        else {
            return "Цена не указана".to_string();
        }
    }
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(57))
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
            .filter(schema::moderateds::types.eq(57))
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
            return "товар сообщества <a href='".to_owned() + &community.get_link() + &"' target='_blank'>" + &community.name + &"</a>"
        }
        else {
            let creator = self.get_creator();
            return "<a href='".to_owned() + &creator.get_link() + &"' target='_blank'>" + &creator.get_full_name() + &"</a>" + &": товар"
        }
    }
}

/////// GoodComment //////

    // 'a' Опубликованный
    // 'b' Изменённый
    // 'c' Удаленый
    // 'd' Изменённый Удаленый
    // 'e' Закрытый модератором
    // 'f' Закрытый Удаленый

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Good)]
#[belongs_to(User)]
#[belongs_to(Sticker)]
pub struct GoodComment {
    pub id:         i32,
    pub good_id:    i32,
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
#[table_name="good_comments"]
pub struct NewGoodComment {
    pub good_id:    i32,
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
#[table_name="good_comments"]
pub struct EditGoodComment {
    pub content:    Option<String>,
    pub attach:     Option<String>,
}

impl GoodComment {
    pub fn get_attach(&self, user_id: i32) -> String {
        if self.attach.is_some() {
            use crate::utils::comment_elements;
            return comment_elements(self.attach.as_ref().unwrap().to_string(), user_id);
        }
        else {
            return "".to_string();
        }
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
}

#[derive(Serialize, AsChangeset)]
#[table_name="good_comments"]
pub struct GoodCommentReactionsUpdate {
    pub liked:    i32,
    pub disliked: i32,
}

/////// UserGoodListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(GoodList)]
pub struct UserGoodListCollection {
    pub id:       i32,
    pub user_id:  i32,
    pub good_list_id:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_good_list_collections"]
pub struct NewUserGoodListCollection {
    pub user_id:  i32,
    pub good_list_id:  i32,
}

/////// CommunityGoodListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(GoodList)]
pub struct CommunityGoodListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub good_list_id:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_good_list_collections"]
pub struct NewCommunityGoodListCollection {
    pub community_id:  i32,
    pub good_list_id:       i32,
}

/////// GoodListPerm //////
    // 'a' Активно
    // 'b' Не активно
    // 'c' Нет значения

#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(GoodList)]
pub struct GoodListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub good_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub can_see_comment: Option<String>,
    pub create_item:     Option<String>,
    pub create_comment:  Option<String>,
    pub can_copy:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="good_list_perms"]
pub struct NewGoodListPerm {
    pub user_id:         i32,
    pub good_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub can_see_comment: Option<String>,
    pub create_item:     Option<String>,
    pub create_comment:  Option<String>,
    pub can_copy:        Option<String>,
}

/////// GoodVote//////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Good)]
pub struct GoodVote {
    pub id:              i32,
    pub vote:            i16,
    pub user_id:         i32,
    pub good_id:         i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="good_votes"]
pub struct NewGoodVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub good_id:         i32,
}


/////// GoodCommentVote //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(GoodComment)]
pub struct GoodCommentVote {
    pub id:              i32,
    pub vote:            i16,
    pub user_id:         i32,
    pub good_comment_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="good_comment_votes"]
pub struct NewGoodCommentVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub good_comment_id: i32,
}
