use crate::tools::stack::Stack;
use std::collections::HashMap;
use crate::tools::tools::{last_char, operation};
pub struct Tracker{
    registers_map: HashMap<String, RegistersData>,
    current_register_zone: String,
    stack: Stack<Option<i64>>,
    inst_map: HashMap::<String, fn (&mut Tracker, Vec<&str>, &str)->Option<String>>,
    last_cmp: Option<(i64, i64)>,
    nb_ret: u32
}

impl Tracker {

    pub fn new() -> Tracker {
        let mut res = Tracker{
            registers_map: HashMap::<String, RegistersData>::new(),
            current_register_zone: String::from("global"),
            stack: Stack::new(),
            inst_map: HashMap::<String, fn (&mut Tracker, Vec<&str>, &str)->Option<String>>::new(),
            last_cmp: None,
            nb_ret: 0
        };
        res.registers_map.insert(String::from("global"), RegistersData::new());
        res.init_inst_map();
        res
    }   

    fn init_inst_map(&mut self) {
        self.inst_map.insert(String::from("pop"), Tracker::pop_inst);
        self.inst_map.insert(String::from("push"), Tracker::push_inst);
        self.inst_map.insert(String::from("xor"), Tracker::xor_inst);
        self.inst_map.insert(String::from("mov"), Tracker::memory_action);
        self.inst_map.insert(String::from("movzx"), Tracker::memory_action);
        self.inst_map.insert(String::from("mul"), Tracker::mul_inst);
        self.inst_map.insert(String::from("cmp"), Tracker::cmp_inst);
        self.inst_map.insert(String::from("jg"), Tracker::cond_inst);
        self.inst_map.insert(String::from("je"), Tracker::cond_inst);
        self.inst_map.insert(String::from("call"), Tracker::call_inst);
        self.inst_map.insert(String::from("func"), Tracker::func_inst);
        self.inst_map.insert(String::from("ret"), Tracker::ret_inst);
        self.inst_map.insert(String::from("jmp"), Tracker::jump_inst);
        self.inst_map.insert(String::from("end_func"), Tracker::end_func_inst);
        self.inst_map.insert(String::from("_deref"), Tracker::deref);
        self.inst_map.insert(String::from("macro_call"), Tracker::macro_call_inst);
    }


    pub fn new_inst(&mut self, inst: &str) -> String {
        let tokens = tokenise_asm_inst(inst);
        let mut res = String::from(inst);
        if last_char(tokens.0[0]) != ':' {
            let opt_inst = self.inst_map.get(tokens.0[0]).unwrap()(self, tokens.0, tokens.1); 
            if opt_inst.is_some() {
                res = opt_inst.unwrap();
            }
        }
        res = self.registers().txt.clone() + &res;
        self.registers_mut().txt = String::new();
        res
    }

    fn registers_mut(&mut self) -> &mut RegistersData {
        self.registers_map.get_mut(&self.current_register_zone).unwrap()
    }

    fn registers(&self) -> &RegistersData {
        self.registers_map.get(&self.current_register_zone).unwrap()
    }

    fn switch_register(&mut self, func_name: &str) {
        self.registers_map.insert(String::from("global"), self.registers_map.get(func_name).unwrap().clone());
    }

