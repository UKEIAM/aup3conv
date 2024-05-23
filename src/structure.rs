use std::io;
use crate::tagstack::Tag;
// pub struct Project {
//     xmlns: String,
//     version: String,
//     audacityversion: String,
//     sel0: f64,
//     sel1: f64,
//     vpos: u8,
//     h: f64,
//     zoom: f64,
//     rate: i32,
//     snapto: String,
//     selectionformat: String,
//     frequencyformat: String,
//     bandwidthformat: String,
//     tracks: Vec<Tracks>,
// }


// pub struct Effects {
//     active: bool,
// }
// 
// enum Tracks {
//     WaveTrack(WaveTrack),
//     LabelTrack(LabelTrack),
// }
// 
// pub struct WaveTrack {
//     name: String,
//     isSelected: bool,
//     height: i16,
//     minimized: bool,
//     channel: u8,
//     linked: u8,
//     mute: bool,
//     solo: bool,
//     rate: i32,
//     gain: f64,
//     pan: f64,
//     colorindex: i32,
//     sampleformat: i64,
//     clips: Vec<WaveClip>,
// }
// 
// pub struct WaveClip {
//     offset: f64,
//     trimLeft: f64,
//     trimRight: f64,
//     name: String,
//     colorindex: i32,
//     sequences: Sequence,
//     envelope: Envelope,
// }
// 
// pub struct Sequence {
//     maxsamples: u64,
//     sampleformat: u64,
//     numsamples: u64,
//     blocks: Vec<WaveBlock>,
// }
#[derive(Debug)]
pub struct WaveBlock {
    pub start: u64,
    pub blockid: u64,
}

impl WaveBlock {
    pub fn from_tag(tag: &Tag) -> io::Result<Self> {
        let start = tag.attributes.get("start")
            .expect("Key 'start' not in tag attributes")
            .parse::<u64>().unwrap();
        let bid = tag.attributes.get("blockid")
            .expect("Key 'blockid' not in tag attributes")
            .parse::<u64>().unwrap();
        Ok(Self { start: start, blockid: bid })

    }
}
// 
// pub struct Envelope {
//     numpoints: u64,
// }
// 
// pub struct LabelTrack {
//     name: String,
//     isSelected: bool,
//     height: i16,
//     minimized: bool,
//     numlabels: i32,
//     labels: Vec<Label>,
// }

#[derive(Debug)]
pub struct Label {
    pub t: f64,
    pub t1: f64,
    pub title: String,
}

impl Label {
    pub fn from_tag(tag: &Tag) -> io::Result<Self> {
        let title = tag.attributes.get("title")
            .expect("Key 'title' not in tag attributes");
        let t = tag.attributes.get("t")
            .expect("Key 't' not in tag attributes")
            .parse::<f64>().unwrap();
        let t1 = tag.attributes.get("t1")
            .expect("Key 't1' not in tag attributes")
            .parse::<f64>().unwrap();
        Ok(Self { title: title.clone(), t: t, t1: t1 })
    }
}
