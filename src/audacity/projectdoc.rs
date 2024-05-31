use std::io::Seek;
use std::io::Result;
use std::fmt::Display;
use rusqlite::{Connection, DatabaseName};
use rusqlite::blob::Blob;

use crate::audacity::tagdict::TagDict;
use crate::audacity::fields::{CharSize, FieldType, ReadDocField};
use crate::audacity::decoder::Decoder;
use crate::tagstack::{Tag, TagStack};
use crate::structure::*;


pub struct ProjectDocReader {
    char_size: u8
}


impl ProjectDocReader {
    pub fn new(cs: u8) -> Self {
        Self {
            char_size: cs
        }
    }
}


impl CharSize for ProjectDocReader {
    fn chs(&self) -> u8 {
        self.char_size
    }
}


impl ReadDocField for ProjectDocReader {
    fn read_field(&self, blob: &mut Blob) -> FieldType {
        let ftc = blob.field_type_code();
        match ftc {
             0 => self.char_size(blob),
             1 => self.start_tag(blob),
             2 => self.end_tag(blob),
             3 => self.str(blob),
             4 => self.integer(blob),
             5 => self.boolean(blob),
             6 => self.long(blob),
             7 => self.longlong(blob),
             8 => self.size_t(blob),
             9 => self.float(blob),
            10 => self.double(blob),
            11 => self.data(blob),
            12 => self.raw(blob),
            13 => self.push(blob),
            14 => self.pop(blob),
            15 => self.name(blob),
             _ => panic!("Unknown field type code detected"),
        }
    }

    fn char_size(&self, blob: &mut Blob) -> FieldType {
        FieldType::CharSize { value: blob.byte() }
    }

    fn start_tag(&self, blob: &mut Blob) -> FieldType {
        let id = blob.short();
        FieldType::StartTag { id: id }
    }
    fn end_tag(&self, blob: &mut Blob) -> FieldType {
        let id = blob.short();
        FieldType::EndTag { id: id }
    }

    fn str(&self, blob: &mut Blob) -> FieldType {
        let id = blob.short();
        let size = blob.integer();
        FieldType::Str { id: id, size: size, value: blob.string(size as usize, self.chs()).clone() }
    }

    fn integer(&self, blob: &mut Blob) -> FieldType {
        FieldType::Int { id: blob.short(), value: blob.integer() }
    }

    fn boolean(&self, blob: &mut Blob) -> FieldType {
        let id = blob.short();
        let value = match blob.byte() {
            1 => true,
            0 => false,
            _ => { panic!("Somethings is really wrong"); }
        };
        FieldType::Bool { id: id, value: value }
    }

    fn long(&self, blob: &mut Blob) -> FieldType {
        FieldType::Long { id: blob.short(), value: blob.integer() }
    }

    fn longlong(&self, blob: &mut Blob) -> FieldType {
        FieldType::LongLong { id: blob.short(), value: blob.longlong() as i64 }
    }

    fn size_t(&self, blob: &mut Blob) -> FieldType {
        FieldType::SizeT { id: blob.short(), value: blob.integer() as usize }
    }

    fn float(&self, blob: &mut Blob) -> FieldType {
        FieldType::Float { id: blob.short(), value: blob.double() as f32, digits: 0 }
    }

    fn double(&self, blob: &mut Blob) -> FieldType {
        FieldType::Double { id: blob.short(), value: blob.double(), digits: 0 }
    }

    fn data(&self, blob: &mut Blob) -> FieldType {
        let size = blob.integer();
        FieldType::Data { size: size, value: blob.string(size as usize, self.chs()).clone() }
    }

    fn raw(&self, blob: &mut Blob) -> FieldType {
        let size = blob.integer();
        FieldType::Raw { size: size, value: blob.string(size as usize, self.chs()).clone() }
    }

    fn push(&self, _blob: &mut Blob) -> FieldType {
        FieldType::Push
    }

    fn pop(&self, _blob: &mut Blob) -> FieldType {
        FieldType::Pop
    }

