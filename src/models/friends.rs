use crate::schema::{friends, new_friends_perms};
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


#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct NewFriendsPerm {
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
#[table_name="new_friends_perms"]
pub struct NewNewFriendsPerm {
    pub id:        i32,
    pub user_id:   i32,
}
