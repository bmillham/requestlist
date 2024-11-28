use chrono;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::requestlist)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct RequestList {
    pub ID: i32,
    pub songID: i32,
    pub t_stamp: chrono::NaiveDateTime,
    pub host: Option<String>,
    pub msg: Option<String>,
    pub name: Option<String>,
    pub code: i32,
    pub ETA: diesel::mysql::MysqlType::DateTime,
    pub status: diesel::mysql::MysqlType::Enum,
}
