//use crate::schema;
//use diesel::prelude::*;
use crate::schema::{
    user_profiles,
    user_locations,
    ip_users,
    user_anketas,
    user_delete_anketas,
    user_love_statuss,
    user_partner_ones,
    user_mom_ones,
    user_dad_ones,
    user_brother_sisters,
    user_children_ones,
    user_grandsons_ones,
    user_colleagues_ones,
    user_blocks,
    list_user_communities_keys,
    featured_user_communities,
    news_user_communities,
    notify_user_communities,
    user_photo_list_positions,
    user_post_list_positions,
    user_music_list_positions,
    user_good_list_positions,
    user_video_list_positions,
    user_survey_list_positions,
    user_doc_list_positions,
    design_settings,
    user_privates,
    user_notifications,
    //user_profile_notifications,
    user_post_notifications,
    user_photo_notifications,
    user_video_notifications,
    user_good_notifications,
    user_music_notifications,
    user_survey_notifications,
    user_populate_smiles,
    user_populate_stickers,
    user_reposts,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
//use crate::utils::establish_connection;
use crate::models::{
    User,
    Sticker,
    Smile,
    Message,
    Post,
};


#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserProfile {
    pub id:             i32,
    pub user_id:        i32,
    pub posts:          i32,
    pub views_post:     i32,
    pub friends:        i32,
    pub follows:        i32,
    pub communities:    i32,
    pub photos:         i32,
    pub goods:          i32,
    pub docs:           i32,
    pub tracks:         i32,
    pub videos:         i32,
    pub articles:       i32,
    //pub messages:     i32,
    pub planners:       i32,
    pub avatar_id:      Option<i32>,
    pub activity:       Option<String>,
    pub interests:      Option<String>,
    pub favorite_music: Option<String>,
    pub favorite_films: Option<String>,
    pub favorite_books: Option<String>,
    pub favorite_game:  Option<String>,
    pub about:          Option<String>,
    pub survey:         i32,
    pub playlist:       i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="user_profiles"]
pub struct NewUserProfile {
    pub user_id:        i32,
    pub posts:          i32,
    pub views_post:     i32,
    pub friends:        i32,
    pub follows:        i32,
    pub communities:    i32,
    pub photos:         i32,
    pub goods:          i32,
    pub docs:           i32,
    pub tracks:         i32,
    pub videos:         i32,
    pub articles:       i32,
    //pub messages:     i32,
    pub planners:       i32,
    pub avatar_id:      Option<i32>,
    pub activity:       Option<String>,
    pub interests:      Option<String>,
    pub favorite_music: Option<String>,
    pub favorite_films: Option<String>,
    pub favorite_books: Option<String>,
    pub favorite_game:  Option<String>,
    pub about:          Option<String>,
    pub survey:         i32,
    pub playlist:       i32,
}

/////// UserLocation //////
#[derive(Queryable, Serialize, Deserialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserLocation {
    pub id:         i32,
    pub user_id:    i32,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name="user_locations"]
pub struct NewUserLocation {
    pub user_id:    i32,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
}

/////// UserLocation //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct IpUser {
    pub id:       i32,
    pub user_id:  i32,
    pub ip:       String,
}
#[derive(Deserialize, Insertable)]
#[table_name="ip_users"]
pub struct NewIpUser {
    pub user_id: i32,
    pub ip:      String,
}

/////// UserAnketa //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserAnketa {
    pub id:                    i32,
    pub user_id:               i32,
    pub political_preferences: Option<String>,
    pub worldview:             Option<String>,
    pub mainthing_in_life:     Option<String>,
    pub mainthing_in_people:   Option<String>,
    pub attitude_to_smoking:   Option<String>,
    pub attitude_to_alcohol:   Option<String>,
    pub inspiration:           Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_anketas"]
pub struct NewUserAnketa {
    pub user_id: i32,
}

/////// UserDeleteAnketa //////
    // 'a' "У меня есть другая страница",
    // 'b' "Соцсеть отнимает много времени",
    // 'c' "Мало свободы самовыражения",
    // 'd' "Соцсеть плохо защищает данные",
    // 'e' "Соцсеть плохо защищает детей",
    // 'f' "Другая причина",

#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserDeleteAnketa {
    pub id:      i32,
    pub user_id: i32,
    pub answer:  String,
    pub other:   Option<String>,
    pub created: chrono::NaiveDateTime,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_delete_anketas"]
pub struct NewUserDeleteAnketa {
    pub user_id: i32,
    pub answer:  String,
    pub other:   Option<String>,
    pub created: chrono::NaiveDateTime,
}

/////// UserLoveStatus //////

// 'a' "Не выбрано",
// 'b' "Не женат",
// 'c' "Есть подруга",
// 'd' "Помолвлен",
// 'e' "Женат",
// 'f' "В гражданском браке",
// 'g' "Влюблён",
// 'h' "Всё сложно",
// 'i' "В активном поиске",

