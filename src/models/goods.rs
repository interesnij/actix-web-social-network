use diesel::prelude::*;
use crate::schema;
use crate::schema::{
    good_categories,
    good_subcategories,
    good_lists,
    goods,
    good_images,
    good_comments,
    user_good_list_collections,
    community_good_list_collections,
    good_list_perms,
    good_votes,
    good_comment_votes,
    good_list_reposts,
    good_reposts,
    good_reactions,
    good_comment_reactions,
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
    UserGoodListPosition,
    CommunityGoodListPosition,
    Sticker,
    Photo,
    Video,
    Post,
    Message,
    Reaction,
};
use actix_web::web::Json;


/////// GoodCategorie //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct GoodCategorie {
    pub id:       i32,
    pub name:     String,
    pub avatar:   Option<String>,
    pub position: i16,
}

impl GoodCategorie {
    pub fn create_category(name: String, avatar: Option<String>,
        position: i16) -> GoodCategorie {

        let _connection = establish_connection();
        let new_form = NewGoodCategorie {
            name: name,
            avatar: avatar,
            position: position,
        };
        let new_cat = diesel::insert_into(schema::good_categories::table)
            .values(&new_form)
            .get_result::<GoodCategorie>(&_connection)
            .expect("Error.");
        return new_cat;
    }
    pub fn edit_category(&self, name: String, avatar: Option<String>,
        position: i16) -> &GoodCategorie {
        let _connection = establish_connection();
        let new_form = NewGoodCategorie {
            name: name,
            avatar: avatar,
            position: position,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<GoodCategorie>(&_connection)
            .expect("Error.");
        return self;
    }
    pub fn create_subcategory(&self, name: String, avatar: Option<String>,
        position: i16) -> GoodSubcategorie {

        let _connection = establish_connection();
        let new_form = NewGoodSubcategorie {
            name:        name,
            category_id: self.id,
            avatar:      avatar,
            position:    position,
        };
        let new_cat = diesel::insert_into(schema::good_subcategories::table)
            .values(&new_form)
            .get_result::<GoodSubcategorie>(&_connection)
            .expect("Error.");
        return new_cat;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="good_categories"]
pub struct NewGoodCategorie {
    pub name:     String,
    pub avatar:   Option<String>,
    pub position: i16,
}

/////// GoodSubcategorie //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct GoodSubcategorie {
    pub id:          i32,
    pub name:        String,
    pub category_id: i32,
    pub avatar:      Option<String>,
    pub position:    i16,
}

impl GoodSubcategorie {
    pub fn edit_subcategory(&self, name: String, category_id: i32,
        avatar: Option<String>, position: i16) -> &GoodSubcategorie {
        let _connection = establish_connection();
        let new_form = NewGoodSubcategorie {
            name:        name,
            category_id: category_id,
            avatar:      avatar,
            position:    position,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<GoodSubcategorie>(&_connection)
            .expect("Error.");
        return self;
    }
}
#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="good_subcategories"]
pub struct NewGoodSubcategorie {
    pub name:        String,
    pub category_id: i32,
    pub avatar:      Option<String>,
    pub position:    i16,
}

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
// 'e' Друзья, кроме
// 'f' Некоторые друзья
// 'g' Подписчики
// 'o' Только я / владелец сообщества
// 'p' Администраторы
// 'h' Подписчики, кроме
// 'i' Некоторые подписчики

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
#[table_name="good_lists"]
pub struct NewGoodList {
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
#[table_name="good_lists"]
pub struct EditGoodList {
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