    fn name(&self, blob: &mut Blob) -> FieldType {
        let id = blob.short();
        let size = blob.short();
        FieldType::Name { id: id, size: size, value: blob.string(size as usize, self.chs()) }
    }
}


pub struct ProjectDoc {
    reader: ProjectDocReader,
    tagdict: TagDict,
    tags: TagStack,
    raw: String,
}


impl ProjectDoc {
    pub fn new(tagdict: TagDict) -> Self {
        Self {
            reader: ProjectDocReader::new(tagdict.chs()),
            tagdict: tagdict,
            tags: TagStack::new(),
            raw: String::new(),
        }
    }

    pub fn decode(&mut self, con: &Connection) -> Result<()>{

        let mut blob = con.blob_open(DatabaseName::Main, "project",
            "doc", 1, true).expect("Failed to read blob");

        while (blob.stream_position().expect("Cannot read position") as usize) < blob.len() {
            match self.reader.read_field(&mut blob) {
                FieldType::CharSize { value } => { self.reader.char_size = value; },
                FieldType::StartTag { id } => { self.add_tag(id); },
                FieldType::EndTag { id: _ } => { self.tags.decrease_level(); },
                FieldType::Str { id, size: _, value } => { self.add_attribute(id, value); },
                FieldType::Int { id, value } => { self.add_attribute(id, value); },
                FieldType::Bool { id, value } => { self.add_attribute(id, value); },
                FieldType::Long { id, value } => { self.add_attribute(id, value); },
                FieldType::LongLong { id, value } => { self.add_attribute(id, value); },
                FieldType::SizeT { id, value } => { self.add_attribute(id, value); },
                FieldType::Float { id, value, .. } => { self.add_attribute(id, value); },
                FieldType::Double { id, value, .. } => { self.add_attribute(id, value); },
                FieldType::Data { size: _, value: _ } => { panic!("FIELD TYPE <DATA> encountered"); },
                FieldType::Raw { size: _, value } => { self.collect(value) },
                FieldType::Push => { },
                FieldType::Pop => { },
                FieldType::Name { id: _, size: _ , value: _ } => { }
            }
        }
        Ok(())
    }

    fn add_tag(&mut self, id: i16) {
        let name = self.tagdict.dict.get(&id).expect("wer");
        self.tags.add_tag(name)
    }

    fn add_attribute<T: Display>(&mut self, id: i16, value: T) {
        let name = self.tagdict.dict.get(&id).expect("Missing");
        match self.tags.stack.last_mut() {
            Some(tag) => tag.add_attribute(name, &value.to_string()),
            None => panic!("WURST")
        }
    }

    fn collect(&mut self, value: String) {
        self.raw.push_str(&value);
    }

    pub fn parse_labels(&mut self) -> Result<Option<Vec<Label>>> {
        let mut out = Vec::<Label>::new();
        for tag in self.tags.stack.iter() {
            if tag.name == "label" { out.push(Label::from_tag(&tag)?) };
        }
        if out.is_empty() {
            return Ok(None)
        }
        Ok(Some(out))
    }

    pub fn parse_sample_rate(&mut self) -> Option<i64> {
        match self.get_tag_by_name("project") {
            Some(tag) => match tag.attributes.get("rate") {
                Some(rate) => match rate.parse::<i64>() {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                None => None,
            },
            None => None
        }
    }

    pub fn parse_waveblocks(&mut self) -> Result<Option<Vec<WaveBlock>>> {
        let mut out = Vec::<WaveBlock>::new();
        for tag in self.tags.stack.iter() {
            if tag.name == "waveblock" { out.push(WaveBlock::from_tag(&tag)?) };
        }
        if out.is_empty() {
            return Ok(None)
        }
        Ok(Some(out))
    }

    fn get_tag_by_name(&mut self, name: &str) -> Option<&Tag> {
        for tag in self.tags.stack.iter() {
            if tag.name == *name {
                return Some(tag);
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_project_from_aup3() {
        let con = Connection::open("data/test-project.aup3").expect("open failed");
        let mut tagdict = TagDict::new();
        tagdict.decode(&con);

        let mut project = ProjectDoc::new(tagdict);
        let _ = project.decode(&con);
    }
}
