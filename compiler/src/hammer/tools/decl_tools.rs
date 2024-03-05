use crate::hammer::memory::Memory;

pub struct DeclTools {
    name: String,
    type_name: String,
    stars: u32
}

impl DeclTools {

    pub fn new() -> DeclTools {
        DeclTools {
            name: String::new(),
            type_name: String::new(),
            stars: 0
        }
    }

    pub fn new_star(&mut self, content: String) {
        if content == "*" {
            self.stars += 1;
        }else{
            panic!("Bad symbol: {} when a star was expected", content);
        }
    }

    pub fn def_type(&mut self, t: String) {
        self.type_name = t;
    }

    pub fn def_name(&mut self, name: String, memory: &mut Memory) {
        self.name = name;
        memory.new_var(self.type_name.clone(), self.name.clone(), self.stars);
    }

    pub fn end(&mut self) {
        self.name.clear();
        self.type_name.clear();
        self.stars = 0;
    }

}