    pub fn count_copy(&self) -> String {
        if self.copy == 0 {
            return "".to_string();
        }
        else {
            return ", копировали - ".to_string() + &self.copy.to_string();
        }
    }
    pub fn message_reposts_count(&self) -> String {
        use crate::schema::good_list_reposts::dsl::good_list_reposts;

        let _connection = establish_connection();

        let count = good_list_reposts
            .filter(schema::good_list_reposts::good_list_id.eq(self.id))
            .filter(schema::good_list_reposts::message_id.is_not_null())
            .load::<GoodListRepost>(&_connection)
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
        use crate::schema::good_list_reposts::dsl::good_list_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = good_list_reposts
            .filter(schema::good_list_reposts::good_list_id.eq(self.id))
            .filter(schema::good_list_reposts::post_id.is_not_null())
            .limit(limit)
            .offset(offset)
            .load::<GoodListRepost>(&_connection)
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
        use crate::schema::good_list_reposts::dsl::good_list_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = good_list_reposts
            .filter(schema::good_list_reposts::good_list_id.eq(self.id))
            .filter(schema::good_list_reposts::post_id.is_not_null())
            .limit(6)
            .load::<GoodListRepost>(&_connection)
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
    pub fn get_paginate_items(&self, limit: i64, offset: i64) -> Vec<Good> {
        use crate::schema::goods::dsl::goods;

        let _connection = establish_connection();
        return goods
            .filter(schema::goods::good_list_id.eq(self.id))
            .filter(schema::goods::types.eq("a"))
            .limit(limit)
            .offset(offset)
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
        reactions: Option<String>) -> GoodList {

        use crate::models::{
            NewCommunityGoodListPosition,
            NewUserGoodListPosition,
        };

        let _connection = establish_connection();
        let new_list_form = NewGoodList{
            name:            name,
            community_id:    community_id,
            user_id:         creator.id,
            types:           2,
            description:     description,
            image: image,
            created:         chrono::Local::now().naive_utc(),
            count:           0,
            repost:          0,
            copy:            0,
            position:        0,
            can_see_el:      can_see_el.clone(),
            can_see_comment: can_see_comment.clone(),
            create_el:       create_el.clone(),
            create_comment:  create_comment.clone(),
            copy_el:         copy_el.clone(),
            reactions:       reactions,
        };
        let new_list = diesel::insert_into(schema::good_lists::table)
            .values(&new_list_form)
            .get_result::<GoodList>(&_connection)
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

            let _new_goods_list_position = NewCommunityGoodListPosition {
                community_id: community.id,
                list_id:      new_list.id,
                position:     community.get_good_lists_new_position(),
                types:        "a".to_string(),
            };
            let _goods_list_position = diesel::insert_into(schema::community_good_list_positions::table)
                .values(&_new_goods_list_position)
                .get_result::<CommunityGoodListPosition>(&_connection)
                .expect("Error saving good_list_position.");
        }
        else {
            let _new_goods_list_position = NewUserGoodListPosition {
                user_id:  creator.id,
                list_id:  new_list.id,
                position: creator.get_good_lists_new_position(),
                types:    "a".to_string(),
            };
            let _goods_list_position = diesel::insert_into(schema::user_good_list_positions::table)
                .values(&_new_goods_list_position)
                .get_result::<UserGoodListPosition>(&_connection)
                .expect("Error saving good_list_position.");
        }

        if can_see_el == "e".to_string() && can_see_el == "h".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_list.id,
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
        else if can_see_el == "f".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_list.id,
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

        if can_see_comment == "e".to_string() && can_see_comment == "h".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_list.id,
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
        else if can_see_comment == "f".to_string() && can_see_comment == "i".to_string() {
            if can_see_comment_users.is_some() {
                for user_id in can_see_comment_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_list.id,
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

        if create_el == "e".to_string() && create_el == "h".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_list.id,
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
        else if create_el == "f".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_list.id,
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

        if create_comment == "e".to_string() && create_comment == "h".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_list.id,
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
        else if create_comment == "f".to_string() && create_comment == "i".to_string() {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_list.id,
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

        if copy_el == "e".to_string() && copy_el == "h".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_list.id,
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
        else if copy_el == "f".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewGoodListPerm {
                        user_id:      user_id,
                        good_list_id: new_list.id,
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
        return new_list;
    }
    pub fn edit_list(&self, name: String, description: Option<String>, image: Option<String>,
        can_see_el: String, can_see_comment: String,
        create_el: String, create_comment: String, copy_el: String,
        can_see_el_users: Option<Vec<i32>>, can_see_comment_users: Option<Vec<i32>>,create_el_users: Option<Vec<i32>>,
        create_comment_users: Option<Vec<i32>>,copy_el_users: Option<Vec<i32>>,
        reactions: Option<String>) -> &GoodList {

        use crate::schema::good_list_perms::dsl::good_list_perms;
        let mut descr: Option<String> = Some("".to_string());
        let mut react: Option<String> = Some("".to_string());
        if description.is_some() {
            descr = description;
        }
        if reactions.is_some() {
            react = reactions;
        }

        let _connection = establish_connection();

        let edit_good_list = EditGoodList{
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
            .set(edit_good_list)
            .get_result::<GoodList>(&_connection)
            .expect("Error.");

        if can_see_el == "e".to_string() && can_see_el == "h".to_string() {
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
        else if can_see_el == "f".to_string() && can_see_el == "i".to_string() {
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

        if can_see_comment == "e".to_string() && can_see_comment == "h".to_string() {
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
        else if can_see_comment == "f".to_string() && can_see_comment == "i".to_string() {
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

        if create_el == "e".to_string() && create_el == "h".to_string() {
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
        else if create_el == "f".to_string() && create_el == "i".to_string() {
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

        if create_comment == "e".to_string() && create_comment == "h".to_string() {
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
        else if create_comment == "f".to_string() && create_comment == "i".to_string() {
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

        if copy_el == "e".to_string() && copy_el == "h".to_string() {
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
        else if copy_el == "f".to_string() && copy_el == "i".to_string() {
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
        if self.community_id.is_some() {
            use crate::schema::community_good_list_positions::dsl::community_good_list_positions;

            let list_positions = community_good_list_positions
                .filter(schema::community_good_list_positions::community_id.eq(self.community_id.unwrap()))
                .filter(schema::community_good_list_positions::list_id.eq(self.id))
                .load::<CommunityGoodListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::community_good_list_positions::types.eq("b"))
                  .get_result::<CommunityGoodListPosition>(&_connection)
                  .expect("Error.");
            }
        } else {
            use crate::schema::user_good_list_positions::dsl::user_good_list_positions;

            let list_positions = user_good_list_positions
                .filter(schema::user_good_list_positions::user_id.eq(self.user_id))
                .filter(schema::user_good_list_positions::list_id.eq(self.id))
                .load::<UserGoodListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::user_good_list_positions::types.eq("b"))
                  .get_result::<UserGoodListPosition>(&_connection)
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
            .set(schema::good_lists::types.eq(close_case))
            .get_result::<GoodList>(&_connection)
            .expect("E");
       return true;
    }
    pub fn restore_item(&self) -> bool {
        let _connection = establish_connection();
        if self.community_id.is_some() {
            use crate::schema::community_good_list_positions::dsl::community_good_list_positions;

            let list_positions = community_good_list_positions
                .filter(schema::community_good_list_positions::community_id.eq(self.community_id.unwrap()))
                .filter(schema::community_good_list_positions::list_id.eq(self.id))
                .load::<CommunityGoodListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::community_good_list_positions::types.eq("b"))
                  .get_result::<CommunityGoodListPosition>(&_connection)
                  .expect("Error.");
            }
        } else {
            use crate::schema::user_good_list_positions::dsl::user_good_list_positions;

            let list_positions = user_good_list_positions
                .filter(schema::user_good_list_positions::user_id.eq(self.user_id))
                .filter(schema::user_good_list_positions::list_id.eq(self.id))
                .load::<UserGoodListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::user_good_list_positions::types.eq("a"))
                  .get_result::<UserGoodListPosition>(&_connection)
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
    pub fn is_user_can_edit_delete_item(&self, user_id: i32) -> bool {
        if self.community_id.is_some() {
            let community = self.get_community();
            return community.get_staff_users_ids().iter().any(|&i| i==user_id);
        }
        else {
            return self.user_id == user_id;
        }
    }
    pub fn create_good(&self, title: String, community_id: Option<i32>, category_id: Option<i32>,
        user_id: i32, price: Option<i32>, description: Option<String>,
        image: Option<String>, comment_enabled: bool,
        images: Vec<String>) -> Good {

        let _connection = establish_connection();

        diesel::update(&*self)
          .set(schema::good_lists::count.eq(self.count + 1))
          .get_result::<GoodList>(&_connection)
          .expect("Error.");

        let new_good_form = NewGood {
            title: title,
            community_id: community_id,
            category_id: category_id,
            user_id: user_id,
            good_list_id: self.id,
            price: price,
            types: "a".to_string(),
            description: description,
            image: image,
            comment_enabled: comment_enabled,

            created: chrono::Local::now().naive_utc(),
            comment: 0,
            view: 0,
            repost: 0,
            copy: 0,
            position: (self.count).try_into().unwrap(),
            reactions: 0,
        };
        let new_good = diesel::insert_into(schema::goods::table)
            .values(&new_good_form)
            .get_result::<Good>(&_connection)
            .expect("Error.");

        for image in images.iter() {
            let new_image = NewGoodImage {
                good_id: self.id,
                src: image.to_string(),
            };
            diesel::insert_into(good_images::table)
                .values(&new_image)
                .get_result::<GoodImage>(&_connection)
                .expect("Error saving good image.");
            }
        if community_id.is_some() {
            let community = self.get_community();
            community.plus_goods(1);
            return new_good;
        }
        else {
            use crate::utils::get_user;

            let creator = get_user(user_id);
            creator.plus_goods(1);
            return new_good;
        }
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
    pub created:         chrono::NaiveDateTime,

    pub comment:         i32,
    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub reactions:       i32,
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
    pub created:         chrono::NaiveDateTime,

    pub comment:         i32,
    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub reactions:       i32,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="goods"]
pub struct EditGood {
    pub title:           String,
    pub price:           Option<i32>,
    pub description:     Option<String>,
    pub image:           Option<String>,
    pub comment_enabled: bool,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="goods"]
pub struct EditGoodPosition {
    pub position: i16,
}

impl Good {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_good(&self) -> bool {
        return true;
    }
    pub fn get_images(&self) -> Vec<GoodImage> {
        use crate::schema::good_images::dsl::good_images;

        let _connection = establish_connection();
        return good_images
            .filter(schema::good_images::good_id.eq(self.id))
            .load::<GoodImage>(&_connection)
            .expect("E");
    }
    pub fn get_images_str(&self) -> Vec<String> {
        use crate::schema::good_images::dsl::good_images;

        let _connection = establish_connection();
        let images_list = good_images
            .filter(schema::good_images::good_id.eq(self.id))
            .load::<GoodImage>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for item in images_list.iter() {
            stack.push(item.src.clone());
        };
        return stack;
    }

    pub fn copy_item(pk: i32, lists: Vec<i32>) -> bool {
        use crate::schema::goods::dsl::goods;
        use crate::schema::good_lists::dsl::good_lists;

        let _connection = establish_connection();
        let item = goods
            .filter(schema::goods::id.eq(pk))
            .filter(schema::goods::types.eq("a"))
            .load::<Good>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
        let mut count = 0;
        for list_id in lists.iter() {
            count += 1;
            let list = good_lists
                .filter(schema::good_lists::id.eq(list_id))
                .filter(schema::good_lists::types.lt(10))
                .load::<GoodList>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            list.create_good (
                item.title.clone(),
                list.community_id,
                item.category_id,
                item.user_id,
                item.price,
                item.description.clone(),
                item.image.clone(),
                item.comment_enabled,
                item.get_images_str(),
            );
        }

        diesel::update(&item)
          .set(schema::goods::copy.eq(item.copy + count))
          .get_result::<Good>(&_connection)
          .expect("Error.");

        if item.community_id.is_some() {
            let community = item.get_community();
            community.plus_goods(count);
        }
        else {
            let creator = item.get_creator();
            creator.plus_goods(count);
          }
        return true;
    }

    pub fn edit_good(&self, title: String, price: Option<i32>, description: Option<String>,
        image: Option<String>, comment_enabled: bool,
        images: Vec<String>) -> &Good {

        use crate::schema::good_images::dsl::good_images;

        let _connection = establish_connection();

        let edit_good = EditGood {
            title: title,
            price: price,
            description: description,
            image: image,
            comment_enabled: comment_enabled,
        };
        diesel::update(self)
            .set(edit_good)
            .get_result::<Good>(&_connection)
            .expect("Error.");

        diesel::delete(good_images.filter(schema::good_images::good_id.eq(self.id))).execute(&_connection).expect("E");
        for image in images.iter() {
            let new_image = NewGoodImage {
                good_id: self.id,
                src: image.to_string(),
            };
            diesel::insert_into(schema::good_images::table)
                .values(&new_image)
                .get_result::<GoodImage>(&_connection)
                .expect("Error saving good image.");
            };
        return self;
    }

    pub fn plus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::goods::comment.eq(self.comment + count))
            .get_result::<Good>(&_connection)
            .expect("Error.");
        return true;
    }

    pub fn plus_reactions(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::goods::reactions.eq(self.reactions + count))
            .get_result::<Good>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_reactions(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::goods::reactions.eq(self.reactions - count))
            .get_result::<Good>(&_connection)
            .expect("Error.");
        return true;
    }

    pub fn minus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::goods::comment.eq(self.comment - count))
            .get_result::<Good>(&_connection)
            .expect("Error.");
        return true;
    }

    pub fn get_code(&self) -> String {
        return "goo".to_string() + &self.get_str_id();
    }
    pub fn get_folder(&self) -> String {
        return "goods".to_string();
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
    pub fn get_list(&self) -> GoodList {
        use crate::schema::good_lists::dsl::good_lists;

        let _connection = establish_connection();
        return good_lists
            .filter(schema::good_lists::id.eq(self.good_list_id))
            .filter(schema::good_lists::types.lt(10))
            .load::<GoodList>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
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
            return "товар сообщества <a href='".to_owned() + &community.link.to_string() + &"' target='_blank'>" + &community.name + &"</a>"
        }
        else {
            let creator = self.get_creator();
            return "<a href='".to_owned() + &creator.link.to_string() + &"' target='_blank'>" + &creator.get_full_name() + &"</a>" + &": товар"
        }
    }
    pub fn count_reposts(&self) -> String {
        if self.repost > 0 {
            return self.repost.to_string()
        }
        else {
            return "".to_string()
        }
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

    pub fn count_copy(&self) -> String {
        if self.copy == 0 {
            return "".to_string();
        }
        else {
            return ", копировали - ".to_string() + &self.copy.to_string();
        }
    }
    pub fn message_reposts_count(&self) -> String {
        use crate::schema::good_reposts::dsl::good_reposts;

        let _connection = establish_connection();

        let count = good_reposts
            .filter(schema::good_reposts::good_id.eq(self.id))
            .filter(schema::good_reposts::message_id.is_not_null())
            .load::<GoodRepost>(&_connection)
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
        use crate::schema::good_reposts::dsl::good_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = good_reposts
            .filter(schema::good_reposts::good_id.eq(self.id))
            .filter(schema::good_reposts::post_id.is_not_null())
            .limit(limit)
            .offset(offset)
            .load::<GoodRepost>(&_connection)
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
        use crate::schema::good_reposts::dsl::good_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = good_reposts
            .filter(schema::good_reposts::good_id.eq(self.id))
            .filter(schema::good_reposts::post_id.is_not_null())
            .limit(6)
            .load::<GoodRepost>(&_connection)
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

    pub fn create_comment(&self, user: &User, attach: Option<String>,
        parent_id: Option<i32>, content: Option<String>, sticker_id: Option<i32>) -> GoodComment {

        let _connection = establish_connection();
        diesel::update(self)
          .set(schema::goods::comment.eq(self.comment + 1))
          .get_result::<Good>(&_connection)
          .expect("Error.");

        let new_comment_form = NewGoodComment {
            good_id:    self.id,
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
        let new_comment = diesel::insert_into(schema::good_comments::table)
            .values(&new_comment_form)
            .get_result::<GoodComment>(&_connection)
            .expect("Error.");

        return new_comment;
    }

    pub fn get_comments(&self, limit: i64, offset: i64) -> Vec<GoodComment> {
        use crate::schema::good_comments::dsl::good_comments;

        let _connection = establish_connection();

        return good_comments
            .filter(schema::good_comments::good_id.eq(self.id))
            .filter(schema::good_comments::types.eq_any(vec!["a","b"]))
            .limit(limit)
            .offset(offset)
            .load::<GoodComment>(&_connection)
            .expect("E.");
    }
    pub fn get_or_create_react_model(&self) -> GoodReaction {
        use crate::schema::good_reactions::dsl::good_reactions;

        let _connection = establish_connection();
        let _react_model = good_reactions
            .filter(schema::good_reactions::good_id.eq(self.id))
            .load::<GoodReaction>(&_connection)
            .expect("E.");
        if _react_model.len() > 0 {
            return _react_model.into_iter().nth(0).unwrap();
        }
        else {
            let new_react_model = NewGoodReaction {
                good_id:  self.id,
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
            let _react_model = diesel::insert_into(schema::good_reactions::table)
                .values(&new_react_model)
                .get_result::<GoodReaction>(&_connection)
                .expect("Error.");

            return _react_model;
        }
    }

    pub fn send_reaction(&self, user_id: i32, types: i16) -> Json<JsonItemReactions> {
        use crate::schema::good_votes::dsl::good_votes;

        let _connection = establish_connection();
        let list = self.get_list();
        let reactions_of_list = list.get_reactions_list();
        let react_model = self.get_or_create_react_model();
        let mut new_plus = false;
        let mut old_type = 0;

        if reactions_of_list.iter().any(|&i| i==types) && list.is_user_can_see_el(user_id) {

            let votes = good_votes
                .filter(schema::good_votes::user_id.eq(user_id))
                .filter(schema::good_votes::good_id.eq(self.id))
                .load::<GoodVote>(&_connection)
                .expect("E.");

            // если пользователь уже реагировал на товар
            if votes.len() > 0 {
                let vote = votes.into_iter().nth(0).unwrap();

                // если пользователь уже реагировал этой реакцией на этот товар
                if vote.reaction == types {
                    diesel::delete(good_votes
                        .filter(schema::good_votes::user_id.eq(user_id))
                        .filter(schema::good_votes::good_id.eq(self.id))
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
                        .set(schema::good_votes::reaction.eq(types))
                        .get_result::<GoodVote>(&_connection)
                        .expect("Error.");

                    react_model.update_model(types, Some(old_type), false);
                }
            }

            // если пользователь не реагировал на этот товар
            else {
                let new_vote = NewGoodVote {
                    vote: 1,
                    user_id: user_id,
                    good_id: self.id,
                    reaction: types,
                };
                diesel::insert_into(schema::good_votes::table)
                    .values(&new_vote)
                    .get_result::<GoodVote>(&_connection)
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
            .set(schema::goods::types.eq(close_case))
            .get_result::<Good>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::good_lists::count.eq(list.count - 1))
            .get_result::<GoodList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.minus_goods(1);
        }
        else {
            let creator = self.get_creator();
            creator.minus_goods(1);
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
            .set(schema::goods::types.eq(close_case))
            .get_result::<Good>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::good_lists::count.eq(list.count + 1))
            .get_result::<GoodList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.plus_goods(1);
        }
        else {
            let creator = self.get_creator();
            creator.plus_goods(1);
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
            .set(schema::goods::types.eq(close_case))
            .get_result::<Good>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::good_lists::count.eq(list.count - 1))
            .get_result::<GoodList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.minus_goods(1);
        }
        else {
            let creator = self.get_creator();
            creator.minus_goods(1);
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
            .set(schema::goods::types.eq(close_case))
            .get_result::<Good>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::good_lists::count.eq(list.count + 1))
            .get_result::<GoodList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.plus_goods(1);
        }
        else {
            let creator = self.get_creator();
            creator.plus_goods(1);
         }
       return true;
    }

    pub fn count_comments(&self) -> String {
        if self.comment == 0 {
            return "".to_string();
        }
        else {
            return self.comment.to_string();
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

    pub fn reposts_count_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.repost,
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }
    pub fn is_have_reactions(&self) -> bool {
        return self.reactions > 0;
    }
    pub fn is_have_reposts(&self) -> bool {
        return self.repost > 0;
    }

    pub fn reactions_ids(&self) -> Vec<i32> {
        use crate::schema::good_votes::dsl::good_votes;

        let _connection = establish_connection();
        let votes = good_votes
            .filter(schema::good_votes::good_id.eq(self.id))
            .load::<GoodVote>(&_connection)
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
        use crate::schema::good_votes::dsl::good_votes;
        // "/static/images/reactions/" + get_user_reaction + ".jpg"
        let _connection = establish_connection();
        let vote = good_votes
            .filter(schema::good_votes::user_id.eq(user_id))
            .filter(schema::good_votes::good_id.eq(self.id))
            .load::<GoodVote>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();

        return vote.reaction;
    }

    pub fn get_reactions_users_of_types(&self, limit: i64, offset: i64, types: i16) -> Vec<User> {
        use crate::schema::good_votes::dsl::good_votes;
        use crate::utils::get_users_from_ids;

        let _connection = establish_connection();
        let votes = good_votes
            .filter(schema::good_votes::good_id.eq(self.id))
            .filter(schema::good_votes::reaction.eq(types))
            .limit(limit)
            .offset(offset)
            .load::<GoodVote>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in votes.iter() {
            stack.push(_item.user_id);
        };
        return get_users_from_ids(stack);
    }

    pub fn get_6_reactions_users_of_types(&self, types: i16) -> Vec<User> {
        use crate::schema::good_votes::dsl::good_votes;
        use crate::utils::get_users_from_ids;

        let _connection = establish_connection();
        let votes = good_votes
            .filter(schema::good_votes::good_id.eq(self.id))
            .filter(schema::good_votes::reaction.eq(types))
            .limit(6)
            .load::<GoodVote>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in votes.iter() {
            stack.push(_item.user_id);
        };
        return get_users_from_ids(stack);
    }

    pub fn change_position(query: Json<Vec<JsonPosition>>) -> bool {
        use crate::schema::goods::dsl::goods;

        let _connection = establish_connection();
        for i in query.iter() {
            let item = goods
                .filter(schema::goods::id.eq(i.key))
                .filter(schema::goods::types.eq("a"))
                .limit(1)
                .load::<Good>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            diesel::update(&item)
                .set(schema::goods::position.eq(i.value))
                .get_result::<Good>(&_connection)
                .expect("Error.");
        }
        return true;
    }
}

/////// GoodImage //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Good)]
pub struct GoodImage {
    pub id:      i32,
    pub good_id: i32,
    pub src:     String,
}
#[derive(Deserialize, Insertable)]
#[table_name="good_images"]
pub struct NewGoodImage {
    pub good_id: i32,
    pub src:     String,
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
    pub repost:     i32,
    pub reactions:  i32,
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
    pub repost:     i32,
    pub reactions:  i32,
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
    pub fn is_good_comment(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "cgo".to_string() + &self.get_str_id();
    }
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(84))
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
            .filter(schema::moderateds::types.eq(84))
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
    pub fn get_item(&self) -> Good {
        use crate::schema::goods::dsl::goods;
        let _connection = establish_connection();
        return goods
            .filter(schema::goods::id.eq(self.good_id))
            .filter(schema::goods::types.eq("a"))
            .load::<Good>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_list(&self) -> GoodList {
        return self.get_item().get_list();
    }
    pub fn get_parent(&self) -> GoodComment {
        use crate::schema::good_comments::dsl::good_comments;

        let _connection = establish_connection();
        return good_comments
            .filter(schema::good_comments::id.eq(self.parent_id.unwrap()))
            .filter(schema::good_comments::types.eq_any(vec!["a", "b"]))
            .load::<GoodComment>(&_connection)
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

    pub fn is_have_reposts(&self) -> bool {
        return self.repost > 0;
    }

    pub fn get_replies(&self) -> Vec<GoodComment> {
        use crate::schema::good_comments::dsl::good_comments;

        let _connection = establish_connection();
        return good_comments
            .filter(schema::good_comments::good_id.eq(self.id))
            .filter(schema::good_comments::types.eq_any(vec!["a", "b"]))
            .load::<GoodComment>(&_connection)
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
            .set(schema::goods::comment.eq(item.comment - 1))
            .get_result::<Good>(&_connection)
            .expect("E");

        diesel::update(self)
            .set(schema::good_comments::types.eq(close_case))
            .get_result::<GoodComment>(&_connection)
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
            .set(schema::goods::comment.eq(item.comment + 1))
            .get_result::<Good>(&_connection)
            .expect("E");

        diesel::update(self)
            .set(schema::good_comments::types.eq(close_case))
            .get_result::<GoodComment>(&_connection)
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
            .set(schema::goods::comment.eq(item.comment - 1))
            .get_result::<Good>(&_connection)
            .expect("E");

        diesel::update(self)
            .set(schema::good_comments::types.eq(close_case))
            .get_result::<GoodComment>(&_connection)
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
            .set(schema::goods::comment.eq(item.comment + 1))
            .get_result::<Good>(&_connection)
            .expect("E");

        diesel::update(self)
            .set(schema::good_comments::types.eq(close_case))
            .get_result::<GoodComment>(&_connection)
            .expect("E");
       return true;
    }
    pub fn is_deleted(&self) -> bool {
        return self.types == "c" && self.types == "d";
    }
    pub fn is_closed(&self) -> bool {
        return self.types == "e" && self.types == "f";
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

    pub fn get_or_create_react_model(&self) -> GoodCommentReaction {
        use crate::schema::good_comment_reactions::dsl::good_comment_reactions;

        let _connection = establish_connection();
        let _react_model = good_comment_reactions
            .filter(schema::good_comment_reactions::good_comment_id.eq(self.id))
            .load::<GoodCommentReaction>(&_connection)
            .expect("E.");
        if _react_model.len() > 0 {
            return _react_model.into_iter().nth(0).unwrap();
        }
        else {
            let new_react_model = NewGoodCommentReaction {
                good_comment_id: self.id,
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
            let _react_model = diesel::insert_into(schema::good_comment_reactions::table)
                .values(&new_react_model)
                .get_result::<GoodCommentReaction>(&_connection)
                .expect("Error.");

            return _react_model;
        }
    }

    pub fn send_reaction(&self, user_id: i32, types: i16) -> Json<JsonItemReactions> {
        use crate::schema::good_comment_votes::dsl::good_comment_votes;

        let _connection = establish_connection();
        let list = self.get_list();
        let reactions_of_list = list.get_reactions_list();
        let react_model = self.get_or_create_react_model();
        let mut new_plus = false;
        let mut old_type = 0;

        if reactions_of_list.iter().any(|&i| i==types) && list.is_user_can_see_el(user_id) && list.is_user_can_see_comment(user_id) {

            let votes = good_comment_votes
                .filter(schema::good_comment_votes::user_id.eq(user_id))
                .filter(schema::good_comment_votes::good_comment_id.eq(self.id))
                .load::<GoodCommentVote>(&_connection)
                .expect("E.");

            // если пользователь уже реагировал на товар
            if votes.len() > 0 {
                let vote = votes.into_iter().nth(0).unwrap();

                // если пользователь уже реагировал этой реакцией на этот товар
                if vote.reaction == types {
                    diesel::delete(good_comment_votes
                        .filter(schema::good_comment_votes::user_id.eq(user_id))
                        .filter(schema::good_comment_votes::good_comment_id.eq(self.id))
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
                        .set(schema::good_comment_votes::reaction.eq(types))
                        .get_result::<GoodCommentVote>(&_connection)
                        .expect("Error.");

                    react_model.update_model(types, Some(old_type), false);
                }
            }

            // если пользователь не реагировал на этот товар
            else {
                let new_vote = NewGoodCommentVote {
                    vote:            1,
                    user_id:         user_id,
                    good_comment_id: self.id,
                    reaction:        types,
                };
                diesel::insert_into(schema::good_comment_votes::table)
                    .values(&new_vote)
                    .get_result::<GoodCommentVote>(&_connection)
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
        use crate::schema::good_comment_votes::dsl::good_comment_votes;

        let _connection = establish_connection();
        let votes = good_comment_votes
            .filter(schema::good_comment_votes::good_comment_id.eq(self.id))
            .load::<GoodCommentVote>(&_connection)
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
        use crate::schema::good_comment_votes::dsl::good_comment_votes;
        // "/static/images/reactions/" + get_user_reaction + ".jpg"
        let _connection = establish_connection();
        let vote = good_comment_votes
            .filter(schema::good_comment_votes::user_id.eq(user_id))
            .filter(schema::good_comment_votes::good_comment_id.eq(self.id))
            .load::<GoodCommentVote>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();

        return vote.reaction;
    }

    pub fn get_reactions_users_of_types(&self, limit: i64, offset: i64, types: i16) -> Vec<User> {
        use crate::schema::good_comment_votes::dsl::good_comment_votes;
        use crate::utils::get_users_from_ids;

        let _connection = establish_connection();
        let votes = good_comment_votes
            .filter(schema::good_comment_votes::good_comment_id.eq(self.id))
            .filter(schema::good_comment_votes::reaction.eq(types))
            .limit(limit)
            .offset(offset)
            .load::<GoodCommentVote>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in votes.iter() {
            stack.push(_item.user_id);
        };
        return get_users_from_ids(stack);
    }

    pub fn get_6_reactions_users_of_types(&self, types: i16) -> Vec<User> {
        use crate::schema::good_comment_votes::dsl::good_comment_votes;
        use crate::utils::get_users_from_ids;

        let _connection = establish_connection();
        let votes = good_comment_votes
            .filter(schema::good_comment_votes::good_comment_id.eq(self.id))
            .filter(schema::good_comment_votes::reaction.eq(types))
            .limit(6)
            .load::<GoodCommentVote>(&_connection)
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
            .set(schema::good_comments::reactions.eq(self.reactions + count))
            .get_result::<GoodComment>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_reactions(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::good_comments::reactions.eq(self.reactions - count))
            .get_result::<GoodComment>(&_connection)
            .expect("Error.");
        return true;
    }
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
    pub id:       i32,
    pub vote:     i16,
    pub user_id:  i32,
    pub good_id:  i32,
    pub reaction: i16,
}
impl GoodVote {
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
#[table_name="good_votes"]
pub struct NewGoodVote {
    pub vote:     i16,
    pub user_id:  i32,
    pub good_id:  i32,
    pub reaction: i16,
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
    pub reaction:        i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="good_comment_votes"]
pub struct NewGoodCommentVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub good_comment_id: i32,
    pub reaction:        i16,
}


/////// GoodListRepost //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(GoodList)]
#[belongs_to(Post)]
#[belongs_to(Message)]
pub struct GoodListRepost {
    pub id:            i32,
    pub good_list_id:  i32,
    pub post_id:       Option<i32>,
    pub message_id:    Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="good_list_reposts"]
pub struct NewGoodListRepost {
    pub good_list_id: i32,
    pub post_id:      Option<i32>,
    pub message_id:   Option<i32>,
}

/////// GoodRepost //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Good)]
#[belongs_to(Post)]
#[belongs_to(Message)]
pub struct GoodRepost {
    pub id:         i32,
    pub good_id:    i32,
    pub post_id:    Option<i32>,
    pub message_id: Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="good_reposts"]
pub struct NewGoodRepost {
    pub good_id:   i32,
    pub post_id:    Option<i32>,
    pub message_id: Option<i32>,
}

/////// GoodReaction //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Good)]
pub struct GoodReaction {
    pub id:       i32,
    pub good_id:  i32,
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
impl GoodReaction {
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
    ) -> &GoodReaction {
        let _connection = establish_connection();
        if old_types_option.is_some() {
            let old_types = old_types_option.unwrap();
            match new_types {
                1 => diesel::update(self)
                    .set(schema::good_reactions::field_1.eq(self.field_1 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::good_reactions::field_2.eq(self.field_2 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::good_reactions::field_3.eq(self.field_3 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::good_reactions::field_4.eq(self.field_4 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::good_reactions::field_5.eq(self.field_5 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::good_reactions::field_6.eq(self.field_6 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::good_reactions::field_7.eq(self.field_7 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::good_reactions::field_8.eq(self.field_8 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::good_reactions::field_9.eq(self.field_9 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::good_reactions::field_10.eq(self.field_10 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::good_reactions::field_11.eq(self.field_11 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::good_reactions::field_12.eq(self.field_12 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::good_reactions::field_13.eq(self.field_13 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::good_reactions::field_14.eq(self.field_14 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::good_reactions::field_15.eq(self.field_15 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::good_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::good_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
            };

            match old_types {
                1 => diesel::update(self)
                    .set(schema::good_reactions::field_1.eq(self.field_1 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::good_reactions::field_2.eq(self.field_2 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::good_reactions::field_3.eq(self.field_3 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::good_reactions::field_4.eq(self.field_4 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::good_reactions::field_5.eq(self.field_5 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::good_reactions::field_6.eq(self.field_6 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::good_reactions::field_7.eq(self.field_7 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::good_reactions::field_8.eq(self.field_8 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::good_reactions::field_9.eq(self.field_9 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::good_reactions::field_10.eq(self.field_10 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::good_reactions::field_11.eq(self.field_11 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::good_reactions::field_12.eq(self.field_12 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::good_reactions::field_13.eq(self.field_13 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::good_reactions::field_14.eq(self.field_14 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::good_reactions::field_15.eq(self.field_15 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::good_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::good_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<GoodReaction>(&_connection)
                    .expect("Error."),
            };
            return &self;
        }
        else {
            if plus {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::good_reactions::field_1.eq(self.field_1 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::good_reactions::field_2.eq(self.field_2 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::good_reactions::field_3.eq(self.field_3 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::good_reactions::field_4.eq(self.field_4 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::good_reactions::field_5.eq(self.field_5 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::good_reactions::field_6.eq(self.field_6 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::good_reactions::field_7.eq(self.field_7 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::good_reactions::field_8.eq(self.field_8 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::good_reactions::field_9.eq(self.field_9 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::good_reactions::field_10.eq(self.field_10 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::good_reactions::field_11.eq(self.field_11 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::good_reactions::field_12.eq(self.field_12 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::good_reactions::field_13.eq(self.field_13 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::good_reactions::field_14.eq(self.field_14 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::good_reactions::field_15.eq(self.field_15 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::good_reactions::field_16.eq(self.field_16 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::good_reactions::field_16.eq(self.field_16 + 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                };
            }
            else {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::good_reactions::field_1.eq(self.field_1 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::good_reactions::field_2.eq(self.field_2 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::good_reactions::field_3.eq(self.field_3 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::good_reactions::field_4.eq(self.field_4 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::good_reactions::field_5.eq(self.field_5 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::good_reactions::field_6.eq(self.field_6 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::good_reactions::field_7.eq(self.field_7 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::good_reactions::field_8.eq(self.field_8 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::good_reactions::field_9.eq(self.field_9 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::good_reactions::field_10.eq(self.field_10 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::good_reactions::field_11.eq(self.field_11 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::good_reactions::field_12.eq(self.field_12 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::good_reactions::field_13.eq(self.field_13 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::good_reactions::field_14.eq(self.field_14 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::good_reactions::field_15.eq(self.field_15 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::good_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::good_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<GoodReaction>(&_connection)
                        .expect("Error."),
                };
            }
            return &self;
        }
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="good_reactions"]
pub struct NewGoodReaction {
    pub good_id:     i32,
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


/////// GoodCommentReaction //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(GoodComment)]
pub struct GoodCommentReaction {
    pub id:              i32,
    pub good_comment_id: i32,
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
impl GoodCommentReaction {
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
    ) -> &GoodCommentReaction {
        let _connection = establish_connection();
        if old_types_option.is_some() {
            let old_types = old_types_option.unwrap();
            match new_types {
                1 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_1.eq(self.field_1 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::good_comment_reactions::field_2.eq(self.field_2 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_3.eq(self.field_3 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_4.eq(self.field_4 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_5.eq(self.field_5 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_6.eq(self.field_6 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_7.eq(self.field_7 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_8.eq(self.field_8 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_9.eq(self.field_9 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_10.eq(self.field_10 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_11.eq(self.field_11 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_12.eq(self.field_12 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_13.eq(self.field_13 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_14.eq(self.field_14 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_15.eq(self.field_15 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::good_comment_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
            };

            match old_types {
                1 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_1.eq(self.field_1 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::good_comment_reactions::field_2.eq(self.field_2 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_3.eq(self.field_3 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_4.eq(self.field_4 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_5.eq(self.field_5 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_6.eq(self.field_6 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_7.eq(self.field_7 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_8.eq(self.field_8 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_9.eq(self.field_9 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_10.eq(self.field_10 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_11.eq(self.field_11 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_12.eq(self.field_12 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_13.eq(self.field_13 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_14.eq(self.field_14 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_15.eq(self.field_15 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::good_comment_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::good_comment_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<GoodCommentReaction>(&_connection)
                    .expect("Error."),
            };
            return &self;
        }
        else {
            if plus {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_1.eq(self.field_1 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::good_comment_reactions::field_2.eq(self.field_2 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_3.eq(self.field_3 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_4.eq(self.field_4 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_5.eq(self.field_5 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_6.eq(self.field_6 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_7.eq(self.field_7 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_8.eq(self.field_8 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_9.eq(self.field_9 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_10.eq(self.field_10 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_11.eq(self.field_11 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_12.eq(self.field_12 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_13.eq(self.field_13 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_14.eq(self.field_14 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_15.eq(self.field_15 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_16.eq(self.field_16 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::good_comment_reactions::field_16.eq(self.field_16 + 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                };
            }
            else {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_1.eq(self.field_1 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::good_comment_reactions::field_2.eq(self.field_2 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_3.eq(self.field_3 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_4.eq(self.field_4 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_5.eq(self.field_5 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_6.eq(self.field_6 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_7.eq(self.field_7 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_8.eq(self.field_8 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_9.eq(self.field_9 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_10.eq(self.field_10 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_11.eq(self.field_11 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_12.eq(self.field_12 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_13.eq(self.field_13 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_14.eq(self.field_14 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_15.eq(self.field_15 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::good_comment_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::good_comment_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<GoodCommentReaction>(&_connection)
                        .expect("Error."),
                };
            }
            return &self;
        }
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="good_comment_reactions"]
pub struct NewGoodCommentReaction {
    pub good_comment_id: i32,
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
