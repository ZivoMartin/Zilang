use crate::stack::Stack;
use std::collections::HashMap;
use crate::tools::tools::{last_char, operation};
pub struct Tracker{
    registers_map: HashMap<String, Registers>,
    current_register_zone: String,
    stack: Stack<Option<i64>>,
    inst_map: HashMap::<String, fn (&mut Tracker, Vec<&str>, &str)->Option<String>>,
    last_cmp: Option<(i64, i64)>,
    nb_ret: u32
}

impl Tracker {

    pub fn new() -> Tracker {
        let mut res = Tracker{
            registers_map: HashMap::<String, Registers>::new(),
            current_register_zone: String::from("global"),
            stack: Stack::new(),
            inst_map: HashMap::<String, fn (&mut Tracker, Vec<&str>, &str)->Option<String>>::new(),
            last_cmp: None,
            nb_ret: 0
        };
        res.registers_map.insert(String::from("global"), Registers::new());
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
        self.inst_map.insert(String::from("_deref"), Tracker::_deref_inst);
        self.inst_map.insert(String::from("call"), Tracker::call_inst);
        self.inst_map.insert(String::from("func"), Tracker::func_inst);
        self.inst_map.insert(String::from("ret"), Tracker::ret_inst);
        self.inst_map.insert(String::from("jmp"), Tracker::jump_inst);
        self.inst_map.insert(String::from("end_func"), Tracker::end_func_inst);
    }


    pub fn new_inst(&mut self, inst: &str) -> String {
        let tokens = tokenise_asm_inst(inst);
        if last_char(tokens.0[0]) != ':' {
            println!("{inst}\n{tokens:?}");
            return match self.inst_map.get(tokens.0[0]).unwrap()(self, tokens.0, tokens.1) {
                Some(new_inst) => {
                    println!("new inst: {}\n", new_inst);
                    new_inst
                }
                _ => {
                    println!("stay the same: {}\n", inst);
                    String::from(inst)
                }
            }
        }
        String::from(inst)
    }

    fn registers_mut(&mut self) -> &mut Registers {
        self.registers_map.get_mut(&self.current_register_zone).unwrap()
    }

    fn registers(&self) -> &Registers {
        self.registers_map.get(&self.current_register_zone).unwrap()
    }

    fn switch_register(&mut self, func_name: &str) {
        self.registers_map.insert(String::from("global"), self.registers_map.get(func_name).unwrap().clone());
    }

    fn memory_action(&mut self, tokens: Vec<&str>, garbage: &str) -> Option<String> {
        let opt_val = self.registers().extract_val(&tokens[2]);
        if opt_val.is_some() {
            let val = opt_val.unwrap();
            match tokens[0] as &str{
                "add" => {
                    if !self.registers_mut().add_val(tokens[1], val){
                        return Some(format!("add {}, {}", tokens[1], val))
                    }
                }
                _ => {
                    if !self.registers_mut().set_val(tokens[1], Some(val)) {
                        match str::parse::<i64>(tokens[2]) {
                            Ok(_) => return Some(format!("{} {}, {}", tokens[0], self.get_memory_access(tokens[1], garbage), val)),
                            _ => return Some(format!("mov {}, {}        ;out of the tracker\n{} {}, {}", tokens[2], val, tokens[0], self.get_memory_access(tokens[1], garbage), tokens[2]))
                        }
                    }
                }
            }
            return Some(String::new())
        }
        let res: Option<String>;
        match self.registers().get_val(tokens[1]) {
            Some(_) => res = Some(format!("{} {}, {}", tokens[0], tokens[1], self.get_memory_access(tokens[2], garbage))),
            _ => res = Some(format!("{} {}, {}", tokens[0], self.get_memory_access(tokens[1], garbage), self.get_memory_access(tokens[2], garbage)))
        }
        self.registers_mut().set_val(tokens[1], None);
        res
    }

