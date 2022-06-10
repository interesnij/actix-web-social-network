use crate::schema;
use diesel::prelude::*;
use crate::schema::{
    survey_lists,
    surveys,
    user_survey_list_collections,
    community_survey_list_collections,
    survey_list_perms,
    survey_answers,
    survey_votes,
    survey_list_reposts,
    survey_reposts,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::{establish_connection, JsonPosition};
use crate::models::{
    User,
    Community,
    UserSurveyListPosition,
    CommunitySurveyListPosition,
    Message,
    Post,
};
use actix_web::web::Json;
use chrono::NaiveDateTime;

/////// SurveyList //////
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

/////// SurveyList //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(User)]
pub struct SurveyList {
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
    pub create_el:       String,
    pub copy_el:         String,
}
#[derive(Deserialize, Insertable)]
#[table_name="survey_lists"]
pub struct NewSurveyList {
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
    pub create_el:       String,
    pub copy_el:         String,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="survey_lists"]
pub struct EditSurveyList {
    pub name:         String,
    pub description:  Option<String>,
    pub image:        Option<String>,
    pub can_see_el:   String,
    pub create_el:    String,
    pub copy_el:      String,
}


impl SurveyList {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_survey_list(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "lsu".to_string() + &self.get_str_id();
    }
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(23))
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
            .filter(schema::moderateds::types.eq(23))
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

    pub fn message_reposts_count(&self) -> String {
        use crate::schema::survey_list_reposts::dsl::survey_list_reposts;

        let _connection = establish_connection();

        let count = survey_list_reposts
            .filter(schema::survey_list_reposts::survey_list_id.eq(self.id))
            .filter(schema::survey_list_reposts::message_id.is_not_null())
            .load::<SurveyListRepost>(&_connection)
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
        use crate::schema::survey_list_reposts::dsl::survey_list_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = survey_list_reposts
            .filter(schema::survey_list_reposts::survey_list_id.eq(self.id))
            .filter(schema::survey_list_reposts::post_id.is_not_null())
            .limit(limit)
            .offset(offset)
            .load::<SurveyListRepost>(&_connection)
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
        use crate::schema::survey_list_reposts::dsl::survey_list_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = survey_list_reposts
            .filter(schema::survey_list_reposts::survey_list_id.eq(self.id))
            .filter(schema::survey_list_reposts::post_id.is_not_null())
            .limit(6)
            .load::<SurveyListRepost>(&_connection)
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
        return "<a data-surveylist='".to_string() + &self.get_str_id() + &"' class='ajax'>".to_string() + &self.name + &"</a>".to_string();
    }
    pub fn is_user_list(&self, user: User) -> bool {
        return self.user_id == user.id;
    }
    pub fn is_community_list(&self, community: Community) -> bool {
        return self.community_id.unwrap() == community.id;
    }
    pub fn get_users_ids(&self) -> Vec<i32> {
        use crate::schema::user_survey_list_collections::dsl::user_survey_list_collections;

        let _connection = establish_connection();
        let ids = user_survey_list_collections
            .filter(schema::user_survey_list_collections::survey_list_id.eq(self.id))
            .load::<UserSurveyListCollection>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in ids.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_communities_ids(&self) -> Vec<i32> {
        use crate::schema::community_survey_list_collections::dsl::community_survey_list_collections;

        let _connection = establish_connection();
        let ids = community_survey_list_collections
            .filter(schema::community_survey_list_collections::survey_list_id.eq(self.id))
            .load::<CommunitySurveyListCollection>(&_connection)
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
    pub fn get_items(&self) -> Vec<Survey> {
        use crate::schema::surveys::dsl::surveys;

        let _connection = establish_connection();
        return surveys
            .filter(schema::surveys::survey_list_id.eq(self.id))
            .filter(schema::surveys::types.eq("a"))
            .order(schema::surveys::created.desc())
            .load::<Survey>(&_connection)
            .expect("E.");
    }
    pub fn get_paginate_items(&self, limit: i64, offset: i64) -> Vec<Survey> {
        use crate::schema::surveys::dsl::surveys;

        let _connection = establish_connection();
        return surveys
            .filter(schema::surveys::survey_list_id.eq(self.id))
            .filter(schema::surveys::types.eq("a"))
            .limit(limit)
            .offset(offset)
            .order(schema::surveys::created.desc())
            .load::<Survey>(&_connection)
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
        use crate::schema::survey_list_perms::dsl::survey_list_perms;

        let _connection = establish_connection();
        let items = survey_list_perms
            .filter(schema::survey_list_perms::survey_list_id.eq(self.id))
            .filter(schema::survey_list_perms::can_see_item.eq("b"))
            .load::<SurveyListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::survey_list_perms::dsl::survey_list_perms;

        let _connection = establish_connection();
        let items = survey_list_perms
            .filter(schema::survey_list_perms::survey_list_id.eq(self.id))
            .filter(schema::survey_list_perms::can_see_item.eq("a"))
            .load::<SurveyListPerm>(&_connection)
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

    pub fn get_create_el_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::survey_list_perms::dsl::survey_list_perms;

        let _connection = establish_connection();
        let items = survey_list_perms
            .filter(schema::survey_list_perms::survey_list_id.eq(self.id))
            .filter(schema::survey_list_perms::create_item.eq("b"))
            .load::<SurveyListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_create_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::survey_list_perms::dsl::survey_list_perms;

        let _connection = establish_connection();
        let items = survey_list_perms
            .filter(schema::survey_list_perms::survey_list_id.eq(self.id))
            .filter(schema::survey_list_perms::create_item.eq("a"))
            .load::<SurveyListPerm>(&_connection)
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

    pub fn get_copy_el_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::survey_list_perms::dsl::survey_list_perms;

        let _connection = establish_connection();
        let items = survey_list_perms
            .filter(schema::survey_list_perms::survey_list_id.eq(self.id))
            .filter(schema::survey_list_perms::can_copy.eq("b"))
            .load::<SurveyListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_copy_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::survey_list_perms::dsl::survey_list_perms;

        let _connection = establish_connection();
        let items = survey_list_perms
            .filter(schema::survey_list_perms::survey_list_id.eq(self.id))
            .filter(schema::survey_list_perms::can_copy.eq("a"))
            .load::<SurveyListPerm>(&_connection)
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
    pub fn is_anon_user_can_create_item(&self) -> bool {
        return self.create_el == "a";
    }
    pub fn is_anon_user_can_copy_el(&self) -> bool {
        return self.copy_el == "a";
    }
    pub fn create_list(creator: User, name: String, description: Option<String>, image: Option<String>,
        community_id: Option<i32>, can_see_el: String, create_el: String, copy_el: String,
        can_see_el_users: Option<Vec<i32>>, create_el_users: Option<Vec<i32>>,
        copy_el_users: Option<Vec<i32>>) -> SurveyList {

        use crate::models::{
            NewCommunitySurveyListPosition,
            NewUserSurveyListPosition,
        };

        let _connection = establish_connection();
        let new_list_form = NewSurveyList {
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
            create_el: create_el.clone(),
            copy_el: copy_el.clone(),
        };
        let new_list = diesel::insert_into(schema::survey_lists::table)
            .values(&new_list_form)
            .get_result::<SurveyList>(&_connection)
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

            let _new_list_position = NewCommunitySurveyListPosition {
                community_id: community.id,
                list_id:      new_list.id,
                position:     community.get_survey_lists_new_position(),
                types:        "a".to_string(),
            };
            let _list_position = diesel::insert_into(schema::community_survey_list_positions::table)
                .values(&_new_list_position)
                .get_result::<CommunitySurveyListPosition>(&_connection)
                .expect("Error saving survey_list_position.");
        }
        else {
            let _new_list_position = NewUserSurveyListPosition {
                user_id:  creator.id,
                list_id:  new_list.id,
                position: creator.get_survey_lists_new_position(),
                types:    "a".to_string(),
            };
            let _list_position = diesel::insert_into(schema::user_survey_list_positions::table)
                .values(&_new_list_position)
                .get_result::<UserSurveyListPosition>(&_connection)
                .expect("Error saving survey_list_position.");
        }

        if can_see_el == "e".to_string() && can_see_el == "h".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewSurveyListPerm {
                        user_id:      user_id,
                        survey_list_id:  new_list.id,
                        can_see_item: Some("b".to_string()),
                        create_item: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::survey_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<SurveyListPerm>(&_connection)
                        .expect("Error saving survey_list_position.");
                }
            }
        }
        else if can_see_el == "f".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewSurveyListPerm {
                        user_id:      user_id,
                        survey_list_id: new_list.id,
                        can_see_item: Some("a".to_string()),
                        create_item: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::survey_list_perms::table)
                        .values(&_new_include)
                        .get_result::<SurveyListPerm>(&_connection)
                        .expect("Error saving survey_list_position.");
                }
            }
        }

        if create_el == "e".to_string() && create_el == "h".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewSurveyListPerm {
                        user_id:      user_id,
                        survey_list_id: new_list.id,
                        can_see_item: None,
                        create_item: Some("b".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::survey_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<SurveyListPerm>(&_connection)
                        .expect("Error saving survey_list_position.");
                }
            }
        }
        else if create_el == "f".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewSurveyListPerm {
                        user_id:      user_id,
                        survey_list_id: new_list.id,
                        can_see_item: None,
                        create_item: Some("a".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::survey_list_perms::table)
                        .values(&_new_include)
                        .get_result::<SurveyListPerm>(&_connection)
                        .expect("Error saving survey_list_position.");
                }
            }
        }

        if copy_el == "e".to_string() && copy_el == "h".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewSurveyListPerm {
                        user_id:      user_id,
                        survey_list_id: new_list.id,
                        can_see_item: None,
                        create_item: None,
                        can_copy: Some("b".to_string()),
                    };
                    diesel::insert_into(schema::survey_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<SurveyListPerm>(&_connection)
                        .expect("Error saving survey_list_position.");
                }
            }
        }
        else if copy_el == "f".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewSurveyListPerm {
                        user_id:      user_id,
                        survey_list_id: new_list.id,
                        can_see_item: None,
                        create_item: None,
                        can_copy: Some("a".to_string()),
                    };
                    diesel::insert_into(schema::survey_list_perms::table)
                        .values(&_new_include)
                        .get_result::<SurveyListPerm>(&_connection)
                        .expect("Error saving survey_list_position.");
                }
            }
        }
        return new_list;
    }
    pub fn edit_list(&self, name: String, description: Option<String>, image: Option<String>,
        can_see_el: String, create_el: String, copy_el: String,
        can_see_el_users: Option<Vec<i32>>, create_el_users: Option<Vec<i32>>,
        copy_el_users: Option<Vec<i32>>) -> &SurveyList {

        use crate::schema::survey_list_perms::dsl::survey_list_perms;

        let _connection = establish_connection();

            let edit_survey_list = EditSurveyList{
                name: name,
                description: description,
                image: image,
                can_see_el: can_see_el.clone(),
                create_el: create_el.clone(),
                copy_el: copy_el.clone(),
            };
            diesel::update(self)
                .set(edit_survey_list)
                .get_result::<SurveyList>(&_connection)
                .expect("Error.");

        if can_see_el == "e".to_string() && can_see_el == "h".to_string() {
            if can_see_el_users.is_some() {
                diesel::delete (
                  survey_list_perms
                    .filter(schema::survey_list_perms::survey_list_id.eq(self.id))
                    .filter(schema::survey_list_perms::can_see_item.is_not_null())
                )
                  .execute(&_connection)
                  .expect("E");
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewSurveyListPerm {
                        user_id:      user_id,
                        survey_list_id: self.id,
                        can_see_item: Some("b".to_string()),
                        create_item: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::survey_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<SurveyListPerm>(&_connection)
                        .expect("Error saving survey_list_position.");
                }
            }
        }
        else if can_see_el == "f".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewSurveyListPerm {
                        user_id:      user_id,
                        survey_list_id: self.id,
                        can_see_item: Some("a".to_string()),
                        create_item: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::survey_list_perms::table)
                        .values(&_new_include)
                        .get_result::<SurveyListPerm>(&_connection)
                        .expect("Error saving survey_list_position.");
                }
            }
        }

        if create_el == "e".to_string() && create_el == "h".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewSurveyListPerm {
                        user_id:      user_id,
                        survey_list_id: self.id,
                        can_see_item: None,
                        create_item: Some("b".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::survey_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<SurveyListPerm>(&_connection)
                        .expect("Error saving survey_list_position.");
                }
            }
        }
        else if create_el == "f".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewSurveyListPerm {
                        user_id:      user_id,
                        survey_list_id: self.id,
                        can_see_item: None,
                        create_item: Some("a".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::survey_list_perms::table)
                        .values(&_new_include)
                        .get_result::<SurveyListPerm>(&_connection)
                        .expect("Error saving survey_list_position.");
                }
            }
        }

        if copy_el == "e".to_string() && copy_el == "h".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewSurveyListPerm {
                        user_id:      user_id,
                        survey_list_id: self.id,
                        can_see_item: None,
                        create_item: None,
                        can_copy: Some("b".to_string()),
                    };
                    diesel::insert_into(schema::survey_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<SurveyListPerm>(&_connection)
                        .expect("Error saving survey_list_position.");
                }
            }
        }
        else if copy_el == "f".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewSurveyListPerm {
                        user_id:      user_id,
                        survey_list_id: self.id,
                        can_see_item: None,
                        create_item: None,
                        can_copy: Some("a".to_string()),
                    };
                    diesel::insert_into(schema::survey_list_perms::table)
                        .values(&_new_include)
                        .get_result::<SurveyListPerm>(&_connection)
                        .expect("Error saving survey_list_position.");
                }
            }
        }
        return self;
    }
    pub fn get_order(&self) -> UserSurveyListPosition {
        use crate::schema::user_survey_list_positions::dsl::user_survey_list_positions;

        let _connection = establish_connection();
        return user_survey_list_positions
            .filter(schema::user_survey_list_positions::list_id.eq(self.id))
            .filter(schema::user_survey_list_positions::types.eq("a"))
            .load::<UserSurveyListPosition>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn add_in_community_collections(&self, community: Community) -> bool {
        use crate::models::NewCommunitySurveyListPosition;

        if !self.get_communities_ids().iter().any(|&i| i==community.id) && self.community_id.is_some() && self.community_id.unwrap() == community.id {
            return false;
        }
        let _connection = establish_connection();
        let new_item = NewCommunitySurveyListCollection {
            community_id: community.id,
            survey_list_id: self.id,
        };
        diesel::insert_into(schema::community_survey_list_collections::table)
            .values(&new_item)
            .get_result::<CommunitySurveyListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewCommunitySurveyListPosition {
            community_id: community.id,
            list_id:      self.id,
            position:     community.get_survey_lists_new_position(),
            types:        "a".to_string(),
        };
        diesel::insert_into(schema::community_survey_list_positions::table)
            .values(&new_pos)
            .get_result::<CommunitySurveyListPosition>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn remove_in_community_collections(&self, community: Community) -> bool {
        use crate::schema::community_survey_list_positions::dsl::community_survey_list_positions;
        use crate::schema::community_survey_list_collections::dsl::community_survey_list_collections;

        if self.get_communities_ids().iter().any(|&i| i==community.id) {
            return false;
        }
        let _connection = establish_connection();
        diesel::delete(community_survey_list_collections
            .filter(schema::community_survey_list_collections::community_id.eq(community.id))
            .filter(schema::community_survey_list_collections::survey_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(community_survey_list_positions
            .filter(schema::community_survey_list_positions::community_id.eq(community.id))
            .filter(schema::community_survey_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return true;
    }

    pub fn add_in_user_collections(&self, user: User) -> bool {
        use crate::models::NewUserSurveyListPosition;

        if !self.get_users_ids().iter().any(|&i| i==user.id) && self.user_id == user.id {
            return false;
        }
        let _connection = establish_connection();
        let new_item = NewUserSurveyListCollection {
            user_id: user.id,
            survey_list_id: self.id,
        };
        diesel::insert_into(schema::user_survey_list_collections::table)
            .values(&new_item)
            .get_result::<UserSurveyListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewUserSurveyListPosition {
            user_id:  user.id,
            list_id:  self.id,
            position: user.get_survey_lists_new_position(),
            types:    "a".to_string(),
        };
        diesel::insert_into(schema::user_survey_list_positions::table)
            .values(&new_pos)
            .get_result::<UserSurveyListPosition>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn remove_in_user_collections(&self, user: User) -> bool {
        use crate::schema::user_survey_list_collections::dsl::user_survey_list_collections;
        use crate::schema::user_survey_list_positions::dsl::user_survey_list_positions;

        if self.get_users_ids().iter().any(|&i| i==user.id) {
            return false;
        }
        let _connection = establish_connection();
        diesel::delete(user_survey_list_collections
            .filter(schema::user_survey_list_collections::user_id.eq(user.id))
            .filter(schema::user_survey_list_collections::survey_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(user_survey_list_positions
            .filter(schema::user_survey_list_positions::user_id.eq(user.id))
            .filter(schema::user_survey_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return true;
    }

    pub fn copy_item(pk: i32, user_or_communities: Vec<String>) -> bool {
        use crate::schema::survey_lists::dsl::survey_lists;
        use crate::schema::users::dsl::users;
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        let lists = survey_lists
            .filter(schema::survey_lists::id.eq(pk))
            .filter(schema::survey_lists::types.lt(10))
            .load::<SurveyList>(&_connection)
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
    pub fn get_surveys_ids(&self) -> Vec<i32> {
        use crate::schema::surveys::dsl::surveys;

        let _connection = establish_connection();
        let fix_list = surveys
            .filter(schema::surveys::survey_list_id.eq(self.id))
            .filter(schema::surveys::types.lt("b"))
            .load::<Survey>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in fix_list.iter() {
            stack.push(_item.id);
        };
        return stack;
    }
    pub fn get_user_lists(user_pk: i32) -> Vec<SurveyList> {
        use crate::schema::user_survey_list_collections::dsl::user_survey_list_collections;
        use crate::schema::user_survey_list_positions::dsl::user_survey_list_positions;
        use crate::schema::survey_lists::dsl::survey_lists;

        let _connection = establish_connection();
        let position_lists = user_survey_list_positions
            .filter(schema::user_survey_list_positions::user_id.eq(user_pk))
            .filter(schema::user_survey_list_positions::types.eq("a"))
            .load::<UserSurveyListPosition>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            let mut stack = Vec::new();
            for _item in position_lists.iter() {
                stack.push(_item.list_id);
            };
            return survey_lists
                .filter(schema::survey_lists::id.eq_any(stack))
                .filter(schema::survey_lists::types.lt(10))
                .load::<SurveyList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let user_lists = survey_lists
            .filter(schema::survey_lists::user_id.eq(user_pk))
            .filter(schema::survey_lists::types.lt(10))
            .load::<SurveyList>(&_connection)
            .expect("E.");
        for _item in user_lists.iter() {
            stack.push(_item.id);
        };
        let user_collections = user_survey_list_collections
            .filter(schema::user_survey_list_collections::user_id.eq(user_pk))
            .load::<UserSurveyListCollection>(&_connection)
            .expect("E.");
        for _item in user_collections.iter() {
            stack.push(_item.survey_list_id);
        };
        return survey_lists
            .filter(schema::survey_lists::id.eq_any(stack))
            .filter(schema::survey_lists::types.lt(10))
            .load::<SurveyList>(&_connection)
            .expect("E.");
    }
    pub fn get_community_lists(community_pk: i32) -> Vec<SurveyList> {
        use crate::schema::community_survey_list_collections::dsl::community_survey_list_collections;
        use crate::schema::community_survey_list_positions::dsl::community_survey_list_positions;
        use crate::schema::survey_lists::dsl::survey_lists;

        let _connection = establish_connection();
        let position_lists = community_survey_list_positions
            .filter(schema::community_survey_list_positions::community_id.eq(community_pk))
            .filter(schema::community_survey_list_positions::types.eq("a"))
            .load::<CommunitySurveyListPosition>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            let mut stack = Vec::new();
            for _item in position_lists.iter() {
                stack.push(_item.list_id);
            };
            return survey_lists
                .filter(schema::survey_lists::id.eq_any(stack))
                .filter(schema::survey_lists::types.lt(10))
                .load::<SurveyList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let community_lists = survey_lists
            .filter(schema::survey_lists::community_id.eq(community_pk))
            .filter(schema::survey_lists::types.lt(10))
            .load::<SurveyList>(&_connection)
            .expect("E.");
        for _item in community_lists.iter() {
            stack.push(_item.id);
        };
        let community_collections = community_survey_list_collections
            .filter(schema::community_survey_list_collections::community_id.eq(community_pk))
            .load::<CommunitySurveyListCollection>(&_connection)
            .expect("E.");
        for _item in community_collections.iter() {
            stack.push(_item.survey_list_id);
        };
        return survey_lists
            .filter(schema::survey_lists::id.eq_any(stack))
            .filter(schema::survey_lists::types.lt(10))
            .load::<SurveyList>(&_connection)
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
            .set(schema::survey_lists::types.eq(close_case))
            .get_result::<SurveyList>(&_connection)
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
            .set(schema::survey_lists::types.eq(close_case))
            .get_result::<SurveyList>(&_connection)
            .expect("E");
       return true;
    }

    pub fn delete_item(&self) -> bool {
        let _connection = establish_connection();
        if self.community_id.is_some() {
            use crate::schema::community_survey_list_positions::dsl::community_survey_list_positions;

            let list_positions = community_survey_list_positions
                .filter(schema::community_survey_list_positions::community_id.eq(self.community_id.unwrap()))
                .filter(schema::community_survey_list_positions::list_id.eq(self.id))
                .load::<CommunitySurveyListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::community_survey_list_positions::types.eq("b"))
                  .get_result::<CommunitySurveyListPosition>(&_connection)
                  .expect("Error.");
            }
        } else {
            use crate::schema::user_survey_list_positions::dsl::user_survey_list_positions;

            let list_positions = user_survey_list_positions
                .filter(schema::user_survey_list_positions::user_id.eq(self.user_id))
                .filter(schema::user_survey_list_positions::list_id.eq(self.id))
                .load::<UserSurveyListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::user_survey_list_positions::types.eq("b"))
                  .get_result::<UserSurveyListPosition>(&_connection)
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
            .set(schema::survey_lists::types.eq(close_case))
            .get_result::<SurveyList>(&_connection)
            .expect("E");
       return true;
    }
    pub fn restore_item(&self) -> bool {
        let _connection = establish_connection();
        if self.community_id.is_some() {
            use crate::schema::community_survey_list_positions::dsl::community_survey_list_positions;

            let list_positions = community_survey_list_positions
                .filter(schema::community_survey_list_positions::community_id.eq(self.community_id.unwrap()))
                .filter(schema::community_survey_list_positions::list_id.eq(self.id))
                .load::<CommunitySurveyListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::community_survey_list_positions::types.eq("b"))
                  .get_result::<CommunitySurveyListPosition>(&_connection)
                  .expect("Error.");
            }
        } else {
            use crate::schema::user_survey_list_positions::dsl::user_survey_list_positions;

            let list_positions = user_survey_list_positions
                .filter(schema::user_survey_list_positions::user_id.eq(self.user_id))
                .filter(schema::user_survey_list_positions::list_id.eq(self.id))
                .load::<UserSurveyListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::user_survey_list_positions::types.eq("a"))
                  .get_result::<UserSurveyListPosition>(&_connection)
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
            .set(schema::survey_lists::types.eq(close_case))
            .get_result::<SurveyList>(&_connection)
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
            .set(schema::survey_lists::types.eq(close_case))
            .get_result::<SurveyList>(&_connection)
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
            .set(schema::survey_lists::types.eq(close_case))
            .get_result::<SurveyList>(&_connection)
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
    pub fn create_survey(&self, title: String, community_id: Option<i32>, user_id: i32,
        image: Option<String>, is_anonymous: bool,
        is_multiple: bool, is_no_edited: bool, time_end: Option<String>) -> Survey {

        let _connection = establish_connection();
        diesel::update(&*self)
          .set(schema::survey_lists::count.eq(self.count + 1))
          .get_result::<SurveyList>(&_connection)
          .expect("Error.");

        let new_survey_form = NewSurvey {
            title: title,
            community_id: community_id,
            user_id: user_id,
            survey_list_id: self.id,
            types: "a".to_string(),
            image: image,
            is_anonymous: is_anonymous,
            is_multiple: is_multiple,
            is_no_edited: is_no_edited,
            time_end: Some(NaiveDateTime::parse_from_str(&time_end.unwrap().clone(), "%Y-%m-%d %H:%M:%S").unwrap()),
            created: chrono::Local::now().naive_utc(),
            view: 0,
            repost: 0,
            copy: 0,
            position: (self.count).try_into().unwrap(),
            vote: 0,
          };
          let new_survey = diesel::insert_into(schema::surveys::table)
              .values(&new_survey_form)
              .get_result::<Survey>(&_connection)
              .expect("Error.");

        if community_id.is_some() {
            let community = self.get_community();
            community.plus_surveys(1);
            return new_survey;
        }
        else {
            use crate::utils::get_user;

            let creator = get_user(user_id);
            creator.plus_surveys(1);
            return new_survey;
        }
    }
}
/////// Survey //////

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
#[belongs_to(SurveyList)]
pub struct Survey {
    pub id:              i32,
    pub title:           String,
    pub community_id:    Option<i32>,
    pub user_id:      i32,
    pub survey_list_id:         i32,
    pub types:           String,
    pub image:           Option<String>,
    pub is_anonymous:    bool,
    pub is_multiple:     bool,
    pub is_no_edited:    bool,
    pub time_end:        Option<chrono::NaiveDateTime>,
    pub created:         chrono::NaiveDateTime,

    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub vote:            i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="surveys"]
pub struct NewSurvey {
    pub title:           String,
    pub community_id:    Option<i32>,
    pub user_id:      i32,
    pub survey_list_id:         i32,
    pub types:           String,
    pub image:           Option<String>,
    pub is_anonymous:    bool,
    pub is_multiple:     bool,
    pub is_no_edited:    bool,
    pub time_end:        Option<chrono::NaiveDateTime>,
    pub created:         chrono::NaiveDateTime,

    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub vote:            i32,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="surveys"]
pub struct EditSurvey {
    pub title:           String,
    pub image:           Option<String>,
    pub is_anonymous:    bool,
    pub is_multiple:     bool,
    pub is_no_edited:    bool,
    pub time_end:        Option<chrono::NaiveDateTime>,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="surveys"]
pub struct EditSurveyPosition {
    pub position:  i16,
}

impl Survey {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_survey(&self) -> bool {
        return true;
    }
    pub fn is_have_votes(&self) -> bool {
        return self.vote > 0;
    }
    pub fn is_full_answers(&self) -> bool {
        return self.get_answers().len() > 9;
    }
    pub fn is_user_voted(&self, user_id: i32) -> bool {
        use crate::models::survey::survey_votes::dsl::survey_votes;
        let _connection = establish_connection();
        return !self.is_time_end() && survey_votes
            .filter(schema::survey_votes::user_id.eq(user_id))
            .filter(schema::survey_votes::survey_id.eq(self.id))
            .load::<SurveyVote>(&_connection)
            .expect("E.")
            .len() > 0;
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
        use crate::schema::survey_reposts::dsl::survey_reposts;

        let _connection = establish_connection();

        let count = survey_reposts
            .filter(schema::survey_reposts::survey_id.eq(self.id))
            .filter(schema::survey_reposts::message_id.is_not_null())
            .load::<SurveyRepost>(&_connection)
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
        use crate::schema::survey_reposts::dsl::survey_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = survey_reposts
            .filter(schema::survey_reposts::survey_id.eq(self.id))
            .filter(schema::survey_reposts::post_id.is_not_null())
            .limit(limit)
            .offset(offset)
            .load::<SurveyRepost>(&_connection)
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
        use crate::schema::survey_reposts::dsl::survey_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = survey_reposts
            .filter(schema::survey_reposts::survey_id.eq(self.id))
            .filter(schema::survey_reposts::post_id.is_not_null())
            .limit(6)
            .load::<SurveyRepost>(&_connection)
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

    pub fn get_time_description(&self) -> String {
        if self.time_end.is_some() {
            return "<p class='small'>Время опроса вышло.</p>".to_string()
        }
        else {
            return "<p class='small'>Бессрочный опрос.</p>".to_string()
        }
    }
    pub fn is_time_end(&self) -> bool {
        return self.time_end.is_some() && self.time_end.unwrap() > chrono::Local::now().naive_utc();
    }
    pub fn get_answers(&self) -> Vec<SurveyAnswer> {
        use crate::schema::survey_answers::dsl::survey_answers;
        let _connection = establish_connection();
        return survey_answers
            .filter(schema::survey_answers::survey_id.eq(self.id))
            .load::<SurveyAnswer>(&_connection)
            .expect("E.");
    }
    pub fn get_users_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.vote,
            " участник".to_string(),
            " участника".to_string(),
            " участников".to_string(),
        );
    }
    pub fn get_6_users(&self) -> String {
        use crate::schema::survey_votes::dsl::survey_votes;
        use crate::utils::get_users_from_ids;
        let _connection = establish_connection();
        let surveys = survey_votes
            .filter(schema::survey_votes::survey_id.eq(self.id))
            .load::<SurveyVote>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in surveys.iter() {
            stack.push(_item.user_id);
        };
        let users = get_users_from_ids(stack);
        let mut list = "<hr class='m-1'>".to_string();
        for voter in users.iter() {
            list = list + &"<a class='pr-1' href='".to_string() + &voter.link.to_string() + &"' target='_blank' tooltip='".to_string() + &voter.get_full_name() + &"' flow='up'>".to_string() + &voter.get_s_avatar() + &"</a>".to_string();
        }
        return list;
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
            .set(schema::surveys::types.eq(close_case))
            .get_result::<Survey>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::survey_lists::count.eq(list.count - 1))
            .get_result::<SurveyList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.minus_surveys(1);
        }
        else {
            let creator = self.get_creator();
            creator.minus_surveys(1);
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
            .set(schema::surveys::types.eq(close_case))
            .get_result::<Survey>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::survey_lists::count.eq(list.count + 1))
            .get_result::<SurveyList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.plus_surveys(1);
        }
        else {
            let creator = self.get_creator();
            creator.plus_surveys(1);
         }
       return true;
    }

    pub fn get_code(&self) -> String {
        return "sur".to_string() + &self.get_str_id();
    }
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(54))
            .load::<ModeratedPenaltie>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
        return penaltie.expiration.unwrap().format("%d/%m/%Y").to_string();
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
    pub fn get_moderated_description(&self) -> String {
        use crate::schema::moderateds::dsl::moderateds;
        use crate::models::Moderated;

        let _connection = establish_connection();

        let moder = moderateds
            .filter(schema::moderateds::object_id.eq(self.id))
            .filter(schema::moderateds::types.eq(54))
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
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/no_img/list.jpg".to_string();
        }
    }

    pub fn get_list(&self) -> SurveyList {
        use crate::schema::survey_lists::dsl::survey_lists;

        let _connection = establish_connection();
        return survey_lists
            .filter(schema::survey_lists::id.eq(self.survey_list_id))
            .filter(schema::survey_lists::types.lt(10))
            .load::<SurveyList>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_description(&self) -> String {
        if self.community_id.is_some() {
            let community = self.get_community();
            return "опрос сообщества <a href='".to_owned() + &community.link.to_string() + &"' target='_blank'>" + &community.name + &"</a>"
        }
        else {
            let creator = self.get_creator();
            return "<a href='".to_owned() + &creator.link.to_string() + &"' target='_blank'>" + &creator.get_full_name() + &"</a>" + &": опрос"
        }
    }

    pub fn edit_survey(&self, title: String,
        image: Option<String>, is_anonymous: bool, is_multiple: bool,
        is_no_edited: bool, time_end:Option<String>) -> &Survey {
        let _connection = establish_connection();

        let edit_survey = EditSurvey {
            title: title,
            image: image,
            is_anonymous: is_anonymous,
            is_multiple: is_multiple,
            is_no_edited: is_no_edited,
            time_end: Some(NaiveDateTime::parse_from_str(&time_end.unwrap().clone(), "%Y-%m-%d %H:%M:%S").unwrap()),
        };
        diesel::update(self)
            .set(edit_survey)
            .get_result::<Survey>(&_connection)
            .expect("Error.");
        return self;
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

    pub fn close_item(&self) -> bool {
        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "a" => "h",
            "b" => "n",
            _ => "h",
        };
        diesel::update(self)
            .set(schema::surveys::types.eq(close_case))
            .get_result::<Survey>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::survey_lists::count.eq(list.count - 1))
            .get_result::<SurveyList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.minus_surveys(1);
        }
        else {
            let creator = self.get_creator();
            creator.minus_surveys(1);
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
            .set(schema::surveys::types.eq(close_case))
            .get_result::<Survey>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::survey_lists::count.eq(list.count + 1))
            .get_result::<SurveyList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.plus_surveys(1);
        }
        else {
            let creator = self.get_creator();
            creator.plus_surveys(1);
         }
       return true;
    }
    pub fn count_reposts(&self) -> String {
        if self.repost > 0 {
            return self.repost.to_string()
        }
        else {
            return "".to_string()
        }
    }

    pub fn change_position(query: Json<Vec<JsonPosition>>) -> bool {
        use crate::schema::surveys::dsl::surveys;

        let _connection = establish_connection();
        for i in query.iter() {
            let item = surveys
                .filter(schema::surveys::id.eq(i.key))
                .filter(schema::surveys::types.eq("a"))
                .limit(1)
                .load::<Survey>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            diesel::update(&item)
                .set(schema::surveys::position.eq(i.value))
                .get_result::<Survey>(&_connection)
                .expect("Error.");
        }
        return true;
    }
    pub fn is_can_edit(&self) -> bool {
        use chrono::Duration;
        return (self.created + Duration::hours(3)) > chrono::Local::now().naive_utc();
    }
}

