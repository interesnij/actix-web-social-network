//use crate::schema;
//use diesel::prelude::*;
use crate::schema::{
    follows,
    community_follows,
    follows_visible_perms,
    follows_work_perms,
    community_invites,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
//use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
};


/////// Follow //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
pub struct Follow {
    pub id:             i32,
    pub user_id:           i32,
    pub followed_user:  i32,
    pub view:           bool,
    pub visited:        i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="follows"]
pub struct NewFollow {
    pub user_id:          i32,
    pub followed_user: i32,
    pub view:          bool,
    pub visited:       i32,
}

/////// CommunityFollow //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Community)]
pub struct CommunityFollow {
    pub id:          i32,
    pub user_id:        i32,
    pub community_id:   i32,
    pub view:        bool,
    pub visited:     i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_follows"]
pub struct NewCommunityFollow {
    pub user_id:        i32,
    pub community_id:   i32,
    pub view:        bool,
    pub visited:     i32,
}

/////// CommunityInvite //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Community)]
//#[belongs_to(User, foreign_key="invite_creator")]
pub struct CommunityInvite {
    pub id:             i32,
    pub user_id:        i32,
    pub community_id:   i32,
    pub invite_creator: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_invites"]
pub struct NewCommunityInvite {
    pub user_id:           i32,
    pub community_id:      i32,
    pub invite_creator:    i32,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct FollowsVisiblePerm {
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
#[table_name="follows_visible_perms"]
pub struct NewFollowsVisiblePerm {
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
pub struct FollowsWorkPerm {
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
#[table_name="follows_work_perms"]
pub struct NewFollowsWorkPerm {
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
