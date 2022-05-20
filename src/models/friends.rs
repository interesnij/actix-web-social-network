//use crate::schema;
//use diesel::prelude::*;
use crate::schema::{
    friends,
    friends_visible_perms,
    friends_work_perms,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
//use crate::utils::establish_connection;
//use crate::models::User;


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


/////// Варианты значения полей приватности //////
    // "a" Может совершать действия
    // "b" Не может совершать действия

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct FriendsVisiblePerm {
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
    pub can_see_all:             Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name="friends_visible_perms"]
pub struct NewFriendsVisiblePerm {
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
    pub can_see_all:             Option<String>,
}

impl NewFriendsVisiblePerm {
    pub fn add_can_see_info(user_id: i32, can_see_info: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            Some(can_see_info),
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_community(user_id: i32, can_see_community: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       Some(can_see_community),
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_friend(user_id: i32, can_see_friend: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          Some(can_see_friend),
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_send_message(user_id: i32, can_send_message: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        Some(can_send_message),
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_add_in_chat(user_id: i32, can_add_in_chat: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         Some(can_add_in_chat),
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_doc(user_id: i32, can_see_doc: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             Some(can_see_doc),
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_music(user_id: i32, can_see_music: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           Some(can_see_music),
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_survey(user_id: i32, can_see_survey: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          Some(can_see_survey),
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_post(user_id: i32, can_see_post: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            Some(can_see_post),
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_post_comment(user_id: i32, can_see_post_comment: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    Some(can_see_post_comment),
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_photo(user_id: i32, can_see_photo: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           Some(can_see_photo),
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_photo_comment(user_id: i32, can_see_photo_comment: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   Some(can_see_photo_comment),
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_good(user_id: i32, can_see_good: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            Some(can_see_good),
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_good_comment(user_id: i32, can_see_good_comment: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    Some(can_see_good_comment),
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_video(user_id: i32, can_see_video: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           Some(can_see_video),
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_video_comment(user_id: i32, can_see_video_comment: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   Some(can_see_video_comment),
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_planner(user_id: i32, can_see_planner: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         Some(can_see_planner),
            can_see_planner_comment: None,
            can_see_all:             None,
        }
    }
    pub fn add_can_see_planner_comment(user_id: i32, can_see_planner_comment: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: Some(can_see_planner_comment),
            can_see_all:             None,
        }
    }
    pub fn add_can_see_all(user_id: i32, can_see_all: String) -> Self {
        NewFriendsVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_friend:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_all:             Some(can_see_all),
        }
    }
}


#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct FriendsWorkPerm {
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
#[table_name="friends_work_perms"]
pub struct NewFriendsWorkPerm {
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
