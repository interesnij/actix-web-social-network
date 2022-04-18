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
//use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
    Sticker,
};


/////// GoodList //////

////////// Тип списка
    // 'a' основной список
    // 'b' пользовательский список
    // 'c' список предложки
    // 'd' Фото со страницы
    // 'e' Фото со стены
    // 'f', 'g' ....

    // 'h' удаленный основной список
    // 'i' удаленный пользовательский список
    // 'j' удаленный список предложки
    // 'k' удаленный Фото со страницы
    // 'l' удаленный Фото со стены
    // удаленный 'm', 'n' ....

    // 'o' закрытый основной список
    // 'p' закрытый пользовательский список
    // 'q' закрытый список предложки
    // 'r' закрытый Фото со страницы
    // 's' закрытый Фото со стены
    // закрытый 't', 'u' ....

    // 'v' замороженный основной список
    // 'w' замороженный пользовательский список
    // 'x' замороженный список предложки
    // 'y' замороженный Фото со страницы
    // 'z' замороженный Фото со стены
    // замороженный '1', '2' ....

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
    pub user_id:      i32,
    pub types:           String,
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
    pub user_id:      i32,
    pub types:           String,
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
    pub vote:            i32,
    pub user_id:         i32,
    pub good_id:         i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="good_votes"]
pub struct NewGoodVote {
    pub vote:            i32,
    pub user_id:         i32,
    pub good_id:         i32,
}


/////// GoodCommentVote //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(GoodComment)]
pub struct GoodCommentVote {
    pub id:              i32,
    pub vote:            i32,
    pub user_id:         i32,
    pub good_comment_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="good_comment_votes"]
pub struct NewGoodCommentVote {
    pub vote:            i32,
    pub user_id:         i32,
    pub good_comment_id: i32,
}
