use std::fs::File;
use std::io;
use std::env;

use aup3conv::decoder::{ProjectDecoder,TagDictDecoder};


fn main() -> io::Result<()> {
    let argv: Vec<String> = env::args().collect();

    let con = rusqlite::Connection::open(&argv[1])
        .expect("open failed");

    let dict = con.blob_open(rusqlite::DatabaseName::Main, "project",
        "dict", 1, true).expect("Failed to read blob");

    let doc = con.blob_open(rusqlite::DatabaseName::Main, "project",
        "doc", 1, true).expect("Failed to read blob");


    let out = File::create("/home/michael/Music/out.xml").expect("Failed to open file");
    let mut writer = io::BufWriter::new(out);

    let mut tags = TagDictDecoder::new(dict);
    tags.decode();

    let mut pdec = ProjectDecoder::new(doc, tags.get_dict(), &mut writer);
    pdec.decode().expect("OK");

    Ok(())
}



// fn get_blocks(con: &rusqlite::Connection) -> Vec<u8> {
//
//     const BLOB_SIZE: usize = 1048576;
//     let mut data: Vec<u8> = Vec::new();
// 
//     for row_id in get_block_index(&con) {
//         let mut blob = con.blob_open(
//             rusqlite::DatabaseName::Main,
//             "project",
//             "doc",
//             row_id,
//             true)
//             .expect("Failed to read blob");
// 
//         let mut buffer = [0u8; BLOB_SIZE];
//         let n_bytes_read = blob.read(&mut buffer[..]).expect("read err");
//         assert_eq!(blob.len(), n_bytes_read);
//         for i in 0..n_bytes_read {
//             data.push(buffer[i]);
//         }
//     }
//     data
// }
// 
// 
// fn get_block_index(con: &rusqlite::Connection) -> Vec<i64> {
//     let mut stmt = con.prepare("SELECT id from project")
//         .expect("prepared failed");
// 
//     let mut rows = stmt.query([])
//         .expect("query faile");
// 
//     let mut index: Vec<i64> = Vec::new();
//     while let Some(row) = rows.next().expect("Cannot get next row") {
//         index.push(row.get(0).expect("Failed to push to vector"));
//     }
//     index
// }
