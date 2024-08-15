use std::io::Read;
use std::cmp::Ordering;

use rusqlite::{DatabaseName,Connection,OpenFlags};
use pyo3::prelude::*;
use pyo3::exceptions::PyIOError;

use crate::audacity::projectdoc::ProjectDoc;
use crate::audacity::tagdict::TagDict;
use crate::structure::*;
use crate::audacity::audio::{AudioLoader, AudioProcessor, AudioError};
use crate::utils::*;


#[pyclass]
pub struct Project {

    #[pyo3(get)]
    path: String,

    #[pyo3(get)]
    fps: u32,

    #[pyo3(get)]
    pub labels: Option<Vec<Label>>,

    #[pyo3(get)]
    waveblocks: Option<Vec<WaveBlock>>,

    #[pyo3(get)]
    sequences: Option<Vec<Sequence>>,

    #[pyo3(get)]
    waveclips: Option<Vec<WaveClip>>,

    con: Connection
}


impl Project {
    pub fn open(path: &str) -> Self {
        let msg = format!("Failed to open path \"{}\"", path);
        let con = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY)
            .expect(&msg);

        let mut tagdict = TagDict::new();
        tagdict.decode(&con);

        let mut doc = ProjectDoc::new(tagdict);
        let (fps, labels, wb, seq, clips) = match doc.decode(&con) {
            Ok(()) => {

                let fps = match doc.parse_sample_rate() {
                    Some(val) => val,
                    None => panic!("Parsing failed")
                };

                (fps,
                 doc.parse_labels().unwrap(),
                 doc.parse_waveblocks().unwrap(),
                 doc.parse_sequences().unwrap(),
                 doc.parse_waveclips().unwrap(),)
            }
            Err(err) => panic!("Error decoding project document: {}", err)
        };

        Self {
            path: path.to_string(),
            fps: fps,
            labels: labels,
            waveblocks: wb,
            sequences: seq,
            waveclips: clips,
            con: con }
    }

    fn clip_idx_from_time(&self, pos: f64) -> usize {
        if pos < 0f64 {
            panic!("POS {} is less than zero", pos);
        }

        let mut index: usize = 0;
        if let Some(clips) = &self.waveclips {
            for (i, clip) in clips.iter().enumerate().rev() {
                if pos >= clip.offset {
                    index = i;
                    break;
                }
            }
        }
        index
    }

    // returns (clip_idx, block_idx, block_id, byte_offset)
    fn pos_from_time(&self, pos: f64) -> Position {
        if pos < 0f64 {
            panic!("POS {} is less than zero", pos);
        }

        let mut block_index: usize = 0;
        let mut block_id: u16 = 0;
        let mut byte_pos: usize = 0;
        let clip_idx = self.clip_idx_from_time(pos);
        if let Some(clips) = &self.waveclips {
            if let Some(seq) = &clips[clip_idx].sequences {
                let fpos = time_to_frame(pos-clips[clip_idx].offset, self.fps);
                for (i, block) in seq.blocks.iter().enumerate().rev() {
                    if fpos >= block.start as u64 {
                        block_index = i;
                        block_id = block.blockid;
                        byte_pos = (block.start - fpos as usize) * 4;
                        break;
                    }
                }
            }
        }
        Position { clip_index: clip_idx, block_index: block_index, block_id: block_id, offset: byte_pos }
    }

    // get the block sequence to be read
    // returns vector of (block_id, start, stop)
    // where start and stop is in bytes!!!
    fn block_range(&self, start: f64, stop: f64) -> Vec<ReadPosition> {
        let mut out = Vec::<ReadPosition>::new();

        let start_pos = self.pos_from_time(start);
        let stop_pos = self.pos_from_time(stop);

        if start_pos.clip_index == stop_pos.clip_index {
            if start_pos.block_index == stop_pos.block_index {
                let rp = ReadPosition { block_id: start_pos.block_id, start: start_pos.offset, stop: stop_pos.offset };
                out.push(rp);
            } else {
                let diff = stop_pos.block_id - start_pos.block_id;
                if diff == 1 {
                    let rp0 = ReadPosition { block_id: start_pos.block_id, start: start_pos.offset, stop: stop_pos.offset };
                    let rpN = ReadPosition { block_id: stop_pos.block_id, start: 0, stop: stop_pos.offset };
                    out.push(rp0);
                    out.push(rpN);
                } else {
                    let rp0 = ReadPosition { block_id: start_pos.block_id, start: start_pos.offset, stop: stop_pos.offset };
                    out.push(rp0);

                    for _ in start_pos.block_index+1..stop_pos.block_index {
                        let rpx = ReadPosition { block_id: start_pos.block_id, start: 0, stop: 262144 };
                        out.push(rpx);
                    }
                    let rpN = ReadPosition { block_id: stop_pos.block_id, start: 0, stop: stop_pos.offset };
                    out.push(rpN);
                }
            }

        } else { 

        }
        out
    }


}


