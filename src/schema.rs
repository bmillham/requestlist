// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct RequestlistStatusEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RequestlistStatusEnum;

    requestlist (ID) {
        ID -> Integer,
        songID -> Integer,
        t_stamp -> Datetime,
        #[max_length = 255]
        host -> Nullable<Varchar>,
        msg -> Nullable<Text>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        code -> Integer,
        ETA -> Datetime,
        #[max_length = 7]
        status -> RequestlistStatusEnum,
    }
}
