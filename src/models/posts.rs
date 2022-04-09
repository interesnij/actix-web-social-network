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
pub enum PostListTypes{
    Main,           // основной список
    List,           // пользовательский список
    Draft,          // список предложки
    Deleted,        // удаленный список
    Closed,         // закрытый список
    ClosedMain,     // закрытый основной список
    Suspended,      // замороженный список
    SuspendedMain,  // замороженный основной список
}

pub enum PostListPerm{
    AllCan,       // Все пользователи
    Friends,      // Друзья
    EachOther,    // Друзья и друзья друзей
    FriendsBut,   // Друзья, кроме
    SomeFriends,  // Некоторые друзья
    Members,      // Подписчики
    Creator,      // Только я / владелец сообщества
    Admins,       // Администраторы
    MembersBut,   // Подписчики, кроме
    SomeMembers,  // Некоторые подписчики
}

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
    pub types:           PostListTypes,
    pub description:     Option<String>,
    pub created:         chrono::NaiveDateTime,
    pub count:           i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i32,
    pub can_see_el:      i32,
    pub can_see_comment: i32,
    pub create_el:       i32,
    pub create_comment:  i32,
    pub copy_el:         i32,
}

/////// Post //////
pub enum PostTypes{
    COffer,         // Предложка сообщества
    UOffer,         // Предложка пользователя
    CreatorDraft,   // Черновик владельца
    OfferDraft,     // Черновик предложки
    Fixed,          // Закрепленый
    Published,      // Опубликовано
    Deleted,        // Удаленый
    Closed,         // Закрыто модератором
    DeletedCOffer,  // Удаленый предложенный в сообщество
    DeletedUOffer,  // Удаленый предложенный у пользователя
}

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
    pub types:           PostTypes,
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
pub enum PostCommentTypes{
    Published,      // Опубликованный
    Edited,         // Изменённый
    Draft,          // Черновик
    Deleted,        // Удаленый
    DeletedEdited,  // Изменённый Удаленый
    Closed,         // Закрытый модератором
    ClosedEdited,   // Закрытый Удаленый
}
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
    pub types:      PostCommentTypes,
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
pub enum PostListPermTypes{
    NoValue,    // Нет значения
    Enable,     // Активно
    Disable,    // Не активно
}
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(PostList)]
#[table_name="post_list_perm"]
pub struct PostListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub list_id:         i32,
    pub can_see_item:    PostListPermTypes,
    pub can_see_comment: PostListPermTypes,
    pub create_item:     PostListPermTypes,
    pub create_comment:  PostListPermTypes,
    pub can_copy:        PostListPermTypes,
}
