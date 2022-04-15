use crate::schema::{
    follows,
    community_follows,
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
