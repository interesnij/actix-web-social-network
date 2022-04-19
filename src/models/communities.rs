use crate::schema::{
    community_categorys,
    community_subcategorys,
    communitys,
    communities_memberships,
    community_infos,
    community_privates,
    community_notifications,
    community_post_notifications,
    community_photo_notifications,
    community_video_notifications,
    community_good_notifications,
    community_music_notifications,
    community_photo_list_positions,
    community_post_list_positions,
    community_music_list_positions,
    community_good_list_positions,
    community_video_list_positions,
    community_survey_list_positions,
    community_doc_list_positions,
    community_visible_perms,
    community_work_perms,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::User;
use crate::schema;
use diesel::prelude::*;

/////// CommunityCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityCategory {
    pub id:       i32,
    pub name:     String,
    pub avatar:   Option<String>,
    pub position: i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_categorys"]
pub struct NewCommunityCategory {
    pub name:     String,
    pub avatar:   Option<String>,
    pub position: i16,
}

/////// CommunitySubCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunitySubcategory {
    pub id:          i32,
    pub name:        String,
    pub category_id: i32,
    pub avatar:      Option<String>,
    pub position:    i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_subcategorys"]
pub struct NewCommunitySubcategory {
    pub name:        String,
    pub category_id: i32,
    pub avatar:      Option<String>,
    pub position:    i16,
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

/////// Статус сообщества //////
    // 'a' стандартное сообщество
    // 'b' детское сообщество
    // 'c' подавшее на идентификацию сообщество
    // 'd' идентификацированное сообщество

/////// Community //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(CommunitySubcategory)]
#[belongs_to(User)]
pub struct Community {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub status:      String,
    pub types:       i16,
    pub perm:        String,
    pub level:       i16,
    pub have_link:   Option<String>,
    pub b_avatar:    Option<String>,
    pub s_avatar:    Option<String>,
    pub cover:       Option<String>,
    pub community_subcategory_id: i32,
    pub user_id:  i32,
    pub created:     chrono::NaiveDateTime,
}
#[derive(Deserialize, Insertable)]
#[table_name="communitys"]
pub struct NewCommunity {
    pub name:        String,
    pub status:      String,
    pub types:       i16,
    pub perm:        String,
    pub level:       i16,
    pub community_subcategory_id: i32,
    pub user_id:  i32,
    pub created:     chrono::NaiveDateTime,
}

impl Community {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn get_description(&self) -> String {
        return "<a href='".to_string() + &self.get_link() + &"' target='_blank'>".to_string() + &self.name + &"</a>".to_string();
    }
    pub fn is_community(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "com".to_string() + &self.get_str_id();
    }
    pub fn get_longest_penalties(&self) -> String {
        use crate::schema::moderated_penalties::dsl::moderated_penalties;
        use crate::models::ModeratedPenaltie;

        let _connection = establish_connection();

        let penaltie = moderated_penalties
            .filter(schema::moderated_penalties::object_id.eq(self.id))
            .filter(schema::moderated_penalties::types.eq(2))
            .load::<ModeratedPenaltie>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
        return penaltie.expiration.unwrap().format("%d/%m/%Y").to_string();
    }
    pub fn get_moderated_description(&self) -> String {
        use crate::schema::moderateds::dsl::moderateds;
        use crate::models::Moderated;

        let _connection = establish_connection();

        let moder = moderateds
            .filter(schema::moderateds::object_id.eq(self.id))
            .filter(schema::moderateds::types.eq(2))
            .load::<Moderated>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
        if moder.description.is_some() {
            return moder.description.unwrap().to_string();
        }
        else {
            return "Предупреждение за нарушение правил соцсети трезвый.рус".to_string();
        }
    }
    pub fn get_link(&self) -> String {
        if self.have_link.is_some() {
            return self.have_link.as_deref().unwrap().to_string();
        }
        else {
            return "/public".to_string() + &self.get_str_id() + &"/".to_string();
        }
    }
    pub fn get_s_avatar(&self) -> String {
        if self.s_avatar.is_some() {
            return self.s_avatar.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/icons/avatar30.svg".to_string();
        }
    }
    pub fn get_info_model(&self) -> CommunityInfo {
        use crate::schema::community_infos::dsl::community_infos;

        let _connection = establish_connection();
        let infos = community_infos
            .filter(schema::community_infos::id.eq(self.id))
            .load::<CommunityInfo>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
    }

}


/////// CommunityMembership //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Community)]
pub struct CommunitiesMembership {
    pub id:               i32,
    pub user_id:          i32,
    pub community_id:     i32,
    pub is_administrator: bool,
    pub is_moderator:     bool,
    pub is_editor:        bool,
    pub is_advertiser:    bool,
    pub created:          chrono::NaiveDateTime,
    pub visited:          i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="communities_memberships"]
pub struct NewCommunitiesMembership {
    pub user_id:          i32,
    pub community_id:     i32,
    pub is_administrator: bool,
    pub is_moderator:     bool,
    pub is_editor:        bool,
    pub is_advertiser:    bool,
    pub created:          chrono::NaiveDateTime,
    pub visited:          i32,
}

/////// CommunityInfo //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
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
    pub planners:     i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_infos"]
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
    pub planners:     i32,
}

/////// CommunityPrivate //////
    // 'a' Все пользователи
    // 'b' Подписчики
    // 'c' Создатель
    // 'd' Подписчики, кроме
    // 'e' Некоторые подписчики
    // 'f' Персонал

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
pub struct CommunityPrivate {
    pub id:               i32,
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
#[derive(Deserialize, Insertable)]
#[table_name="community_privates"]
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
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
pub struct CommunityNotification {
    pub id:                   i32,
    pub community_id:         i32,
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub community_invite:     bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_notifications"]
pub struct NewCommunityNotification {
    pub community_id:         i32,
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub community_invite:     bool,
}

/////// CommunityNotificationsPost //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
pub struct CommunityPostNotification {
    pub id:                     i32,
    pub community_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub liked:                  bool,
    pub disliked:               bool,
    pub comment_liked:          bool,
    pub comment_disliked:       bool,
    pub comment_reply_liked:    bool,
    pub comment_reply_disliked: bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_post_notifications"]
pub struct NewCommunityPostNotification {
    pub community_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub liked:                  bool,
    pub disliked:               bool,
    pub comment_liked:          bool,
    pub comment_disliked:       bool,
    pub comment_reply_liked:    bool,
    pub comment_reply_disliked: bool,
}

/////// CommunityNotificationsPhoto //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
pub struct CommunityPhotoNotification {
    pub id:                     i32,
    pub community_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub liked:                  bool,
    pub disliked:               bool,
    pub comment_liked:          bool,
    pub comment_disliked:       bool,
    pub comment_reply_liked:    bool,
    pub comment_reply_disliked: bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_photo_notifications"]
pub struct NewCommunityPhotoNotification {
    pub community_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub liked:                  bool,
    pub disliked:               bool,
    pub comment_liked:          bool,
    pub comment_disliked:       bool,
    pub comment_reply_liked:    bool,
    pub comment_reply_disliked: bool,
}

/////// CommunityNotificationsVideo //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
pub struct CommunityVideoNotification {
    pub id:                     i32,
    pub community_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub liked:                  bool,
    pub disliked:               bool,
    pub comment_liked:          bool,
    pub comment_disliked:       bool,
    pub comment_reply_liked:    bool,
    pub comment_reply_disliked: bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_video_notifications"]
pub struct NewCommunityVideoNotification {
    pub community_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub liked:                  bool,
    pub disliked:               bool,
    pub comment_liked:          bool,
    pub comment_disliked:       bool,
    pub comment_reply_liked:    bool,
    pub comment_reply_disliked: bool,
}

/////// CommunityNotificationsGood //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
pub struct CommunityGoodNotification {
    pub id:                     i32,
    pub community_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub liked:                  bool,
    pub disliked:               bool,
    pub comment_liked:          bool,
    pub comment_disliked:       bool,
    pub comment_reply_liked:    bool,
    pub comment_reply_disliked: bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_good_notifications"]
pub struct NewCommunityGoodNotification {
    pub community_id:           i32,
    pub comment:                bool,
    pub comment_reply:          bool,
    pub mention:                bool,
    pub comment_mention:        bool,
    pub repost:                 bool,
    pub liked:                  bool,
    pub disliked:               bool,
    pub comment_liked:          bool,
    pub comment_disliked:       bool,
    pub comment_reply_liked:    bool,
    pub comment_reply_disliked: bool,
}

/////// CommunityNotificationsMusic //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
pub struct CommunityMusicNotification {
    pub id:                     i32,
    pub community_id:           i32,
    pub repost:                 bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_music_notifications"]
pub struct NewCommunityMusicNotification {
    pub community_id:           i32,
    pub repost:                 bool,
}

/////// CommunityPhotoListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityPhotoListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="community_photo_list_positions"]
pub struct NewCommunityPhotoListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}

/////// CommunityPostListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityPostListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="community_post_list_positions"]
pub struct NewCommunityPostListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}

/////// CommunityMusicListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityMusicListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="community_music_list_positions"]
pub struct NewCommunityMusicListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}

/////// CommunityGoodListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityGoodListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="community_good_list_positions"]
pub struct NewCommunityGoodListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}

/////// CommunityVideoListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityVideoListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="community_video_list_positions"]
pub struct NewCommunityVideoListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}

/////// CommunitySurveyListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunitySurveyListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="community_survey_list_positions"]
pub struct NewCommunitySurveyListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}

/////// CommunityDocListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityDocListPosition {
    pub id:       i32,
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="community_doc_list_positions"]
pub struct NewCommunityDocListPosition {
    pub community_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    String,
}


#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityVisiblePerm {
    pub id:                      i32,
    pub user_id:                 i32,
    pub can_see_info:            Option<String>,
    pub can_see_community:       Option<String>,
    pub can_see_friend:          Option<String>,
    pub can_send_message:        Option<String>,
    pub can_add_in_chat:         Option<String>,
    pub can_see_doc:             Option<String>,
    pub can_see_music:           Option<String>,
    pub can_see_survey:          Option<String>,
    pub can_see_post:            Option<String>,
    pub can_see_post_comment:    Option<String>,
    pub can_see_photo:           Option<String>,
    pub can_see_photo_comment:   Option<String>,
    pub can_see_good:            Option<String>,
    pub can_see_good_comment:    Option<String>,
    pub can_see_video:           Option<String>,
    pub can_see_video_comment:   Option<String>,
    pub can_see_planner:         Option<String>,
    pub can_see_planner_comment: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name="community_visible_perms"]
