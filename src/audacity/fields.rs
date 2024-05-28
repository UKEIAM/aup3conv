use rusqlite::blob::Blob;

#[derive(Debug)]
pub enum FieldType {
    CharSize { value: u8 },
    StartTag { id: i16 },
    EndTag { id: i16 },
    Str { id: i16, size: i32, value: String },
    Int { id: i16, value: i32 },
    Bool { id: i16, value: bool },
    Long { id: i16, value: i32 },
    LongLong { id: i16, value: i64},
    SizeT { id: i16, value: usize },
    Float { id: i16, value: f32, digits: u8 },
    Double { id: i16, value: f64, digits: u8 },
    Data { size: i32, value: String },
    Raw { size: i32, value: String },
    Push,
    Pop,
    Name { id: i16, size: i16, value: String }
}


pub trait ReadDocField {
    fn read_field(&self, blob: &mut Blob) -> FieldType;
    fn char_size(&self, blob: &mut Blob) -> FieldType;
    fn start_tag(&self, blob: &mut Blob) -> FieldType;
    fn end_tag(&self, blob: &mut Blob) -> FieldType;
    fn str(&self, blob: &mut Blob) -> FieldType;
    fn integer(&self, blob: &mut Blob) -> FieldType;
    fn boolean(&self, blob: &mut Blob) -> FieldType;
    fn long(&self, blob: &mut Blob) -> FieldType;
    fn longlong(&self, blob: &mut Blob) -> FieldType;
    fn size_t(&self, blob: &mut Blob) -> FieldType;
    fn float(&self, blob: &mut Blob) -> FieldType;
    fn double(&self, blob: &mut Blob) -> FieldType;
    fn data(&self, blob: &mut Blob) -> FieldType;
    fn raw(&self, blob: &mut Blob) -> FieldType;
    fn push(&self, blob: &mut Blob) -> FieldType;
    fn pop(&self, blob: &mut Blob) -> FieldType;
    fn name(&self, blob: &mut Blob) -> FieldType;
}


pub trait CharSize {
    fn chs(&self) -> u8;
}

pub trait ReadDictField: CharSize {
    fn read_field(&self, blob: &mut Blob) -> FieldType;
    fn char_size(&self, blob: &mut Blob) -> FieldType;
    fn name(&self, blob: &mut Blob) -> FieldType;
}
