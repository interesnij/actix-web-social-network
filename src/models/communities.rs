use crate::schema::{
    community_categories,
    community_subcategories,
    communities,
    communities_memberships,
    community_ie_settings,
    community_info,
    community_private,
    community_notifications,
    community_post_notifications,
    community_photo_notifications,
    community_video_notifications,
    community_good_notifications,
    community_music_notifications,
    community_photo_list_position,
    community_post_list_position,
    community_music_list_position,
    community_good_list_position,
    community_video_list_position,
    community_survey_list_position,
    community_doc_list_position,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;

/////// CommunityCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityCategories {
    pub id:       i32,
    pub name:     String,
    pub avatar:   Option<String>,
    pub position: i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_categories"]
pub struct NewCommunityCategories {
    pub name:     String,
    pub avatar:   Option<String>,
    pub position: i32,
}

/////// CommunitySubCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunitySubCategories {
    pub id:          i32,
    pub name:        String,
    pub category_id: i32,
    pub avatar:      Option<String>,
    pub position:    i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_subcategories"]
pub struct NewCommunitySubCategories {
    pub name:        String,
    pub category_id: i32,
    pub avatar:      Option<String>,
    pub position:    i32,
}


pub enum CommunityTypes {
    Private,           // приватное сообщество
    Close,             // закрытое сообщество
    Public,            // публичное сообщество
    DeletedPublic,     // удаленное публичное
    DeletedPrivate,    // удаленное приватное
    DeletedClose,      // удаленное закрытое
    BannerPublic,      // баннер публичный
    BannerPrivate,     // баннер приватный
    BannerClose,       // баннер закрытый
    ClosedPublic,      // заблокированное публичное
    ClosedPrivate,     // заблокированное приватное
    ClosedClose,       // заблокированное закрытое
    SuspendedPublic,   // приостановленный стандартный
    SuspendedPrivate,  // приостановленный ребенок
    SuspendedClose,    // приостановленный идентифицированный
}

pub enum CommunityPerm {
    Standart,          // стандартное сообщество
    Child,             // детское сообщество
    IdentifiedSend,    // подавшее на идентификацию сообщество
    Identified,        // идентификацированное сообщество
}

/////// Community //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(CommunitySubCategories)]
#[belongs_to(User)]
#[table_name="communities"]
pub struct Community {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub status:      Option<String>,
    pub types:       CommunityTypes,
    pub perm:        CommunityPerm,
    pub level:       i32,
    pub have_link:   Option<String>,
    pub b_avatar:    Option<String>,
    pub s_avatar:    Option<String>,
    pub cover:       Option<String>,
    pub category_id: i32,
    pub creator_id:  i32,
    pub created:     chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Community)]
#[table_name="communities_memberships"]
pub struct CommunityMembership {
    pub id:               i32,
    pub user_id:          i32,
    pub community_id:     i32,
    pub is_administrator: Bool,
    pub is_moderator:     Bool,
    pub is_editor:        Bool,
    pub is_advertiser:    Bool,
    pub created:          chrono::NaiveDateTime,
    pub visited:          i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="communities_memberships"]
pub struct NewCommunityMembership {
    pub user_id:          i32,
    pub community_id:     i32,
    pub is_administrator: Bool,
    pub is_moderator:     Bool,
    pub is_editor:        Bool,
    pub is_advertiser:    Bool,
    pub created:          chrono::NaiveDateTime,
    pub visited:          i32,
}

pub enum CommunityMemberPermTypes{
    NoValue,    // Нет значения
    Enable,     // Активно
    Disable,    // Не активно
}
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(CommunityMembership)]
#[table_name="community_ie_settings"]
pub struct CommunityMemberPerm {
    pub id:                      i32,
    pub user_id:                 i32,

    pub can_see_info:            CommunityMemberPermTypes,
    pub can_see_member:          CommunityMemberPermTypes,
    pub can_send_message:        CommunityMemberPermTypes,
    pub can_see_doc:             CommunityMemberPermTypes,
    pub can_see_music:           CommunityMemberPermTypes,
    pub can_see_survey:          CommunityMemberPermTypes,
    pub can_see_post:            CommunityMemberPermTypes,
    pub can_see_post_comment:    CommunityMemberPermTypes,
    pub can_see_photo:           CommunityMemberPermTypes,
    pub can_see_photo_comment:   CommunityMemberPermTypes,
    pub can_see_good:            CommunityMemberPermTypes,
    pub can_see_good_comment:    CommunityMemberPermTypes,
    pub can_see_video:           CommunityMemberPermTypes,
    pub can_see_video_comment:   CommunityMemberPermTypes,
    pub can_see_planner:         CommunityMemberPermTypes,
    pub can_see_planner_comment: CommunityMemberPermTypes,

    pub can_add_post:            CommunityMemberPermTypes,
    pub can_add_photo:           CommunityMemberPermTypes,
    pub can_add_good:            CommunityMemberPermTypes,
    pub can_add_video:           CommunityMemberPermTypes,
    pub can_add_planner:         CommunityMemberPermTypes,
    pub can_add_doc:             CommunityMemberPermTypes,
    pub can_add_music:           CommunityMemberPermTypes,
    pub can_add_survey:          CommunityMemberPermTypes,

