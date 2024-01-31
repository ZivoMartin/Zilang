use crate::stack::Stack;
use std::collections::HashMap;
struct Insertion {
    value: Option<i64>,
    line: u32,
}

pub struct Tracker{
    registers: Registers,
    stack: Stack<Insertion>,
    line_to_delete: Vec<u32>,
    inst_map: HashMap::<String, fn (&mut Tracker, Vec<&str>, u32)->Option<String>>,
    useless_inst: &'static str,
    last_cmp: Option<(i64, i64)>
}

impl Tracker {

    pub fn new() -> Tracker {
        let mut res = Tracker{
            registers: Registers::new(),
            stack: Stack::new(),
            line_to_delete: Vec::new(),
            inst_map: HashMap::<String, fn (&mut Tracker, Vec<&str>, u32)->Option<String>>::new(),
            useless_inst: "call ret jmp _deref",
            last_cmp: None
        };
        res.init_inst_map();
        res
    }   

    fn init_inst_map(&mut self) {
        self.inst_map.insert(String::from("pop"), Tracker::pop_inst);
        self.inst_map.insert(String::from("push"), Tracker::push_inst);
        self.inst_map.insert(String::from("xor"), Tracker::xor_inst);
        self.inst_map.insert(String::from("mov"), Tracker::memory_action);
        self.inst_map.insert(String::from("movzx"), Tracker::memory_action);
        self.inst_map.insert(String::from("movsx"), Tracker::memory_action);
        self.inst_map.insert(String::from("xor"), Tracker::xor_inst);
        self.inst_map.insert(String::from("add"), Tracker::memory_action);
        self.inst_map.insert(String::from("mul"), Tracker::mul_inst);
        self.inst_map.insert(String::from("cmp"), Tracker::cmp_inst);
        self.inst_map.insert(String::from("jg"), Tracker::cond_inst);
        self.inst_map.insert(String::from("je"), Tracker::cond_inst);
    }


    pub fn new_inst(&mut self, inst: &str, inst_number: u32) -> String {
        let tokens = tokenise_asm_inst(inst);
        println!("{inst}\n{tokens:?}");
        if !self.useless_inst.contains(tokens[0]) && tokens.len() > 1{
            return match self.inst_map.get(tokens[0]).unwrap()(self, tokens.clone(), inst_number) {
                Some(new_inst) => {
                    println!("new inst: {}\n", new_inst);
                    return new_inst
                }
                _ => {
                    println!("stay the same: {}\n", inst);
                    String::from(inst)
                }
            }
        }
        return String::from(inst)
    }

    pub fn clean_line(&mut self, txt: &mut String) {
        let mut split_txt: Vec<&str> = txt.split("\n").collect();
        for line_number in self.line_to_delete.iter().rev() {
            split_txt.remove(*line_number as usize);
        }
        self.line_to_delete = Vec::new();
    }

    fn memory_action(&mut self, tokens: Vec<&str>, _inst_number: u32) -> Option<String> {
        let opt_val = self.registers.extract_val(&tokens[2]);
        if opt_val.is_some() {
            let val = opt_val.unwrap();
            match tokens[0] as &str{
                "add" => {
                    if !self.registers.add_val(tokens[1], val){
                        return Some(format!("add {}, {}", tokens[1], val))
                    }
                }
                _ => self.registers.set_val(tokens[1], Some(val))
            }
            return Some(String::new())
        }
        self.registers.set_val(tokens[1], None);
        return None
    }

    fn mul_inst(&mut self, tokens: Vec<&str>, _inst_number: u32) -> Option<String> {
        let val_rax = self.registers.get_val("rax");
        if val_rax.is_some(){
            if self.registers.mul_val(tokens[1], val_rax.unwrap()) {
                return Some(String::new())
            }else{
                return Some(format!("mov rax, {}\nmul {}", val_rax.unwrap(), tokens[1]))
            }   
        }else{
            let val_token = self.registers.get_val(tokens[1]);
            if val_token.is_some(){
                return Some(format!("mov {}, {}\nmul {}", tokens[1], val_token.unwrap(), tokens[1]))
            }else{
                return None
            }   
        }
    }

    fn cmp_inst(&mut self, tokens: Vec<&str>, _inst_number: u32) -> Option<String> {
        match self.registers.get_val(tokens[1]) {
            Some(val1) => {
                match self.registers.get_val(tokens[2]) {
                    Some(val2) => {
                        self.last_cmp = Some((val1, val2));
                        return Some(String::new())
                    }
                    _ => return Some(format!("cmp {}, {}", val1, tokens[2]))
                }
            }
            _ => {
                match self.registers.get_val(tokens[2]) {
                    Some(val2) => return Some(format!("cmp {}, {}", tokens[1], val2)),
                    _ => return None
                }
            }
        }
    }