pub struct NewCommunityVisiblePerm {
    pub id:        i32,
    pub user_id:   i32,

    pub can_see_info:            Option<String>,
    pub can_see_community:       Option<String>,
    pub can_see_friend:          Option<String>,
    pub can_send_message:        Option<String>,
    pub can_add_in_chat:         Option<String>,
    pub can_see_doc:             Option<String>,
    pub can_see_music:           Option<String>,
    pub can_see_survey:          Option<String>,
    pub can_see_post:            Option<String>,
    pub can_see_post_comment:    Option<String>,
    pub can_see_photo:           Option<String>,
    pub can_see_photo_comment:   Option<String>,
    pub can_see_good:            Option<String>,
    pub can_see_good_comment:    Option<String>,
    pub can_see_video:           Option<String>,
    pub can_see_video_comment:   Option<String>,
    pub can_see_planner:         Option<String>,
    pub can_see_planner_comment: Option<String>,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityWorkPerm {
    pub id:               i32,
    pub user_id:          i32,

    pub can_copy_post:    Option<String>,
    pub can_copy_photo:   Option<String>,
    pub can_copy_good:    Option<String>,
    pub can_copy_video:   Option<String>,
    pub can_copy_planner: Option<String>,
    pub can_copy_doc:     Option<String>,
    pub can_copy_music:   Option<String>,
    pub can_copy_survey:  Option<String>,

    pub can_work_post:    Option<String>,
    pub can_work_photo:   Option<String>,
    pub can_work_good:    Option<String>,
    pub can_work_video:   Option<String>,
    pub can_work_planner: Option<String>,
    pub can_work_doc:     Option<String>,
    pub can_work_music:   Option<String>,
    pub can_work_survey:  Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name="community_work_perms"]
pub struct NewCommunityWorkPerm {
    pub id:        i32,
    pub user_id:   i32,

    pub can_copy_post:    Option<String>,
    pub can_copy_photo:   Option<String>,
    pub can_copy_good:    Option<String>,
    pub can_copy_video:   Option<String>,
    pub can_copy_planner: Option<String>,
    pub can_copy_doc:     Option<String>,
    pub can_copy_music:   Option<String>,
    pub can_copy_survey:  Option<String>,

    pub can_work_post:    Option<String>,
    pub can_work_photo:   Option<String>,
    pub can_work_good:    Option<String>,
    pub can_work_video:   Option<String>,
    pub can_work_planner: Option<String>,
    pub can_work_doc:     Option<String>,
    pub can_work_music:   Option<String>,
    pub can_work_survey:  Option<String>,
}
