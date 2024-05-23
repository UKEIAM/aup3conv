use rusqlite::Connection;
use aup3conv::audacity::tagdict::TagDict;


#[test]
fn get_started() {
    let con = Connection::open("/home/michael/Music/id-9.aup3").expect("open failed");
    let mut tags = TagDict::new();
    tags.decode(&con);
}
