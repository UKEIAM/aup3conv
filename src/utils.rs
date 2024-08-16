//! Common utilities.

/// Convert time to frame index.
///
/// Convert a `time` measured in seconds to the corresponding
/// audio frame index given the samplerate `fps`.
pub fn time_to_frame(time: f64, fps: u32) -> u64 {
    (time * fps as f64).round() as u64
}


/// Convert time to byte index.
///
/// Convert a `time` measured in seconds to the index of the first byte
/// of the corresponding audio frame. This function assumes the given audio
/// data is a single channel 32 bit stream.
pub fn time_to_byte(time: f64, fps: u32) -> usize {
    (time_to_frame(time, fps) * 4) as usize
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
