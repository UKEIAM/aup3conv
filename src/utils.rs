pub fn time_to_frame(time: f64, fps: i64) -> u64 {
    (time * fps as f64).round() as u64
}

pub fn time_to_byte(time: f64, fps: i64) -> usize {
    (time_to_frame(time, fps) * 4) as usize
}


pub fn block_index(pos: usize, block_size: usize) -> u16 {
    let n = pos as f64 / block_size as f64;
    n.floor() as u16 + 1
}


pub fn rel_block_offset(pos: usize, block_idx: u16, block_size: usize) -> usize {
    let clean_idx = block_idx as usize - 1;
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

    #[test]
    fn test_block_index() {
        assert_eq!(block_index(44099, 44100), 1);
        assert_eq!(block_index(44100, 44100), 2);
        assert_eq!(block_index(44101, 44100), 2);
    }
}
