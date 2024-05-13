
pub mod metadata {


    use std::io::Read;
    use std::io::Seek;
    use rusqlite::blob::Blob;

    pub struct ProjectData {
        buffer: Vec<u8>,
        field_type: [u8; 1],
    }

    impl ProjectData {
        pub fn new() -> ProjectData {

            ProjectData {
                buffer: Vec::with_capacity(1024),
                field_type: [255; 1],
            }
        }

        pub fn values(&self) -> &Vec<u8> {
            &self.buffer
        }

        pub fn code(&self) -> u8 {
            self.field_type[0]
        }

        fn read_field_type(&mut self, blob: &mut Blob) {
            let pos = blob.stream_position().expect("ASDASD");
            match blob.read_exact(&mut self.field_type) {
                Ok(()) => (), //println!("POS: {}, CODE: {}", pos, self.field_type[0]),
                Err(error) => println!("Error: {}", error)
           }
        }

        pub fn parse(&mut self, mut blob: &mut Blob) {
            let mut parser = ReadFT::new(false);

            loop  {
                let p = blob.stream_position().expect("ASD");
                if p as usize >= blob.len() { break }
                self.read_field_type(&mut blob);
                match self.code() {
                    1 => parser.start_tag(&mut blob),
                    2 => parser.end_tag(&mut blob),
                    3 => parser.parse_string(&mut blob),
                    4 => parser.parse_int(&mut blob),
                    5 => parser.parse_bool(&mut blob),
                    6 => parser.parse_long(&mut blob),
                    7 => parser.parse_long_long(&mut blob),
                    8 => parser.parse_size_t(&mut blob),
                    10 => parser.parse_double(&mut blob),
                    12 => parser.raw(&mut blob),
                    x => { println!("BREAK BC {}", x); break; },
                };
            }
        }

    }

    pub struct ReadFT {
        data: String,
        verbose: bool,
        id_stack: Vec<u16>,
        fmt: String,
    }


    impl ReadFT {
        pub fn new(verbose: bool) -> Self {
            ReadFT {
                data: String::new(),
                verbose: verbose,
                id_stack: Vec::<u16>::new(),
                fmt: String::from(""),
            }
        }

        pub fn print(&self) {
            println!("{}", self.data);
        }

        pub fn ids(&self) -> &Vec<u16> {
            &self.id_stack
        }

        pub fn parse_string(&mut self, blob: &mut Blob) {
            let _id  = ReadFT::read_id(blob);
            let size = ReadFT::field_size(blob);
            let bytes = ReadFT::read_bytes(blob, size as usize);
            let (_pre, chars, _post) = unsafe { bytes.align_to::<char>() };
            self.data.extend(chars.iter());
            let mut x = String::new();
            x.extend(chars.iter());
            println!(" {}{:?}", self.fmt, x);
        }

        pub fn parse_double(&mut self, blob: &mut Blob) {
            let _id  = ReadFT::read_id(blob);
            let bytes = ReadFT::read_bytes(blob, 8);
            let (_pre, val1, _post) = unsafe { bytes.align_to::<f64>() };
            let bytes = ReadFT::read_bytes(blob, 4);
            let (_pre, _val2, _post) = unsafe { bytes.align_to::<i32>() };
            println!(" {}{}", self.fmt, val1[0]);
        }

        pub fn parse_int(&mut self, blob: &mut Blob) {
            self.parse_field_id(blob);
            let bytes = ReadFT::read_bytes(blob, 4);
            let (_pre, val, _post) = unsafe { bytes.align_to::<i32>() };
            println!(" {}{}", self.fmt, val[0]);
        }

        pub fn parse_bool(&mut self, blob: &mut Blob) {
            self.parse_field_id(blob);
            let bytes = ReadFT::read_bytes(blob, 1);
            let x = match bytes[0] {
                1 => "true",
                _ => "false"};
            println!(" {}{}", self.fmt, x);
        }

