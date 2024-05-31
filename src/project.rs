use std::io::{Seek, Read, SeekFrom};
use std::cmp::Ordering;

use rusqlite::{DatabaseName,Connection};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::audacity::projectdoc::ProjectDoc;
use crate::audacity::tagdict::TagDict;
use crate::structure::*;
use crate::audacity::audio::{AudioLoader, AudioProcessor, MAX_SAMPLE_BLOCK_SIZE};
use crate::utils;



#[pyclass]
pub struct Project {

    #[pyo3(get)]
    path: String,

    #[pyo3(get)]
    fps: i64,

    #[pyo3(get)]
    pub labels: Option<Vec<Label>>,

    #[pyo3(get)]
    waveblocks: Option<Vec<WaveBlock>>,

    con: Connection
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

        Self { path: path.to_string(), fps: fps, labels: labels, waveblocks: wb, con: con }
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

    fn load_waveblock(&self, block_id: i64) -> PyResult<Vec<f32>> {
        let mut blob = self.con.blob_open(DatabaseName::Main, "sampleblocks",
            "samples", block_id, true)
            .expect("Cannot read blob");

        let mut buff = vec![0u8; 1048576];
        blob.read(&mut buff).expect("cannot read blob");

        let mut out = Vec::<f32>::new();
        match bytes_to_audio(&mut buff, &mut out) {
            Err(_) => Err(PyValueError::new_err("Could not align bytes to float.")),
                _ => Ok(out)
        }
    }

    fn load_slice_from_label(&self, label: &Label) -> PyResult<Vec<f32>> {
        let mut out = Vec::<f32>::new();
        match self.load_audio_slice(label.t, label.t1, &mut out) {
            Err(_) => Err(PyValueError::new_err("Could not do it")),
            _ => Ok(out)
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
    fn load_audio_slice(&self, t0: f64, t1: f64, out: &mut Vec<f32>) -> Result<(), ()> {

        let start = utils::time_to_byte(t0, self.fps);
        let stop = utils::time_to_byte(t1, self.fps);

        let start_block = utils::block_index(start, MAX_SAMPLE_BLOCK_SIZE);
        let stop_block = utils::block_index(stop, MAX_SAMPLE_BLOCK_SIZE);

        let con = Connection::open(self.path.clone())
            .expect("Cannot open database");

        let total_size = stop - start;
        let mut bytes = Vec::<u8>::new();
        let mut read_count: usize = 0;

        match start_block.cmp(&stop_block) {
            Ordering::Less => {
                for idx in start_block..=stop_block {
                    let mut blob = con.blob_open(DatabaseName::Main, "sampleblocks",
                        "samples", idx as i64, true)
                        .expect("Cannot read blob");

                    if idx == start_block {
                        let offset = utils::rel_block_offset(start, idx, MAX_SAMPLE_BLOCK_SIZE);
                        match blob.seek(SeekFrom::Start(offset as u64)) {
                            Ok(val) => { assert_eq!(val, offset as u64) },
                            Err(_) => ()
                        }
                        let rc = blob.read_to_end(&mut bytes).expect("Cannont read");
                        read_count+=rc;

                    } else if idx == stop_block {
                        let n = total_size - read_count;
                        let mut foo = vec![0u8; n];
                        blob.read_exact(&mut foo).expect("cannot read");
                        read_count += n;
                        bytes.append(&mut foo);

                    } else {
                        let mut foo = vec![0u8; MAX_SAMPLE_BLOCK_SIZE];
                        let rc = blob.read_to_end(&mut foo).expect("Cannont read");
                        match rc.cmp(&MAX_SAMPLE_BLOCK_SIZE) {
                            Ordering::Equal => {
                                read_count += rc;
                                bytes.append(&mut foo);
                            },
                            _ => { return Err(()) }
                        }
                    }
                }
                bytes_to_audio(&mut bytes, out)
            },

            Ordering::Equal => {
                let mut buf = vec![0u8; total_size];

                let mut blob = con.blob_open(DatabaseName::Main, "sampleblocks",
                    "samples", start_block as i64, true)
                    .expect("Cannot read blob");
                let _ = blob.seek(SeekFrom::Start(start as u64));
                let _ = blob.read_exact(&mut buf);

                bytes_to_audio(&mut buf, out)
            },

            Ordering::Greater => Err(())
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
    fn audioloader_project_load_audio_slice() {
        let pro = Project::new("data/test-project.aup3");
        let mut out = Vec::<f32>::new();

        match pro.labels {
            Some(ref labels) => {
                pro.load_audio_slice(labels[3].t, labels[3].t1, &mut out).expect("Aerg");
            },
            _ => ()
        }
    }
}
