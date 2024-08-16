
pub fn time_to_frame(time: f64, fps: u32) -> u64 {
    (time * fps as f64).round() as u64
}


pub fn time_to_byte(time: f64, fps: u32) -> usize {
    (time_to_frame(time, fps) * 4) as usize
}



pub fn rel_block_offset(pos: usize, block_idx: u16, block_size: usize) -> usize {
    let clean_idx = block_idx as usize;
    pos - (clean_idx * block_size)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ttf_lower() {
        assert_eq!(time_to_frame(0f64, 44100), 0);
    }

    #[test]
    fn ttf_one_sec() {
        assert_eq!(time_to_frame(1f64, 44100), 44100);
    }

    #[test]
    fn ttb_lower() {
        assert_eq!(time_to_byte(0f64, 44100), 0);
    }

    #[test]
    fn ttb_one_sec() {
        assert_eq!(time_to_byte(1f64, 44100), 44100*4);
    }
}