    fn memory_action(&mut self, tokens: Vec<&str>, garbage: &str) -> Option<String> {
        let token2_val = self.registers().extract_val(&tokens[2]);
        if token2_val.is_some() {
            let val = token2_val.unwrap();
            match tokens[0] as &str{
                "add" => {
                    if !self.registers_mut().add_val(tokens[1], val){
                        return Some(format!("add {}, {}", tokens[1], val))
                    }
                }
                _ => {
                    if !self.set_val(tokens[1], Some(val)) {
                        match str::parse::<i64>(tokens[2]) {
                            Ok(_) => return Some(format!("{} {}, {}", tokens[0], self.get_memory_access(tokens[1], garbage), val)),
                            _ => return Some(format!("{}\n{} {}, {}", self.put_in_asm(tokens[2]), tokens[0], self.get_memory_access(tokens[1], garbage), tokens[2]))
                        }
                    }
                }
            }
            return Some(String::new())
        }
        let res: String;
        match self.get_val(tokens[1]) {
            Some(_) => {
                if garbage == "" {
                    if tokens[0] != "add" {
                            self.set_val(tokens[1], Some(0));
                    }
                    self.set_static_reg(tokens[1], tokens[2]);
                    res = String::new()
                }else{
                    match tokens[0] {
                        "add" => res = format!("{}\n{} {}, {}", self.put_in_asm(tokens[1]), tokens[0], tokens[1], self.get_memory_access(tokens[2], garbage)),
                        _ => res = format!("{} {}, {}", tokens[0], tokens[1], self.get_memory_access(tokens[2], garbage))
                    }
                    self.set_val(tokens[1], None);
                }
            },
            _ => res = format!("{} {}, {}", tokens[0], self.get_memory_access(tokens[1], garbage), self.get_memory_access(tokens[2], garbage))
        }
        Some(res)
    }

    fn get_memory_access(&self, spot: &str, garbage: &str) -> String {
        if last_char(spot) == ']' {
            match self.get_val(&spot[0..spot.len()-1]) {
                Some(val) => return format!("{} {}]", self.garbage_analyse(garbage), val),
                _ => return format!("{} {}", self.garbage_analyse(garbage), spot)
            }
        }
        match self.get_val(spot) {
            Some(val) => return format!("{}", val),
            _ => return format!("{}", spot)
        }    
    }

    fn garbage_analyse(&self, garbage: &str) -> String {
        let mut res: Vec<String> = garbage.split("+").map(String::from).collect();
        if res[1] == "r15" {
            let r15_val = self.get_val("r15");
            if r15_val.is_some() {
                res[1] = format!("{}", r15_val.unwrap());
            } 
        }
        res.join("+")
    }

    fn mul_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String> {
        let val_rax = self.get_val("rax");
        if val_rax.is_some(){
            if self.registers_mut().mul_val(tokens[1], val_rax.unwrap()) {
                return Some(String::new())
            }else{
                return Some(format!("{}\nmul {}", self.put_in_asm("rax"), tokens[1]))
            }   
        }else{
            let val_token = self.get_val(tokens[1]);
            if val_token.is_some(){
                return Some(format!("{}\nmul {}", self.put_in_asm(tokens[1]), tokens[1]))
            }else{
                return None
            }   
        }
    }

