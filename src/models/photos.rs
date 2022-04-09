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
    Stickers,
};


/////// PhotoList //////
pub enum PhotoListTypes{
    Main,           // основной список
    List,           // пользовательский список
    Draft,          // список предложки
    Deleted,        // удаленный список
    Closed,         // закрытый список
    ClosedMain,     // закрытый основной список
    Suspended,      // замороженный список
    SuspendedMain,  // замороженный основной список
}

pub enum PhotoListPerm{
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
    pub types:           PhotoListTypes,
    pub description:     Option<String>,
    pub created:         chrono::NaiveDateTime,
    pub count:           i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i32,
    pub can_see_el:      PhotoListPerm,
    pub can_see_comment: PhotoListPerm,
    pub create_el:       PhotoListPerm,
    pub create_comment:  PhotoListPerm,
    pub copy_el:         PhotoListPerm,
}

/////// Photo //////
pub enum PhotoTypes{
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
#[belongs_to(User)]
#[belongs_to(PhotoList)]
#[table_name="photos"]
pub struct Photo {
    pub id:              i32,
    pub title:         Option<String>,
    pub community_id:    Option<i32>,
    pub creator_id:      i32,
    pub list_id:         i32,
    pub types:           PhotoTypes,
    pub preview:     Option<String>,
    pub file:     Option<String>,
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
pub enum PhotoCommentTypes{
    Published,      // Опубликованный
    Edited,         // Изменённый
    Draft,          // Черновик
    Deleted,        // Удаленый
    DeletedEdited,  // Изменённый Удаленый
    Closed,         // Закрытый модератором
    ClosedEdited,   // Закрытый Удаленый
}
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
    pub types:      PhotoCommentTypes,
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
pub enum PhotoListPermTypes{
    NoValue,    // Нет значения
    Enable,     // Активно
    Disable,    // Не активно
}
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(PhotoList)]
#[table_name="photo_list_perm"]
pub struct PhotoListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub list_id:         i32,
    pub can_see_item:    PhotoListPermTypes,
    pub can_see_comment: PhotoListPermTypes,
    pub create_item:     PhotoListPermTypes,
    pub create_comment:  PhotoListPermTypes,
    pub can_copy:        PhotoListPermTypes,
}
