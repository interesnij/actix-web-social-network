//use diesel::prelude::*;
//use crate::schema;
use crate::schema::{
    moderateds,
    moderated_reports,
    moderated_penalties,
    moderated_logs,
    staff_logs,
    support_users,
    support_user_votes,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
//use crate::utils::establish_connection;
use crate::models::{
    User,
    //Community,
};


/////// Moderated //////

////////// Тип модерируемого объекта
    // 1  Пользователь
    // 2  Сообщество
    // 3  Сайт
    // 4  Почта

    // 20 Список записей
    // 21 Плейлист
    // 22 Список документов
    // 23 Список опросов
    // 24 Список фотографий
    // 25 Список роликов
    // 26 Список товаров
    // 27 Список обсуждений
    // 28 Список википедии
    // 29 Список статей
    // 30 Чат

    // 51 Запись
    // 52 Трек
    // 53 Документ
    // 54 Опрос
    // 55 Фотография
    // 56 Ролик
    // 57 Товар
    // 58 Обсуждение
    // 59 Статья википедии
    // 60 Статья пользователя
    // 61 Сообщение чата

    // 81 Коммент к записи
    // 82 Коммент к фотографии
    // 83 Коммент к ролику
    // 84 Коммент к товару
    // 85 Коммент к обсуждению
    // 86 Коммент к статье википедии

    // 101 Рабочее пространство
    // 102  Доска
    // 103  Колонка
    // 104  Карточка
    // 105  Коммент к карточке

////////// Статус
    // 'a' На рассмотрении
    // 'b' Объект заморожен
    // 'c' Объект закрыт
    // 'd' Объекту присвоен баннер
    // 'e' Отвергнутый

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Moderated {
    pub id:              i32,
    pub description:     Option<String>,
    pub verified:        bool,
    pub status:          String,
    pub types:           i16,
    pub object_id:       i32,
    pub created:         chrono::NaiveDateTime,

    pub count:           i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="moderateds"]
pub struct NewModerated {
    pub description:     Option<String>,
    pub verified:        bool,
    pub status:          String,
    pub types:           i16,
    pub object_id:       i32,
    pub created:         chrono::NaiveDateTime,

    pub count:           i32,
}

/////// ModerationReport //////

////////// Тип жалобы
    // 'a' Порнография
    // 'b' Для взрослых
    // 'c' Оскорбительное содержание
    // 'd' Мошенничество
    // 'e' Наркотики
    // 'f' Продажа оружия
    // 'g' Насилие
    // 'h' Призыв к травле
    // 'i' Призыв к суициду
    // 'j' Жестокое обращение c животными
    // 'k' Введение в заблуждение
    // 'l' Экстремизм
    // 'm' Риторика ненависти

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Moderated)]
pub struct ModeratedReport {
    pub id:               i32,
    pub user_id:         i32,
    pub moderated_id: i32,
    pub description:      Option<String>,
    pub types:            String,
    pub created:          chrono::NaiveDateTime,
}
#[derive(Deserialize, Insertable)]
#[table_name="moderated_reports"]
pub struct NewModeratedReport {
    pub user_id:         i32,
    pub moderated_id: i32,
    pub description:      Option<String>,
    pub types:            String,
    pub created:          chrono::NaiveDateTime,
}

/////// ModerationPenalty //////

////////// Статус штрафа
    // 'a' Приостановлено
    // 'b' Закрыто
    // 'c' Вывешен баннер

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Moderated)]
pub struct ModeratedPenaltie {
    pub id:                  i32,
    pub user_id:          i32,
    pub moderated_id: i32,
    pub expiration:          Option<chrono::NaiveDateTime>,
    pub types:               i16, // описан в самом начале, одно и то же - объект.
    pub object_id:           i32,
    pub status:              String,
    pub created:             chrono::NaiveDateTime,
}
#[derive(Deserialize, Insertable)]
#[table_name="moderated_penalties"]
pub struct NewModeratedPenaltie {
    pub user_id:          i32,
    pub moderated_id: i32,
    pub expiration:          Option<chrono::NaiveDateTime>,
    pub types:               i16, // описан в самом начале, одно и то же - объект.
    pub object_id:           i32,
    pub status:              String,
    pub created:             chrono::NaiveDateTime,
}

/////// ModeratedLogs //////
// 'a' Приостановлено
// 'b' Закрыто
// 'c' Вывешен баннер
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct ModeratedLog {
    pub id:              i32,
    pub user_id:      i32,
    pub object_id:       i32,
    pub action:          String,
    pub description:     Option<String>,
    pub types:           i16,            // описан в самом начале, одно и то же - объект.
    pub created:         chrono::NaiveDateTime,
    pub time_to_suspend: Option<chrono::NaiveDateTime>,
}
#[derive(Deserialize, Insertable)]
#[table_name="moderated_logs"]
pub struct NewModeratedLog {
    pub user_id:      i32,
    pub object_id:       i32,
    pub action:          String,
    pub description:     Option<String>,
    pub types:           i16,                 // описан в самом начале, одно и то же - объект.
    pub created:         chrono::NaiveDateTime,
    pub time_to_suspend: Option<chrono::NaiveDateTime>,
}

/////// StaffLogs //////
// 'a' Создан
// 'b' Удален
// 'c' Вывешен баннер
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct StaffLog {
    pub id:              i32,
    pub types:           i16,            // описано в полномочиях пользователя
    pub action:          String,           // создано-удалено и так далее
    pub manager_id:      i32,
    pub user_id:         i32,
    pub created:         chrono::NaiveDateTime,
}
#[derive(Deserialize, Insertable)]
#[table_name="staff_logs"]
pub struct NewStaffLog {
    pub types:           i16,            // описано в полномочиях пользователя
    pub action:          String,           // создано-удалено и так далее
    pub manager_id:      i32,
    pub user_id:         i32,
    pub created:         chrono::NaiveDateTime,
}

/////// SupportUser //////

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct SupportUser {
    pub id:              i32,
    pub manager_id:      i32,
    pub level:           i16,
    pub points:          i32,
    pub chats:           i16,
    pub created:         chrono::NaiveDateTime,
}
#[derive(Deserialize, Insertable)]
#[table_name="support_users"]
pub struct NewSupportUser {
    pub manager_id:      i32,
    pub level:           i16,
    pub points:          i32,
    pub chats:           i16,
    pub created:         chrono::NaiveDateTime,
}

/////// SupportUser //////

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct SupportUserVote {
    pub id:              i32,
    pub vote:            i16,
    pub user_id:         i32,
    pub manager_id:      i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="support_user_votes"]
pub struct NewSupportUserVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub manager_id:      i32,
}
