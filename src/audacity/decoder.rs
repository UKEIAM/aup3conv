use rusqlite::blob::Blob;
use std::io::Read;


pub trait Decoder {
    fn byte(&mut self) -> u8;
    fn nbytes(&mut self, size: usize) -> Vec<u8>;
    fn short(&mut self) -> i16;
    fn integer(&mut self) -> i32;
    fn longlong(&mut self) -> i64;
    fn double(&mut self) -> f64;
    fn string(&mut self, size: usize) -> String;
    fn field_type_code(&mut self) -> u8;
}


impl Decoder for Blob<'_> {

    fn byte(&mut self) -> u8 {
        let mut buffer = [0u8; 1];
        match self.read_exact(&mut buffer) {
            Ok(()) => buffer[0],
            Err(error) => panic!("Failed to read byte: {}", error),
        }
    }

    fn nbytes(&mut self, size: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; size];
        match self.read_exact(&mut buffer) {
            Ok(()) => buffer,
            Err(error) => panic!("Failed to read {} bytes: {}", size, error),
        }
    }

    fn short(&mut self) -> i16 {
        let mut buffer = [0u8; 2];
        if let Err(error) = self.read_exact(&mut buffer) {
            panic!("LIB ERROR: Could not read bytes: {}", error);
        }
        let (pre, val, post) = unsafe { buffer.align_to::<i16>() };
        assert_eq!(pre, []);
        assert_eq!(post, []);
        val[0]
    }

    fn integer(&mut self) -> i32 {
        let mut buffer = [0u8; 4];
        if let Err(error) = self.read_exact(&mut buffer) {
            panic!("LIB ERROR: Could not read bytes: {}", error);
        }
        let (pre, val, post) = unsafe { buffer.align_to::<i32>() };
        assert_eq!(pre, []);
        assert_eq!(post, []);
        val[0]
    }

    fn longlong(&mut self) -> i64 {
        let mut buffer = [0u8; 8];
        if let Err(error) = self.read_exact(&mut buffer) {
            panic!("LIB ERROR: Could not read bytes: {}", error);
        }
        let (pre, val, post) = unsafe { buffer.align_to::<i64>() };
        assert_eq!(pre, []);
        assert_eq!(post, []);
        val[0]
    }

    fn double(&mut self) -> f64 {
        let mut buffer = [0u8; 8];
        if let Err(error) = self.read_exact(&mut buffer) {
            panic!("LIB ERROR: Could not read bytes: {}", error);
        }
        let (pre, val, post) = unsafe { buffer.align_to::<f64>() };
        assert_eq!(pre, []);
        assert_eq!(post, []);

        let _digits = self.integer();
        val[0]
    }

    fn string(&mut self, size: usize) -> String {
        let mut out = String::new();
        let buffer = self.nbytes(size);
        let (pre, chars, post) = unsafe { buffer.align_to::<char>() };
        assert_eq!(pre, []);
        assert_eq!(post, []);

        out.extend(chars.iter());
        out
    }

    fn field_type_code(&mut self) -> u8 {
        self.byte()
    }
}
