use crate::schema;
use diesel::prelude::*;
use crate::schema::{
    survey_lists,
    surveys,
    user_survey_list_collections,
    community_survey_list_collections,
    survey_list_perms,
    survey_answers,
    survey_votes,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
    UserSurveyListPosition,
    CommunitySurveyListPosition,
};


/////// SurveyList //////
////////// Тип списка
    // 1 основной список
    // 2 пользовательский список
    // 3 список предложки
    // 4 Фото со страницы
    // 5 Фото со стены

    // 11 удаленный основной список
    // 12 удаленный пользовательский список
    // 13 удаленный список предложки
    // 14 удаленный Фото со страницы
    // 15 удаленный Фото со стены

    // 21 закрытый основной список
    // 22 закрытый пользовательский список
    // 23 закрытый список предложки
    // 24 закрытый Фото со страницы
    // 25 закрытый Фото со стены

    // 31 замороженный основной список
    // 32 замороженный пользовательский список
    // 33 замороженный список предложки
    // 34 замороженный Фото со страницы
    // 35 замороженный Фото со стены

//////////// Приватность списка
    // 'a' Все пользователи
    // 'b' Друзья
    // 'c' Друзья и друзья друзей
    // 'd' Друзья, кроме
    // 'e' Некоторые друзья
    // 'f' Подписчики
    // 'g' Только я / владелец сообщества
    // 'h' Администраторы
    // 'i' Подписчики, кроме
    // 'j' Некоторые подписчики

/////// SurveyList //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(User)]
pub struct SurveyList {
    pub id:              i32,
    pub name:            String,
    pub community_id:    Option<i32>,
    pub user_id:         i32,
    pub types:           i16,
    pub description:     Option<String>,
    pub created:         chrono::NaiveDateTime,
    pub count:           i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub can_see_el:      String,
    pub create_el:       String,
    pub copy_el:         String,
}
#[derive(Deserialize, Insertable)]
#[table_name="survey_lists"]
pub struct NewSurveyList {
    pub name:            String,
    pub community_id:    Option<i32>,
    pub user_id:         i32,
    pub types:           i16,
    pub description:     Option<String>,
    pub created:         chrono::NaiveDateTime,
    pub count:           i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub can_see_el:      String,
    pub create_el:       String,
    pub copy_el:         String,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="survey_lists"]
pub struct EditSurveyList {
    pub name:            String,
    pub description:     Option<String>,
    pub can_see_el:      String,
    pub create_el:       String,
    pub copy_el:         String,
}

/////// Survey //////

//////////// тип
// 'a' Опубликовано
// 'b' Закрепленый
// 'c' Удаленый
// 'd' Черновик владельца
// 'e' Черновик предложки
// 'f' Предложка сообщества
// 'g' Предложка пользователя
// 'h' Закрыто модератором
// 'i' Удаленый предложенный в сообщество
// 'y' Удаленый предложенный у пользователя

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(User)]
#[belongs_to(SurveyList)]
pub struct Survey {
    pub id:              i32,
    pub title:           String,
    pub community_id:    Option<i32>,
    pub user_id:      i32,
    pub survey_list_id:         i32,
    pub types:           String,
    pub image:           Option<String>,
    pub is_anonymous:    bool,
    pub is_multiple:     bool,
    pub is_no_edited:    bool,
    pub time_end:        Option<chrono::NaiveDateTime>,
    pub created:         chrono::NaiveDateTime,

    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub vote:            i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="surveys"]
pub struct NewSurvey {
    pub title:           String,
    pub community_id:    Option<i32>,
    pub user_id:      i32,
    pub survey_list_id:         i32,
    pub types:           String,
    pub image:           Option<String>,
    pub is_anonymous:    bool,
    pub is_multiple:     bool,
    pub is_no_edited:    bool,
    pub time_end:        Option<chrono::NaiveDateTime>,
    pub created:         chrono::NaiveDateTime,

    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub vote:            i32,
}


/////// UserSurveyListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(SurveyList)]
pub struct UserSurveyListCollection {
    pub id:       i32,
    pub user_id:  i32,
    pub survey_list_id:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_survey_list_collections"]
pub struct NewUserSurveyListCollection {
    pub user_id:  i32,
    pub survey_list_id:  i32,
}

/////// CommunitySurveyListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Community)]
#[belongs_to(SurveyList)]
pub struct CommunitySurveyListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub survey_list_id:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_survey_list_collections"]
pub struct NewCommunitySurveyListCollection {
    pub community_id:  i32,
    pub survey_list_id:       i32,
}

/////// SurveyListPerm //////
    // 'a' Активно
    // 'b' Не активно
    // 'c' Нет значения

#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(SurveyList)]
pub struct SurveyListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub survey_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub create_item:     Option<String>,
    pub can_copy:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="survey_list_perms"]
pub struct NewSurveyListPerm {
    pub user_id:         i32,
    pub survey_list_id:         i32,
    pub can_see_item:    Option<String>,
    pub create_item:     Option<String>,
    pub can_copy:        Option<String>,
}


#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Survey)]
pub struct SurveyAnswer {
    pub id:          i32,
    pub content:     String,
    pub survey_id:   i32,
    pub vote:        i32,
    pub position:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="survey_answers"]
pub struct NewSurveyAnswer {
    pub content:     String,
    pub survey_id:   i32,
    pub vote:        i32,
    pub position:       i32,
}

#[derive(Debug ,Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(SurveyAnswer)]
pub struct SurveyVote {
    pub id:               i32,
    pub user_id:          i32,
    pub survey_answer_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="survey_votes"]
pub struct NewSurveyVote  {
    pub user_id:          i32,
    pub survey_answer_id: i32,
}
