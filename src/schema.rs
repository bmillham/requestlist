// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct RequestlistStatusEnum;
}

diesel::table! {
    use diesel::sql_types::*;

    requestlist (ID) {
        ID -> Integer,
        songID -> Unsigned<Integer>,
        t_stamp -> Datetime,
        #[max_length = 255]
        host -> Nullable<Varchar>,
        msg -> Nullable<Text>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        code -> Integer,
        ETA -> Datetime,
        #[max_length = 7]
        status -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    song (id) {
        id -> Unsigned<Integer>,
        file -> Nullable<Text>,
        catalog -> Unsigned<Integer>,
        album -> Unsigned<Integer>,
        year -> Integer,
        artist -> Unsigned<Integer>,
        title -> Nullable<Text>,
        bitrate -> Integer,
        rate -> Integer,
        #[max_length = 3]
        mode -> Nullable<Varchar>,
        size -> Unsigned<Integer>,
        time -> Unsigned<Smallint>,
        track -> Nullable<Unsigned<Smallint>>,
        #[max_length = 36]
        mbid -> Nullable<Varchar>,
        played -> Unsigned<Tinyint>,
        enabled -> Unsigned<Tinyint>,
        update_time -> Nullable<Unsigned<Integer>>,
        addition_time -> Nullable<Unsigned<Integer>>,
        modification_time -> Nullable<Unsigned<Integer>>,
    }
}

diesel::joinable!(requestlist -> song(songID));

diesel::allow_tables_to_appear_in_same_query!(song, requestlist);