    fn cmp_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String> {
        let res: Option<String>;
        match self.get_val(tokens[1]) {
            Some(val1) => {
                match self.get_val(tokens[2]) {
                    Some(val2) => {
                        self.last_cmp = Some((val1, val2));
                        res = Some(String::new())
                    }
                    _ => res = Some(format!("cmp {}, {}", val1, tokens[2]))
                }
            }
            _ => {
                match self.get_val(tokens[2]) {
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
        self.stack.push(self.get_val(tokens[1]));
        return match self.get_val(tokens[1]) {
            Some(_) => Some(String::new()),
            _ => None
        }
    }

    fn pop_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String> {
        let last_insert = self.stack.pop();
        if last_insert.is_some(){
            self.set_val(tokens[1], Some(last_insert.unwrap()));
            return Some(String::new());
        }
        self.set_val(tokens[1], None);
        None
    }

    fn xor_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String> {
        self.set_val(tokens[1], Some(0));
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


    fn func_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String>{
        self.registers_map.insert(String::from(tokens[1]), RegistersData::new());
        self.current_register_zone = String::from(tokens[1]);
        self.set_val("r15", None);
        None
    }

    fn call_inst(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String>{
        let mut res: String = String::new();
        let r15_val = self.get_val("r15");
        match tokens[1] {
            "_operation" => {
                let r12 = self.get_val("r12").unwrap() as u8;
                match self.get_val("r11") {
                    Some(r11) => {
                        match self.get_val("r10") {
                            Some(r10) => {self.set_val("rax", Some(operation(r10, r11, r12)));}
                            _ => res.push_str(&format!("{}\n{}\n", self.put_in_asm("r11"), self.put_in_asm("r12")))
                        }        
                    }
                    _ => {
                        match self.get_val("r10") {
                            Some(_) => res.push_str(&format!("{}\n{}\n", self.put_in_asm("r11"), self.put_in_asm("r12"))),
                            _ => res.push_str(&format!("{}\n", self.put_in_asm("r12")))
                        }
                    }
                }
            }
            _ => self.switch_register(tokens[1])
        }
        if res != String::new() {
            self.set_val("rax", None);
        }else if tokens[1] != "_operation"{
            match r15_val {
                Some(val) => {
                    self.set_val("r15", Some(0));
                    res.push_str(&format!("mov r15, {}\n", val));
                }
                _ => ()
            }
            
        }
        res.push_str(&format!("call {}", tokens[1]));
        return Some(res)
    }

    fn jump_inst(&mut self, _tokens: Vec<&str>, _garbage: &str) -> Option<String>{
        None
    }

    fn ret_inst(&mut self, _tokens: Vec<&str>, _garbage: &str) -> Option<String>{
        let res = match self.get_val("rax") {
            Some(_) => Some(format!("{}\nret", self.put_in_asm("rbx"))),
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

    fn deref(&mut self, tokens: Vec<&str>, _garbage: &str) -> Option<String>{
        match self.get_val("rbx") {
            Some(_) => {
                self.set_val("rbx", None);
                return Some(format!("{}\n_deref {}", self.put_in_asm("rbx"), tokens[1]))
            }
            _ => return None
        }
    }

    fn macro_call_inst(&mut self, tokens: Vec<&str>, garbage: &str) -> Option<String>{
        match tokens[1] {
                "_deref" => self.deref(tokens, garbage),
                _ => {
                    let mut i = 2;
                    let mut res = vec!(String::from(tokens[1]));
                    while i<tokens.len() - 1 {
                        res.push(self.get_memory_access(tokens[i+1], tokens[i]));
                        i += 2;   
                    }
                    res[i-3].push_str("\n");
                    Some(res.join(" "))
                }
            }
    } 

    fn put_in_asm(&self, reg: &str) -> String {
        self.registers().put_in_asm(reg)
    }

    fn get_val(&self, reg: &str) -> Option<i64> {
        self.registers().get_val(reg)
    }

    fn set_val(&mut self, reg: &str, val: Option<i64>) -> bool{
        self.registers_mut().set_val(reg, val)
    }

    fn set_static_reg(&mut self, reg1: &str, reg2: &str) {
        self.registers_mut().set_static_reg(reg1, reg2)
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


struct Register{
    val: Option<i64>,
    static_register: Option<String>,
}

impl Register{

    fn new() -> Register {
        Register{
            val: Some(0),
            static_register: None,
        }
    }

    fn lost_static_register_information(&mut self) {
        self.static_register = None
    }

    fn is_good_reg(&self, reg: &str) -> bool {
        self.static_register.is_some() && self.static_register.clone().unwrap() == reg
    }

    fn set_static_reg(&mut self, reg: &str) {
        self.static_register = Some(String::from(reg));
    }

}

impl Clone for Register {

    fn clone(&self) -> Register {
        Register {
            val: self.val.clone(),
            static_register: self.static_register.clone(),
        }
    }

}

struct RegistersData {
    map: HashMap<&'static str, Register>,
    convert: HashMap<String, &'static str>,
    txt: String
}

impl RegistersData {
    fn new() -> RegistersData{
        let mut res = RegistersData{
            map: HashMap::<&'static str, Register>::new(),
            convert: HashMap::<String, &'static str>::new(),
            txt: String::new()
        };
        res.map.insert("rax", Register::new());
        res.map.insert("rbx", Register::new());
        res.map.insert("rcx", Register::new());
        res.map.insert("rdx", Register::new());
        res.map.insert("r15", Register::new());
        res.map.insert("r10", Register::new());
        res.map.insert("r11", Register::new());
        res.map.insert("r12", Register::new());
        res.convert.insert(String::from("rax"), "rax");
        res.convert.insert(String::from("eax"), "rax");
        res.convert.insert(String::from("ax"), "rax");
        res.convert.insert(String::from("rbx"), "rbx");
        res.convert.insert(String::from("ebx"), "rbx");
        res.convert.insert(String::from("bx"), "rbx");
        res.convert.insert(String::from("rcx"), "rcx");
        res.convert.insert(String::from("ecx"), "rcx");
        res.convert.insert(String::from("cx"), "rcx");
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

    fn clone(&self) -> RegistersData {
        RegistersData{
            map: self.map.clone(),
            convert: self.convert.clone(),
            txt: self.txt.clone()
        }
    }
    
    fn reset(&mut self) {
        self.set_val("rax", None);
        self.set_val("rbx", None);
        self.set_val("rdx", None);
        self.set_val("r15", None);
    }

    fn actualise_registers(&mut self, reg: &str) -> String {
        let mut res = String::new();
        for (key, val) in &mut self.map {
            if key != &reg && val.is_good_reg(reg){
                val.lost_static_register_information();
                res.push_str(&format!("mov, {}, {}", key, reg))
            }
        }
        res
    }


    fn get_val(&self, register: &str) -> Option<i64> {
        if self.is_followed(register){
            return self.convert(register).val.clone()
        }
        return match str::parse::<i64>(register) {
            Ok(nb) => Some(nb),
            _ => None
        }
    }

    fn get_static_reg(&self, register: &str) -> Option<String> {
        if self.is_followed(register){
            return self.convert(register).static_register.clone()
        }
        return None
    }

    fn set_val(&mut self, register: &str, val: Option<i64>) -> bool {
        if self.is_followed(register){
            self.convert_mut(register).val = val;
            self.actualise_registers(register);
            return true
        }
        return false
    }

    fn set_static_reg(&mut self, reg1: &str, reg2: &str) {
        if self.is_followed(reg1) && self.is_followed(reg2) {
            self.map.get_mut(reg1).unwrap().set_static_reg(reg2);
        }else{
            if !self.is_followed(reg1) {
                panic!("The register {reg1} isn't follow.")
            }else{
                panic!("The register {reg2} isn't follow.")
            }
        }
        
    }

    fn add_val(&mut self, register: &str, val: i64) -> bool{
        if self.is_followed(register){
            let previous_val = self.convert(register).val;
            if previous_val.is_some(){
                self.set_val(register, Some(val+previous_val.unwrap()));
                return true
            }
        }
        return false
    }

    fn put_in_asm(&self, register: &str) -> String {
        let mut res = String::new();
        let reg_val = self.get_val(register); 
        if reg_val.is_some() {
            res.push_str(&format!("mov {}, {}   ;out of the tracker", register, reg_val.unwrap())) 
        }
        let static_reg = self.get_static_reg(register);
        if static_reg.is_some() {
            res.push_str(&format!("\nadd {}, {}     ;out of the tracker", self.convert.get(register).unwrap(), static_reg.unwrap()));
        }
        res
    }
    

    pub fn mul_val(&mut self, register: &str, val_rax: i64) -> bool{
        if self.is_followed(register){
            let reg_val = self.convert(register).val;
            if reg_val.is_some(){
                self.set_val("rax", Some(val_rax*reg_val.unwrap()));
                return true
            }
        }
        return false
    }

    fn convert(&self, register: &str) -> &Register {
        self.map.get(self.convert.get(register).unwrap()).unwrap()
    }

    fn convert_mut(&mut self, register: &str) -> &mut Register {
        self.map.get_mut(self.convert.get(register).unwrap()).unwrap()
    }

    fn is_followed(&self, register: &str) -> bool {
        self.convert.contains_key(register)
    }

    fn extract_val(&self, elt: &str) -> Option<i64> {
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
