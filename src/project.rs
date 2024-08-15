use std::io::{Read,Seek,SeekFrom};
use std::cmp::Ordering;

use rusqlite::{DatabaseName,Connection,OpenFlags};
use pyo3::prelude::*;
use pyo3::exceptions::PyIOError;

use crate::audacity::projectdoc::ProjectDoc;
use crate::audacity::tagdict::TagDict;
use crate::io::*;
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
                if clip.is_empty() { continue }
                if pos >= clip.offset {
                    index = i;
                    break;
                }
            }
        }
        index
    }


    // Convert a time to a Position.
    //
    // The positive floating point value `time` is converted to the
    // exact position in the data structure, packed into an Position object.
    fn pos_from_time(&self, pos: f64) -> Position {
        if pos < 0f64 {
            panic!("POS {} is less than zero", pos);
        }

        let mut block_index: usize = 0;
        let mut block_id: u16 = 0;
        let mut byte_pos: usize = 0;
        let mut offtrack: bool = true;
        let clip_idx = self.clip_idx_from_time(pos);
        if let Some(clips) = &self.waveclips {
            if let Some(seq) = &clips[clip_idx].sequences {
                // position in frames relative to the clip
                let fpos = time_to_frame(pos-clips[clip_idx].offset, self.fps);
                for (i, block) in seq.blocks.iter().enumerate().rev() {
                    if fpos >= block.start as u64 {
                        block_index = i;
                        block_id = block.blockid;
                        byte_pos = (fpos as usize - block.start) * 4;
                        offtrack = if fpos > seq.numsamples {true} else {false};
                        break;
                    }
                }
            }
        }
        Position { clip_index: clip_idx, block_index: block_index, block_id: block_id,
                    offset: byte_pos, offtrack: offtrack }

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
        let items = self.block_range(start, stop);

        for item in items {
            match AudioLoader::load_block_slice(self, &item, &mut buffer) {
                Ok(()) => {},
                Err(err) => {
                    panic!("ERROR: Could not read block, because {:?}.", err);
                }
            }
        }

        bytes_to_audio(&buffer, out).unwrap();
        Ok(())
    }

    // Read chunk from waveblock.
    //
    // Chunk size is determined by `item`.
    fn load_block_slice(&self, item: &ReadPosition, out: &mut Vec<u8>) -> Result<(), AudioError> {

        let mut blob = self.con.blob_open(DatabaseName::Main, "sampleblocks",
            "samples", item.block_id as i64, true)
            .expect("Cannot read blob");


        let mut buffer = Vec::<u8>::with_capacity(blob.size() as usize);
        if let Ok(_) = blob.seek(SeekFrom::Start(item.start as u64)) {
            if let Some(chunk_size) = item.size() {
                buffer.resize(chunk_size, 0u8);
                if let Err(_) = blob.read_exact(&mut buffer) {
                    return Err(AudioError::ReadFailed)
                }
            } else {
                if let Err(_) = blob.read_to_end(&mut buffer) {
                    return Err(AudioError::ReadFailed)
                }
            }
            out.append(&mut buffer);
            Ok(())
        } else {
            Err(AudioError::SeekFailed)
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
