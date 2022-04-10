use crate::schema::{
    photo_lists,
    photos,
    photo_comments,
    user_photo_list_collections,
    community_photo_list_collections,
    photo_list_perm,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
    Sticker,
};


/////// PhotoList //////

////////// Тип списка
    // 'a' основной список
    // 'b' Фото со страницы
    // 'c' Фото со стены
    // 'd' пользовательский список
    // 'e' список предложки
    // 'g' удаленный список
    // 'h' Фото со страницы
    // 'i' Фото со стены
    // 'k' закрытый список
    // 'l' закрытый основной список
    // 'm' Фото со страницы
    // 'n' Фото со стены
    // 'p' замороженный список
    // 'r' замороженный основной список
    // 's' Фото со страницы
    // 't' Фото со стены

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
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(User)]
#[table_name="photo_lists"]
pub struct PhotoList {
    pub id:              i32,
    pub name:            String,
    pub community_id:    Option<i32>,
    pub creator_id:      i32,
    pub types:           char,
    pub description:     Option<String>,
    pub created:         chrono::NaiveDateTime,
    pub count:           i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i32,
    pub can_see_el:      char,
    pub can_see_comment: char,
    pub create_el:       char,
    pub create_comment:  char,
    pub copy_el:         char,
}

/////// Photo //////
    // 'a' Опубликовано
    // 'b' Удаленый
    // 'c' Закрыто модератором

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(User)]
#[belongs_to(PhotoList)]
#[table_name="photos"]
pub struct Photo {
    pub id:              i32,
    pub title:           Option<String>,
    pub community_id:    Option<i32>,
    pub creator_id:      i32,
    pub list_id:         i32,
    pub types:           char,
    pub preview:         Option<String>,
    pub file:            Option<String>,
    pub description:     Option<String>,
    pub comment_enabled: Bool,
    pub votes_on:        Bool,
    pub created:         chrono::NaiveDateTime,

    pub comment:         i32,
    pub view:            i32,
    pub liked:           i32,
    pub disliked:        i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i32,
}

/////// PhotoComment //////

    // 'a' Опубликованный
    // 'b' Изменённый
    // 'c' Удаленый
    // 'd' Изменённый Удаленый
    // 'e' Закрытый модератором
    // 'f' Закрытый Удаленый

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Photo)]
#[belongs_to(User)]
#[belongs_to(Stickers)]
#[table_name="photo_comments"]
pub struct PhotoComment {
    pub id:         i32,
    pub item_id:    i32,
    pub creator_id: i32,
    pub sticker_id: Option<i32>,
    pub parent_id:  Option<i32>,
    pub content:    Option<String>,
    pub attach:     Option<String>,
    pub types:      char,
    pub created:    chrono::NaiveDateTime,
    pub liked:      i32,
    pub disliked:   i32,
    pub repost:     i32,
}

/////// UserPhotoListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(PhotoList)]
#[table_name="user_photo_list_collections"]
pub struct UserPhotoListCollection {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
}

/////// CommunityPhotoListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(PhotoList)]
#[table_name="community_photo_list_collections"]
pub struct CommunityPhotoListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub list_id:       i32,
}

/////// PhotoListPerm //////

    // 'c' Нет значения
    // 'a' Активно
    // 'b' Не активно

#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(PhotoList)]
#[table_name="photo_list_perm"]
pub struct PhotoListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub list_id:         i32,
    pub can_see_item:    char,
    pub can_see_comment: char,
    pub create_item:     char,
    pub create_comment:  char,
    pub can_copy:        char,
}
