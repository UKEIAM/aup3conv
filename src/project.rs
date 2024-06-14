use std::io::Read;
use std::cmp::Ordering;

use rusqlite::{DatabaseName,Connection,OpenFlags};
use pyo3::prelude::*;
use pyo3::exceptions::{PyValueError, PyIOError};

use crate::audacity::projectdoc::ProjectDoc;
use crate::audacity::tagdict::TagDict;
use crate::structure::*;
use crate::audacity::audio::{AudioLoader, AudioProcessor, AudioError};


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
        let (fps, labels, wb, seq) = match doc.decode(&con) {
            Ok(()) => {

                let fps = match doc.parse_sample_rate() {
                    Some(val) => val,
                    None => panic!("Parsing failed")
                };

                (fps,
                 doc.parse_labels().unwrap(),
                 doc.parse_waveblocks().unwrap(),
                 doc.parse_sequences().unwrap())
            }
            Err(err) => panic!("Error decoding project document: {}", err)
        };

        Self {
            path: path.to_string(),
            fps: fps,
            labels: labels,
            waveblocks: wb,
            sequences: seq,
            con: con }
    }

    fn block_id_from_index(&self, idx: u16) -> i64 {
        match &self.waveblocks {
            Some(wb) => wb[idx as usize ].blockid as i64,
            None => panic!("No such block")
        }
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
    fn load_audio(&self, buffer: &mut Vec<f32>) -> Result<(), AudioError> {
        let mut raw_buffer = Vec::<u8>::new();

        match &self.waveblocks {
            Some(blocks) => {
                for blk in blocks.iter() {
                    match AudioLoader::load_wave_block(self, blk.blockid) {
                        Ok(bytes) => { raw_buffer.extend(bytes.iter()); },
                        Err(_) => { return Err(AudioError::NoWaveblocks); }
                    };
                }
                bytes_to_audio(&raw_buffer, buffer).unwrap();
                Ok(())
            },
            None => Err(AudioError::NoWaveblocks)
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
            Err(e) => Err(AudioError::ReadFailed)
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

}
