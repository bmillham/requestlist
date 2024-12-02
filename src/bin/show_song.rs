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
            .expect("Unable to find song"),
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
}
