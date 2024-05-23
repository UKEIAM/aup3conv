use std::collections::HashMap;

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub attributes: HashMap<String, String>,
    _children: Option<Vec<Tag>>
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            attributes: HashMap::new(),
            _children: None
        }
    }

    pub fn add_attribute(&mut self, name: &String, value: &String) {
        self.attributes.insert(name.clone(), value.clone());
    }
}


#[derive(Debug)]
pub struct TagStack {
    pub stack: Vec<Tag>,
    pub level: Vec<u8>,
    pub current_level: u8,
}

impl TagStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::<Tag>::new(),
            level: Vec::<u8>::new(),
            current_level: 0u8,
        }
    }

    pub fn add_tag(&mut self, name: &String) {
        self.stack.push(Tag::new(name.clone()));
        self.increase_level();
        self.level.push(self.current_level);
    }

    pub fn view(&self) {
        for (tag, level) in self.stack.iter().zip(self.level.iter()) {
            for _ in 1..*level {
                print!("  ");
            }
            println!("{:?} -- {:?}", tag.name, tag.attributes);
        }
    }

    pub fn increase_level(&mut self) {
        self.current_level += 1;
    }

    pub fn decrease_level(&mut self) {
        self.current_level -= 1;
    }

}
