use rusqlite::Connection;
use aup3conv::audacity::projectdoc::ProjectDoc;

#[test]
fn read_project_from_aup3() {
    let con = Connection::open("/home/michael/Music/id-9.aup3").expect("open failed");
    let mut project = ProjectDoc::new();
}
