use crate::hammer::memory::Memory;
pub struct DeclTools {
    name: String,
    type_name: String,
    stars: u32,
    equal_op: String
}

impl DeclTools {

    pub fn new() -> DeclTools {
        DeclTools {
            name: String::new(),
            type_name: String::new(),
            stars: 0,
            equal_op: String::new()
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

    pub fn def_equal_operator(&mut self, op: String) {
        self.equal_op = op;
    }


    pub fn end(&mut self, _memory: &mut Memory) {
        println!("{} {} {} {}", self.type_name, self.name, self.stars, self.equal_op);
        if !self.equal_op.is_empty() {
            match &self.equal_op as &str {
                "=" =>  (),
                "-=" => (),
                "+=" => (),
                "*=" => (),
                "/=" => (),
                "%=" => (),
                _ => panic!("This affect operator is unknow: {}", self.equal_op)
            }
        }
        self.name.clear();
        self.type_name.clear();
        self.equal_op.clear();
        self.stars = 0;
    }

}