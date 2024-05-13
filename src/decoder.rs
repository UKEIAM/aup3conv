use std::fs::File;
use std::io::{Read, Result, Seek, Write, BufWriter};
use std::collections::HashMap;
use rusqlite::blob::Blob;


pub struct TagDictDecoder<'a> {
    pub xcs: u8,
    pub dict: HashMap<i16, String>,
    blob: Blob<'a>,
}


impl<'a> TagDictDecoder<'a> {
    pub fn new(blob:  Blob<'a>) -> TagDictDecoder<'a> {
        TagDictDecoder {
            xcs: 0,
            dict: HashMap::new(),
            blob: blob,
        }
    }

    pub fn get_dict(&mut self) -> &HashMap<i16, String> {
        &mut self.dict
    }

    fn read_char_size(&mut self) {
        self.xcs = self.blob.byte();
    }

    fn read_ft_name(&mut self) {
        let id = self.read_field_id();
        let size = self.read_name_length();
        let name = self.blob.string(size);
        self.dict.insert(id, name);
    }

    fn read_name_length(&mut self) -> usize {
        self.blob.short() as usize
    }

    fn read_field_id(&mut self) -> i16 {
        self.blob.short()
    }

    pub fn decode(&mut self) {
        loop {
            let pos = self.blob.stream_position().expect("Cannot read position");
            if pos as usize >= self.blob.len() { break }
            let a = self.blob.field_type_code();
            match a {
                0 => { self.read_char_size() },
                15 => { self.read_ft_name() },
                x => { println!("break {}", x); },
            };
        }
    }

}


pub struct ProjectDecoder<'a> {
    pub xcs: u8,
    blob: Blob<'a>,
    id_stack: Vec<i16>,
    offset: String,
    dict: &'a HashMap<i16, String>,
    out: &'a mut BufWriter<File>,
}

impl<'a> ProjectDecoder<'a> {
    pub fn new(
        blob: Blob<'a>,
        dict: &'a HashMap<i16, String>,
        fw: &'a mut BufWriter<File>) -> Self {

        ProjectDecoder {
            xcs: 0,
            blob: blob,
            id_stack: Vec::<i16>::new(),
            offset: String::from(""),
            dict: dict,
            out: fw,
        }
    }

    pub fn decode(&mut self) -> Result<()> {
        loop {
            let pos = self.blob.stream_position().expect("Cannot read position");
            if pos as usize >= self.blob.len() { break; }
            let a = self.blob.field_type_code();
            match a {
                0 => self.read_ft_char_size()?,
                1 => self.read_ft_start_tag()?,
                2 => self.read_ft_end_tag()?,
                3 => self.read_ft_string()?,
                4 => self.read_ft_int()?,
                5 => self.read_ft_bool()?,
                6 => self.read_ft_long()?,
                7 => self.read_ft_long_long()?,
                8 => self.read_ft_size_t()?,
                9 => self.read_ft_float()?,
                10 => self.read_ft_double()?,
                11 => self.read_ft_data()?,
                12 => self.read_ft_raw()?,
                13 => self.read_ft_push()?,
                14 => self.read_ft_pop()?,
                15 => self.read_ft_name()?,
                xyz => { println!("BREAK {}", xyz); break; },
            };
        }
        Ok(())
    }

    fn read_ft_char_size(&mut self) -> Result<()> {
        self.xcs = self.blob.byte();
        Ok(())
    }

    fn read_ft_start_tag(&mut self) -> Result<()> {
        let id = self.read_field_id();
        self.id_stack.push(id);
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));

        if self.id_stack.len() == 0 {
            write!(&mut self.out, ">")?;
        }
        write!(&mut self.out, "{}<{}", self.offset, name)?;
        self.offset.push('\t');
        Ok(())
    }

    fn read_ft_end_tag(&mut self) -> Result<()> {
        let id = self.read_field_id();
        self.offset.pop();
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        if self.id_stack.len() == 1 {
            write!(&mut self.out, ">")?;
        }
        writeln!(&mut self.out, "{}</{}>", self.offset, name)
    }

    fn read_ft_int(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let val = self.blob.integer();
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, val)
    }

    fn read_ft_string(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let size = self.read_field_size();
        let val = self.blob.string(size);
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, val)
    }

    fn read_ft_bool(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let val = self.blob.byte();
        let out = match val {
            1 => "true",
            _ => "false"};
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, out)
    }

    fn read_ft_long(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let val = self.blob.integer();
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, val)
    }

    fn read_ft_long_long(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let val = self.blob.longlong();
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, val)
    }

    fn read_ft_size_t(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let val = self.blob.integer();
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, val)
    }

    fn read_ft_float(&mut self)  -> Result<()> {
        Ok(())
    }

    fn read_ft_double(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let val = self.blob.double();
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, val)
    }

    fn read_ft_data(&mut self)  -> Result<()> {
        Ok(())
    }

    fn read_ft_raw(&mut self) -> Result<()> {
        let size = self.read_field_size();
        let val = self.blob.string(size);
        write!(&mut self.out, "{}", val)?;
        Ok(())
    }

    fn read_ft_push(&mut self) -> Result<()> {
        let _ = self.blob.byte();
        Ok(())
    }

    fn read_ft_pop(&mut self) -> Result<()> {
        let _ = self.blob.byte();
        Ok(())
    }

    fn read_ft_name(&mut self) -> Result<()> {
        let _id = self.read_field_id();
        let size = self.read_name_length();
        let _name = self.blob.string(size);
        Ok(())
    }

    fn read_field_id(&mut self) -> i16 {
        self.blob.short()
    }

    fn read_field_size(&mut self) -> usize {
        self.blob.integer() as usize
    }

    fn read_name_length(&mut self) -> usize {
        self.blob.short() as usize
    }
}

pub trait Decode {
    fn byte(&mut self) -> u8;
    fn nbytes(&mut self, size: usize) -> Vec<u8>;
    fn short(&mut self) -> i16;
    fn integer(&mut self) -> i32;
    fn longlong(&mut self) -> i64;
    fn double(&mut self) -> f64;
    fn string(&mut self, size: usize) -> String;
    fn field_type_code(&mut self) -> u8;
}


impl Decode for Blob<'_> {

    fn byte(&mut self) -> u8 {
        let mut buffer = [0u8; 1];
        match self.read_exact(&mut buffer) {
            Ok(()) => buffer[0],
            Err(error) => panic!("In byte: {}", error),
        }
    }

    fn nbytes(&mut self, size: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; size];
        match self.read_exact(&mut buffer) {
            Ok(()) => buffer,
            Err(error) => panic!("In bytes: {}", error),
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
