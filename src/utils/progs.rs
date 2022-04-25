use crate::utils::establish_connection;
use crate::schema;
use diesel::prelude::*;
use crate::utils::User;
use crate::models::{
    User, Community,
    PostList, Post, PostComment,
    PhotoList, Photo, PhotoComment,
    Chat, Message, Doc, DocList, Music, MusicList,
    GoodList, Good, GoodComment,
    Survey, SurveyList,
    VideoList, Video, VideoComment,
};


pub fn get_user(pk: i32) -> User {
    return schema::users
        .filter(schema::users::id.eq(pk))
        .filter(schema::users::types.lt(10))
        .load::<User>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
