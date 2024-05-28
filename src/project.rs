use std::io::{Seek, Read, SeekFrom};
use rusqlite::{DatabaseName,Connection};

use crate::audacity::projectdoc::ProjectDoc;
use crate::audacity::tagdict::TagDict;
use crate::structure::*;
use crate::audacity::audio::{AudioLoader, AudioProcessor};


pub struct Project {
    path: String,
    fps: i64,
    labels: Option<Vec<Label>>,
    waveblocks: Option<Vec<WaveBlock>>,
}


impl Project {
    pub fn new(path: &str) -> Self {
        let msg = format!("Failed to open path \"{}\"", path);
        let con = Connection::open(path).expect(&msg);

        let mut tagdict = TagDict::new();
        tagdict.decode(&con);

        let mut doc = ProjectDoc::new(tagdict);
        let (fps, labels, wb) = match doc.decode(&con) {
            Ok(()) => {

                let fps = match doc.parse_sample_rate() {
                    Some(val) => val,
                    None => panic!("Parsing failed")
                };

                (fps, doc.parse_labels().unwrap(), doc.parse_waveblocks().unwrap())
            }
            Err(err) => panic!("Error decoding project document: {}", err)
        };

        Self { path: path.to_string(), fps: fps, labels: labels, waveblocks: wb }
    }

    pub fn list_labels(&self) {
        if let Some(labels) = &self.labels {
            for item in labels.iter() {
                println!("Title: {} -- ({}, {})", item.title, item.t, item.t1);
            }
        }
    }
}


impl AudioProcessor for Project {
    fn fps(&self) -> i64 {
        self.fps
    }

    fn get_waveblocks(&self) -> Option<&Vec<WaveBlock>> {
        self.waveblocks.as_ref()
    }
}


impl AudioLoader for Project {
    fn load_audio_slice(&self, start: u64, stop: u64, buffer: &mut Vec<f32>) {

        let start_block = self.find_waveblock(start).unwrap();
        let stop_block = self.find_waveblock(stop).unwrap();

        let mut buf = vec![0u8; 4*buffer.len()];

        let con = Connection::open(self.path.clone())
            .expect("Cannot open database");

        if start_block == stop_block {
            let mut blob = con.blob_open(DatabaseName::Main, "sampleblocks",
                "samples", start_block as i64, true)
                .expect("Cannot read blob");
            let _ = blob.seek(SeekFrom::Start(start));

            let _ = blob.read_exact(&mut buf);
            let (pre, samples, post) = unsafe { buf.align_to::<f32>() };
            assert_eq!(pre, []);
            assert_eq!(post, []);
            buffer.copy_from_slice(samples);
        }
    }
}