// 'a' "Не выбрано",
// 'b' "Не женат",
// 'c' "Есть подруга",
// 'd' "Помолвлен",
// 'e' "Женат",
// 'f' "В гражданском браке",
// 'g' "Влюблён",
// 'h' "Всё сложно",
// 'i' "В активном поиске",

#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserLoveStatus {
    pub id:             i32,
    pub user_id:        i32,
    pub male_status:    String,
    pub female_status:  String,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_love_statuss"]
pub struct NewUserLoveStatus {
    pub user_id:        i32,
    pub male_status:    String,
    pub female_status:  String,
}

/////// UserPartnerOne //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
pub struct UserPartnerOne {
    pub id:         i32,
    pub partner_user_i:    i32,
    pub partner_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_partner_ones"]
pub struct NewUserPartnerOne {
    pub partner_user_i:    i32,
    pub partner_id: i32,
}

/////// UserMomOne //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserMomOne {
    pub id:      i32,
    pub mom_user_i: i32,
    pub mom_id:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_mom_ones"]
pub struct NewUserMomOne {
    pub mom_user_i: i32,
    pub mom_id:  i32,
}

/////// UserDadOne //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserDadOne {
    pub id:      i32,
    pub dad_user_i: i32,
    pub dad_id:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_dad_ones"]
pub struct NewUserDadOne {
    pub dad_user_i: i32,
    pub dad_id:  i32,
}

/////// UserBrothersSisters //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserBrotherSister {
    pub id:         i32,
    pub brother_user_i:    i32,
    pub brother_target_id:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_brother_sisters"]
pub struct NewUserBrotherSister {
    pub brother_user_i:    i32,
    pub brother_target_id:  i32,
}

/////// UserChildren //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserChildrenOne {
    pub id:       i32,
    pub child_user_i:  i32,
    pub child_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_children_ones"]
pub struct NewUserChildrenOne {
    pub child_user_i:  i32,
    pub child_id: i32,
}

/////// UserGrandsons //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserGrandsonsOne {
    pub id:          i32,
    pub grandson_user_i:     i32,
    pub grandson_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_grandsons_ones"]
pub struct NewUserGrandsonsOne {
    pub grandson_user_i:     i32,
    pub grandson_id: i32,
}

/////// UserColleagues //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserColleaguesOne {
    pub id:           i32,
    pub user_colleague_i:      i32,
    pub colleague_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_colleagues_ones"]
pub struct NewUserColleaguesOne {
    pub user_colleague_i:      i32,
    pub colleague_id: i32,
}

/////// UserBlocks //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserBlock {
    pub id:              i32,
    pub user_block_i:         i32,
    pub blocked_user_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_blocks"]
pub struct NewUserBlock {
    pub user_block_i:         i32,
    pub blocked_user_id: i32,
}

//////////////////////////////////////////////////////
/////// ListUC //////
    // 'b' Не активный
    // 'a' Активный список

#[derive(Queryable, Serialize, Identifiable)]
pub struct ListUserCommunitiesKey {
    pub id:     i32,
    pub types:  String,
    pub name:   String,
    pub owner:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="list_user_communities_keys"]
pub struct NewListUserCommunitiesKey {
    pub types: String,
    pub name:  String,
    pub owner: i32,
}

/////// FeaturedUC //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
pub struct FeaturedUserCommunitie {
    pub id:           i32,
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}
#[derive(Deserialize, Insertable)]
#[table_name="featured_user_communities"]
pub struct NewFeaturedUserCommunitie {
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}

/////// NewsUC //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
pub struct NewsUserCommunitie {
    pub id:           i32,
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}
#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="news_user_communities"]
pub struct NewNewsUserCommunitie {
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}

/////// NotifyUC //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
pub struct NotifyUserCommunitie {
    pub id:           i32,
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}
#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="notify_user_communities"]
pub struct NewNotifyUserCommunitie {
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}
/////====================================////

//////////////////////////////////////////////////////
/////// UserPhotoListPosition //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserPhotoListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="user_photo_list_positions"]
pub struct NewUserPhotoListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}

/////// UserPostListPosition //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserPostListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // a - open, b - close
}
#[derive(Deserialize, Insertable)]
#[table_name="user_post_list_positions"]
pub struct NewUserPostListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}

/////// UserMusicListPosition //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserMusicListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="user_music_list_positions"]
pub struct NewUserMusicListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}

/////// UserGoodListPosition //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserGoodListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="user_good_list_positions"]
pub struct NewUserGoodListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}

/////// UserVideoListPosition //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserVideoListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="user_video_list_positions"]
pub struct NewUserVideoListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}

/////// UserSurveyListPosition //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserSurveyListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="user_survey_list_positions"]
pub struct NewUserSurveyListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}

