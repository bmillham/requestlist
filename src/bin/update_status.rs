use self::models::*;
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
    let new_status = args().nth(2).expect("Status required");
    let connection = &mut establish_connection();

    let request = connection
        .transaction(|connection| {
            let request = requestlist
                .find(id)
                .select(RequestList::as_select())
                .first(connection)?;

            diesel::update(requestlist.find(id))
                .set(status.eq(new_status.clone()))
                .execute(connection)?;
            Ok(request)
        })
        .unwrap_or_else(|_: diesel::result::Error| panic!("Unable to find request {}", id));
    println!(
        "Updated request: by {} {id} from {} to {}",
        request.name.unwrap(),
        request.status,
        new_status
    );
}
