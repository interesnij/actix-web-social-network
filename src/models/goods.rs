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
use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
    Sticker,
};


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