/////// UserSurveyListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(SurveyList)]
pub struct UserSurveyListCollection {
    pub id:       i32,
    pub user_id:  i32,
    pub survey_list_id:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_survey_list_collections"]
pub struct NewUserSurveyListCollection {
    pub user_id:  i32,
    pub survey_list_id:  i32,
}

/////// CommunitySurveyListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(SurveyList)]
pub struct CommunitySurveyListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub survey_list_id:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_survey_list_collections"]
pub struct NewCommunitySurveyListCollection {
    pub community_id:  i32,
    pub survey_list_id:       i32,
}

/////// SurveyListPerm //////
    // 'a' Активно
    // 'b' Не активно
    // 'c' Нет значения

#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(SurveyList)]
pub struct SurveyListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub survey_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub create_item:     Option<String>,
    pub can_copy:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="survey_list_perms"]
pub struct NewSurveyListPerm {
    pub user_id:         i32,
    pub survey_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub create_item:     Option<String>,
    pub can_copy:        Option<String>,
}


#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Survey)]
pub struct SurveyAnswer {
    pub id:          i32,
    pub content:     String,
    pub survey_id:   i32,
    pub vote:        i32,
    pub position:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="survey_answers"]
pub struct NewSurveyAnswer {
    pub content:     String,
    pub survey_id:   i32,
    pub vote:        i32,
    pub position:       i32,
}
impl SurveyAnswer {
    pub fn get_survey(&self) -> Survey {
        use crate::models::survey::surveys::dsl::surveys;

        let _connection = establish_connection();
        return surveys
            .filter(schema::surveys::id.eq(self.survey_id))
            .load::<Survey>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
        }

