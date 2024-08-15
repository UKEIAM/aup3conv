use crate::structure::WaveBlock;


#[derive(Debug)]
pub enum AudioError {
    NoWaveblocks,
    ReadFailed,
}


pub trait AudioProcessor {
    fn fps(&self) -> u32;
    fn get_waveblocks(&self) -> Option<&Vec<WaveBlock>>;
}

pub trait AudioLoader: AudioProcessor {
    fn load_slice(&self, start: f64, stop: f64, buffer: &mut Vec<f32>) -> Result<(), AudioError>;
    fn load_wave_block(&self, block_id: u16) -> Result<Vec::<u8>, AudioError>;
    fn load_block_slice(&self, block: &WaveBlock, start: u64, stop: u64, out: &mut Vec<f32>) -> Result<(), AudioError>;
}
