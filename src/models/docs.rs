use crate::schema;
use diesel::prelude::*;
use crate::schema::{
    doc_lists,
    docs,
    user_doc_list_collections,
    community_doc_list_collections,
    doc_list_perms,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
    UserDocListPosition,
    CommunityDocListPosition,
};

/////// DocList //////

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

/////// DocList //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(User)]
pub struct DocList {
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
    pub create_el:       String,
    pub copy_el:         String,
}
#[derive(Deserialize, Insertable)]
#[table_name="doc_lists"]
pub struct NewDocList {
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
    pub create_el:       String,
    pub copy_el:         String,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="doc_lists"]
pub struct EditDocList {
    pub name:            String,
    pub description:     Option<String>,
    pub can_see_el:      String,
    pub create_el:       String,
    pub copy_el:         String,
}

impl DocList {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_doc_list(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "ldo".to_string() + &self.get_str_id();
    }
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(22))
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
            .filter(schema::moderateds::types.eq(22))
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
        return "<a data-doclist='".to_string() + &self.get_str_id() + &"' class='ajax'>".to_string() + &self.name + &"</a>".to_string();
    }
    pub fn is_user_list(&self, user: User) -> bool {
        return self.user_id == user.id;
    }
    pub fn is_community_list(&self, community: Community) -> bool {
        return self.community_id.unwrap() == community.id;
    }
    pub fn get_users_ids(&self) -> Vec<i32> {
        use crate::schema::user_doc_list_collections::dsl::user_doc_list_collections;

        let _connection = establish_connection();
        let ids = user_doc_list_collections
            .filter(schema::user_doc_list_collections::doc_list_id.eq(self.id))
            .load::<UserDocListCollection>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in ids.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_communities_ids(&self) -> Vec<i32> {
        use crate::schema::community_doc_list_collections::dsl::community_doc_list_collections;

        let _connection = establish_connection();
        let ids = community_doc_list_collections
            .filter(schema::community_doc_list_collections::doc_list_id.eq(self.id))
            .load::<CommunityDocListCollection>(&_connection)
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
    pub fn get_items(&self) -> Vec<Doc> {
        use crate::schema::docs::dsl::docs;

        let _connection = establish_connection();
        return docs
            .filter(schema::docs::doc_list_id.eq(self.id))
            .filter(schema::docs::types.eq("a"))
            .order(schema::docs::created.desc())
            .load::<Doc>(&_connection)
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
        use crate::schema::doc_list_perms::dsl::doc_list_perms;

        let _connection = establish_connection();
        let items = doc_list_perms
            .filter(schema::doc_list_perms::doc_list_id.eq(self.id))
            .filter(schema::doc_list_perms::can_see_item.eq("b"))
            .load::<DocListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::doc_list_perms::dsl::doc_list_perms;

        let _connection = establish_connection();
        let items = doc_list_perms
            .filter(schema::doc_list_perms::doc_list_id.eq(self.id))
            .filter(schema::doc_list_perms::can_see_item.eq("a"))
            .load::<DocListPerm>(&_connection)
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
        use crate::schema::doc_list_perms::dsl::doc_list_perms;

        let _connection = establish_connection();
        let items = doc_list_perms
            .filter(schema::doc_list_perms::doc_list_id.eq(self.id))
            .filter(schema::doc_list_perms::create_item.eq("b"))
            .load::<DocListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_create_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::doc_list_perms::dsl::doc_list_perms;

        let _connection = establish_connection();
        let items = doc_list_perms
            .filter(schema::doc_list_perms::doc_list_id.eq(self.id))
            .filter(schema::doc_list_perms::create_item.eq("a"))
            .load::<DocListPerm>(&_connection)
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
        use crate::schema::doc_list_perms::dsl::doc_list_perms;

        let _connection = establish_connection();
        let items = doc_list_perms
            .filter(schema::doc_list_perms::doc_list_id.eq(self.id))
            .filter(schema::doc_list_perms::can_copy.eq("b"))
            .load::<DocListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_copy_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::doc_list_perms::dsl::doc_list_perms;

        let _connection = establish_connection();
        let items = doc_list_perms
            .filter(schema::doc_list_perms::doc_list_id.eq(self.id))
            .filter(schema::doc_list_perms::can_copy.eq("a"))
            .load::<DocListPerm>(&_connection)
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
    pub fn is_anon_user_can_create_item(&self) -> bool {
        return self.create_el == "a".to_string()
    }
    pub fn is_anon_user_can_copy_el(&self) -> bool {
        return self.copy_el == "a".to_string()
    }
    pub fn create_list(creator: User, name: String, description: Option<String>,
        community_id: Option<i32>, can_see_el: String, create_el: String, copy_el: String,
        can_see_el_users: Option<Vec<i32>>, create_el_users: Option<Vec<i32>>,
        copy_el_users: Option<Vec<i32>>) -> i32 {

        use crate::models::{
            CommunityDocListPosition,NewCommunityDocListPosition,
            UserDocListPosition,NewUserDocListPosition,
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

            let new_list = NewDocList{
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
                create_el: create_el.clone(),
                copy_el: copy_el.clone(),
            };
            let new_list = diesel::insert_into(schema::doc_lists::table)
                .values(&new_list)
                .get_result::<DocList>(&_connection)
                .expect("Error.");
            new_id = new_list.id;

            let _new_list_position = NewCommunityDocListPosition {
                community_id: community.id,
                list_id:      new_id,
                position:     community.get_doc_lists_new_position(),
                types:        "a".to_string(),
            };
            let _list_position = diesel::insert_into(schema::community_doc_list_positions::table)
                .values(&_new_list_position)
                .get_result::<CommunityDocListPosition>(&_connection)
                .expect("Error saving doc_list_position.");
        }
        else {
            let new_list_form = NewDocList{
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
                create_el: create_el.clone(),
                copy_el: copy_el.clone(),
            };
            let new_list = diesel::insert_into(schema::doc_lists::table)
                .values(&new_list_form)
                .get_result::<DocList>(&_connection)
                .expect("Error.");
            new_id = new_list.id;

            let _new_list_position = NewUserDocListPosition {
                user_id:  creator.id,
                list_id:  new_id,
                position: creator.get_doc_lists_new_position(),
                types:    "a".to_string(),
            };
            let _list_position = diesel::insert_into(schema::user_doc_list_positions::table)
                .values(&_new_list_position)
                .get_result::<UserDocListPosition>(&_connection)
                .expect("Error saving doc_list_position.");
        }

        if can_see_el == "d".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewDocListPerm {
                        user_id:      user_id,
                        doc_list_id:  new_id,
                        can_see_item: Some("b".to_string()),
                        create_item: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::doc_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<DocListPerm>(&_connection)
                        .expect("Error saving doc_list_position.");
                }
            }
        }
        else if can_see_el == "e".to_string() && can_see_el == "j".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewDocListPerm {
                        user_id:      user_id,
                        doc_list_id: new_id,
                        can_see_item: Some("a".to_string()),
                        create_item: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::doc_list_perms::table)
                        .values(&_new_include)
                        .get_result::<DocListPerm>(&_connection)
                        .expect("Error saving doc_list_position.");
                }
            }
        }

        if create_el == "d".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewDocListPerm {
                        user_id:      user_id,
                        doc_list_id: new_id,
                        can_see_item: None,
                        create_item: Some("b".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::doc_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<DocListPerm>(&_connection)
                        .expect("Error saving doc_list_position.");
                }
            }
        }
        else if create_el == "e".to_string() && create_el == "j".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewDocListPerm {
                        user_id:      user_id,
                        doc_list_id: new_id,
                        can_see_item: None,
                        create_item: Some("a".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::doc_list_perms::table)
                        .values(&_new_include)
                        .get_result::<DocListPerm>(&_connection)
                        .expect("Error saving doc_list_position.");
                }
            }
        }

        if copy_el == "d".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewDocListPerm {
                        user_id:      user_id,
                        doc_list_id: new_id,
                        can_see_item: None,
                        create_item: None,
                        can_copy: Some("b".to_string()),
                    };
                    diesel::insert_into(schema::doc_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<DocListPerm>(&_connection)
                        .expect("Error saving doc_list_position.");
                }
            }
        }
        else if copy_el == "e".to_string() && copy_el == "j".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewDocListPerm {
                        user_id:      user_id,
                        doc_list_id: new_id,
                        can_see_item: None,
                        create_item: None,
                        can_copy: Some("a".to_string()),
                    };
                    diesel::insert_into(schema::doc_list_perms::table)
                        .values(&_new_include)
                        .get_result::<DocListPerm>(&_connection)
                        .expect("Error saving doc_list_position.");
                }
            }
        }
        return new_id;
    }
    pub fn edit_list(&self, name: String, description: Option<String>,
        can_see_el: String, create_el: String, copy_el: String,
        can_see_el_users: Option<Vec<i32>>, create_el_users: Option<Vec<i32>>,
        copy_el_users: Option<Vec<i32>>) -> &DocList {

        use crate::schema::doc_list_perms::dsl::doc_list_perms;

        let _connection = establish_connection();

            let edit_doc_list = EditDocList{
                name: name,
                description: description,
                can_see_el: can_see_el.clone(),
                create_el: create_el.clone(),
                copy_el: copy_el.clone(),
            };
            diesel::update(self)
                .set(edit_doc_list)
                .get_result::<DocList>(&_connection)
                .expect("Error.");

        if can_see_el == "d".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                diesel::delete (
                  doc_list_perms
                    .filter(schema::doc_list_perms::doc_list_id.eq(self.id))
                    .filter(schema::doc_list_perms::can_see_item.is_not_null())
                )
                  .execute(&_connection)
                  .expect("E");
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewDocListPerm {
                        user_id:      user_id,
                        doc_list_id: self.id,
                        can_see_item: Some("b".to_string()),
                        create_item: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::doc_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<DocListPerm>(&_connection)
                        .expect("Error saving doc_list_position.");
                }
            }
        }
        else if can_see_el == "e".to_string() && can_see_el == "j".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewDocListPerm {
                        user_id:      user_id,
                        doc_list_id: self.id,
                        can_see_item: Some("a".to_string()),
                        create_item: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::doc_list_perms::table)
                        .values(&_new_include)
                        .get_result::<DocListPerm>(&_connection)
                        .expect("Error saving doc_list_position.");
                }
            }
        }

        if create_el == "d".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewDocListPerm {
                        user_id:      user_id,
                        doc_list_id: self.id,
                        can_see_item: None,
                        create_item: Some("b".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::doc_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<DocListPerm>(&_connection)
                        .expect("Error saving doc_list_position.");
                }
            }
        }
        else if create_el == "e".to_string() && create_el == "j".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewDocListPerm {
                        user_id:      user_id,
                        doc_list_id: self.id,
                        can_see_item: None,
                        create_item: Some("a".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::doc_list_perms::table)
                        .values(&_new_include)
                        .get_result::<DocListPerm>(&_connection)
                        .expect("Error saving doc_list_position.");
                }
            }
        }

        if copy_el == "d".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewDocListPerm {
                        user_id:      user_id,
                        doc_list_id: self.id,
                        can_see_item: None,
                        create_item: None,
                        can_copy: Some("b".to_string()),
                    };
                    diesel::insert_into(schema::doc_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<DocListPerm>(&_connection)
                        .expect("Error saving doc_list_position.");
                }
            }
        }
        else if copy_el == "e".to_string() && copy_el == "j".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewDocListPerm {
                        user_id:      user_id,
                        doc_list_id: self.id,
                        can_see_item: None,
                        create_item: None,
                        can_copy: Some("a".to_string()),
                    };
                    diesel::insert_into(schema::doc_list_perms::table)
                        .values(&_new_include)
                        .get_result::<DocListPerm>(&_connection)
                        .expect("Error saving doc_list_position.");
                }
            }
        }
        return self;
    }
    pub fn get_order(&self) -> UserDocListPosition {
        use crate::schema::user_doc_list_positions::dsl::user_doc_list_positions;
        use crate::models::UserDocListPosition;

        let _connection = establish_connection();
        return user_doc_list_positions
            .filter(schema::user_doc_list_positions::list_id.eq(self.id))
            .filter(schema::user_doc_list_positions::types.eq("a"))
            .load::<UserDocListPosition>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn add_in_community_collections(&self, community: Community) -> bool {
        use crate::models::NewCommunityDocListPosition;

        if !self.get_communities_ids().iter().any(|&i| i==community.id) && self.community_id.is_some() && self.community_id.unwrap() == community.id {
            return false;
        }
        let _connection = establish_connection();
        let new_item = NewCommunityDocListCollection {
            community_id: community.id,
            doc_list_id: self.id,
        };
        diesel::insert_into(schema::community_doc_list_collections::table)
            .values(&new_item)
            .get_result::<CommunityDocListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewCommunityDocListPosition {
            community_id: community.id,
            list_id:      self.id,
            position:     community.get_doc_lists_new_position(),
            types:        "a".to_string(),
        };
        diesel::insert_into(schema::community_doc_list_positions::table)
            .values(&new_pos)
            .get_result::<CommunityDocListPosition>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn remove_in_community_collections(&self, community: Community) -> bool {
        use crate::models::communities::community_doc_list_positions::dsl::community_doc_list_positions;
        use crate::models::docs::community_doc_list_collections::dsl::community_doc_list_collections;

        if self.get_communities_ids().iter().any(|&i| i==community.id) {
            return false;
        }
        let _connection = establish_connection();
        diesel::delete(community_doc_list_collections
            .filter(schema::community_doc_list_collections::community_id.eq(community.id))
            .filter(schema::community_doc_list_collections::doc_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(community_doc_list_positions
            .filter(schema::community_doc_list_positions::community_id.eq(community.id))
            .filter(schema::community_doc_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return true;
    }

    pub fn add_in_user_collections(&self, user: User) -> bool {
        use crate::models::NewUserDocListPosition;

        if !self.get_users_ids().iter().any(|&i| i==user.id) && self.user_id == user.id {
            return false;
        }
        let _connection = establish_connection();
        let new_item = NewUserDocListCollection {
            user_id: user.id,
            doc_list_id: self.id,
        };
        diesel::insert_into(schema::user_doc_list_collections::table)
            .values(&new_item)
            .get_result::<UserDocListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewUserDocListPosition {
            user_id:  user.id,
            list_id:  self.id,
            position: user.get_doc_lists_new_position(),
            types:    "a".to_string(),
        };
        diesel::insert_into(schema::user_doc_list_positions::table)
            .values(&new_pos)
            .get_result::<UserDocListPosition>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn remove_in_user_collections(&self, user: User) -> bool {
        use crate::schema::user_doc_list_collections::dsl::user_doc_list_collections;
        use crate::schema::user_doc_list_positions::dsl::user_doc_list_positions;

        if self.get_users_ids().iter().any(|&i| i==user.id) {
            return false;
        }
        let _connection = establish_connection();
        diesel::delete(user_doc_list_collections
            .filter(schema::user_doc_list_collections::user_id.eq(user.id))
            .filter(schema::user_doc_list_collections::doc_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(user_doc_list_positions
            .filter(schema::user_doc_list_positions::user_id.eq(user.id))
            .filter(schema::user_doc_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return true;
    }

    pub fn copy_item(pk: i32, user_or_communities: Vec<String>) -> bool {
        use crate::schema::doc_lists::dsl::doc_lists;
        use crate::schema::users::dsl::users;
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        let lists = doc_lists
            .filter(schema::doc_lists::id.eq(pk))
            .filter(schema::doc_lists::types.lt(10))
            .load::<DocList>(&_connection)
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
    pub fn get_docs_ids(&self) -> Vec<i32> {
        use crate::schema::docs::dsl::docs;

        let _connection = establish_connection();
        let fix_list = docs
            .filter(schema::docs::doc_list_id.eq(self.id))
            .filter(schema::docs::types.lt("b"))
            .load::<Doc>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in fix_list.iter() {
            stack.push(_item.id);
        };
        return stack;
    }
    pub fn get_user_lists(user_pk: i32) -> Vec<DocList> {
        use crate::schema::user_doc_list_collections::dsl::user_doc_list_collections;
        use crate::schema::user_doc_list_positions::dsl::user_doc_list_positions;
        use crate::schema::doc_lists::dsl::doc_lists;
        use crate::models::UserDocListPosition;

        let _connection = establish_connection();
        let position_lists = user_doc_list_positions
            .filter(schema::user_doc_list_positions::user_id.eq(user_pk))
            .filter(schema::user_doc_list_positions::types.eq("a"))
            .load::<UserDocListPosition>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            let mut stack = Vec::new();
            for _item in position_lists.iter() {
                stack.push(_item.list_id);
            };
            return doc_lists
                .filter(schema::doc_lists::id.eq_any(stack))
                .filter(schema::doc_lists::types.lt(10))
                .load::<DocList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let user_lists = doc_lists
            .filter(schema::doc_lists::user_id.eq(user_pk))
            .filter(schema::doc_lists::types.lt(10))
            .load::<DocList>(&_connection)
            .expect("E.");
        for _item in user_lists.iter() {
            stack.push(_item.id);
        };
        let user_collections = user_doc_list_collections
            .filter(schema::user_doc_list_collections::user_id.eq(user_pk))
            .load::<UserDocListCollection>(&_connection)
            .expect("E.");
        for _item in user_collections.iter() {
            stack.push(_item.doc_list_id);
        };
        return doc_lists
            .filter(schema::doc_lists::id.eq_any(stack))
            .filter(schema::doc_lists::types.lt(10))
            .load::<DocList>(&_connection)
            .expect("E.");
    }
    pub fn get_community_lists(community_pk: i32) -> Vec<DocList> {
        use crate::schema::community_doc_list_collections::dsl::community_doc_list_collections;
        use crate::schema::community_doc_list_positions::dsl::community_doc_list_positions;
        use crate::schema::doc_lists::dsl::doc_lists;
        use crate::models::CommunityDocListPosition;

        let _connection = establish_connection();
        let position_lists = community_doc_list_positions
            .filter(schema::community_doc_list_positions::community_id.eq(community_pk))
            .filter(schema::community_doc_list_positions::types.eq("a"))
            .load::<CommunityDocListPosition>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            let mut stack = Vec::new();
            for _item in position_lists.iter() {
                stack.push(_item.list_id);
            };
            return doc_lists
                .filter(schema::doc_lists::id.eq_any(stack))
                .filter(schema::doc_lists::types.lt(10))
                .load::<DocList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let community_lists = doc_lists
            .filter(schema::doc_lists::community_id.eq(community_pk))
            .filter(schema::doc_lists::types.lt(10))
            .load::<DocList>(&_connection)
            .expect("E.");
        for _item in community_lists.iter() {
            stack.push(_item.id);
        };
        let community_collections = community_doc_list_collections
            .filter(schema::community_doc_list_collections::community_id.eq(community_pk))
            .load::<CommunityDocListCollection>(&_connection)
            .expect("E.");
        for _item in community_collections.iter() {
            stack.push(_item.doc_list_id);
        };
        return doc_lists
            .filter(schema::doc_lists::id.eq_any(stack))
            .filter(schema::doc_lists::types.lt(10))
            .load::<DocList>(&_connection)
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
            .set(schema::doc_lists::types.eq(close_case))
            .get_result::<DocList>(&_connection)
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
            .set(schema::doc_lists::types.eq(close_case))
            .get_result::<DocList>(&_connection)
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
            .set(schema::doc_lists::types.eq(close_case))
            .get_result::<DocList>(&_connection)
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
            .set(schema::doc_lists::types.eq(close_case))
            .get_result::<DocList>(&_connection)
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
            .set(schema::doc_lists::types.eq(close_case))
            .get_result::<DocList>(&_connection)
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
            .set(schema::doc_lists::types.eq(close_case))
            .get_result::<DocList>(&_connection)
            .expect("E");
       return true;
    }

}

/////// Doc //////

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
#[belongs_to(DocList)]
pub struct Doc {
    pub id:              i32,
    pub title:           String,
    pub community_id:    Option<i32>,
    pub user_id:      i32,
    pub doc_list_id:         i32,
    pub types:           String,
    pub types_2:         String,
    pub file:            String,
    pub created:         chrono::NaiveDateTime,

    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="docs"]
pub struct NewDoc {
    pub title:           String,
    pub community_id:    Option<i32>,
    pub user_id:      i32,
    pub doc_list_id:     i32,
    pub types:           String,
    pub types_2:         String,
    pub file:            String,
    pub created:         chrono::NaiveDateTime,

    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
}


/////// UserDocListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(DocList)]
pub struct UserDocListCollection {
    pub id:       i32,
    pub user_id:  i32,
    pub doc_list_id:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_doc_list_collections"]
pub struct NewUserDocListCollection {
    pub user_id:  i32,
    pub doc_list_id:  i32,
}

/////// CommunityDocListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(DocList)]
pub struct CommunityDocListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub doc_list_id:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_doc_list_collections"]
pub struct NewCommunityDocListCollection {
    pub community_id:  i32,
    pub doc_list_id:       i32,
}

/////// DocListPerm //////
    // 'a' Активно
    // 'b' Не активно
    // 'c' Нет значения

#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(DocList)]
pub struct DocListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub doc_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub create_item:     Option<String>,
    pub can_copy:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="doc_list_perms"]
pub struct NewDocListPerm {
    pub user_id:         i32,
    pub doc_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub create_item:     Option<String>,
    pub can_copy:        Option<String>,
}
