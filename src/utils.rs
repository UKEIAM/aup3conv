use crate::structure::{WaveBlock,WaveClip,Label};

pub fn time_to_frame(time: f64, fps: u32) -> u64 {
    (time * fps as f64).round() as u64
}

pub fn time_to_byte(time: f64, fps: u32) -> usize {
    (time_to_frame(time, fps) * 4) as usize
}


pub fn block_index(pos: usize, block_size: usize) -> u16 {
    let n = pos as f64 / block_size as f64;
    n.floor() as u16
}


pub fn rel_block_offset(pos: usize, block_idx: u16, block_size: usize) -> usize {
    let clean_idx = block_idx as usize;
    pos - (clean_idx * block_size)
}


pub fn clip_index(clips: &Vec<WaveClip>, label: &Label) -> (usize, usize) {
    let mut start = 0;
    let mut stop = 0;

    for (clip, i) in clips.iter().zip(0..clips.len()).rev() {
        if label.t >= clip.offset {
            start = i;
            break;
        }
    }

    for (clip, i) in clips.iter().zip(0..clips.len()).rev() {
        if label.t1 >= clip.offset {
            stop = i;
            break;
        }
    }

    (start, stop)
}

fn find_block(pos: u64, blocks: &Vec<WaveBlock>) -> usize {
    let mut out = 0;
    for (block, i) in blocks.iter().zip(0..blocks.len()).rev() {
        if pos >= block.start as u64 {
            out = i;
            break;
        }
    }
    out
}

pub fn block_index_from_label(clips: &Vec<WaveClip>, label: &Label) -> (usize, usize) {
    let mut start = 0;
    let mut stop = 0;
    let start_frame = time_to_frame(label.t, 44100);
    let stop_frame = time_to_frame(label.t1, 44100);

    let (clip_start_idx, clip_stop_idx) = clip_index(clips, label);

    if let Some(seq) = &clips[clip_start_idx].sequences {
        start = find_block(start_frame, &seq.blocks);
    }

    if let Some(seq) = &clips[clip_stop_idx].sequences {
        stop = find_block(stop_frame, &seq.blocks);
    }
    (start, stop)
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
        assert_eq!(block_index(44099, 44100), 0);
        assert_eq!(block_index(44100, 44100), 1);
        assert_eq!(block_index(44101, 44100), 1);
    }
}
