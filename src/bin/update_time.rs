use self::models::*;
use chrono::prelude::*;
use diesel::prelude::*;
use requestlist::*;
use std::env::args;

fn main() {
    use self::schema::requestlist::dsl::*;

    let id = args()
        .nth(1)
        .expect("ID required")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let ts: NaiveDateTime = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);

    let request = connection
        .transaction(|connection| {
            let request = requestlist
                .find(id)
                .select(RequestList::as_select())
                .first(connection)?;

            diesel::update(requestlist.find(id))
                .set(ETA.eq(ts))
                .execute(connection)?;
            Ok(request)
        })
        .unwrap_or_else(|_: diesel::result::Error| panic!("Unable to find request {}", id));
    println!(
        "Updated request: by {} {id} ETA from {} to {}",
        request.name.unwrap(),
        request.ETA,
        ts,
    );
}
