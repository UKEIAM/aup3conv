use rusqlite::Connection;
use aup3conv::audacity::projectdoc::ProjectDoc;
use aup3conv::audacity::tagdict::TagDict;

#[test]
fn read_project_from_aup3() {
    let con = Connection::open("data/test-project.aup3").expect("open failed");
    let mut tagdict = TagDict::new();
    tagdict.decode(&con);

    let mut project = ProjectDoc::new(tagdict);
    let _ = project.decode(&con);
}
