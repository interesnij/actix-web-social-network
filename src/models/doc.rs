use crate::schema::{
    doc_lists,
    docs,
    user_doc_list_collections,
    community_doc_list_collections,
    doc_list_perm,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
    Sticker,
};


/////// DocList //////

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

/////// DocList //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(User)]
pub struct DocList {
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
    pub position:        i16,
    pub can_see_el:      char,
    pub create_el:       char,
    pub copy_el:         char,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="doc_lists"]
pub struct NewDocList {
    pub name:            String,
    pub community_id:    Option<i32>,
    pub creator_id:      i32,
    pub types:           String,
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

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(User)]
#[belongs_to(DocList)]
pub struct Doc {
    pub id:              i32,
    pub title:           String,
    pub community_id:    Option<i32>,
    pub creator_id:      i32,
    pub list_id:         i32,
    pub types:           char,
    pub types_2:         char,
    pub file:            String,
    pub created:         chrono::NaiveDateTime,

    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="docs"]
pub struct NewDoc {
    pub title:           String,
    pub community_id:    Option<i32>,
    pub creator_id:      i32,
    pub list_id:         i32,
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
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(DocList)]
pub struct UserDocListCollection {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_doc_list_collections"]
pub struct NewUserDocListCollection {
    pub user_id:  i32,
    pub list_id:  i32,
}

/////// CommunityDocListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(DocList)]
pub struct CommunityDocListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub list_id:       i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_doc_list_collections"]
pub struct NewCommunityDocListCollection {
    pub community_id:  i32,
    pub list_id:       i32,
}

/////// DocListPerm //////
    // 'a' Активно
    // 'b' Не активно
    // 'c' Нет значения

#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(DocList)]
pub struct DocListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub list_id:         i32,
    pub can_see_item:    Option<char>,
    pub create_item:     Option<char>,
    pub can_copy:        Option<char>,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="doc_list_perm"]
pub struct NewDocListPerm {
    pub user_id:         i32,
    pub list_id:         i32,
    pub can_see_item:    Option<String>,
    pub create_item:     Option<String>,
    pub can_copy:        Option<String>,
}
