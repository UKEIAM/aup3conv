use std::fs::File;
use std::io::{Read, Result, Seek, Write, BufWriter};
use std::collections::HashMap;
use rusqlite::blob::Blob;

use crate::audacity::{Decoder, ReadFieldType, TagDictReader};
use crate::tagstack::{Tag, TagStack};

use rusqlite::{Connection, DatabaseName};


pub struct ProjectDecoder<'a> {
    pub xcs: u8,
    id_stack: Vec<i16>,
    offset: String,
    dict: &'a HashMap<i16, String>,
    out: &'a mut BufWriter<File>,
    tagstack: Vec<Tag>,
    current_tag: Option<&'a mut Tag>,
}


    fn read_ft_start_tag(&mut self) -> Result<()> {
        let id = self.read_field_id();

        self.id_stack.push(id);
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        let tag = Tag::new(name.clone());
        self.tagstack.push(tag);

        if self.id_stack.len() == 0 {
            write!(&mut self.out, ">")?;
        }
        // println!("ID: {} -- {} start", id, name);
        write!(&mut self.out, "\n{}<{}", self.offset, name)?;
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
        // println!("ID: {} -- {} end", id, name);
        writeln!(&mut self.out, ">\n{}</{}>", self.offset, name)
    }

    fn read_ft_int(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let val = blob.integer();
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));

        // println!("Name: {}, val: {}, int", name, val);
        if let Some(tag) = self.tagstack.last() {

        }

        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, val)
    }

    fn read_ft_string(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let size = self.read_field_size();
        let val = blob.string(size);
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        // println!("Name: {}, val: {}, str", name, val);
        if let Some(tag) = self.tagstack.last_mut() {
            tag.add_attribute(String::from(name), val.to_string());
        }
        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, val)
    }

    fn read_ft_bool(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let val = self.blob.byte();
        let out = match val {
            1 => "true",
            _ => "false"};
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        if let Some(tag) = self.tagstack.last_mut() {
            tag.add_attribute(String::from(name), val.to_string());
        }
        // println!("Name: {}, val: {}, bool", name, val);
        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, out)
    }


    fn read_ft_long(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let val = self.blob.integer();
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        // println!("Name: {}, val: {}, long", name, val);
        if let Some(tag) = self.tagstack.last_mut() {
            tag.add_attribute(String::from(name), val.to_string());
        }
        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, val)
    }

    fn read_ft_long_long(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let val = self.blob.longlong();
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        // println!("Name: {}, val: {}, long long", name, val);
        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, val)
    }

    fn read_ft_size_t(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let val = self.blob.integer();
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        // println!("Name: {}, val: {}, size_t", name, val);
        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, val)
    }

    fn read_ft_float(&mut self)  -> Result<()> {
        Ok(())
    }

    fn read_ft_double(&mut self) -> Result<()> {
        let id = self.read_field_id();
        let val = self.blob.double();
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        // println!("Name: {}, val: {}, doublej", name, val);
        write!(&mut self.out, "{}{}=\"{}\"", self.offset, name, val)
    }

    fn read_ft_data(&mut self)  -> Result<()> {
        Ok(())
    }

    fn read_ft_raw(&mut self) -> Result<()> {
        let size = self.read_field_size();
        let val = self.blob.string(size);
        println!("{}", val);
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
