use crate::schema::{
    friends,
    connect_ie_settings,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::User;


/////// Friend //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Friend {
    pub id:             i32,
    pub user:           i32,
    pub target_user:    i32,
    pub visited:        i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="friends"]
pub struct NewFriend {
    pub user:           i32,
    pub target_user:    i32,
    pub visited:        i32,
}


/////// FollowPrivate //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Friend)]
pub struct ConnectIeSetting {
    pub id:                      i32,
    pub friend_id:               i32,

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
#[derive(Deserialize, Insertable)]
#[table_name="connect_ie_settings"]
pub struct NewConnectIeSetting {
    pub friend_id:                 i32,

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

    pub can_add_post:            Option<String>,
    pub can_add_photo:           Option<String>,
    pub can_add_good:            Option<String>,
    pub can_add_video:           Option<String>,
    pub can_add_planner:         Option<String>,
    pub can_add_doc:             Option<String>,
    pub can_add_music:           Option<String>,
    pub can_add_survey:          Option<String>,

    pub can_create_post:         Option<String>,
    pub can_create_photo:        Option<String>,
    pub can_create_good:         Option<String>,
    pub can_create_video:        Option<String>,
    pub can_create_planner:      Option<String>,
    pub can_create_doc:          Option<String>,
    pub can_create_music:        Option<String>,
    pub can_create_survey:       Option<String>,
}