    fn get_memory_access(&self, spot: &str, garbage: &str) -> String {
        if last_char(spot) == ']' {
            match self.registers().get_val(&spot[0..spot.len()-1]) {
                Some(val) => return format!("{} {}]", garbage, val),
                _ => return format!("{} {}", garbage, spot)
            }
        }
        match self.registers().get_val(spot) {
            Some(val) => return format!("{}", val),
            _ => return format!("{}", spot)
        }    
    }

    fn mul_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String> {
        let val_rax = self.registers().get_val("rax");
        if val_rax.is_some(){
            if self.registers_mut().mul_val(tokens[1], val_rax.unwrap()) {
                return Some(String::new())
            }else{
                return Some(format!("mov rax, {}        ;out of the tracker\nmul {}", val_rax.unwrap(), tokens[1]))
            }   
        }else{
            let val_token = self.registers().get_val(tokens[1]);
            if val_token.is_some(){
                return Some(format!("mov {}, {}     ;out of the tracker\nmul {}", tokens[1], val_token.unwrap(), tokens[1]))
            }else{
                return None
            }   
        }
    }

    fn cmp_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String> {
        let res: Option<String>;
        match self.registers().get_val(tokens[1]) {
            Some(val1) => {
                match self.registers().get_val(tokens[2]) {
                    Some(val2) => {
                        self.last_cmp = Some((val1, val2));
                        res = Some(String::new())
                    }
                    _ => res = Some(format!("cmp {}, {}", val1, tokens[2]))
                }
            }
            _ => {
                match self.registers().get_val(tokens[2]) {
                    Some(val2) => res = Some(format!("cmp {}, {}", tokens[1], val2)),
                    _ => res = None
                }
            }
        }
        if res.is_some() && res.clone().unwrap() != String::new() {
            self.last_cmp = None;
        }
        res
    }

    fn push_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String> {
        self.stack.push(self.registers().get_val(tokens[1]));
        return match self.registers().get_val(tokens[1]) {
            Some(_) => Some(String::new()),
            _ => None
        }
    }

    fn pop_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String> {
        let last_insert = self.stack.pop();
        if last_insert.is_some(){
            self.registers_mut().set_val(tokens[1], Some(last_insert.unwrap()));
            return Some(String::new());
        }
        self.registers_mut().set_val(tokens[1], None);
        None
    }

    fn xor_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String> {
        self.registers_mut().set_val(tokens[1], Some(0));
        Some(String::new())
    }


    fn cond_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String>{
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
            self.registers_mut().reset();
            return Some(format!("jmp {}", tokens[1]));
        }
        return Some(String::new())
    }
    

    fn _deref_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String>{
        match self.registers().get_val("rbx") {
            Some(val) => {
                self.registers_mut().set_val("rbx", None);
                return Some(format!("mov rbx, {}     ;out of the tracker\n_deref {}", val, tokens[1]))
            }
            _ => return None
        }
    }

    fn func_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String>{
        self.registers_map.insert(String::from(tokens[1]), Registers::new());
        self.current_register_zone = String::from(tokens[1]);
        self.registers_mut().set_val("r15", None);
        None
    }

    fn call_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String>{
        let mut res: Option<String> = None;
        match tokens[1] {
            "_operation" => {
                let r12 = self.registers().get_val("r12").unwrap() as u8;
                match self.registers().get_val("r11") {
                    Some(r11) => {
                        match self.registers().get_val("r10") {
                            Some(r10) => {
                                self.registers_mut().set_val("rax", Some(operation(r10, r11, r12)));  
                                res = Some(String::new())
                            }
                            _ => res = Some(format!("mov r11, {}\nmov r12, {}\ncall _operation", r11, r12))
                        }        
                    }
                    _ => {
                        match self.registers().get_val("r10") {
                            Some(r10) => res = Some(format!("mov r10, {}\nmov r12, {}\ncall _operation", r10, r12)),
                            _ => res = Some(format!("mov r12, {}\ncall _operation", r12))
                        }
                    }
                }
            }
            _ => self.switch_register(tokens[1])
        }
        self.registers_mut().set_val("r15", Some(0));
        if res.is_some() && res.clone().unwrap() != String::new() {
            self.registers_mut().set_val("rax", None);
        }
        return res
    }

    fn jump_inst(&mut self, _tokens: Vec<&str>, _garbage: &str) -> Option<String>{
        // if !tokens[1].starts_with("_real_end_condition_") {
        //     self.registers_mut().reset();
        // }
        None
    }

    fn ret_inst(&mut self, _tokens: Vec<&str>, _garbage: &str) -> Option<String>{
        let res = match self.registers().get_val("rax") {
            Some(val) => Some(format!("mov rax, {}      ;out of the tracker\nret", val)),
            _ => None
        };
        self.nb_ret += 1;
        res
    }

    fn end_func_inst(&mut self, _tokens: Vec<&str>, _garbage: &str) -> Option<String>{
        if self.nb_ret > 1 {
            self.registers_mut().reset();
        }
        self.current_register_zone = String::from("global");
        self.nb_ret = 0;
        None
    } 
}

