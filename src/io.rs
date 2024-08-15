#[derive(Debug)]
pub struct Position {
    pub clip_index: usize,
    pub block_index: usize,
    pub block_id: u16,
    pub offset: usize,
    pub offtrack: bool
}


#[derive(Debug)]
pub struct ReadPosition {
    pub block_id: u16,
    pub start: usize,
    pub stop: Option<usize>
}


impl ReadPosition {

    // Construct a new ReadPosition object.
    pub fn new(block_id: u16, start: usize, stop: Option<usize>) -> Self {
        match stop {
            Some(stop) => {
                if stop > start {
                    Self { block_id: block_id, start: start, stop: Some(stop) }
                } else {
                    panic!(">>> ERROR: Cannot construct Position.
                        `start` is greater than `stop`: {} > {}", start, stop);
                }
            },
            None => Self { block_id: block_id, start: start, stop: stop }
        }
    }

    // Return the size in bytes.
    pub fn size(&self) -> Option<usize> {
        match self.stop {
            Some(stop) => { Some(stop-self.start) },
            None => None
        }
    }
}
