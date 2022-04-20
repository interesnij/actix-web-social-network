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
    community_survey_notifications,
    community_photo_list_positions,
    community_post_list_positions,
    community_music_list_positions,
    community_good_list_positions,
    community_video_list_positions,
    community_survey_list_positions,
    community_doc_list_positions,
    community_visible_perms,
    community_work_perms,
    community_banner_users,
};
use crate::schema;
use diesel::prelude::*;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{
    User, PostList, PhotoList, DocList, VideoList,
    SurveyList, MusicList, GoodList,
};


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
    pub fn get_slug(&self) -> String {
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
        let _connection = establish_connection();
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
                    community_id: Some(self.id),
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

    pub fn add_notify_subscriber(&self, user_id: i32) -> bool {
        use crate::models::{NotifyUserCommunitie, NewNotifyUserCommunitie};
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        if notify_user_communities
            .filter(schema::notify_user_communities::community_id.eq(self.id))
            .filter(schema::notify_user_communities::owner.eq(user_id))
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
    pub fn add_notify_subscriber_in_list(&self, new_id: i32, list_id: i32) -> bool {
        use crate::models::{NotifyUserCommunitie, ListUserCommunitiesKey};
        use crate::schema::notify_user_communities::dsl::notify_user_communities;
        use crate::schema::list_user_communities_keys::dsl::list_user_communities_keys;

        let _connection = establish_connection();
        let _new = notify_user_communities
            .filter(schema::notify_user_communities::id.eq(new_id))
            .load::<NotifyUserCommunitie>(&_connection)
            .expect("E");
        let _list = list_user_communities_keys
            .filter(schema::list_user_communities_keys::id.eq(list_id))
            .load::<ListUserCommunitiesKey>(&_connection)
            .expect("E");

        if _new.len() > 0 && _new[0].community_id.unwrap() == self.id && _list.len() > 0 {
            diesel::update(notify_user_communities.filter(schema::notify_user_communities::id.eq(new_id)))
                .set(schema::notify_user_communities::list_id.eq(list_id))
                .get_result::<NotifyUserCommunitie>(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn delete_notify_subscriber(&self, user_id: i32) -> bool {
        use crate::models::NotifyUserCommunitie;
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        let _new = notify_user_communities
            .filter(schema::notify_user_communities::community_id.eq(self.id))
            .filter(schema::notify_user_communities::user_id.eq(user_id))
            .load::<NotifyUserCommunitie>(&_connection)
            .expect("E");
        if _new.len() > 0 && _new[0].community_id.unwrap() == self.id {
            diesel::delete(notify_user_communities
                    .filter(schema::notify_user_communities::community_id.eq(self.id))
                    .filter(schema::notify_user_communities::user_id.eq(user_id))
                ).execute(&_connection)
                .expect("E");
            return true;
        }
        return false;
    }
    pub fn delete_notify_subscriber_from_list(&self, new_id: i32) -> bool {
        use crate::models::{NotifyUserCommunitie, NewNotifyUserCommunitie};
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        let _new = notify_user_communities.filter(schema::notify_user_communities::id.eq(new_id)).load::<NewsUserCommunitie>(&_connection).expect("E");
        if _new.len() > 0 && _new[0].community_id == self.id {
            let _new = NewNotifyUserCommunitie {
                owner: self.id,
                list_id: None,
                user_id: None,
                community_id: _new[0].community_id,
                mute: _new[0].mute,
                sleep: _new[0].sleep,
            };
            diesel::update(notify_user_communities.filter(schema::notify_user_communities::id.eq(new_id)))
                .set(_new)
                .get_result::<NotifyUserCommunitie>(&_connection)
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
    pub fn create_community(name: String, category: i32, user: User, types: i16) -> Community {
        let _connection = establish_connection();
        let new_community_form = NewCommunity{
                name:                    name,
                status:                  "a".to_string(),
                types:                    types,
                perm:                     "a".to_string(),
                level:                    100,
                community_subcategory_id: category,
                user_id:                  user.id,
                created:                  chrono::Local::now().naive_utc(),
            };
        let new_community = diesel::insert_into(schema::communitys::table)
            .values(&new_community_form)
            .get_result::<Community>(&_connection)
            .expect("Error.");

        CommunitiesMembership::create_membership(user, new_community, true, false, false, false);
        user.plus_community_visited(new_community.id);
        new_community.add_new_subscriber(user.id);
        new_community.add_notify_subscriber(user.id);

        // записываем профиль нового пользователя
        let _community_info = NewCommunityInfo {
            community_id: new_community.id,
            posts:        0,
            members:      0,
            photos:       0,
            goods:        0,
            tracks:       0,
            videos:       0,
            docs:         0,
            articles:     0,
            survey:       0,
            planners:     0,
        };
        diesel::insert_into(schema::community_infos::table)
            .values(&_community_info)
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error saving user_profile.");

        // создаем список записей нового пользователя,
        // а также запись в позициях списков записей
        let _new_posts_list = NewPostList {
            name:            "Список записей".to_string(),
            community_id:    Some(new_community.id),
            user_id:         user.id,
            types:           "a".to_string(),
            description:     None,
            created:         NaiveDateTime::new(d, t),
            count:           0,
            repost:          0,
            copy:            0,
            position:        0,
            can_see_el:      "a".to_string(),
            can_see_comment: "a".to_string(),
            create_el:       "g".to_string(),
            create_comment:  "a".to_string(),
            copy_el:         "g".to_string(),
        };
        let _posts_list = diesel::insert_into(schema::post_lists::table)
            .values(&_new_posts_list)
            .get_result::<PostList>(&_connection)
            .expect("Error saving post_list.");

        let _new_posts_list_position = NewCommunityPostListPosition {
            community_id:  new_community.id,
            list_id:  _posts_list.id,
            position: 1,
            types:    "a".to_string(),
        };
        let _posts_list_position = diesel::insert_into(schema::community_post_list_positions::table)
            .values(&_new_posts_list_position)
            .get_result::<CommunityPostListPosition>(&_connection)
            .expect("Error saving post_list_position.");

        // создаем фотоальбомы нового пользователя,
        // а также записи в позициях списков записей
        let _new_photos_list = NewPhotoList {
            name:            "Основной альбом".to_string(),
            community_id:    Some(new_community.id),
            user_id:         user.id,
            types:           "a".to_string(),
            description:     None,
            cover_photo:     None,
            created:         NaiveDateTime::new(d, t),
            count:           0,
            repost:          0,
            copy:            0,
            position:        0,
            can_see_el:      "a".to_string(),
            can_see_comment: "a".to_string(),
            create_el:       "g".to_string(),
            create_comment:  "a".to_string(),
            copy_el:         "g".to_string(),
        };
        let _photos_list = diesel::insert_into(schema::photo_lists::table)
            .values(&_new_photos_list)
            .get_result::<PhotoList>(&_connection)
            .expect("Error saving photo_list.");

        let _new_photos_list_position = NewCommunityPhotoListPosition {
            community_id:  new_community.id,
            list_id:  _photos_list.id,
            position: 1,
            types:    "a".to_string(),
        };
        let _photos_list_position = diesel::insert_into(schema::community_photo_list_positions::table)
            .values(&_new_photos_list_position)
            .get_result::<CommunityPhotoListPosition>(&_connection)
            .expect("Error saving photo_list_position.");

        let _new_photos_list = NewPhotoList {
            name:            "Фото со страницы".to_string(),
            community_id:    Some(new_community.id),
            user_id:         user.id,
            types:           "d".to_string(),
            description:     None,
            cover_photo:     None,
            created:         NaiveDateTime::new(d, t),
            count:           0,
            repost:          0,
            copy:            0,
            position:        0,
            can_see_el:      "a".to_string(),
            can_see_comment: "a".to_string(),
            create_el:       "0".to_string(),
            create_comment:  "a".to_string(),
            copy_el:         "g".to_string(),
            };
        let _photos_list = diesel::insert_into(schema::photo_lists::table)
            .values(&_new_photos_list)
            .get_result::<PhotoList>(&_connection)
            .expect("Error saving photo_list.");

        let _new_photos_list_position = NewCommunityPhotoListPosition {
            community_id:  new_community.id,
            list_id:  _photos_list.id,
            position: 2,
            types:    "a".to_string(),
        };
        let _photos_list_position = diesel::insert_into(schema::community_photo_list_positions::table)
            .values(&_new_photos_list_position)
            .get_result::<CommunityPhotoListPosition>(&_connection)
            .expect("Error saving photo_list_position.");

        let _new_photos_list = NewPhotoList {
            name:            "Фото со стены".to_string(),
            community_id:    Some(new_community.id),
            user_id:         user.id,
            types:           "e".to_string(),
            description:     None,
            cover_photo:     None,
            created:         NaiveDateTime::new(d, t),
            count:           0,
            repost:          0,
            copy:            0,
            position:        0,
            can_see_el:      "a".to_string(),
            can_see_comment: "a".to_string(),
            create_el:       "0".to_string(),
            create_comment:  "a".to_string(),
            copy_el:         "g".to_string(),
        };
        let _photos_list = diesel::insert_into(schema::photo_lists::table)
            .values(&_new_photos_list)
            .get_result::<PhotoList>(&_connection)
            .expect("Error saving photo_list.");

        let _new_photos_list_position = NewCommunityPhotoListPosition {
            community_id:  new_community.id,
            list_id:  _photos_list.id,
            position: 3,
            types:    "a".to_string(),
        };
        let _photos_list_position = diesel::insert_into(schema::community_photo_list_positions::table)
            .values(&_new_photos_list_position)
            .get_result::<CommunityPhotoListPosition>(&_connection)
            .expect("Error saving photo_list_position.");

        // создаем видеоальбом нового пользователя,
        // а также запись в позиции списка записей
        let _new_videos_list = NewVideoList {
            name:            "Основной альбом".to_string(),
            community_id:    Some(new_community.id),
            user_id:         user.id,
            types:           "a".to_string(),
            description:     None,
            created:         NaiveDateTime::new(d, t),
            count:           0,
            repost:          0,
            copy:            0,
            position:        0,
            can_see_el:      "a".to_string(),
            can_see_comment: "a".to_string(),
            create_el:       "g".to_string(),
            create_comment:  "a".to_string(),
            copy_el:         "g".to_string(),
        };
        let _videos_list = diesel::insert_into(schema::video_lists::table)
            .values(&_new_videos_list)
            .get_result::<VideoList>(&_connection)
            .expect("Error saving video_list.");

        let _new_videos_list_position = NewCommunityVideoListPosition {
            community_id:  new_community.id,
            list_id:  _videos_list.id,
            position: 1,
            types:    "a".to_string(),
        };
        let _videos_list_position = diesel::insert_into(schema::community_video_list_positions::table)
            .values(&_new_videos_list_position)
            .get_result::<CommunityVideoListPosition>(&_connection)
            .expect("Error saving video_list_position.");

        // создаем список товаров нового пользователя,
        // а также запись в позиции списка товаров
        let _new_goods_list = NewGoodList {
            name:            "Основной альбом".to_string(),
            community_id:    Some(new_community.id),
            user_id:         user.id,
            types:           "a".to_string(),
            description:     None,
            created:         NaiveDateTime::new(d, t),
            count:           0,
            repost:          0,
            copy:            0,
            position:        0,
            can_see_el:      "a".to_string(),
            can_see_comment: "a".to_string(),
            create_el:       "g".to_string(),
            create_comment:  "a".to_string(),
            copy_el:         "g".to_string(),
        };
        let _goods_list = diesel::insert_into(schema::good_lists::table)
            .values(&_new_goods_list)
            .get_result::<GoodList>(&_connection)
            .expect("Error saving good_list.");

        let _new_goods_list_position = NewCommunityGoodListPosition {
            community_id:  new_community.id,
            list_id:  _goods_list.id,
            position: 1,
            types:    "a".to_string(),
        };
        let _goods_list_position = diesel::insert_into(schema::community_good_list_positions::table)
            .values(&_new_goods_list_position)
            .get_result::<CommunityGoodListPosition>(&_connection)
            .expect("Error saving good_list_position.");

        // создаем плейлист нового пользователя,
        // а также запись в позиции списков плейлистов
        let _new_musics_list = NewMusicList {
            name:            "Основной плейлист".to_string(),
            community_id:    Some(new_community.id),
            user_id:         user.id,
            types:           "a".to_string(),
            description:     None,
            image:           None,
            created:         NaiveDateTime::new(d, t),
            count:           0,
            repost:          0,
            copy:            0,
            position:        0,
            can_see_el:      "a".to_string(),
            create_el:       "g".to_string(),
            copy_el:         "g".to_string(),
        };
        let _musics_list = diesel::insert_into(schema::music_lists::table)
            .values(&_new_musics_list)
            .get_result::<MusicList>(&_connection)
            .expect("Error saving music_list.");

        let _new_musics_list_position = NewCommunityMusicListPosition {
            community_id:  new_community.id,
            list_id:  _musics_list.id,
            position: 1,
            types:    "a".to_string(),
        };
        let _musics_list_position = diesel::insert_into(schema::community_music_list_positions::table)
            .values(&_new_musics_list_position)
            .get_result::<CommunityMusicListPosition>(&_connection)
            .expect("Error saving music_list_position.");

        // создаем список документов нового пользователя,
        // а также запись в позиции списков документов
        let _new_docs_list = NewDocList {
            name:            "Основной список".to_string(),
            community_id:    Some(new_community.id),
            user_id:         user.id,
            types:           "a".to_string(),
            description:     None,
            created:         NaiveDateTime::new(d, t),
            count:           0,
            repost:          0,
            copy:            0,
            position:        0,
            can_see_el:      "a".to_string(),
            create_el:       "g".to_string(),
            copy_el:         "g".to_string(),
        };
        let _docs_list = diesel::insert_into(schema::doc_lists::table)
            .values(&_new_docs_list)
            .get_result::<DocList>(&_connection)
            .expect("Error saving doc_list.");

        let _new_docs_list_position = NewCommunityDocListPosition {
            community_id:  new_community.id,
            list_id:  _docs_list.id,
            position: 1,
            types:    "a".to_string(),
        };
        let _docs_list_position = diesel::insert_into(schema::community_doc_list_positions::table)
            .values(&_new_docs_list_position)
            .get_result::<CommunityDocListPosition>(&_connection)
            .expect("Error saving doc_list_position.");

        // создаем список опросов нового пользователя,
        // а также запись в позиции списков опросов
        let _new_surveys_list = NewSurveyList {
            name:            "Основной список".to_string(),
            community_id:    Some(new_community.id),
            user_id:         user.id,
            types:           "a".to_string(),
            description:     None,
            created:         NaiveDateTime::new(d, t),
            count:           0,
            repost:          0,
            copy:            0,
            position:        0,
            can_see_el:      "a".to_string(),
            create_el:       "g".to_string(),
            copy_el:         "g".to_string(),
        };
        let _surveys_list = diesel::insert_into(schema::survey_lists::table)
            .values(&_new_surveys_list)
            .get_result::<SurveyList>(&_connection)
            .expect("Error saving survey_list.");

        let _new_surveys_list_position = NewCommunitySurveyListPosition {
            community_id:  new_community.id,
            list_id:  _surveys_list.id,
            position: 1,
            types:    "a".to_string(),
        };
        let _surveys_list_position = diesel::insert_into(schema::community_survey_list_positions::table)
            .values(&_new_surveys_list_position)
            .get_result::<CommunitySurveyListPosition>(&_connection)
            .expect("Error saving survey_list_position.");

        // записываем приватность нового пользователя
        let _private = NewCommunityPrivate {
            community_id:            new_community.id,
            can_see_member:  "a".to_string(),
            can_see_info:       "a".to_string(),
            can_send_message:   "a".to_string(),
            can_see_post:       "a".to_string(),
            can_see_photo:      "a".to_string(),
            can_see_good:       "a".to_string(),
            can_see_video:      "a".to_string(),
            can_see_music:      "a".to_string(),
            can_see_planner:    "a".to_string(),
            can_see_doc:        "a".to_string(),
            can_see_survey:     "a".to_string(),
            can_see_settings:   "c".to_string(),
            can_see_log:        "c".to_string(),
            can_see_stat:       "c".to_string(),
        };
        diesel::insert_into(schema::community_privates::table)
            .values(&_private)
            .get_result::<CommunityPrivate>(&_connection)
            .expect("Error saving community_private.");

        // записываем уведомления профиля нового пользователя
        let _community_notification = NewCommunityNotification {
            community_id:              new_community.id,
            connection_request:   true,
            connection_confirmed: true,
            community_invite:     true,
        };
        diesel::insert_into(schema::community_notifications::table)
            .values(&_community_notification)
            .get_result::<CommunityNotification>(&_connection)
            .expect("Error saving community_notification.");

        // записываем уведомления записей нового пользователя
        let _post_notification = NewCommunityPostNotification {
            community_id:                new_communityo.id,
            comment:                true,
            comment_reply:          true,
            mention:                true,
            comment_mention:        true,
            repost:                 true,
            liked:                  true,
            disliked:               true,
            comment_liked:          true,
            comment_disliked:       true,
            comment_reply_liked:    true,
            comment_reply_disliked: true,
        };
        diesel::insert_into(schema::community_post_notifications::table)
            .values(&_post_notification)
            .get_result::<CommunityPostNotification>(&_connection)
            .expect("Error saving community_photo_notification.");

        // записываем уведомления фотографий нового пользователя
        let _photo_notification = NewCommunityPhotoNotification {
            community_id:                new_community.id,
            comment:                true,
            comment_reply:          true,
            mention:                true,
            comment_mention:        true,
            repost:                 true,
            liked:                  true,
            disliked:               true,
            comment_liked:          true,
            comment_disliked:       true,
            comment_reply_liked:    true,
            comment_reply_disliked: true,
        };
        diesel::insert_into(schema::community_photo_notifications::table)
            .values(&_photo_notification)
            .get_result::<CommunityPhotoNotification>(&_connection)
            .expect("Error saving community_photo_notification.");

        // записываем уведомления товаров нового пользователя
        let _good_notification = NewCommunityGoodNotification {
            community_id:                new_community.id,
            comment:                true,
            comment_reply:          true,
            mention:                true,
            comment_mention:        true,
            repost:                 true,
            liked:                  true,
            disliked:               true,
            comment_liked:          true,
            comment_disliked:       true,
            comment_reply_liked:    true,
            comment_reply_disliked: true,
        };
        diesel::insert_into(schema::community_good_notifications::table)
            .values(&_good_notification)
            .get_result::<CommunityGoodNotification>(&_connection)
            .expect("Error saving community_good_notification.");

        // записываем уведомления роликов нового пользователя
        let _video_notification = NewCommunityVideoNotification {
            community_id:                new_community.id,
            comment:                true,
            comment_reply:          true,
            mention:                true,
            comment_mention:        true,
            repost:                 true,
            liked:                  true,
            disliked:               true,
            comment_liked:          true,
            comment_disliked:       true,
            comment_reply_liked:    true,
            comment_reply_disliked: true,
        };
        diesel::insert_into(schema::community_video_notifications::table)
            .values(&_video_notification)
            .get_result::<CommunityVideoNotification>(&_connection)
            .expect("Error saving community_video_notification.");

        // записываем уведомления роликов нового пользователя
        let _music_notification = NewCommunityMusicNotification {
            community_id:                new_community.id,
            repost:                 true,
        };
        diesel::insert_into(schema::community_music_notifications::table)
            .values(&_music_notification)
            .get_result::<CommunityMusicNotification>(&_connection)
            .expect("Error saving community_music_notification.");

        // записываем уведомления роликов нового пользователя
        let _survey_notification = NewCommunitySurveyNotification {
            community_id:  new_community.id,
            vote:     true,
            repost:   true,
        };
        diesel::insert_into(schema::community_survey_notifications::table)
            .values(&_survey_notification)
            .get_result::<CommunitySurveyNotification>(&_connection)
            .expect("Error saving community_survey_notification.");
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
impl CommunitiesMembership {
    pub fn create_membership(user: User, community: Community, is_administrator: bool, is_editor: bool, is_advertiser: bool, is_moderator: bool) -> CommunitiesMembership {
        let _connection = establish_connection();

        let new_member = NewCommunitiesMembership {
            user_id: user.id,
            community_id: community.id,
            is_administrator: is_administrator,
            is_moderator: is_moderator,
            is_editor: is_editor,
            is_advertiser: is_advertiser,
            created: chrono::Local::now().naive_utc(),
            visited: 0,
        };
        diesel::insert_into(schema::communities_memberships::table)
            .values(&new_member)
            .get_result::<CommunitiesMembership>(&_connection)
            .expect("E.");

        community.add_new_subscriber(user.id);
        community.plus_member(1);
        user.plus_communities();
        return new_member;
    }

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

/////// CommunityNotificationsSurvey //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
pub struct CommunitySurveyNotification {
    pub id:           i32,
    pub community_id: i32,
    pub vote:         bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_survey_notifications"]
pub struct NewCommunitySurveyNotification {
    pub community_id: i32,
    pub vote:         bool,
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
