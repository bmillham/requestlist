use self::models::*;
use diesel::prelude::*;
use requestlist::*;

fn main() {
    use self::schema::requestlist::dsl::*;

    let connection = &mut establish_connection();

    let results = requestlist
        //.limit(10)
        //.find(5622)
        //.filter(status.eq("ignored"))
        .filter(songID.eq(23549))
        .load::<RequestList>(connection)
        .expect("Unable to read requests");

    println!("Found {} requests", results.len());

    for request in results {
        println!(
            "{} {} {} {} {} {} {} {}",
            request.id,
            request.song_id,
            request.status,
            request.t_stamp,
            request.eta,
            request.host.unwrap(),
            request.name.unwrap(),
            request.msg.unwrap_or_else(|| String::from("None")),
        );
    }
}
