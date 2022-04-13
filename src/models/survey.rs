use crate::schema::{
    survey_lists,
    surveys,
    user_survey_list_collections,
    community_survey_list_collections,
    survey_list_perms,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{
    User,
    Community,
};


/////// SurveyList //////

////////// Тип списка
    // 'a' основной список
    // 'b' пользовательский список
    // 'c' список предложки
    // 'd' Фото со страницы
    // 'e' Фото со стены
    // 'f', 'g' ....

    // 'h' удаленный основной список
    // 'i' удаленный пользовательский список
    // 'j' удаленный список предложки
    // 'k' удаленный Фото со страницы
    // 'l' удаленный Фото со стены
    // удаленный 'm', 'n' ....

    // 'o' закрытый основной список
    // 'p' закрытый пользовательский список
    // 'q' закрытый список предложки
    // 'r' закрытый Фото со страницы
    // 's' закрытый Фото со стены
    // закрытый 't', 'u' ....

    // 'v' замороженный основной список
    // 'w' замороженный пользовательский список
    // 'x' замороженный список предложки
    // 'y' замороженный Фото со страницы
    // 'z' замороженный Фото со стены
    // замороженный '1', '2' ....

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
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(User)]
pub struct SurveyList {
    pub id:              i32,
    pub name:            String,
    pub community_id:    Option<i32>,
    pub creator_id:      i32,
    pub types:           char,
    pub description:     Option<String>,
    pub created:         chrono::NaiveDateTime,
    pub count:           i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub can_see_el:      char,
    pub create_el:       char,
    pub copy_el:         char,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="survey_lists"]
pub struct NewSurveyList {
    pub name:            String,
    pub community_id:    Option<i32>,
    pub creator_id:      i32,
    pub types:           String,
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

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(User)]
#[belongs_to(SurveyList)]
pub struct Survey {
    pub id:              i32,
    pub title:           String,
    pub community_id:    Option<i32>,
    pub creator_id:      i32,
    pub list_id:         i32,
    pub types:           char,
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
    pub vote:            i16,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="surveys"]
pub struct NewSurvey {
    pub title:           String,
    pub community_id:    Option<i32>,
    pub creator_id:      i32,
    pub list_id:         i32,
    pub types:           char,
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
    pub vote:            i16,
}


/////// UserSurveyListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(SurveyList)]
pub struct UserSurveyListCollection {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="user_survey_list_collections"]
pub struct NewUserSurveyListCollection {
    pub user_id:  i32,
    pub list_id:  i32,
}

/////// CommunitySurveyListCollection //////
#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(Community)]
#[belongs_to(SurveyList)]
pub struct CommunitySurveyListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub list_id:       i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="community_survey_list_collections"]
pub struct NewCommunitySurveyListCollection {
    pub community_id:  i32,
    pub list_id:       i32,
}

/////// SurveyListPerm //////
    // 'a' Активно
    // 'b' Не активно
    // 'c' Нет значения

#[derive(Debug ,Queryable, Serialize, Identifiable)]
#[belongs_to(User)]
#[belongs_to(SurveyList)]
pub struct SurveyListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub list_id:         i32,
    pub can_see_item:    Option<char>,
    pub create_item:     Option<char>,
    pub can_copy:        Option<char>,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="survey_list_perms"]
pub struct NewSurveyListPerm {
    pub user_id:         i32,
    pub list_id:         i32,
    pub can_see_item:    Option<String>,
    pub create_item:     Option<String>,
    pub can_copy:        Option<String>,
}
