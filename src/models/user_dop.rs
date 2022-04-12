use crate::schema::{
    user_profile,
    user_location,
    ip_user,
    user_anketa,
    user_delete_anketa,
    user_love_status,
    user_partner_one,
    user_mom_one,
    user_dad_one,
    user_brother_sister,
    user_children_one,
    user_grandsons_one,
    user_colleagues_one,
    user_blocks,
    list_uc,
    featured_uc,
    news_uc,
    notify_uc,
    user_photo_list_position,
    user_post_list_position,
    user_music_list_position,
    user_good_list_position,
    user_video_list_position,
    user_survey_list_position,
    user_doc_list_position,
    color_settings,
    user_private,
    user_profile_notifications,
    user_post_notifications,
    user_photo_notifications,
    user_video_notifications,
    user_good_notifications,
    user_music_notifications,
    user_populate_smiles,
    user_populate_stickers,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{
    User,
    Sticker,
    Smile,
};


#[derive(Debug ,Queryable, Serialize, Identifiable)]
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
    pub _time:          chrono::NaiveDateTime,
    pub height:         Option<f32>,
    pub activity:       Option<String>,
    pub interests:      Option<String>,
    pub favorite_music: Option<String>,
    pub favorite_films: Option<String>,
    pub favorite_books: Option<String>,
    pub favorite_game:  Option<String>,
    pub about:          Option<String>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_profile"]
pub struct NewUserProfile {
    pub user_id: i32,
}

/////// UserLocation //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
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

#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_location"]
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
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
pub struct IpUser {
    pub id:       i32,
    pub user_id:  i32,
    pub ip:       String,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="ip_user"]
pub struct NewIpUser {
    pub user_id: i32,
    pub ip:      String,
}

/////// UserAnketa //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
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
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_anketa"]
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

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
pub struct UserDeleteAnketa {
    pub id:      i32,
    pub user_id: i32,
    pub answer:  char,
    pub other:   Option<String>,
    pub created: chrono::NaiveDateTime,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_delete_anketa"]
pub struct NewUserAnketa {
    pub user_id: i32,
    pub answer:  char,
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

#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
pub struct UserLoveStatus {
    pub id:             i32,
    pub user_id:        i32,
    pub male_status:    char,
    pub female_status:  char,
}

/////// UserPartnerOne //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User), foreign_key="user_partners")]
#[belongs_to(User), foreign_key="partner_by_users")]
pub struct UserPartnerOne {
    pub id:         i32,
    pub user_id:    i32,
    pub partner_id: i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_partner_one"]
pub struct NewUserPartnerOne {
    pub user_id:    i32,
    pub partner_id: i32,
}

/////// UserMomOne //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User), foreign_key="user_mom")]
#[belongs_to(User), foreign_key="mom_by_users")]
pub struct UserMomOne {
    pub id:      i32,
    pub user_id: i32,
    pub mom_id:  i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_mom_one"]
pub struct NewUserMomOne {
    pub user_id: i32,
    pub mom_id:  i32,
}

/////// UserDadOne //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User), foreign_key="user_dad")]
#[belongs_to(User), foreign_key="dad_by_users")]
pub struct UserDadOne {
    pub id:      i32,
    pub user_id: i32,
    pub dad_id:  i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_dad_one"]
pub struct NewUserDadOne {
    pub user_id: i32,
    pub dad_id:  i32,
}

/////// UserBrothersSisters //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User), foreign_key="user_bro_sist")]
#[belongs_to(User), foreign_key="brother_sister_by_users")]
pub struct UserBrothersSisters {
    pub id:         i32,
    pub user_id:    i32,
    pub target_id:  i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_brother_sister"]
pub struct NewUserBrothersSisters {
    pub user_id:    i32,
    pub target_id:  i32,
}

/////// UserChildren //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User), foreign_key="user_children")]
#[belongs_to(User), foreign_key="children_by_users")]
pub struct UserChildren {
    pub id:       i32,
    pub user_id:  i32,
    pub child_id: i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_children_one"]
pub struct NewUserChildren {
    pub user_id:  i32,
    pub child_id: i32,
}

/////// UserGrandsons //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User), foreign_key="user_grandsons")]
#[belongs_to(User), foreign_key="grandsons_by_users")]
pub struct UserGrandsons {
    pub id:          i32,
    pub user_id:     i32,
    pub grandson_id: i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_grandsons_one"]
pub struct NewUserGrandsons {
    pub user_id:     i32,
    pub grandson_id: i32,
}

/////// UserColleagues //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User), foreign_key="user_colleagues")]
#[belongs_to(User), foreign_key="colleagues_by_users")]
pub struct UserColleagues {
    pub id:           i32,
    pub user_id:      i32,
    pub colleague_id: i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_colleagues_one"]
pub struct NewUserColleagues {
    pub user_id:      i32,
    pub colleague_id: i32,
}

/////// UserBlocks //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User, foreign_key="user_blocks")]
#[belongs_to(User, foreign_key="blocked_by_users")]
pub struct UserBlocks {
    pub id:              i32,
    pub user_id:         i32,
    pub blocked_user_id: i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_blocks"]
pub struct NewUserBlocks {
    pub user_id:         i32,
    pub blocked_user_id: i32,
}

//////////////////////////////////////////////////////
/////// ListUC //////
    // 'b' Не активный
    // 'a' Активный список

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct ListUC {
    pub id:     i32,
    pub types:  char,
    pub name:   String,
    pub owner:  i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="list_uc"]
pub struct NewListUC {
    pub types: char,
    pub name:  String,
    pub owner: i32,
}

/////// FeaturedUC //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(ListUC)]
pub struct FeaturedUC {
    pub id:           i32,
    pub owner:        i32,
    pub list_id:      i32,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         Bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="featured_uc"]
pub struct NewFeaturedUC {
    pub owner:        i32,
    pub list_id:      i32,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         Bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}

/////// NewsUC //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(ListUC)]
pub struct NewsUC {
    pub id:           i32,
    pub owner:        i32,
    pub list_id:      i32,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         Bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="news_uc"]
pub struct NewNewsUC {
    pub owner:        i32,
    pub list_id:      i32,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         Bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}

/////// NotifyUC //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(ListUC)]
pub struct NotifyUC {
    pub id:           i32,
    pub owner:        i32,
    pub list_id:      i32,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         Bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="notify_uc"]
pub struct NewNotifyUC {
    pub owner:        i32,
    pub list_id:      i32,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         Bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}
/////====================================////

//////////////////////////////////////////////////////
/////// UserPhotoListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct UserPhotoListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_photo_list_position"]
pub struct NewUserPhotoListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char,
}

/////// UserPostListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct UserPostListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_post_list_position"]
pub struct NewUserPostListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char,
}

/////// UserMusicListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct UserMusicListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_music_list_position"]
pub struct NewUserMusicListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char,
}

/////// UserGoodListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct UserGoodListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_good_list_position"]
pub struct NewUserGoodListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char,
}

/////// UserVideoListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct UserVideoListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_video_list_position"]
pub struct NewUserVideoListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char,
}

/////// UserSurveyListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct UserSurveyListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_survey_list_position"]
pub struct NewUserSurveyListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char,
}

/////// UserDocListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct UserDocListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char, // 1 - open, 2 - close
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_doc_list_position"]
pub struct NewUserDocListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    char,
}

/////// UserPrivate //////
    // 'a' Все пользователи
    // 'b' Друзья
    // 'c' Друзья и друзья друзей
    // 'd' Только я
    // 'e' Друзья, кроме
    // 'f' Некоторые друзья
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
pub struct UserPrivate {
    pub id:                 i32,
    pub user_id:            char,
    pub can_see_community:  char,
    pub can_see_info:       char,
    pub can_see_friend:     char,
    pub can_send_message:   char,
    pub can_add_in_chat:    char,
    pub can_see_post:       char,
    pub can_see_photo:      char,
    pub can_see_good:       char,
    pub can_see_video:      char,
    pub can_see_music:      char,
    pub can_see_planner:    char,
    pub can_see_doc:        char,
    pub can_see_survey:     char,
}

/////// UserPopulateSmiles //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Smiles)]
pub struct UserPopulateSmiles {
    pub id:       i32,
    pub user_id:  i32,
    pub smile_id: i32,
    pub count:    i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_populate_smiles"]
pub struct NewUserPopulateSmiles {
    pub user_id:   i32,
    pub smile_id:  i32,
    pub count:     i32,
}

/////// UserPopulateStickers //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Stickers)]
pub struct UserPopulateStickers {
    pub id:         i32,
    pub user_id:    i32,
    pub sticker_id: i32,
    pub count:      i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_populate_stickers"]
pub struct NewUserPopulateStickers {
    pub user_id:    i32,
    pub sticker_id: i32,
    pub count:      i32,
}
