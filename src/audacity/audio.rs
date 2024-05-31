use rusqlite::{Connection, DatabaseName,Result};
use std::io::Read;
use crate::structure::WaveBlock;

pub const MAX_SAMPLE_BLOCK_SIZE: usize = 1048576;

#[derive(Debug)]
pub struct FakeBlock {
    blockid: usize,
    sampleformat: i64,
    pub summin: f64,
    summax: f64,
    pub sumrms: f64,
}


#[derive(Debug)]
pub struct SampleBlock {
    pub blockid: usize,
    sampleformat: i64,
    pub summin: f64,
    summax: f64,
    pub sumrms: f64,
    pub summary256: [u8; 12288],
    summary64k: [u8; 48],
    pub samples: [u8; 1048576],
}

pub trait AudioProcessor {
    fn fps(&self) -> i64;
    fn get_waveblocks(&self) -> Option<&Vec<WaveBlock>>;
}

pub trait AudioLoader: AudioProcessor {
    fn load_audio_slice(&self, start: f64, stop: f64, buffer: &mut Vec<f32>) -> Result<(), ()>;

    fn load_sampleblock(&self, con: &Connection, block_id: usize) -> Result<SampleBlock> {

        let q = "SELECT blockid, sampleformat, summin, summax, sumrms
                 FROM sampleblocks WHERE blockid = ?";

        let mut stmt = con.prepare(&q)?;
        let fb = stmt.query_row([block_id], |row| Ok(FakeBlock {
                blockid: row.get(0)?,
                sampleformat: row.get(1)?,
                summin: row.get(2)?,
                summax: row.get(3)?,
                sumrms: row.get(4)?,
            }))?;

        let mut out = SampleBlock {
            blockid: fb.blockid,
            sampleformat: fb.sampleformat,
            summin: fb.summin,
            summax: fb.summax,
            sumrms: fb.sumrms,
            summary256: [0u8; 12288],
            summary64k: [0u8; 48],
            samples: [0u8; 1048576]
        };


        let mut s256 = con.blob_open(
            DatabaseName::Main, "sampleblocks", "summary256", block_id as i64, true)?;
        s256.read(&mut out.summary256).expect("read error");

        let mut s64k = con.blob_open(
            DatabaseName::Main, "sampleblocks", "summary64k", block_id as i64, true)?;
        s64k.read(&mut out.summary64k).expect("read error");

        let mut samples = con.blob_open(
            DatabaseName::Main, "sampleblocks", "samples", block_id as i64, true)?;
        samples.read(&mut out.samples).expect("read error");

        Ok(out)
    }
}
