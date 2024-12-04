use self::models::*;
use diesel::prelude::*;
use requestlist::*;
use std::env::args;

fn main() {
    use self::schema::song::dsl::*;

    let song_id = args()
        .nth(1)
        .expect("ID Required")
        .parse::<u32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let results = match song_id {
        0 => song.load::<Song>(connection).expect("Unable to read song"),
        x => song
            .find(x)
            .load::<Song>(connection)
            .expect("Unable to read song"),
    };

    println!("Found {} songs", results.len());

    for s in results {
        println!(
            "{} {} {} {} {} {}",
            s.id,
            s.title.unwrap(),
            s.file.unwrap(),
            s.track.unwrap(),
            s.artist,
            s.album,
        );
    }

    if song_id > 0 {
        use self::schema::requestlist::dsl::*;
        /* let result = RequstList::belonging_to(&results)
        .select(RequestList::as_select())
        .load(connection); */
        // This fails with: error[E0599]: no function or associated item named `belonging_to` found for struct `requestlist::models::RequestList` in the current scope

        let result = requestlist
            .filter(songID.eq(song_id))
            .load::<RequestList>(connection)
            .expect("No requests");
        println!("Requests {}", result.len());
        for req in result {
            println!("{} {}", req.name.unwrap(), req.msg.unwrap());
        }
    }
}