/////// UserDocListPosition //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserDocListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="user_doc_list_positions"]
pub struct NewUserDocListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}

/////// UserPrivate //////
    // 'a' Все пользователи
    // 'b' Друзья
    // 'c' Друзья и друзья друзей
    // 'd' Только я
    // 'e' Друзья, кроме
    // 'f' Некоторые друзья

#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserPrivate {
    pub id:                 i32,
    pub user_id:            i32,
    pub can_see_all:        String,
    pub can_see_community:  String,
    pub can_see_info:       String,
    pub can_see_friend:     String,
    pub can_send_message:   String,
    pub can_add_in_chat:    String,
    pub can_see_post:       String,
    pub can_see_photo:      String,
    pub can_see_good:       String,
    pub can_see_video:      String,
    pub can_see_music:      String,
    pub can_see_planner:    String,
    pub can_see_doc:        String,
    pub can_see_survey:     String,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_privates"]
pub struct NewUserPrivate {
    pub user_id:            i32,
    pub can_see_all:        String,
    pub can_see_community:  String,
    pub can_see_info:       String,
    pub can_see_friend:     String,
    pub can_send_message:   String,
    pub can_add_in_chat:    String,
    pub can_see_post:       String,
    pub can_see_photo:      String,
    pub can_see_good:       String,
    pub can_see_video:      String,
    pub can_see_music:      String,
    pub can_see_planner:    String,
    pub can_see_doc:        String,
    pub can_see_survey:     String,
}

/////// UserPopulateSmiles //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Smile)]
pub struct UserPopulateSmile {
    pub id:       i32,
    pub user_id:  i32,
    pub smile_id: i32,
    pub count:    i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_populate_smiles"]
pub struct NewUserPopulateSmile {
    pub user_id:   i32,
    pub smile_id:  i32,
    pub count:     i32,
}

/////// UserPopulateStickers //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Sticker)]
pub struct UserPopulateSticker {
    pub id:         i32,
    pub user_id:    i32,
    pub sticker_id: i32,
    pub count:      i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_populate_stickers"]
pub struct NewUserPopulateSticker {
    pub user_id:    i32,
    pub sticker_id: i32,
    pub count:      i32,
}


/////// UserNotifications //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserNotification {
    pub id:                   i32,
    pub user_id:         i32,
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub user_invite:     bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_notifications"]
pub struct NewUserNotification {
    pub user_id:         i32,
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub user_invite:     bool,
}

/////// UserNotificationsPost //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserPostNotification {
    pub id:                     i32,
    pub user_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub reactions:              bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_post_notifications"]
pub struct NewUserPostNotification {
    pub user_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub reactions:              bool,
}

/////// UserSurveyNotification //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserSurveyNotification {
    pub id:                  i32,
    pub user_id:             i32,
    pub vote:                bool,
    pub repost:              bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_survey_notifications"]
pub struct NewUserSurveyNotification {
    pub user_id:             i32,
    pub vote:                bool,
    pub repost:              bool,
}

/////// UserNotificationsPhoto //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserPhotoNotification {
    pub id:                     i32,
    pub user_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub reactions:              bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_photo_notifications"]
pub struct NewUserPhotoNotification {
    pub user_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub reactions:              bool,
}

/////// UserNotificationsVideo //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserVideoNotification {
    pub id:                     i32,
    pub user_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub reactions:              bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_video_notifications"]
pub struct NewUserVideoNotification {
    pub user_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub reactions:              bool,
}

/////// UserNotificationsGood //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserGoodNotification {
    pub id:                     i32,
    pub user_id:                i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub reactions:              bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_good_notifications"]
pub struct NewUserGoodNotification {
    pub user_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub reactions:              bool,
}

/////// UserNotificationsMusic //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserMusicNotification {
    pub id:       i32,
    pub user_id:  i32,
    pub repost:   bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_music_notifications"]
pub struct NewUserMusicNotification {
    pub user_id:   i32,
    pub repost:    bool,
}

/////// design_settings //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct DesignSetting {
    pub id:         i32,
    pub user_id:    i32,
    pub background: String,
}
#[derive(Deserialize, Insertable)]
#[table_name="design_settings"]
pub struct NewDesignSetting {
    pub user_id:    i32,
    pub background: String,
}
#[derive(Deserialize, AsChangeset)]
#[table_name="design_settings"]
pub struct EditDesignSetting {
    pub background: String,
}

/////// UserRepost //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Post)]
#[belongs_to(Message)]
pub struct UserRepost {
    pub id:         i32,
    pub user_id:    i32,
    pub post_id:    Option<i32>,
    pub message_id: Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_reposts"]
pub struct NewUserRepost {
    pub user_id:    i32,
    pub post_id:    Option<i32>,
    pub message_id: Option<i32>,
}