    pub fn get_procent(&self) -> i32 {
        return self.vote / self.get_survey().vote * 100;
    }
    pub fn is_user_voted(&self, user_id: i32) -> bool {
        use crate::models::survey::survey_votes::dsl::survey_votes;
        let _connection = establish_connection();
        return survey_votes
            .filter(schema::survey_votes::user_id.eq(user_id))
            .filter(schema::survey_votes::survey_answer_id.eq(self.id))
            .load::<SurveyVote>(&_connection)
            .expect("E.")
            .len() > 0;
    }
}

#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(SurveyAnswer)]
pub struct SurveyVote {
    pub id:               i32,
    pub user_id:          i32,
    pub survey_answer_id: i32,
    pub survey_id:        i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="survey_votes"]
pub struct NewSurveyVote  {
    pub user_id:          i32,
    pub survey_answer_id: i32,
    pub survey_id:        i32,
}

impl SurveyVote {
    pub fn is_user_voted(&self, user_id: i32) -> bool {
        use crate::models::survey::survey_votes::dsl::survey_votes;
        let _connection = establish_connection();
        return survey_votes
            .filter(schema::survey_votes::user_id.eq(user_id))
            .filter(schema::survey_votes::survey_answer_id.eq(self.id))
            .load::<SurveyVote>(&_connection)
            .expect("E.")
            .len() > 0;
    }
}

/////// SurveyListRepost //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(SurveyList)]
#[belongs_to(Post)]
#[belongs_to(Message)]
pub struct SurveyListRepost {
    pub id:            i32,
    pub survey_list_id: i32,
    pub post_id:       Option<i32>,
    pub message_id:    Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="survey_list_reposts"]
pub struct NewSurveyListRepost {
    pub survey_list_id: i32,
    pub post_id:       Option<i32>,
    pub message_id:    Option<i32>,
}

/////// SurveyRepost //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Survey)]
#[belongs_to(Post)]
#[belongs_to(Message)]
pub struct SurveyRepost {
    pub id:         i32,
    pub survey_id:   i32,
    pub post_id:    Option<i32>,
    pub message_id: Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="survey_reposts"]
pub struct NewSurveyRepost {
    pub survey_id:  i32,
    pub post_id:    Option<i32>,
    pub message_id: Option<i32>,
}
