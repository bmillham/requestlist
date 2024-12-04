use crate::schema::requestlist;
use crate::schema::song;
use chrono;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, PartialEq)]
#[diesel(primary_key(ID))]
#[diesel(belongs_to(Song, foreign_key=songID))]
#[diesel(table_name = requestlist)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct RequestList {
    #[diesel(column_name = ID)]
    pub id: u32,
    #[diesel(column_name = songID)]
    pub song_id: u32,
    pub t_stamp: chrono::NaiveDateTime,
    pub host: Option<String>,
    pub msg: Option<String>,
    pub name: Option<String>,
    pub code: i32,
    #[diesel(column_name = ETA)]
    pub eta: chrono::NaiveDateTime,
    pub status: String,
}

#[derive(Queryable, Debug, Identifiable, PartialEq)]
#[diesel(table_name = song)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Song {
    pub id: u32,
    pub file: Option<String>,
    pub catalog: u32,
    pub album: u32,
    pub year: i32,
    pub artist: u32,
    pub title: Option<String>,
    pub bitrate: i32,
    pub rate: i32,
    pub mode: Option<String>,
    pub size: u32,
    pub time: u16,
    pub track: Option<u16>,
    pub mbid: Option<String>,
    pub played: u8,
    pub enabled: u8,
    pub update_time: Option<u32>,
    pub addition_time: Option<u32>,
    pub modification_time: Option<u32>,
}
