use crate::schema::{
    post_categories,
    post_lists,
    posts,
    post_comments,
    user_post_list_collections,
    community_post_list_collections,
    post_list_perm,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
    Sticker,
};


/////// CommunityCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct PostCategories {
    pub id:       i32,
    pub name:     String,
    pub avatar:   Option<String>,
    pub position: i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="post_categories"]
pub struct NewPostCategories {
    pub name:     String,
    pub avatar:   Option<String>,
    pub position: i32,
}

/////// PostList //////
////////// Тип списка
    // 'a' основной список
    // 'd' пользовательский список
    // 'e' список предложки
    // 'g' удаленный список
    // 'k' закрытый список
    // 'l' закрытый основной список
    // 'p' замороженный список
    // 'r' замороженный основной список

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

/////// PostList //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(User)]
#[table_name="post_lists"]
pub struct PostList {
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

/////// Post //////

    // 'f' Предложка сообщества
    // 'g' Предложка пользователя
    // 'd' Черновик владельца
    // 'e' Черновик предложки
    // 'b' Закрепленый
    // 'a' Опубликовано
    // 'c' Удаленый
    // 'h' Закрыто модератором
    // 'i' Удаленый предложенный в сообщество
    // 'y' Удаленый предложенный у пользователя


#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(PostCategories)]
#[belongs_to(User)]
#[belongs_to(PostList)]
#[table_name="posts"]
pub struct Post {
    pub id:              i32,
    pub content:         Option<String>,
    pub community_id:    Option<i32>,
    pub category_id:     Option<i32>,
    pub creator_id:      i32,
    pub list_id:         i32,
    pub types:           char,
    pub attach:          Option<String>,
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

/////// PostComment //////

// 'a' Опубликованный
// 'b' Изменённый
// 'c' Удаленый
// 'd' Изменённый Удаленый
// 'e' Закрытый модератором
// 'f' Закрытый Удаленый

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Post)]
#[belongs_to(User)]
#[belongs_to(Stickers)]
#[table_name="post_comments"]
pub struct PostComment {
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

/////// UserPostListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(PostList)]
#[table_name="user_post_list_collections"]
pub struct UserPostListCollection {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
}

/////// CommunityPostListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(PostList)]
#[table_name="community_post_list_collections"]
pub struct CommunityPostListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub list_id:       i32,
}

/////// PostListPerm //////

#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(PostList)]
#[table_name="post_list_perm"]
pub struct PostListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub list_id:         i32,
    pub can_see_item:    char,
    pub can_see_comment: char,
    pub create_item:     char,
    pub create_comment:  char,
    pub can_copy:        char,
}
