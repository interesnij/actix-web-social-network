use crate::schema::{friends, friend_perms2};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
//use crate::utils::establish_connection;
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


/////// Friend //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct FriendPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub can_see_info:      Option<String>,
    pub can_see_community: Option<String>,
    pub can_see_friend:      Option<String>,
    pub can_send_message: Option<String>,
    pub can_add_in_chat:      Option<String>,
    pub can_see_doc: Option<String>,
    //pub can_see_music:      Option<String>,
    //pub can_see_survey: Option<String>,
    //pub can_see_post:      Option<String>,
    //pub can_see_post_comment: Option<String>,
    //pub can_see_photo:      Option<String>,
    //pub can_see_photo_comment: Option<String>,
    //pub can_see_good:      Option<String>,
    //pub can_see_good_comment: Option<String>,
    //pub can_see_video:      Option<String>,
    //pub can_see_video_comment: Option<String>,
    //pub can_see_planner:      Option<String>,
    //pub can_see_planner_comment: Option<String>,
    //pub can_copy_post:      Option<String>,
    //pub can_copy_photo: Option<String>,
    //pub can_copy_good:      Option<String>,
    //pub can_copy_video: Option<String>,

    //pub can_copy_planner:      Option<String>,
    //pub can_copy_doc: Option<String>,
    //pub can_copy_music:      Option<String>,
    //pub can_copy_survey: Option<String>,
    //pub can_create_post:      Option<String>,
    //pub can_create_photo: Option<String>,
    //pub can_create_good:      Option<String>,
    //pub can_create_video: Option<String>,
    //pub can_create_planner:      Option<String>,
    //pub can_create_doc: Option<String>,
    //pub can_create_music:      Option<String>,
    //pub can_create_survey: Option<String>,

}
#[derive(Deserialize, Insertable)]
#[table_name="test_perms"]
pub struct NewTestPerm {
    pub user_id:         i32,
    pub can_see_info:      Option<String>,
    pub can_see_community: Option<String>,
    pub can_see_friend:      Option<String>,
    pub can_send_message: Option<String>,
    pub can_add_in_chat:      Option<String>,
    pub can_see_doc: Option<String>,
}
