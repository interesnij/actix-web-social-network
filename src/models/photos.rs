use crate::schema::{
    photo_lists,
    photos,
    photo_comments,
    user_photo_list_collections,
    community_photo_list_collections,
    photo_list_perms,
    photo_votes,
    photo_comment_votes,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
//use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
    Sticker,
};


/////// PhotoList //////

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

/////// PhotoList //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(User)]
pub struct PhotoList {
    pub id:              i32,
    pub name:            String,
    pub community_id:    Option<i32>,
    pub user_id:         i32,
    pub types:           i16,
    pub description:     Option<String>,
    pub cover_photo:     Option<String>,
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
#[table_name="photo_lists"]
pub struct NewPhotoList {
    pub name:            String,
    pub community_id:    Option<i32>,
    pub user_id:         i32,
    pub types:           i16,
    pub description:     Option<String>,
    pub cover_photo:     Option<String>,
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

/////// Photo //////

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
#[belongs_to(PhotoList)]
pub struct Photo {
    pub id:              i32,
    pub community_id:    Option<i32>,
    pub user_id:      i32,
    pub photo_list_id:         i32,
    pub types:           String,
    pub preview:         String,
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
#[table_name="photos"]
pub struct NewPhoto {
    pub community_id:    Option<i32>,
    pub user_id:      i32,
    pub photo_list_id:         i32,
    pub types:           String,
    pub preview:         String,
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

/////// PhotoComment //////

    // 'a' Опубликованный
    // 'b' Изменённый
    // 'c' Удаленый
    // 'd' Изменённый Удаленый
    // 'e' Закрытый модератором
    // 'f' Закрытый Удаленый

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Photo)]
#[belongs_to(User)]
#[belongs_to(Sticker)]
pub struct PhotoComment {
    pub id:         i32,
    pub photo_id:    i32,
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
#[table_name="photo_comments"]
pub struct NewPhotoComment {
    pub photo_id:    i32,
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

/////// UserPhotoListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(PhotoList)]
pub struct UserPhotoListCollection {
    pub id:       i32,
    pub user_id:  i32,
    pub photo_list_id:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_photo_list_collections"]
pub struct NewUserPhotoListCollection {
    pub user_id:  i32,
    pub photo_list_id:  i32,
}

/////// CommunityPhotoListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(PhotoList)]
pub struct CommunityPhotoListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub photo_list_id:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_photo_list_collections"]
pub struct NewCommunityPhotoListCollection {
    pub community_id:  i32,
    pub photo_list_id:       i32,
}

/////// PhotoListPerm //////
    // 'a' Активно
    // 'b' Не активно
    // 'c' Нет значения

#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(PhotoList)]
pub struct PhotoListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub photo_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub can_see_comment: Option<String>,
    pub create_item:     Option<String>,
    pub create_comment:  Option<String>,
    pub can_copy:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="photo_list_perms"]
pub struct NewPhotoListPerm {
    pub user_id:         i32,
    pub photo_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub can_see_comment: Option<String>,
    pub create_item:     Option<String>,
    pub create_comment:  Option<String>,
    pub can_copy:        Option<String>,
}


/////// PhotoVote//////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Photo)]
pub struct PhotoVote {
    pub id:              i32,
    pub vote:            i16,
    pub user_id:         i32,
    pub photo_id:         i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="photo_votes"]
pub struct NewPhotoVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub photo_id:         i32,
}


/////// PhotoCommentVote //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(PhotoComment)]
pub struct PhotoCommentVote {
    pub id:              i32,
    pub vote:            i16,
    pub user_id:         i32,
    pub photo_comment_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="photo_comment_votes"]
pub struct NewPhotoCommentVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub photo_comment_id: i32,
}
