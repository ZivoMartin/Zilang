use std::collections::HashMap;
use super::collections::Stack;
#[allow(dead_code)]
pub struct Tools{
    authorized_char_for_variable: &'static str,
    operators: Vec<&'static str>,
    operator_priority: HashMap<String, u8>,
    operator_ascii_val: HashMap<&'static str, i32>,
    reverse_ascii_map: HashMap::<i32, &'static str>
}

#[allow(dead_code)]
impl Tools{

    pub fn new() -> Tools{
        let mut res = Tools{
            authorized_char_for_variable: "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN1234567890-_",
            operators: vec!{"+", "-", "*", "/", "%", "||", "&&", "==", "!=", "<", ">", "<=", ">="},
            operator_priority: build_operator_priority(),
            operator_ascii_val: HashMap::<&'static str, i32>::new(),
            reverse_ascii_map: HashMap::<i32, &'static str>::new()
        };
        res.init_ascii_map();
        res
    }


    
    pub fn is_valid_name(&self, name: &str) -> bool{
        for letter in name.chars(){
            if !self.authorized_char_for_variable.contains(letter){
                return false;
            }
        }
        true
    }

    pub fn is_operator(&self, x: &str) -> bool{
        self.operators.contains(&x) && x != ""
    }

    pub fn convert_in_postfix_exp(&self, exp: Vec::<String>) -> Vec::<String>{
        let mut result = Vec::<String>::new();
        let mut stack = Stack::<String>::new();
        for e_elt in exp.iter(){
            let elt = String::from(e_elt);
            if self.is_operator(&elt){
                while !stack.is_empty() && stack.val() != "(" && self.operator_priority[stack.val()] >= self.operator_priority[&elt]{
                    result.push(stack.pop());
                }
                stack.push(elt);
            }else if elt == ")"{
                while stack.val() != "(" {
                    result.push(stack.pop());
                }
                stack.pop();
            }else if elt == "("{
                stack.push(elt);
            }else{
                result.push(elt);
            }
        }
        while stack.size() != 0 {
            result.push(stack.pop());
        }
        result
    }


    fn init_ascii_map(&mut self) {
        for op in self.operators.iter(){
            if op.len() == 1 {
                self.operator_ascii_val.insert(&op, op.chars().nth(0).unwrap() as i32);
                self.reverse_ascii_map.insert(op.chars().nth(0).unwrap() as i32, &op);
            }
        }
        self.operator_ascii_val.insert("==", '=' as i32);
        self.operator_ascii_val.insert("||", '|' as i32);
        self.operator_ascii_val.insert("&&", '&' as i32);
        self.operator_ascii_val.insert("!=", '=' as i32 +3);
        self.operator_ascii_val.insert("<=", '<' as i32 -1);
        self.operator_ascii_val.insert(">=", '>' as i32 +1);
        self.reverse_ascii_map.insert('=' as i32, "==");
        self.reverse_ascii_map.insert('|' as i32, "||");
        self.reverse_ascii_map.insert('&' as i32, "&&");
        self.reverse_ascii_map.insert('=' as i32 +3, "!=");
        self.reverse_ascii_map.insert('<' as i32 -1, "<=");
        self.reverse_ascii_map.insert('>' as i32 +1, ">=");
    }

    pub fn ascii_val(&self, op: &str) -> i32 {
        self.operator_ascii_val[op]
    }

    pub fn get_operator_string(&self, op: i32) -> &'static str {
        self.reverse_ascii_map[&op]
    }

    pub fn get_full_op(&self, c1: char, c2: char) -> String {
        let mut res = String::from(c1);
        match c1 {
            '<' | '>' | '=' | '!' => {
                if c2 == '=' {
                    res = format!("{}=", c1);
                }
            }
            '|' =>  {
                if c2 == '|' {
                    res = String::from("||");
                }
            }
            '&' =>  {
                if c2 == '&' {
                    res = String::from("&&");
                }
            }
            _ => ()
        }
        res
    }

    pub fn can_be_operator(&self, c: char) -> bool {
        "!=|&".contains(c) || self.is_operator(&String::from(c))
    }

}

#[allow(dead_code)]
pub fn split(string: &str, splitter: &str) -> Vec::<String>{
    string.split(splitter).map(String::from).collect()
}

fn build_operator_priority() -> HashMap<String, u8>{
    let mut res = HashMap::<String, u8>::new();
    res.insert(String::from("+"), 3);
    res.insert(String::from("-"), 3);
    res.insert(String::from("%"), 4);
    res.insert(String::from("*"), 4);
    res.insert(String::from("/"), 4);
    res.insert(String::from("("), 5);
    res.insert(String::from(")"), 4);
    res.insert(String::from("<"), 2);
    res.insert(String::from("<="), 2);
    res.insert(String::from(">"), 2);
    res.insert(String::from(">="), 2);
    res.insert(String::from("=="), 2);
    res.insert(String::from("!="), 2);
    res.insert(String::from("||"), 1);
    res.insert(String::from("&&"), 1);
    res
}

#[allow(dead_code)]
pub fn from_char_to_number(chara: &mut String) -> Option<i8> {
    if chara.len() <= 2 || !chara.starts_with('\'') || !chara.ends_with('\''){
        return None
    }
    let mut chara_iter = chara.chars();
    if chara.len() == 4 {
        match convert_in_one_char(chara_iter.nth(2).unwrap()){
            Some(ch) => *chara = format!("'{}'", ch),
            _ => return None
        }
    }
    match str::parse::<char>(&chara[1..2]) {
        Ok(nb) => Some(nb as i8),
        _ => None
    }
}

pub fn convert_in_one_char(ch: char) -> Option<char> {
    match ch {
        '0' => Some('\0'),
        'n' => Some('\n'),
        'r' => Some('\r'),
        _ => None
    }
}

#[allow(dead_code)]
pub fn extract_end_char(s: &mut String, chara: char) -> u32 {
    let mut result: u32 = 0;
    while s.len() > 0 && s.ends_with(chara) {
        s.pop();
        result += 1;
    }
    result    
}

#[allow(dead_code)]
pub fn last_char(s: &str) -> char {
    s.chars().last().unwrap()
}

#[allow(dead_code)]
pub fn operation(val1: i64, val2: i64, operator: u8) -> i64 {
    match operator as char {
        '@' => (val1 != val2) as i64,
        ';' => (val1 <= val2) as i64,
        '?' => (val1 >= val2) as i64,
        '&' => (val1 != 0 && val2 != 0) as i64,
        '|' => (val1 != 0 || val2 != 0) as i64,
        '<' => (val1 < val2) as i64,
        '>' => (val1 > val2) as i64,
        '=' => (val1 == val2) as i64,
        '+' => val1 + val2,
        '-'=> val1 - val2,
        '*'=> val1 * val2,
        '/'=> val1 / val2,
        '%'=> val1 % val2,
        _ => panic!("You forgot this operator: {}", operator)
    }
}

