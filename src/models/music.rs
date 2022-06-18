use diesel::prelude::*;
use crate::schema;
use crate::schema::{
    sound_genres,
    artists,
    music_albums,
    music_lists,
    musics,
    user_music_list_collections,
    community_music_list_collections,
    music_list_perms,
    music_list_reposts,
    music_reposts,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::{establish_connection, JsonPosition};
use crate::models::{
    User,
    Community,
    UserMusicListPosition,
    CommunityMusicListPosition,
    Post,
    Message,
};
use actix_web::web::Json;

/////// SoundGenres //////

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct SoundGenre {
    pub id:       i32,
    pub name:     String,
    pub count:    i32,
    pub copy:     i32,
}

impl SoundGenre {
    pub fn create_genre(name: String) -> SoundGenre {
        let _connection = establish_connection();
        let new_form = NewSoundGenre {
            name: name,
            count: 0,
            copy: 0,
        };
        let new_genre = diesel::insert_into(schema::sound_genres::table)
            .values(&new_form)
            .get_result::<SoundGenre>(&_connection)
            .expect("Error.");
        return new_genre;
    }
    pub fn edit_genre(&self, name: String) -> &SoundGenre {
        let _connection = establish_connection();
        let new_form = NewSoundGenre {
            name: name,
            count: 0,
            copy: 0,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<SoundGenre>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="sound_genres"]
pub struct NewSoundGenre {
    pub name:     String,
    pub count:    i32,
    pub copy:     i32,
}

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


/////// Artist //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Artist {
    pub id:           i32,
    pub name:         String,
    pub description:  Option<String>,
    pub image:        Option<String>,
    pub created:      chrono::NaiveDateTime,
    pub count:        i32,
    pub repost:       i32,
    pub copy:         i32,
    pub position:     i16,
    pub can_see_el:   String,
}

impl Artist {
    pub fn create_artist(name: String, description:Option<String>,
        image:Option<String>, position:i16) -> Artist {
        let _connection = establish_connection();
        let new_form = NewArtist {
            name: name,
            description: description,
            image: image,
            created: chrono::Local::now().naive_utc(),
            count: 0,
            repost: 0,
            copy: 0,
            position: position,
            can_see_el: "a".to_string(),
        };
        let new_artist = diesel::insert_into(schema::artists::table)
            .values(&new_form)
            .get_result::<Artist>(&_connection)
            .expect("Error.");
        return new_artist;
    }
    pub fn edit_artist(&self, name: String, description:Option<String>,
        image:Option<String>, position:i16) -> &Artist {
        let _connection = establish_connection();
        let new_form = NewArtist {
            name: name,
            description: description,
            image: image,
            created: chrono::Local::now().naive_utc(),
            count: self.count,
            repost: self.repost,
            copy: self.copy,
            position: position,
            can_see_el: self.can_see_el.clone(),
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<Artist>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="artists"]
pub struct NewArtist {
    pub name:         String,
    pub description:  Option<String>,
    pub image:        Option<String>,
    pub created:      chrono::NaiveDateTime,
    pub count:        i32,
    pub repost:       i32,
    pub copy:         i32,
    pub position:     i16,
    pub can_see_el:   String,
}

/////// MusicAlbum //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Artist)]
#[belongs_to(User)]
pub struct MusicAlbum {
    pub id:          i32,
    pub name:        String,
    pub artist_id:   Option<i32>,
    pub user_id:     i32,
    pub description: Option<String>,
    pub image:       Option<String>,
    pub created:     chrono::NaiveDateTime,

    pub count:       i32,
    pub repost:      i32,
    pub copy:        i32,
    pub position:    i16,

    pub can_see_el:  String,
    pub create_el:   String,
    pub copy_el:     String,
}

impl MusicAlbum {
    pub fn create_album(name: String, artist_id:Option<i32>,
        user_id: i32, description:Option<String>,
        image:Option<String>, position:i16) -> MusicAlbum {
        let _connection = establish_connection();
        let new_form = NewMusicAlbum {
            name: name,
            artist_id: artist_id,
            user_id: user_id,
            description: description,
            image: image,
            created: chrono::Local::now().naive_utc(),
            count: 0,
            repost: 0,
            copy: 0,
            position: position,
            can_see_el: "a".to_string(),
            create_el: "a".to_string(),
            copy_el: "a".to_string(),
        };
        let new_album = diesel::insert_into(schema::music_albums::table)
            .values(&new_form)
            .get_result::<MusicAlbum>(&_connection)
            .expect("Error.");
        return new_album;
    }

    pub fn edit_album(&self, name: String, artist_id: Option<i32>,
        description:Option<String>, image:Option<String>, position: i16) -> &MusicAlbum {
        let _connection = establish_connection();
        let new_form = EditMusicAlbum {
            name:        name,
            artist_id:   artist_id,
            description: description,
            image:       image,
            position:    position,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<MusicAlbum>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="music_albums"]
pub struct NewMusicAlbum {
    pub name:        String,
    pub artist_id:   Option<i32>,
    pub user_id:     i32,
    pub description: Option<String>,
    pub image:       Option<String>,
    pub created:     chrono::NaiveDateTime,

    pub count:       i32,
    pub repost:      i32,
    pub copy:        i32,
    pub position:    i16,

    pub can_see_el:  String,
    pub create_el:   String,
    pub copy_el:     String,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="music_albums"]
pub struct EditMusicAlbum {
    pub name:        String,
    pub artist_id:   Option<i32>,
    pub description: Option<String>,
    pub image:       Option<String>,
    pub position:    i16,
}
/////// MusicAlbum //////

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

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(User)]
pub struct MusicList {
    pub id:           i32,
    pub name:         String,
    pub community_id: Option<i32>,
    pub user_id:      i32,
    pub types:        i16,
    pub description:  Option<String>,
    pub image:        Option<String>,
    pub created:      chrono::NaiveDateTime,

    pub count:        i32,
    pub repost:       i32,
    pub copy:         i32,
    pub position:     i16,

    pub can_see_el:   String,
    pub create_el:    String,
    pub copy_el:      String,
}
#[derive(Deserialize, Insertable)]
#[table_name="music_lists"]
pub struct NewMusicList {
    pub name:         String,
    pub community_id: Option<i32>,
    pub user_id:      i32,
    pub types:        i16,
    pub description:  Option<String>,
    pub image:        Option<String>,
    pub created:      chrono::NaiveDateTime,

    pub count:        i32,
    pub repost:       i32,
    pub copy:         i32,
    pub position:     i16,

    pub can_see_el:   String,
    pub create_el:    String,
    pub copy_el:      String,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="music_lists"]
pub struct EditMusicList {
    pub name:            String,
    pub description:     Option<String>,
    pub image:           Option<String>,
    pub can_see_el:      String,
    pub create_el:       String,
    pub copy_el:         String,
}


impl MusicList {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_music_list(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "lmu".to_string() + &self.get_str_id();
    }
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(21))
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
            .filter(schema::moderateds::types.eq(21))
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
        use crate::schema::music_list_reposts::dsl::music_list_reposts;

        let _connection = establish_connection();

        let count = music_list_reposts
            .filter(schema::music_list_reposts::music_list_id.eq(self.id))
            .filter(schema::music_list_reposts::message_id.is_not_null())
            .load::<MusicListRepost>(&_connection)
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
        use crate::schema::music_list_reposts::dsl::music_list_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = music_list_reposts
            .filter(schema::music_list_reposts::music_list_id.eq(self.id))
            .filter(schema::music_list_reposts::post_id.is_not_null())
            .limit(limit)
            .offset(offset)
            .load::<MusicListRepost>(&_connection)
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
        use crate::schema::music_list_reposts::dsl::music_list_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = music_list_reposts
            .filter(schema::music_list_reposts::music_list_id.eq(self.id))
            .filter(schema::music_list_reposts::post_id.is_not_null())
            .limit(6)
            .load::<MusicListRepost>(&_connection)
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
        return "<a data-musiclist='".to_string() + &self.get_str_id() + &"' class='ajax'>".to_string() + &self.name + &"</a>".to_string();
    }
    pub fn is_user_list(&self, user: User) -> bool {
        return self.user_id == user.id;
    }
    pub fn is_community_list(&self, community: Community) -> bool {
        return self.community_id.unwrap() == community.id;
    }
    pub fn get_users_ids(&self) -> Vec<i32> {
        use crate::schema::user_music_list_collections::dsl::user_music_list_collections;

        let _connection = establish_connection();
        let ids = user_music_list_collections
            .filter(schema::user_music_list_collections::music_list_id.eq(self.id))
            .load::<UserMusicListCollection>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in ids.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_communities_ids(&self) -> Vec<i32> {
        use crate::schema::community_music_list_collections::dsl::community_music_list_collections;

        let _connection = establish_connection();
        let ids = community_music_list_collections
            .filter(schema::community_music_list_collections::music_list_id.eq(self.id))
            .load::<CommunityMusicListCollection>(&_connection)
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
    pub fn get_items(&self) -> Vec<Music> {
        use crate::schema::musics::dsl::musics;

        let _connection = establish_connection();
        return musics
            .filter(schema::musics::music_list_id.eq(self.id))
            .filter(schema::musics::types.eq("a"))
            .order(schema::musics::created.desc())
            .load::<Music>(&_connection)
            .expect("E.");
    }
    pub fn get_paginate_items(&self, limit: i64, offset: i64) -> Vec<Music> {
        use crate::schema::musics::dsl::musics;

        let _connection = establish_connection();
        return musics
            .filter(schema::musics::music_list_id.eq(self.id))
            .filter(schema::musics::types.eq("a"))
            .limit(limit)
            .offset(offset)
            .order(schema::musics::created.desc())
            .load::<Music>(&_connection)
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
        use crate::schema::music_list_perms::dsl::music_list_perms;

        let _connection = establish_connection();
        let items = music_list_perms
            .filter(schema::music_list_perms::music_list_id.eq(self.id))
            .filter(schema::music_list_perms::can_see_item.eq("b"))
            .load::<MusicListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::music_list_perms::dsl::music_list_perms;

        let _connection = establish_connection();
        let items = music_list_perms
            .filter(schema::music_list_perms::music_list_id.eq(self.id))
            .filter(schema::music_list_perms::can_see_item.eq("a"))
            .load::<MusicListPerm>(&_connection)
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
        use crate::schema::music_list_perms::dsl::music_list_perms;

        let _connection = establish_connection();
        let items = music_list_perms
            .filter(schema::music_list_perms::music_list_id.eq(self.id))
            .filter(schema::music_list_perms::create_item.eq("b"))
            .load::<MusicListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_create_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::music_list_perms::dsl::music_list_perms;

        let _connection = establish_connection();
        let items = music_list_perms
            .filter(schema::music_list_perms::music_list_id.eq(self.id))
            .filter(schema::music_list_perms::create_item.eq("a"))
            .load::<MusicListPerm>(&_connection)
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
        use crate::schema::music_list_perms::dsl::music_list_perms;

        let _connection = establish_connection();
        let items = music_list_perms
            .filter(schema::music_list_perms::music_list_id.eq(self.id))
            .filter(schema::music_list_perms::can_copy.eq("b"))
            .load::<MusicListPerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_copy_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::music_list_perms::dsl::music_list_perms;

        let _connection = establish_connection();
        let items = music_list_perms
            .filter(schema::music_list_perms::music_list_id.eq(self.id))
            .filter(schema::music_list_perms::can_copy.eq("a"))
            .load::<MusicListPerm>(&_connection)
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
        copy_el_users: Option<Vec<i32>>) -> MusicList {

        use crate::models::{
            NewCommunityMusicListPosition,
            NewUserMusicListPosition,
        };

        let _connection = establish_connection();
        let new_list_form = NewMusicList {
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
        let new_list = diesel::insert_into(schema::music_lists::table)
            .values(&new_list_form)
            .get_result::<MusicList>(&_connection)
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

            let _new_list_position = NewCommunityMusicListPosition {
                community_id: community.id,
                list_id:      new_list.id,
                position:     community.get_music_lists_new_position(),
                types:        "a".to_string(),
            };
            let _list_position = diesel::insert_into(schema::community_music_list_positions::table)
                .values(&_new_list_position)
                .get_result::<CommunityMusicListPosition>(&_connection)
                .expect("Error saving music_list_position.");
        }
        else {
            let _new_list_position = NewUserMusicListPosition {
                user_id:  creator.id,
                list_id:  new_list.id,
                position: creator.get_music_lists_new_position(),
                types:    "a".to_string(),
            };
            let _list_position = diesel::insert_into(schema::user_music_list_positions::table)
                .values(&_new_list_position)
                .get_result::<UserMusicListPosition>(&_connection)
                .expect("Error saving music_list_position.");
        }

        if can_see_el == "e".to_string() && can_see_el == "h".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewMusicListPerm {
                        user_id:      user_id,
                        music_list_id:  new_list.id,
                        can_see_item: Some("b".to_string()),
                        create_item: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::music_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<MusicListPerm>(&_connection)
                        .expect("Error saving music_list_position.");
                }
            }
        }
        else if can_see_el == "f".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewMusicListPerm {
                        user_id:      user_id,
                        music_list_id: new_list.id,
                        can_see_item: Some("a".to_string()),
                        create_item: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::music_list_perms::table)
                        .values(&_new_include)
                        .get_result::<MusicListPerm>(&_connection)
                        .expect("Error saving music_list_position.");
                }
            }
        }

        if create_el == "e".to_string() && create_el == "h".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewMusicListPerm {
                        user_id:      user_id,
                        music_list_id: new_list.id,
                        can_see_item: None,
                        create_item: Some("b".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::music_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<MusicListPerm>(&_connection)
                        .expect("Error saving music_list_position.");
                }
            }
        }
        else if create_el == "f".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewMusicListPerm {
                        user_id:      user_id,
                        music_list_id: new_list.id,
                        can_see_item: None,
                        create_item: Some("a".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::music_list_perms::table)
                        .values(&_new_include)
                        .get_result::<MusicListPerm>(&_connection)
                        .expect("Error saving music_list_position.");
                }
            }
        }

        if copy_el == "e".to_string() && copy_el == "h".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewMusicListPerm {
                        user_id:      user_id,
                        music_list_id: new_list.id,
                        can_see_item: None,
                        create_item: None,
                        can_copy: Some("b".to_string()),
                    };
                    diesel::insert_into(schema::music_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<MusicListPerm>(&_connection)
                        .expect("Error saving music_list_position.");
                }
            }
        }
        else if copy_el == "f".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewMusicListPerm {
                        user_id:      user_id,
                        music_list_id: new_list.id,
                        can_see_item: None,
                        create_item: None,
                        can_copy: Some("a".to_string()),
                    };
                    diesel::insert_into(schema::music_list_perms::table)
                        .values(&_new_include)
                        .get_result::<MusicListPerm>(&_connection)
                        .expect("Error saving music_list_position.");
                }
            }
        }
        return new_list;
    }
    pub fn edit_list(&self, name: String, description: Option<String>, image: Option<String>,
        can_see_el: String, create_el: String, copy_el: String,
        can_see_el_users: Option<Vec<i32>>, create_el_users: Option<Vec<i32>>,
        copy_el_users: Option<Vec<i32>>) -> &MusicList {

        use crate::schema::music_list_perms::dsl::music_list_perms;

        let _connection = establish_connection();

            let edit_music_list = EditMusicList{
                name: name,
                description: description,
                image: image,
                can_see_el: can_see_el.clone(),
                create_el: create_el.clone(),
                copy_el: copy_el.clone(),
            };
            diesel::update(self)
                .set(edit_music_list)
                .get_result::<MusicList>(&_connection)
                .expect("Error.");

        if can_see_el == "e".to_string() && can_see_el == "h".to_string() {
            if can_see_el_users.is_some() {
                diesel::delete (
                  music_list_perms
                    .filter(schema::music_list_perms::music_list_id.eq(self.id))
                    .filter(schema::music_list_perms::can_see_item.is_not_null())
                )
                  .execute(&_connection)
                  .expect("E");
                for user_id in can_see_el_users.unwrap() {
                    let _new_exclude = NewMusicListPerm {
                        user_id:      user_id,
                        music_list_id: self.id,
                        can_see_item: Some("b".to_string()),
                        create_item: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::music_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<MusicListPerm>(&_connection)
                        .expect("Error saving music_list_position.");
                }
            }
        }
        else if can_see_el == "f".to_string() && can_see_el == "i".to_string() {
            if can_see_el_users.is_some() {
                for user_id in can_see_el_users.unwrap() {
                    let _new_include = NewMusicListPerm {
                        user_id:      user_id,
                        music_list_id: self.id,
                        can_see_item: Some("a".to_string()),
                        create_item: None,
                        can_copy: None,
                    };
                    diesel::insert_into(schema::music_list_perms::table)
                        .values(&_new_include)
                        .get_result::<MusicListPerm>(&_connection)
                        .expect("Error saving music_list_position.");
                }
            }
        }

        if create_el == "e".to_string() && create_el == "h".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewMusicListPerm {
                        user_id:      user_id,
                        music_list_id: self.id,
                        can_see_item: None,
                        create_item: Some("b".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::music_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<MusicListPerm>(&_connection)
                        .expect("Error saving music_list_position.");
                }
            }
        }
        else if create_el == "f".to_string() && create_el == "i".to_string() {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewMusicListPerm {
                        user_id:      user_id,
                        music_list_id: self.id,
                        can_see_item: None,
                        create_item: Some("a".to_string()),
                        can_copy: None,
                    };
                    diesel::insert_into(schema::music_list_perms::table)
                        .values(&_new_include)
                        .get_result::<MusicListPerm>(&_connection)
                        .expect("Error saving music_list_position.");
                }
            }
        }

        if copy_el == "e".to_string() && copy_el == "h".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewMusicListPerm {
                        user_id:      user_id,
                        music_list_id: self.id,
                        can_see_item: None,
                        create_item: None,
                        can_copy: Some("b".to_string()),
                    };
                    diesel::insert_into(schema::music_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<MusicListPerm>(&_connection)
                        .expect("Error saving music_list_position.");
                }
            }
        }
        else if copy_el == "f".to_string() && copy_el == "i".to_string() {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewMusicListPerm {
                        user_id:      user_id,
                        music_list_id: self.id,
                        can_see_item: None,
                        create_item: None,
                        can_copy: Some("a".to_string()),
                    };
                    diesel::insert_into(schema::music_list_perms::table)
                        .values(&_new_include)
                        .get_result::<MusicListPerm>(&_connection)
                        .expect("Error saving music_list_position.");
                }
            }
        }
        return self;
    }
    pub fn get_order(&self) -> UserMusicListPosition {
        use crate::schema::user_music_list_positions::dsl::user_music_list_positions;

        let _connection = establish_connection();
        return user_music_list_positions
            .filter(schema::user_music_list_positions::list_id.eq(self.id))
            .filter(schema::user_music_list_positions::types.eq("a"))
            .load::<UserMusicListPosition>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn add_in_community_collections(&self, community: Community) -> bool {
        use crate::models::NewCommunityMusicListPosition;

        if !self.get_communities_ids().iter().any(|&i| i==community.id) && self.community_id.is_some() && self.community_id.unwrap() == community.id {
            return false;
        }
        let _connection = establish_connection();
        let new_item = NewCommunityMusicListCollection {
            community_id: community.id,
            music_list_id: self.id,
        };
        diesel::insert_into(schema::community_music_list_collections::table)
            .values(&new_item)
            .get_result::<CommunityMusicListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewCommunityMusicListPosition {
            community_id: community.id,
            list_id:      self.id,
            position:     community.get_music_lists_new_position(),
            types:        "a".to_string(),
        };
        diesel::insert_into(schema::community_music_list_positions::table)
            .values(&new_pos)
            .get_result::<CommunityMusicListPosition>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn remove_in_community_collections(&self, community: Community) -> bool {
        use crate::schema::community_music_list_positions::dsl::community_music_list_positions;
        use crate::schema::community_music_list_collections::dsl::community_music_list_collections;

        if self.get_communities_ids().iter().any(|&i| i==community.id) {
            return false;
        }
        let _connection = establish_connection();
        diesel::delete(community_music_list_collections
            .filter(schema::community_music_list_collections::community_id.eq(community.id))
            .filter(schema::community_music_list_collections::music_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(community_music_list_positions
            .filter(schema::community_music_list_positions::community_id.eq(community.id))
            .filter(schema::community_music_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return true;
    }

    pub fn add_in_user_collections(&self, user: User) -> bool {
        use crate::models::NewUserMusicListPosition;

        if !self.get_users_ids().iter().any(|&i| i==user.id) && self.user_id == user.id {
            return false;
        }
        let _connection = establish_connection();
        let new_item = NewUserMusicListCollection {
            user_id: user.id,
            music_list_id: self.id,
        };
        diesel::insert_into(schema::user_music_list_collections::table)
            .values(&new_item)
            .get_result::<UserMusicListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewUserMusicListPosition {
            user_id:  user.id,
            list_id:  self.id,
            position: user.get_music_lists_new_position(),
            types:    "a".to_string(),
        };
        diesel::insert_into(schema::user_music_list_positions::table)
            .values(&new_pos)
            .get_result::<UserMusicListPosition>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn remove_in_user_collections(&self, user: User) -> bool {
        use crate::schema::user_music_list_collections::dsl::user_music_list_collections;
        use crate::schema::user_music_list_positions::dsl::user_music_list_positions;

        if self.get_users_ids().iter().any(|&i| i==user.id) {
            return false;
        }
        let _connection = establish_connection();
        diesel::delete(user_music_list_collections
            .filter(schema::user_music_list_collections::user_id.eq(user.id))
            .filter(schema::user_music_list_collections::music_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(user_music_list_positions
            .filter(schema::user_music_list_positions::user_id.eq(user.id))
            .filter(schema::user_music_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return true;
    }

    pub fn copy_item(pk: i32, user_or_communities: Vec<String>) -> bool {
        use crate::schema::music_lists::dsl::music_lists;
        use crate::schema::users::dsl::users;
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        let lists = music_lists
            .filter(schema::music_lists::id.eq(pk))
            .filter(schema::music_lists::types.lt(10))
            .load::<MusicList>(&_connection)
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
    pub fn get_musics_ids(&self) -> Vec<i32> {
        use crate::schema::musics::dsl::musics;

        let _connection = establish_connection();
        let fix_list = musics
            .filter(schema::musics::music_list_id.eq(self.id))
            .filter(schema::musics::types.lt("b"))
            .load::<Music>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in fix_list.iter() {
            stack.push(_item.id);
        };
        return stack;
    }
    pub fn get_user_lists(user_pk: i32) -> Vec<MusicList> {
        use crate::schema::user_music_list_collections::dsl::user_music_list_collections;
        use crate::schema::user_music_list_positions::dsl::user_music_list_positions;
        use crate::schema::music_lists::dsl::music_lists;

        let _connection = establish_connection();
        let position_lists = user_music_list_positions
            .filter(schema::user_music_list_positions::user_id.eq(user_pk))
            .filter(schema::user_music_list_positions::types.eq("a"))
            .load::<UserMusicListPosition>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            let mut stack = Vec::new();
            for _item in position_lists.iter() {
                stack.push(_item.list_id);
            };
            return music_lists
                .filter(schema::music_lists::id.eq_any(stack))
                .filter(schema::music_lists::types.lt(10))
                .load::<MusicList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let user_lists = music_lists
            .filter(schema::music_lists::user_id.eq(user_pk))
            .filter(schema::music_lists::types.lt(10))
            .load::<MusicList>(&_connection)
            .expect("E.");
        for _item in user_lists.iter() {
            stack.push(_item.id);
        };
        let user_collections = user_music_list_collections
            .filter(schema::user_music_list_collections::user_id.eq(user_pk))
            .load::<UserMusicListCollection>(&_connection)
            .expect("E.");
        for _item in user_collections.iter() {
            stack.push(_item.music_list_id);
        };
        return music_lists
            .filter(schema::music_lists::id.eq_any(stack))
            .filter(schema::music_lists::types.lt(10))
            .load::<MusicList>(&_connection)
            .expect("E.");
    }
    pub fn get_community_lists(community_pk: i32) -> Vec<MusicList> {
        use crate::schema::community_music_list_collections::dsl::community_music_list_collections;
        use crate::schema::community_music_list_positions::dsl::community_music_list_positions;
        use crate::schema::music_lists::dsl::music_lists;

        let _connection = establish_connection();
        let position_lists = community_music_list_positions
            .filter(schema::community_music_list_positions::community_id.eq(community_pk))
            .filter(schema::community_music_list_positions::types.eq("a"))
            .load::<CommunityMusicListPosition>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            let mut stack = Vec::new();
            for _item in position_lists.iter() {
                stack.push(_item.list_id);
            };
            return music_lists
                .filter(schema::music_lists::id.eq_any(stack))
                .filter(schema::music_lists::types.lt(10))
                .load::<MusicList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let community_lists = music_lists
            .filter(schema::music_lists::community_id.eq(community_pk))
            .filter(schema::music_lists::types.lt(10))
            .load::<MusicList>(&_connection)
            .expect("E.");
        for _item in community_lists.iter() {
            stack.push(_item.id);
        };
        let community_collections = community_music_list_collections
            .filter(schema::community_music_list_collections::community_id.eq(community_pk))
            .load::<CommunityMusicListCollection>(&_connection)
            .expect("E.");
        for _item in community_collections.iter() {
            stack.push(_item.music_list_id);
        };
        return music_lists
            .filter(schema::music_lists::id.eq_any(stack))
            .filter(schema::music_lists::types.lt(10))
            .load::<MusicList>(&_connection)
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
            .set(schema::music_lists::types.eq(close_case))
            .get_result::<MusicList>(&_connection)
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
            .set(schema::music_lists::types.eq(close_case))
            .get_result::<MusicList>(&_connection)
            .expect("E");
       return true;
    }

    pub fn delete_item(&self) -> bool {
        let _connection = establish_connection();
        if self.community_id.is_some() {
            use crate::schema::community_music_list_positions::dsl::community_music_list_positions;

            let list_positions = community_music_list_positions
                .filter(schema::community_music_list_positions::community_id.eq(self.community_id.unwrap()))
                .filter(schema::community_music_list_positions::list_id.eq(self.id))
                .load::<CommunityMusicListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::community_music_list_positions::types.eq("b"))
                  .get_result::<CommunityMusicListPosition>(&_connection)
                  .expect("Error.");
            }
        } else {
            use crate::schema::user_music_list_positions::dsl::user_music_list_positions;

            let list_positions = user_music_list_positions
                .filter(schema::user_music_list_positions::user_id.eq(self.user_id))
                .filter(schema::user_music_list_positions::list_id.eq(self.id))
                .load::<UserMusicListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::user_music_list_positions::types.eq("b"))
                  .get_result::<UserMusicListPosition>(&_connection)
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
            .set(schema::music_lists::types.eq(close_case))
            .get_result::<MusicList>(&_connection)
            .expect("E");
       return true;
    }
    pub fn restore_item(&self) -> bool {
        let _connection = establish_connection();
        if self.community_id.is_some() {
            use crate::schema::community_music_list_positions::dsl::community_music_list_positions;

            let list_positions = community_music_list_positions
                .filter(schema::community_music_list_positions::community_id.eq(self.community_id.unwrap()))
                .filter(schema::community_music_list_positions::list_id.eq(self.id))
                .load::<CommunityMusicListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::community_music_list_positions::types.eq("b"))
                  .get_result::<CommunityMusicListPosition>(&_connection)
                  .expect("Error.");
            }
        } else {
            use crate::schema::user_music_list_positions::dsl::user_music_list_positions;

            let list_positions = user_music_list_positions
                .filter(schema::user_music_list_positions::user_id.eq(self.user_id))
                .filter(schema::user_music_list_positions::list_id.eq(self.id))
                .load::<UserMusicListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::user_music_list_positions::types.eq("a"))
                  .get_result::<UserMusicListPosition>(&_connection)
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
            .set(schema::music_lists::types.eq(close_case))
            .get_result::<MusicList>(&_connection)
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
            .set(schema::music_lists::types.eq(close_case))
            .get_result::<MusicList>(&_connection)
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
            .set(schema::music_lists::types.eq(close_case))
            .get_result::<MusicList>(&_connection)
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

    pub fn create_track(&self, title: String, community_id: Option<i32>,
        user_id: i32, genre_id: Option<i32>, album_id: Option<i32>,
        file: String, image: Option<String>) -> Music {

        let _connection = establish_connection();

        let new_music_form = NewMusic {
            title: title,
            community_id: community_id,
            user_id: user_id,
            music_list_id: self.id,
            genre_id: genre_id,
            album_id: album_id,
            types: "a".to_string(),
            file: file,
            image: image,
            created: chrono::Local::now().naive_utc(),
            view: 0,
            repost: 0,
            copy: 0,
            position: (self.count).try_into().unwrap(),
          };
          let new_music = diesel::insert_into(schema::musics::table)
              .values(&new_music_form)
              .get_result::<Music>(&_connection)
              .expect("Error.");

        if community_id.is_some() {
            let community = self.get_community();
            community.plus_tracks(1);
            return new_music;
        }
        else {
            use crate::utils::get_user;

            let creator = get_user(user_id);
            creator.plus_tracks(1);
            return new_music;
        }
    }
}
/////// Music //////

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
#[belongs_to(MusicList)]
pub struct Music {
    pub id:              i32,
    pub title:           String,
    pub community_id:    Option<i32>,
    pub user_id:      i32,
    pub music_list_id:         i32,
    pub genre_id:        Option<i32>,
    pub album_id:        Option<i32>,
    pub types:           String,
    pub file:            String,
    pub image:           Option<String>,
    pub created:         chrono::NaiveDateTime,

    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="musics"]
pub struct NewMusic {
    pub title:           String,
    pub community_id:    Option<i32>,
    pub user_id:        i32,
    pub music_list_id:  i32,
    pub genre_id:       Option<i32>,
    pub album_id:       Option<i32>,
    pub types:          String,
    pub file:           String,
    pub image:          Option<String>,
    pub created:        chrono::NaiveDateTime,

    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="musics"]
pub struct EditMusic {
    pub title:           String,
    pub genre_id:       Option<i32>,
    pub album_id:       Option<i32>,
    pub image:          Option<String>,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="musics"]
pub struct EditMusicPosition {
    pub position: i16,
}

impl Music {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return "<img style='width:40px;' alt='image' src='".to_string() +  &self.image.as_deref().unwrap().to_string() + &"' />".to_string();
        }
        else {
            return "<svg fill='currentColor' class='svg_default' width='40' height='40' viewBox='0 0 24 24'><path fill='none' d='M0 0h24v24H0z'></path><path d='M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z'></path></svg>".to_string();
        }
    }
    pub fn get_s_image(&self) -> String {
        if self.image.is_some() {
            return "<img style='width:30px' alt='image' src='".to_string() + &self.image.as_deref().unwrap().to_string() + &" />".to_string();
        }
        else {
            return "<svg fill='currentColor' class='svg_default' width='30' height='30' viewBox='0 0 24 24'><path fill='none' d='M0 0h24v24H0z'></path><path d='M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z'></path></svg>".to_string();
        }
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
            .set(schema::musics::types.eq(close_case))
            .get_result::<Music>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::music_lists::count.eq(list.count - 1))
            .get_result::<MusicList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.minus_tracks(1);
        }
        else {
            let creator = self.get_creator();
            creator.minus_tracks(1);
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
            .set(schema::musics::types.eq(close_case))
            .get_result::<Music>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::music_lists::count.eq(list.count + 1))
            .get_result::<MusicList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.plus_tracks(1);
        }
        else {
            let creator = self.get_creator();
            creator.plus_tracks(1);
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
            .set(schema::musics::types.eq(close_case))
            .get_result::<Music>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::music_lists::count.eq(list.count - 1))
            .get_result::<MusicList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.minus_tracks(1);
        }
        else {
            let creator = self.get_creator();
            creator.minus_tracks(1);
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
            .set(schema::musics::types.eq(close_case))
            .get_result::<Music>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::music_lists::count.eq(list.count + 1))
            .get_result::<MusicList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.plus_tracks(1);
        }
        else {
            let creator = self.get_creator();
            creator.plus_tracks(1);
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
    pub fn is_music(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "mus".to_string() + &self.get_str_id();
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
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(52))
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
            .filter(schema::moderateds::types.eq(52))
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

    pub fn get_list(&self) -> MusicList {
        use crate::schema::music_lists::dsl::music_lists;

        let _connection = establish_connection();
        return music_lists
            .filter(schema::music_lists::id.eq(self.music_list_id))
            .filter(schema::music_lists::types.lt(10))
            .load::<MusicList>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
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
        use crate::schema::music_reposts::dsl::music_reposts;

        let _connection = establish_connection();

        let count = music_reposts
            .filter(schema::music_reposts::music_id.eq(self.id))
            .filter(schema::music_reposts::message_id.is_not_null())
            .load::<MusicRepost>(&_connection)
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
        use crate::schema::music_reposts::dsl::music_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = music_reposts
            .filter(schema::music_reposts::music_id.eq(self.id))
            .filter(schema::music_reposts::post_id.is_not_null())
            .limit(limit)
            .offset(offset)
            .load::<MusicRepost>(&_connection)
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
        use crate::schema::music_reposts::dsl::music_reposts;
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let item_reposts = music_reposts
            .filter(schema::music_reposts::music_id.eq(self.id))
            .filter(schema::music_reposts::post_id.is_not_null())
            .limit(6)
            .load::<MusicRepost>(&_connection)
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
        if self.community_id.is_some() {
            let community = self.get_community();
            return "аудиозапись сообщества <a href='".to_owned() + &community.link.to_string() + &"' target='_blank'>" + &community.name + &"</a>"
        }
        else {
            let creator = self.get_creator();
            return "<a href='".to_owned() + &creator.link.to_string() + &"' target='_blank'>" + &creator.get_full_name() + &"</a>" + &": аудиозапись"
        }
    }

    pub fn copy_item(pk: i32, lists: Vec<i32>) -> bool {
        use crate::schema::musics::dsl::musics;
        use crate::schema::music_lists::dsl::music_lists;

        let _connection = establish_connection();
        let item = musics
            .filter(schema::musics::id.eq(pk))
            .filter(schema::musics::types.eq("a"))
            .load::<Music>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
        let mut count = 0;
        for list_id in lists.iter() {
            count += 1;
            let list = music_lists
                .filter(schema::music_lists::id.eq(list_id))
                .filter(schema::music_lists::types.lt(10))
                .load::<MusicList>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            list.create_track (
                item.title.clone(),
                item.community_id,
                list.user_id,
                item.genre_id,
                item.album_id,
                item.file.clone(),
                item.image.clone(),
            );
        }
        diesel::update(&item)
          .set(schema::musics::copy.eq(item.copy + count))
          .get_result::<Music>(&_connection)
          .expect("Error.");

        if item.community_id.is_some() {
            let community = item.get_community();
            community.plus_tracks(count);
        }
        else {
            let creator = item.get_creator();
            creator.plus_tracks(count);
          }
        return true;
    }
    pub fn edit_music(&self, title: String, genre_id: Option<i32>,
        album_id: Option<i32>, image: Option<String>) -> &Music {
        let _connection = establish_connection();

        let edit_music = EditMusic {
            title: title,
            genre_id: genre_id,
            album_id: album_id,
            image: image,
        };
        diesel::update(self)
            .set(edit_music)
            .get_result::<Music>(&_connection)
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

    pub fn change_position(query: Json<Vec<JsonPosition>>) -> bool {
        use crate::schema::musics::dsl::musics;

        let _connection = establish_connection();
        for i in query.iter() {
            let item = musics
                .filter(schema::musics::id.eq(i.key))
                .filter(schema::musics::types.eq("a"))
                .limit(1)
                .load::<Music>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            diesel::update(&item)
                .set(schema::musics::position.eq(i.value))
                .get_result::<Music>(&_connection)
                .expect("Error.");
        }
        return true;
    }
}

/////// UserMusicListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(MusicList)]
pub struct UserMusicListCollection {
    pub id:       i32,
    pub user_id:  i32,
    pub music_list_id:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_music_list_collections"]
pub struct NewUserMusicListCollection {
    pub user_id:  i32,
    pub music_list_id:  i32,
}

/////// CommunityMusicListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(MusicList)]
pub struct CommunityMusicListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub music_list_id:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_music_list_collections"]
pub struct NewCommunityMusicListCollection {
    pub community_id:  i32,
    pub music_list_id:       i32,
}

/////// MusicListPerm //////
    // 'a' Активно
    // 'b' Не активно
    // 'c' Нет значения

#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(MusicList)]
pub struct MusicListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub music_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub create_item:     Option<String>,
    pub can_copy:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="music_list_perms"]
pub struct NewMusicListPerm {
    pub user_id:         i32,
    pub music_list_id:   i32,
    pub can_see_item:    Option<String>,
    pub create_item:     Option<String>,
    pub can_copy:        Option<String>,
}

/////// MusicListRepost //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(MusicList)]
#[belongs_to(Post)]
#[belongs_to(Message)]
pub struct MusicListRepost {
    pub id:            i32,
    pub music_list_id: i32,
    pub post_id:       Option<i32>,
    pub message_id:    Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="music_list_reposts"]
pub struct NewMusicListRepost {
    pub music_list_id: i32,
    pub post_id:       Option<i32>,
    pub message_id:    Option<i32>,
}

/////// MusicRepost //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Music)]
#[belongs_to(Post)]
#[belongs_to(Message)]
pub struct MusicRepost {
    pub id:         i32,
    pub music_id:   i32,
    pub post_id:    Option<i32>,
    pub message_id: Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="music_reposts"]
pub struct NewMusicRepost {
    pub music_id:   i32,
    pub post_id:    Option<i32>,
    pub message_id: Option<i32>,
}