       pub fn parse_long(&mut self, blob: &mut Blob) {
            self.parse_field_id(blob);
            let bytes = ReadFT::read_bytes(blob, 4);
            let (_pre, val, _post) = unsafe { bytes.align_to::<i32>() };
            println!(" {}{}", self.fmt, val[0]);
       }

        pub fn parse_long_long(&mut self, blob: &mut Blob) {
            self.parse_field_id(blob);
            let bytes = ReadFT::read_bytes(blob, 8);
            let (_pre, val, _post) = unsafe { bytes.align_to::<i32>() };
            println!(" {}{}", self.fmt, val[0]);
        }

        pub fn parse_size_t(&mut self, blob: &mut Blob) {
            self.parse_field_id(blob);
            let bytes = ReadFT::read_bytes(blob, 4);
            let (_pre, val, _post) = unsafe { bytes.align_to::<i32>() };
            println!(" {}{}", self.fmt, val[0]);
       }

        pub fn start_tag(&mut self, blob: &mut Blob) {
            let id = ReadFT::read_id(blob);
            self.id_stack.push(id);
            println!("{}{} {}", self.fmt, "<-- Start tag: ", id);
            self.fmt.push('\t');
        }

        pub fn end_tag(&mut self, blob: &mut Blob) {
            let id = ReadFT::read_id(blob);
            self.fmt.pop();
            println!("{}{} {}", self.fmt, "<-- End tag: ", id);

        }

        pub fn parse_tag(&mut self, blob: &mut Blob, descr: String) {
            self.parse_field_id(blob);
            if self.verbose {
                match self.id_stack.last() {
                    Some(x) => println!("{} -- ID: {}", descr, x),
                    None => println!("EMptY"),
                }
            }
        }

        pub fn raw(&mut self, blob: &mut Blob) {
            let fs = ReadFT::field_size(blob);
            let bytes = ReadFT::read_bytes(blob, fs as usize);
            let (_pre, chars, _post) = unsafe { bytes.align_to::<char>() };
            self.data.extend(chars.iter());
        }

        pub fn field_size(mut blob: &mut Blob) -> i32 {
            let raw = ReadFT::read_bytes(&mut blob, 4);
            let (_pre, value, _post) = unsafe { raw.align_to::<i32>() };
            value[0]
        }

        fn parse_field_id(&mut self, blob: &mut Blob) {
            let _id = ReadFT::read_id(blob);
        }


        fn read_id(blob: &mut Blob) -> u16 {
            let mut bytes = [9u8; 2];
            blob.read_exact(&mut bytes).expect("Cannot read ID");
            let (_pre, value, _post) = unsafe { bytes.align_to::<u16>() };
            value[0]
        }

        fn read_bytes(blob: &mut Blob, size: usize) -> Vec<u8> {
            let mut raw = vec![0u8; size];
            match blob.read_exact(&mut raw) {
                Ok(()) => raw,
                Err(error) => {
                    let mut y = Vec::<u8>::new();
                    blob.read_to_end(&mut y).expect("HARD ERROR");
                    y
                }
            }
        }
    }

    pub enum FieldType {
        CharSize = 0,
        StartTag = 1,
        EndTag = 2 ,
        String = 3,
        Int = 4,
        Bool = 5,
        Long = 6,
        LongLong = 7,
        SizeT = 8,
        Float = 9,
        Double = 10,
        Data = 11,
        Raw = 12,
        Push = 13,
        Pop = 14,
        Name = 15,
    }

}


#[cfg(test)]
mod tests {

    #[test]
    fn test_a() {
        assert!(true);
    }
}

       // fn reinterpret<T>(bytes: &[u8]) -> &T {
       //     let (pre, val, post) = unsafe { bytes.align_to::<T>() };
       //     assert_eq!(pre, []);
       //     assert_eq!(post, []);
       //     &val[0]
       // }
