use crate::structure::WaveBlock;
use crate::io::ReadPosition;


#[derive(Debug)]
pub enum AudioError {
    NoWaveblocks,
    ReadFailed,
    SeekFailed,
}


pub trait AudioProcessor {
    fn fps(&self) -> u32;
    fn get_waveblocks(&self) -> Option<&Vec<WaveBlock>>;
}

pub trait AudioLoader: AudioProcessor {
    fn load_slice(&self, start: f64, stop: f64, buffer: &mut Vec<f32>) -> Result<(), AudioError>;
    fn load_wave_block(&self, block_id: u16) -> Result<Vec::<u8>, AudioError>;
    fn load_block_slice(&self, read_pos: &ReadPosition, out: &mut Vec<u8>) -> Result<(), AudioError>;
}