    pub can_create_post:         CommunityMemberPermTypes,
    pub can_create_photo:        CommunityMemberPermTypes,
    pub can_create_good:         CommunityMemberPermTypes,
    pub can_create_video:        CommunityMemberPermTypes,
    pub can_create_planner:      CommunityMemberPermTypes,
    pub can_create_doc:          CommunityMemberPermTypes,
    pub can_create_music:        CommunityMemberPermTypes,
    pub can_create_survey:       CommunityMemberPermTypes,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[table_name="community_info"]
pub struct CommunityInfo {
    pub id:           i32,
    pub community_id: i32,
    pub posts:        i32,
    pub members:      i32,
    pub photos:       i32,
    pub goods:        i32,
    pub tracks:       i32,
    pub videos:       i32,
    pub docs:         i32,
    pub articles:     i32,
    pub survey:       i32,
}

pub enum CommunityPrivate{
    AllCan,          // Все пользователи
    Memberships,     // Подписчики
    Creator,         // Создатель
    MembershipsBut,  // Подписчики, кроме
    SomeMemberships, // Некоторые подписчики
    Staff,           // Персонал
}
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[table_name="community_private"]
pub struct CommunityPrivate {
    pub id:               i32,
    pub community_id:     i32,
    pub can_see_member:   CommunityPrivate,
    pub can_see_info:     CommunityPrivate,
    pub can_send_message: CommunityPrivate,
    pub can_see_post:     CommunityPrivate,
    pub can_see_photo:    CommunityPrivate,
    pub can_see_good:     CommunityPrivate,
    pub can_see_video:    CommunityPrivate,
    pub can_see_music:    CommunityPrivate,
    pub can_see_planner:  CommunityPrivate,
    pub can_see_doc:      CommunityPrivate,
    pub can_see_survey:   CommunityPrivate,
    pub can_see_settings: CommunityPrivate,
    pub can_see_log:      CommunityPrivate,
    pub can_see_stat:     CommunityPrivate,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[table_name="community_notifications"]
pub struct CommunityNotifications {
    pub id:                   i32,
    pub community_id:         i32,
    pub connection_request:   Bool,
    pub connection_confirmed: Bool,
    pub community_invite:     Bool,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[table_name="community_post_notifications"]
pub struct CommunityNotificationsPost {
    pub id:                     i32,
    pub community_id:           i32,
    pub comment:                Bool,
    pub comment_reply:          Bool,
    pub mention:                Bool,
    pub comment_mention:        Bool,
    pub repost:                 Bool,
    pub liked:                  Bool,
    pub disliked:               Bool,
    pub comment_liked:          Bool,
    pub comment_disliked:       Bool,
    pub comment_reply_liked:    Bool,
    pub comment_reply_disliked: Bool,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[table_name="community_photo_notifications"]
pub struct CommunityNotificationsPhoto {
    pub id:                     i32,
    pub community_id:           i32,
    pub comment:                Bool,
    pub comment_reply:          Bool,
    pub mention:                Bool,
    pub comment_mention:        Bool,
    pub repost:                 Bool,
    pub liked:                  Bool,
    pub disliked:               Bool,
    pub comment_liked:          Bool,
    pub comment_disliked:       Bool,
    pub comment_reply_liked:    Bool,
    pub comment_reply_disliked: Bool,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[table_name="community_video_notifications"]
pub struct CommunityNotificationsVideo {
    pub id:                     i32,
    pub community_id:           i32,
    pub comment:                Bool,
    pub comment_reply:          Bool,
    pub mention:                Bool,
    pub comment_mention:        Bool,
    pub repost:                 Bool,
    pub liked:                  Bool,
    pub disliked:               Bool,
    pub comment_liked:          Bool,
    pub comment_disliked:       Bool,
    pub comment_reply_liked:    Bool,
    pub comment_reply_disliked: Bool,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[table_name="community_good_notifications"]
pub struct CommunityNotificationsGood {
    pub id:                     i32,
    pub community_id:           i32,
    pub comment:                Bool,
    pub comment_reply:          Bool,
    pub mention:                Bool,
    pub comment_mention:        Bool,
    pub repost:                 Bool,
    pub liked:                  Bool,
    pub disliked:               Bool,
    pub comment_liked:          Bool,
    pub comment_disliked:       Bool,
    pub comment_reply_liked:    Bool,
    pub comment_reply_disliked: Bool,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[table_name="community_music_notifications"]
pub struct CommunityNotificationsMusic {
    pub id:                     i32,
    pub community_id:           i32,
    pub repost:                 Bool,
}


#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityPhotoListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_photo_list_position"]
pub struct NewCommunityPhotoListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32,
}

/////// CommunityPostListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityPostListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_post_list_position"]
pub struct NewCommunityPostListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32,
}

/////// CommunityMusicListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityMusicListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_music_list_position"]
pub struct NewCommunityMusicListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32,
}

/////// CommunityGoodListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityGoodListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_good_list_position"]
pub struct NewCommunityGoodListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32,
}

/////// CommunityVideoListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityVideoListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_video_list_position"]
pub struct NewCommunityVideoListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32,
}

/////// CommunitySurveyListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunitySurveyListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_survey_list_position"]
pub struct NewCommunitySurveyListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32,
}

/////// CommunityDocListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityDocListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_doc_list_position"]
pub struct NewCommunityDocListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    i32,
}
