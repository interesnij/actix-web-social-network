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

/////// Community //////

/////// Тип сообщества //////
    // 1 приватное сообщество
    // 2 закрытое сообщество
    // 3 публичное сообщество
    // 13 удаленное публичное
    // 11 удаленное приватное
    // 12 удаленное закрытое
    // 23 баннер публичный
    // 21 баннер приватный
    // 22 баннер закрытый
    // 33 заблокированное публичное
    // 31 заблокированное приватное
    // 32 заблокированное закрытое
    // 43 приостановленное публичное
    // 41 приостановленное приватное
    // 42 приостановленное закрытое
}

/////// Статус сообщества //////
    // 'a' стандартное сообщество
    // 'b' детское сообщество
    // 'c' подавшее на идентификацию сообщество
    // 'd' идентификацированное сообщество
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
    pub types:       i16,
    pub perm:        char,
    pub level:       i16,
    pub have_link:   Option<String>,
    pub b_avatar:    Option<String>,
    pub s_avatar:    Option<String>,
    pub cover:       Option<String>,
    pub category_id: i32,
    pub creator_id:  i32,
    pub created:     chrono::NaiveDateTime,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="communities"]
pub struct NewCommunity {
    pub name:        String,
    pub types:       i16,
    pub perm:        String,
    pub level:       i16,
    pub category_id: i32,
    pub creator_id:  i32,
    pub created:     chrono::NaiveDateTime,
}

/////// CommunityMembership //////
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

/////// CommunityMemberPerm //////
/////// Исключения/ включения подписчиков сообщества //////
    // 'c' Нет значения
    // 'a' Активно
    // 'b' Не активно

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(CommunityMembership)]
#[table_name="community_ie_settings"]
pub struct CommunityMemberPerm {
    pub id:                      i32,
    pub user_id:                 i32,

    pub can_see_info:            Option<char>,
    pub can_see_member:          Option<char>,
    pub can_send_message:        Option<char>,
    pub can_see_doc:             Option<char>,
    pub can_see_music:           Option<char>,
    pub can_see_survey:          Option<char>,
    pub can_see_post:            Option<char>,
    pub can_see_post_comment:    Option<char>,
    pub can_see_photo:           Option<char>,
    pub can_see_photo_comment:   Option<char>,
    pub can_see_good:            Option<char>,
    pub can_see_good_comment:    Option<char>,
    pub can_see_video:           Option<char>,
    pub can_see_video_comment:   Option<char>,
    pub can_see_planner:         Option<char>,
    pub can_see_planner_comment: Option<char>,

    pub can_add_post:            Option<char>,
    pub can_add_photo:           Option<char>,
    pub can_add_good:            Option<char>,
    pub can_add_video:           Option<char>,
    pub can_add_planner:         Option<char>,
    pub can_add_doc:             Option<char>,
    pub can_add_music:           Option<char>,
    pub can_add_survey:          Option<char>,

    pub can_create_post:         Option<char>,
    pub can_create_photo:        Option<char>,
    pub can_create_good:         Option<char>,
    pub can_create_video:        Option<char>,
    pub can_create_planner:      Option<char>,
    pub can_create_doc:          Option<char>,
    pub can_create_music:        Option<char>,
    pub can_create_survey:       Option<char>,
}

/////// CommunityInfo //////
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
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_info"]
pub struct NewCommunityInfo {
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

/////// CommunityPrivate //////
    // 'a' Все пользователи
    // 'b' Подписчики
    // 'c' Создатель
    // 'd' Подписчики, кроме
    // 'e' Некоторые подписчики
    // 'f' Персонал

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[table_name="community_private"]
pub struct CommunityPrivate {
    pub id:               i32,
    pub community_id:     i32,
    pub can_see_member:   char,
    pub can_see_info:     char,
    pub can_send_message: char,
    pub can_see_post:     char,
    pub can_see_photo:    char,
    pub can_see_good:     char,
    pub can_see_video:    char,
    pub can_see_music:    char,
    pub can_see_planner:  char,
    pub can_see_doc:      char,
    pub can_see_survey:   char,
    pub can_see_settings: char,
    pub can_see_log:      char,
    pub can_see_stat:     char,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_info"]
pub struct NewCommunityPrivate {
    pub community_id:     i32,
    pub can_see_member:   String,
    pub can_see_info:     String,
    pub can_send_message: String,
    pub can_see_post:     String,
    pub can_see_photo:    String,
    pub can_see_good:     String,
    pub can_see_video:    String,
    pub can_see_music:    String,
    pub can_see_planner:  String,
    pub can_see_doc:      String,
    pub can_see_survey:   String,
    pub can_see_settings: String,
    pub can_see_log:      String,
    pub can_see_stat:     String,
}

/////// CommunityNotifications //////
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
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_notifications"]
pub struct NewCommunityNotifications {
    pub community_id:         i32,
    pub connection_request:   Bool,
    pub connection_confirmed: Bool,
    pub community_invite:     Bool,
}

/////// CommunityNotificationsPost //////
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
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_post_notifications"]
pub struct NewCommunityNotificationsPost {
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

/////// CommunityNotificationsPhoto //////
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
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_photo_notifications"]
pub struct NewCommunityNotificationsPhoto {
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

/////// CommunityNotificationsVideo //////
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
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_video_notifications"]
pub struct NewCommunityNotificationsVideo {
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

/////// CommunityNotificationsGood //////
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
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_good_notifications"]
pub struct NewCommunityNotificationsGood {
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

/////// CommunityNotificationsMusic //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[table_name="community_music_notifications"]
pub struct CommunityNotificationsMusic {
    pub id:                     i32,
    pub community_id:           i32,
    pub repost:                 Bool,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_music_notifications"]
pub struct NewCommunityNotificationsMusic {
    pub community_id:           i32,
    pub repost:                 Bool,
}

/////// CommunityPhotoListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityPhotoListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_photo_list_position"]
pub struct NewCommunityPhotoListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    String,
}

/////// CommunityPostListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityPostListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_post_list_position"]
pub struct NewCommunityPostListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    String,
}

/////// CommunityMusicListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityMusicListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_music_list_position"]
pub struct NewCommunityMusicListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    String,
}

/////// CommunityGoodListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityGoodListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_good_list_position"]
pub struct NewCommunityGoodListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    String,
}

/////// CommunityVideoListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityVideoListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_video_list_position"]
pub struct NewCommunityVideoListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    String,
}

/////// CommunitySurveyListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunitySurveyListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_survey_list_position"]
pub struct NewCommunitySurveyListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    String,
}

/////// CommunityDocListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityDocListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_doc_list_position"]
pub struct NewCommunityDocListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i32,
    pub types:    String,
}
