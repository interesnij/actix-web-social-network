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
use crate::schema;
use diesel::prelude::*;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{User, CommunityBannerUser};


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

/////// CommunityBannerUser //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(User)]
pub struct CommunityBannerUser {
    pub id:           i32,
    pub community_id: i32,
    pub user_id:      i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_banner_users"]
pub struct NewCommunityBannerUser {
    pub community_id: i32,
    pub user_id:      i32,
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
    pub fn get_link(&self) -> String {
        if self.have_link.is_some() {
            return "@".to_string() + &self.have_link.as_deref().unwrap().to_string();
        }
        else {
            return "@public".to_string() + &self.get_str_id();
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
        return community_infos
            .filter(schema::community_infos::id.eq(self.id))
            .load::<CommunityInfo>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn plus_photos(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::photos.eq(profile.photos + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_goods(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::goods.eq(profile.goods + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_posts(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::posts.eq(profile.posts + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_videos(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::videos.eq(profile.videos + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_docs(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::docs.eq(profile.docs + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_tracks(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::tracks.eq(profile.tracks + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_communities(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::communities.eq(profile.communities + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_articles(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::articles.eq(profile.articles + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_members(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::members.eq(profile.members + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_photos(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::photos.eq(profile.photos - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_goods(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::goods.eq(profile.goods - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_posts(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::posts.eq(profile.posts - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_videos(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::videos.eq(profile.videos - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_docs(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::docs.eq(profile.docs - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_tracks(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::tracks.eq(profile.tracks - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_articles(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::articles.eq(profile.articles - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_members(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::members.eq(profile.members - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn is_deleted(&self) -> bool {
        return self.types > 10 &&  self.types < 20;
    }
    pub fn is_suspended(&self) -> bool {
        return self.types > 40 &&  self.types < 50;
    }
    pub fn is_closed(&self) -> bool {
        return self.types > 30 &&  self.types < 40;
    }
    pub fn is_have_warning_banner(&self) -> bool {
        return self.types > 20 &&  self.types < 30;
    }
    pub fn is_standart(&self) -> bool {
        return self.perm == "a".to_string();
    }
    pub fn is_child(&self) -> bool {
        return self.perm == "b".to_string();
    }
    pub fn is_private(&self) -> bool {
        return self.types == 1;
    }
    pub fn is_close(&self) -> bool {
        return self.types == 2;
    }
    pub fn is_public(&self) -> bool {
        return self.types == 3;
    }
    pub fn is_open(&self) -> bool {
        return self.types < 10;
    }

    pub fn create_banned_user(&self, user: User) -> bool {
        use crate::schema::community_banner_users::dsl::community_banner_users;
        use crate::models::NewCommunityBannerUser;

        if user.is_banned_from_community(self.id) {
            return false;
        }
        let new_banned_user = NewCommunityBannerUser {
                user_id: user.id,
                community_id: self.id,
            };
        diesel::insert_into(schema::community_banner_users::table)
            .values(&new_banned_user)
            .get_result::<CommunityBannerUser>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn delete_banned_user(&self, user: User) -> bool {
        use crate::schema::community_banner_users::dsl::community_banner_users;

        if !user.is_banned_from_community(self.id) {
            return false;
        }
        diesel::delete(community_banner_users
                .filter(schema::community_banner_users::community_id.eq(self.id))
                .filter(schema::community_banner_users::user_id.eq(user.id))
            )
            .execute(&_connection)
            .expect("E");
        return true;
    }


    pub fn add_new_subscriber(&self, user_id: i32) -> bool {
        use crate::models::{NewsUserCommunitie, NewNewsUserCommunitie};
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        if news_user_communities
            .filter(schema::news_user_communities::community_id.eq(self.id))
            .filter(schema::news_user_communities::owner.eq(user_id))
            .load::<NewsUserCommunitie>(&_connection)
            .expect("E").len() == 0 {
                let _new = NewNewsUserCommunitie {
                    owner: user_id,
                    list_id: None,
                    user_id: None,
                    community_id: Some(community_id),
                    mute: false,
                    sleep: None,
                };
            diesel::insert_into(schema::news_user_communities::table)
                .values(&_new)
                .get_result::<NewsUserCommunitie>(&_connection)
                .expect("Error.");
        }
        return true;
    }
    pub fn add_new_subscriber_in_list(&self, new_id: i32, list_id: i32) -> bool {
        use crate::models::{NewsUserCommunitie, ListUserCommunitiesKey};
        use crate::schema::news_user_communities::dsl::news_user_communities;
        use crate::schema::list_user_communities_keys::dsl::list_user_communities_keys;

        let _connection = establish_connection();
        let _new = news_user_communities
            .filter(schema::news_user_communities::id.eq(new_id))
            .load::<NewsUserCommunitie>(&_connection)
            .expect("E");
        let _list = list_user_communities_keys
            .filter(schema::list_user_communities_keys::id.eq(list_id))
            .load::<ListUserCommunitiesKey>(&_connection)
            .expect("E");

        if _new.len() > 0 && _new[0].community_id == self.id && _list.len() > 0 {
            diesel::update(news_user_communities.filter(schema::news_user_communities::id.eq(new_id)))
                .set(schema::news_user_communities::list_id.eq(list_id))
                .get_result::<NewsUserCommunitie>(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn delete_new_subscriber(&self, user_id: i32) -> bool {
        use crate::models::NewsUserCommunitie;
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        let _new = news_user_communities
            .filter(schema::news_user_communities::community_id.eq(self.id))
            .filter(schema::news_user_communities::user_id.eq(user_id))
            .load::<NewsUserCommunitie>(&_connection)
            .expect("E");
        if _new.len() > 0 && _new[0].community_id == self.id {
            diesel::delete(news_user_communities
                    .filter(schema::news_user_communities::community_id.eq(self.id))
                    .filter(schema::news_user_communities::user_id.eq(user_id))
                ).execute(&_connection)
                .expect("E");
            return true;
        }
        return false;
    }
    pub fn delete_new_subscriber_from_list(&self, new_id: i32) -> bool {
        use crate::models::{NewsUserCommunitie, NewNewsUserCommunitie};
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        let _new = news_user_communities.filter(schema::news_user_communities::id.eq(new_id)).load::<NewsUserCommunitie>(&_connection).expect("E");
        if _new.len() > 0 && _new[0].community_id == self.id {
            let _new = NewNewsUserCommunitie {
                owner: self.id,
                list_id: None,
                user_id: None,
                community_id: _new[0].community_id,
                mute: _new[0].mute,
                sleep: _new[0].sleep,
            };
            diesel::update(news_user_communities.filter(schema::news_user_communities::id.eq(new_id)))
                .set(_new)
                .get_result::<NewsUserCommunitie>(&_connection)
                .expect("Error.");
                return true;
            }
        return false;
    }


    pub fn add_notification_subscriber(&self, user_id: i32) -> bool {
        use crate::models::{NotifyUserCommunitie, NewNotifyUserCommunitie};
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        if notify_user_communities
            .filter(schema::notify_user_communities::owner.eq(user_id))
            .filter(schema::notify_user_communities::community_id.eq(self.id))
            .load::<NotifyUserCommunitie>(&_connection)
            .expect("E").len() == 0 {
                let _new = NewNotifyUserCommunitie {
                    owner: user_id,
                    list_id: None,
                    user_id: None,
                    community_id: Some(self.id),
                    mute: false,
                    sleep: None,
                };
                diesel::insert_into(schema::notify_user_communities::table)
                    .values(&_new)
                    .get_result::<NotifyUserCommunitie>(&_connection)
                    .expect("Error.");
        }
        return true;
    }
    pub fn add_notification_subscriber_in_list(&self, notify_id: i32, list_id: i32, user_id: i32) -> bool {
        use crate::models::{NotifyUserCommunitie, ListUserCommunitiesKey};
        use crate::schema::notify_user_communities::dsl::notify_user_communities;
        use crate::schema::list_user_communities_keys::dsl::list_user_communities_keys;

        let _connection = establish_connection();
        let _notify = notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)).load::<NotifyUserCommunitie>(&_connection).expect("E");
        let _list = list_user_communities_keys.filter(schema::list_user_communities_keys::id.eq(list_id)).load::<ListUserCommunitiesKey>(&_connection).expect("E");

        if _notify.len() > 0 && _notify[0].owner == user_id && _list.len() > 0 && _list[0].owner == user_id {
            diesel::update(notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)))
                .set(schema::notify_user_communities::list_id.eq(_list[0].id))
                .get_result::<NotifyUserCommunitie>(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn delete_notification_subscriber(&self, user_id: i32) -> bool {
        use crate::models::NotifyUserCommunitie;
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        let _notify = notify_user_communities
            .filter(schema::notify_user_communities::owner.eq(user_id))
            .filter(schema::notify_user_communities::community_id.eq(self.id))
            .load::<NotifyUserCommunitie>(&_connection)
            .expect("E");
        if _notify.len() > 0 && _notify[0].owner == user_id {
            diesel::delete(
                notify_user_communities
                    .filter(schema::notify_user_communities::owner.eq(user_id))
                    .filter(schema::notify_user_communities::community_id.eq(self.id))
                )
                .execute(&_connection)
                .expect("E");
            return true;
        }
        return false;
    }
    pub fn delete_notification_subscriber_from_list(&self, notify_id: i32, user_id: i32) -> bool {
        use crate::models::{NotifyUserCommunitie, NewNotifyUserCommunitie};
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        let _notify = notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)).load::<NotifyUserCommunitie>(&_connection).expect("E");
        if _notify.len() > 0 && _notify[0].owner == user_id {
            let _new = NewNotifyUserCommunitie {
                owner: self.id,
                list_id: None,
                user_id: None,
                community_id: _notify[0].community_id,
                mute: _notify[0].mute,
                sleep: _notify[0].sleep,
            };
            diesel::update(notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)))
                .set(_new)
                .get_result::<NotifyUserCommunitie>(&_connection)
                .expect("Error.");
                return true;
            }
        return false;
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

/////// CommunityBannerUser //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(User)]
pub struct CommunityBannerUser {
    pub id:           i32,
    pub community_id: i32,
    pub user_id:      i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_banner_users"]
pub struct NewCommunityBannerUser {
    pub community_id: i32,
    pub user_id:      i32,
}
