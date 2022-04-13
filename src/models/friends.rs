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
    pub user_id:           i32,
    pub target_user_id:    i32,
    pub visited:        i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="friends"]
pub struct NewFriend {
    pub user_id:           i32,
    pub target_user_id:    i32,
    pub visited:        i32,
}


/////// ConnectIeSetting //////
#[derive(Deserialize)]
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
    pub can_copy_post:            Option<char>,
    pub can_copy_photo:           Option<char>,
    pub can_copy_good:            Option<char>,
    pub can_copy_video:           Option<char>,
    pub can_copy_planner:         Option<char>,
    pub can_copy_doc:             Option<char>,
    pub can_copy_music:           Option<char>,
    pub can_copy_survey:          Option<char>,
    pub can_create_post:          Option<char>,
    pub can_create_photo:         Option<char>,
    pub can_create_good:          Option<char>,
    pub can_create_video:         Option<char>,
    pub can_create_planner:       Option<char>,
    pub can_create_doc:           Option<char>,
    pub can_create_music:         Option<char>,
    pub can_create_survey:        Option<char>,
}
