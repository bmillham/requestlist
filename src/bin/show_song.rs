use self::models::*;
use diesel::prelude::*;
use requestlist::*;

fn main() {
    use self::schema::song::dsl::*;

    let connection = &mut establish_connection();

    let results = song
        //.limit(10)
        //.find(5622)
        //.filter(status.eq("ignored"))
        .load::<Song>(connection)
        .expect("Unable to read song");

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