#[pymethods]
impl Project {
    fn __str__(&self) -> String {
        format!("Project(path={})", self.path)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn load_audio(&self) -> PyResult<Vec<f32>> {
        let mut samples = Vec::<f32>::new();
        if let Err(_) = AudioLoader::load_audio(self, &mut samples) {
            return Err(PyIOError::new_err("Could not read audio"));
        }
        Ok(samples)
    }

    fn load_label(&self, label: &Label) -> PyResult<Vec<f32>> {
        let mut samples = Vec::<f32>::new();

        if let Err(_) = AudioLoader::load_slice(self, label.t, label.t1, &mut samples) {
            return Err(PyIOError::new_err("Could not load audio"));
        }
        Ok(samples)
    }
}


impl AudioProcessor for Project {
    fn fps(&self) -> u32 {
        self.fps
    }

    fn get_waveblocks(&self) -> Option<&Vec<WaveBlock>> {
        self.waveblocks.as_ref()
    }
}


impl AudioLoader for Project {
    fn load_slice(&self, start: f64, stop: f64, out: &mut Vec<f32>) -> Result<(), AudioError> {

        let mut buffer = Vec::<u8>::new();
        match &self.waveblocks {
            Some(blocks) => {},
            None => {}
        }
        Err(AudioError::NoWaveblocks)
    }

    // start and stop in SAMPLES !!
    fn load_block_slice(&self, block: &WaveBlock, start: u64, stop: u64, out: &mut Vec<f32>) -> Result<(), AudioError> {
        if stop < start {
            panic!("Stop position before start position");
        }

        let mut blob = self.con.blob_open(DatabaseName::Main, "sampleblocks",
            "samples", block.blockid as i64, true)
            .expect("Cannot read blob");

        let n_bytes: usize = (stop - start) as usize * 4;
        let mut buffer = Vec::<u8>::with_capacity(n_bytes);

        match blob.read_exact(&mut buffer) {
            Ok(()) => { 
                bytes_to_audio(&buffer, out).unwrap();
                Ok(()) 
            },
            Err(_) => Err(AudioError::ReadFailed)
        }

    }

    fn load_wave_block(&self, block_id: u16) -> Result<Vec::<u8>, AudioError> {
        let mut blob = self.con.blob_open(DatabaseName::Main, "sampleblocks",
            "samples", block_id as i64, true)
            .expect("Cannot read blob");
        let mut buffer = Vec::<u8>::with_capacity(blob.len());

        match blob.read_to_end(&mut buffer) {
            Ok(count) => {
                if count != blob.len() {
                    return Err(AudioError::ReadFailed);
                }
                Ok(buffer)
            },
            Err(_) => Err(AudioError::ReadFailed)
        }
    }
}

pub fn bytes_to_audio(buffer: &[u8], out: &mut Vec<f32>) ->  Result<(), ()> {
    let (pre, samples, post) = unsafe { buffer.align_to::<f32>() };
    let overspilled = pre.len() + post.len();
    if let Ordering::Greater = overspilled.cmp(&0) {
        return Err(());
    }

    out.resize(samples.len(), 0f32);
    out.copy_from_slice(samples);
    Ok(())
}


struct Position {
    clip_index: usize,
    block_index: usize,
    block_id: u16,
    offset: usize
}

struct ReadPosition {
    block_id: u16,
    start: usize,
    stop: usize
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        let p = Project::open("/home/michael/Downloads/id-35.aup3");
        if let Some(labels) = p.labels {
            for item in labels {
                let a = block_index_from_label(p.waveclips.as_ref().expect("no waveclip"), &item);
                println!("{:?}", a);
            }
        }
    }
}