fn tokenise_asm_inst(inst: &str) -> (Vec<&str>, &str) {
    let split_inst: Vec::<&str> = inst.split(" ").collect();
    match split_inst.len() {
        4 => {
            match last_char(inst) {
                ']' => {
                    return (vec!(split_inst[0], &split_inst[1][0..split_inst[1].len()-1], &split_inst[3][0..split_inst[3].len()]), split_inst[2])
                }
                _ => return (vec!(split_inst[0], &split_inst[2][0..split_inst[2].len()-1], split_inst[3]), split_inst[1]),
            }
        }
        3 => return (vec!(split_inst[0], &split_inst[1][0..split_inst[1].len()-1], split_inst[2]), ""),
        _ => return (split_inst, "")
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
        res.map.insert("rax", Some(0));
        res.map.insert("rbx", Some(0));
        res.map.insert("rdx", Some(0));
        res.map.insert("r15", Some(0));
        res.map.insert("r10", Some(0));
        res.map.insert("r11", Some(0));
        res.map.insert("r12", Some(0));
        res.convert.insert(String::from("rax"), "rax");
        res.convert.insert(String::from("eax"), "rax");
        res.convert.insert(String::from("ax"), "rax");
        res.convert.insert(String::from("rbx"), "rbx");
        res.convert.insert(String::from("ebx"), "rbx");
        res.convert.insert(String::from("bx"), "rbx");
        res.convert.insert(String::from("rdx"), "rdx");
        res.convert.insert(String::from("edx"), "rdx");
        res.convert.insert(String::from("dx"), "rdx");
        res.convert.insert(String::from("r15"), "r15");
        res.convert.insert(String::from("r15b"), "r15");
        res.convert.insert(String::from("r12"), "r12");
        res.convert.insert(String::from("r11"), "r11");
        res.convert.insert(String::from("r10"), "r10");
        res
    }

    pub fn clone(&self) -> Registers {
        Registers{
            map: self.map.clone(),
            convert: self.convert.clone()
        }
    }
    
    pub fn reset(&mut self) {
        self.map.insert("rax", None);
        self.map.insert("rbx", None);
        self.map.insert("rdx", None);
        self.map.insert("r15", None);
    }

    pub fn get_val(&self, register: &str) -> Option<i64> {
        if self.is_followed(register){
            return self.convert(register).clone()
        }
        return match str::parse::<i64>(register) {
            Ok(nb) => Some(nb),
            _ => None
        }
    }

    pub fn set_val(&mut self, register: &str, val: Option<i64>) -> bool {
        if self.is_followed(register){
            *self.convert_mut(register) = val;
            return true
        }
        return false
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
