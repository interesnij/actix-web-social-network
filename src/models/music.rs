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
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
    UserMusicListPosition,
    CommunityMusicListPosition,
};


/////// SoundGenres //////

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct SoundGenre {
    pub id:       i32,
    pub name:     String,
    pub count:    i32,
    pub copy:     i32,
}
#[derive(Deserialize, Insertable)]
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
    // 'd' Друзья, кроме
    // 'e' Некоторые друзья
    // 'f' Подписчики
    // 'g' Только я / владелец сообщества
    // 'h' Администраторы
    // 'i' Подписчики, кроме
    // 'j' Некоторые подписчики


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
#[derive(Deserialize, Insertable)]
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
    pub artist_id:   i32,
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
#[derive(Deserialize, Insertable)]
#[table_name="music_albums"]
pub struct NewMusicAlbum {
    pub name:        String,
    pub artist_id:   i32,
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
    pub can_see_el:      String,
    pub create_el:       String,
    pub copy_el:         String,
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
