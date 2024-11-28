use self::models::*;
use diesel::prelude::*;
use requestlist::*;

fn main() {
    use self::schema::requestlist::dsl::*;

    let connection = &mut establish_connection();

    let results = requestlist
        .limit(10)
        //.filter(id.gt(0))
        //.select(Artist::as_select())
        //.load(connection)
        .load::<RequestList>(connection)
        .expect("Unable to read artists");

    //
    println!("Found {} requests", results.len());

    for request in results {
        println!("{} {}", request.ID, request.songID);
    }
}