    fn push_inst(&mut self, tokens: Vec<&str>, inst_number: u32) -> Option<String> {
        self.stack.push(Insertion{
            value: self.registers.get_val(tokens[1]),
            line: inst_number,
        });
        None
    }

    fn pop_inst(&mut self, tokens: Vec<&str>, _inst_number: u32) -> Option<String> {
        let last_insert = self.stack.pop();
        if last_insert.value.is_some(){
            self.line_to_delete.push(last_insert.line);
            return Some(String::from(format!("mov {}, {}", tokens[1], last_insert.value.unwrap())));
        }
        None
    }

    fn xor_inst(&mut self, tokens: Vec<&str>, _inst_number: u32) -> Option<String> {
        self.registers.set_val(tokens[1], Some(0));
        Some(String::new())
    }


    fn cond_inst(&mut self, tokens: Vec<&str>, _inst_number: u32) -> Option<String>{
        if !self.last_cmp.is_some() {
            return None
        }
        let cmp = self.last_cmp.unwrap();
        let cond: bool;
        match tokens[0] {
            "jg" => cond = cmp.0 > cmp.1,
            "je" => cond = cmp.0 == cmp.1,
            _ => panic!("found this token: {}", tokens[1])
        }
        if cond {
            return Some(format!("jmp {}", tokens[1]));
        }
        return Some(String::new())
    }
    

} 

fn tokenise_asm_inst(inst: &str) -> Vec<&str> {
    let split_inst: Vec::<&str> = inst.split(" ").collect();
    match split_inst.len() {
        4 => {
            match inst.chars().last().unwrap() {
                ']' => {
                    return vec!(split_inst[0], &split_inst[1][0..split_inst[1].len()-1], &split_inst[3][0..split_inst[3].len()])
                }
                _ => return vec!(split_inst[0], &split_inst[2][0..split_inst[2].len()-1], split_inst[3]),
            }
        }
        3 => return vec!(split_inst[0], &split_inst[1][0..split_inst[1].len()-1], split_inst[2]),
        _ => return split_inst
    }
}


struct Registers {
    map: HashMap<&'static str, Option<i64>>,
    convert: HashMap<String, &'static str>
}

impl Registers {
    pub fn new() -> Registers{
        let mut res = Registers{
            map: HashMap::<&'static str, Option<i64>>::new(),
            convert: HashMap::<String, &'static str>::new() 
        };
        res.map.insert("rax", None);
        res.map.insert("rbx", None);
        res.map.insert("rdx", None);
        res.convert.insert(String::from("rax"), "rax");
        res.convert.insert(String::from("eax"), "rax");
        res.convert.insert(String::from("ax"), "rax");
        res.convert.insert(String::from("rbx"), "rax");
        res.convert.insert(String::from("ebx"), "rax");
        res.convert.insert(String::from("bx"), "rax");
        res.convert.insert(String::from("rdx"), "rdx");
        res.convert.insert(String::from("edx"), "rdx");
        res.convert.insert(String::from("dx"), "rdx");
        res
    }

    pub fn get_val(&self, register: &str) -> Option<i64> {
        if self.is_followed(register){
            return self.convert(register).clone()
        }
        None
    }

    pub fn set_val(&mut self, register: &str, val: Option<i64>) {
        if self.is_followed(register){
            *self.convert_mut(register) = val;
        }
    }

    pub fn add_val(&mut self, register: &str, val: i64) -> bool{
        if self.is_followed(register){
            let previous_val = self.convert(register);
            if previous_val.is_some(){
                self.set_val(register, Some(val+previous_val.unwrap()));
                return true
            }
        }
        return false
    }

    pub fn mul_val(&mut self, register: &str, val: i64) -> bool{
        if self.is_followed(register){
            let previous_val = self.convert(register);
            if previous_val.is_some(){
                self.set_val(register, Some(val*previous_val.unwrap()));
                return true
            }
        }
        return false
    }

    fn convert(&self, register: &str) -> &Option<i64> {
        self.map.get(self.convert.get(register).unwrap()).unwrap()
    }

    fn convert_mut(&mut self, register: &str) -> &mut Option<i64> {
        self.map.get_mut(self.convert.get(register).unwrap()).unwrap()
    }

    pub fn is_followed(&self, register: &str) -> bool {
        self.convert.contains_key(register)
    }

    pub fn extract_val(&self, elt: &str) -> Option<i64> {
        match str::parse::<i64>(elt) {
            Ok(res) => Some(res),
            _ => {
                if self.is_followed(elt) {
                    return self.get_val(elt)
                }
                None
            }
        }
    }
}
