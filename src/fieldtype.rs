use std::io::Result;


pub trait ReadFieldType {
    fn char_size(&mut self) -> Result<()> {
        self.char_size = self.blob.byte();
        Ok(())
    }

    fn start_tag(&mut self) -> Result<()> {
        let id = self.read_field_id();
        self.id_stack.push(id);
        let name = self.dict.get(&id).expect(&format!("No key {}", id));

        if self.id_stack.len() == 0 {
            write!(&mut self.out, ">")?;
        }
        self.offset.push('\t');

        write!(&mut self.out, "{}<{}", self.offset, name)
    }

    fn end_tag(&mut self) -> Result<()> {
        let id = self.read_field_id();
        self.offset.pop();
        let name = self.dict.get(&id).expect(&format!("Bad key `{}`", id));
        if self.id_stack.len() == 1 {
            write!(&mut self.out, ">")?;
        }
        writeln!(&mut self.out, "{}</{}>", self.offset, name)
    }

    fn integer(&mut self) -> Result<()>;
    fn string(&mut self) -> Result<()>;
    fn bool(&mut self) -> Result<()>;
    fn long(&mut self) -> Result<()>;
    fn long_long(&mut self) -> Result<()>;
    fn size_t(&mut self) -> Result<()>;
    fn float(&mut self) -> Result<()>;
    fn double(&mut self) -> Result<()>;
    fn data(&mut self) -> Result<()>;
    fn raw(&mut self) -> Result<()>;
    fn push(&mut self) -> Result<()>;
    fn pop(&mut self) -> Result<()>;
    fn name(&mut self) -> Result<()>;
    fn field_id(&mut self) -> Result<()>;
    fn field_size(&mut self) -> Result<()>;
    fn name_length(&mut self) -> Result<()>;
}
