use crate::schema::{
    post_categories,
    post_lists,
    posts,
    post_comments,
    user_post_list_collections,
    community_post_list_collections,
    post_list_perms,
    post_votes,
    post_comment_votes,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
//use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
    Sticker,
};


/////// CommunityCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct PostCategorie {
    pub id:       i32,
    pub name:     String,
    //pub avatar:   Option<String>,
    pub position: i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_categories"]
pub struct NewPostCategorie {
    pub name:     String,
    //pub avatar:   Option<String>,
    pub position: i16,
}

/////// PostList //////
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
        // 'o' Никто

/////// PostList //////
#[derive(Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(Community)]
#[belongs_to(User)]
pub struct PostList {
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
#[table_name="post_lists"]
pub struct NewPostList {
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

/////// Post //////

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
#[belongs_to(PostCategorie)]
#[belongs_to(User)]
#[belongs_to(PostList)]
pub struct Post {
    pub id:                i32,
    pub content:           Option<String>,
    pub community_id:      Option<i32>,
    pub post_categorie_id: Option<i32>,
    pub user_id:           i32,
    pub post_list_id:      i32,
    pub types:             String,
    pub attach:            Option<String>,
    pub comment_enabled:   bool,
    pub votes_on:          bool,
    pub created:           chrono::NaiveDateTime,
    pub comment:           i32,
    pub view:              i32,
    pub liked:             i32,
    pub disliked:          i32,
    pub repost:            i32,
    pub copy:              i32,
    pub position:          i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub content:         Option<String>,
    pub community_id:    Option<i32>,
    pub post_categorie_id:     Option<i32>,
    pub user_id:      i32,
    pub post_list_id:         i32,
    pub types:           String,
    pub attach:          Option<String>,
    pub comment_enabled: bool,
    pub votes_on:        bool,
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

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Post)]
#[belongs_to(User)]
#[belongs_to(Sticker)]
pub struct PostComment {
    pub id:         i32,
    pub post_id:    i32,
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
#[table_name="post_comments"]
pub struct NewPostComment {
    pub post_id:    i32,
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

/////// UserPostListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(PostList)]
pub struct UserPostListCollection {
    pub id:       i32,
    pub user_id:  i32,
    pub post_list_id:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_post_list_collections"]
pub struct NewUserPostListCollection {
    pub user_id:  i32,
    pub post_list_id:  i32,
}

/////// CommunityPostListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(PostList)]
pub struct CommunityPostListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub post_list_id:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_post_list_collections"]
pub struct NewCommunityPostListCollection {
    pub community_id:  i32,
    pub post_list_id:       i32,
}

/////// PostListPerm //////
// 'a' Активно
// 'b' Не активно
// 'c' Нет значения
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(PostList)]
pub struct PostListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub post_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub can_see_comment: Option<String>,
    pub create_item:     Option<String>,
    pub create_comment:  Option<String>,
    pub can_copy:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_list_perms"]
pub struct NewPostListPerm {
    pub user_id:         i32,
    pub post_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub can_see_comment: Option<String>,
    pub create_item:     Option<String>,
    pub create_comment:  Option<String>,
    pub can_copy:        Option<String>,
}


/////// PostVote//////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Post)]
pub struct PostVote {
    pub id:              i32,
    pub vote:            i16,
    pub user_id:         i32,
    pub post_id:         i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_votes"]
pub struct NewPostVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub post_id:         i32,
}


/////// PostCommentVote //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(PostComment)]
pub struct PostCommentVote {
    pub id:              i32,
    pub vote:            i16,
    pub user_id:         i32,
    pub post_comment_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_comment_votes"]
pub struct NewPostCommentVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub post_comment_id: i32,
}
