use crate::schema::{
    follows,
    community_follows,
    follow_ie_settings,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
};


/////// Follow //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(User), foreign_key="follows_user")]
#[belongs_to(User), foreign_key="followers_user")]
pub struct Follow {
    pub id:             i32,
    pub user:           i32,
    pub followed_user:  i32,
    pub view:           bool,
    pub visited:        i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="follows"]
pub struct NewFollow {
    pub user:          i32,
    pub followed_user: i32,
    pub view:          bool,
    pub visited:       i32,
}

/////// CommunityFollow //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(User), foreign_key="community_follows_user")]
#[belongs_to(Community), foreign_key="community_follows_community")]
pub struct CommunityFollow {
    pub id:          i32,
    pub user:        i32,
    pub community:   i32,
    pub view:        bool,
    pub visited:     i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_follows"]
pub struct NewCommunityFollow {
    pub user:        i32,
    pub community:   i32,
    pub view:        bool,
    pub visited:     i32,
}

/////// FollowPrivate //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Follow)]
#[table_name="follow_ie_settings"]
pub struct FollowPerm {
    pub id:                      i32,
    pub user_id:                 i32,

    pub can_see_info:            Option<char>,
    pub can_see_community:       Option<char>,
    pub can_see_friend:          Option<char>,
    pub can_send_message:        Option<char>,
    pub can_add_in_chat:         Option<char>,
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
