use crate::tools::collections::Stack;
use std::collections::HashMap;

pub struct ExpTools {
    pub op_stack: Stack<String>,
    pub pf_exp: Vec<String>,
    operator_priority: HashMap<String, u8>
}


impl ExpTools {

    pub fn new() -> ExpTools {
        ExpTools{
            op_stack : Stack::new(),
            pf_exp : Vec::new(),
            operator_priority: build_prio_map()
        }
    }

    pub fn new_operator(&mut self, content: String) {
        while !self.op_stack.is_empty() && self.op_stack.val() != "(" && self.get_priority(self.op_stack.val()) >= self.get_priority(&content){
            self.pf_exp.push(self.op_stack.pop());
        }
        self.op_stack.push(content);
    }

    pub fn new_number(&mut self, number: String) {
        self.pf_exp.push(number);
    }

    pub fn new_parenthesis(&mut self, par: String) {
        match &par as &str {
            "(" => self.op_stack.push(par),
            ")" => {
                while self.op_stack.val() != "(" {
                    self.pf_exp.push(self.op_stack.pop());
                }
                self.op_stack.pop();
            }
            _ => panic!("Unknow parenthesis: {par}")
        } 
    }

    pub fn end(&mut self) {
        while self.op_stack.size() != 0 {
            self.pf_exp.push(self.op_stack.pop());
        }
        println!("This is ou final expression: {}", self.pf_exp.join(" "));
        self.pf_exp.clear();
    }

    fn get_priority(&self, op: &String) -> u8 {
        *self.operator_priority.get(op).unwrap_or_else(
            || panic!("This operator doesn't have priority yet: {op}")
        )
    }

}

fn build_prio_map() -> HashMap<String, u8>{
    let mut res = HashMap::<String, u8>::new();
    for op in vec!["%", "*", "/"].iter() {
        res.insert(String::from(*op), 4);
    }
    for op in vec!("<", "<=", ">", ">=", "==", "!=", "||", "&&").iter() {
        res.insert(String::from(*op), 2);
    }
    res.insert(String::from("+"), 3);
    res.insert(String::from("-"), 3);
    res.insert(String::from(")"), 4);
    res.insert(String::from("("), 5);
    res
}