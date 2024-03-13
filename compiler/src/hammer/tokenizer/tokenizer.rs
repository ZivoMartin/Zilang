use std::collections::HashMap;
use std::collections::VecDeque;
use crate::hammer::Hammer;
use super::include::*;
use super::grammar_tree::build_grammar_tree;
use std::iter::Peekable;
use std::fs::File;
use std::str::Chars;
use std::io::prelude::*;

pub struct Tokenizer {
    hammer: *mut Hammer,
    group_map: HashMap<TokenType, Node>,
    priority_map: HashMap<TokenType, u8>,
    identity_map: HashMap<fn(char)->bool, Vec<TokenType>>
}

unsafe impl Send for Tokenizer{}

// struct Chars {
//     chars: Cursor<BufReader<File>>,
// }



// impl Chars {

//     fn new(f: File) -> Chars {
//         Chars{
//             chars: Cursor::new(BufReader::new(f))
//         }
//     }
    
//     fn peek(&mut self) -> Option<char> {
//         match self.chars.consume() {
//             Some(r) => Some(*r.as_ref().unwrap_or_else(|e| panic!("{e}")) as char),
//             _ => None
//         }
//     }

//     fn next(&mut self) -> Option<char> {
//         match self.chars.next() {
//             Some(r) => Some(r.unwrap_or_else(|e| panic!("{e}")) as char),
//             _ => None
//         }
//     }
    
// }


fn build_priority_map() -> HashMap<TokenType, u8> {
    let mut priority_map = HashMap::<TokenType, u8>::new();
    priority_map.insert(TokenType::Ident, 1);
    priority_map.insert(TokenType::Number, 1);
    priority_map.insert(TokenType::Symbol, 2);
    priority_map.insert(TokenType::Operator, 1);
    priority_map.insert(TokenType::Type, 2);
    priority_map.insert(TokenType::Keyword, 3);
    priority_map
}


fn build_identity_map() -> HashMap<fn(char)->bool, Vec<TokenType>> {
    let mut res = HashMap::<fn(char)->bool, Vec<TokenType>>::new();
    res.insert(is_number, vec!(TokenType::Number));
    res.insert(is_letter, vec!(TokenType::Ident, TokenType::Type, TokenType::Keyword));
    res.insert(is_sign, vec!(TokenType::Symbol, TokenType::Operator));
    res.insert(is_operator, vec!(TokenType::Operator, TokenType::Symbol));
    res
}


impl<'a> Tokenizer {

    pub fn new(hammer: &'a mut Hammer) -> Tokenizer {
        Tokenizer{
            group_map: build_grammar_tree(),
            priority_map: build_priority_map(),
            identity_map: build_identity_map(),
            hammer
        }
    }

    pub fn tokenize(&mut self, mut input: File) -> Result<(), &'static str> {
        let first_node = self.group_map.get(&TokenType::Program).unwrap();
        let mut s = String::new();
        input.read_to_string(&mut s).unwrap();
        let mut chars = s.chars().peekable();
        while chars.peek().is_some() {  
            match self.curse(first_node, &mut chars) {
                Ok(()) => (),
                Err(_) => return Err("Failed to tokenise")
            }
            self.skip_garbage(&mut chars); 
        }   
        unsafe{
            (**&self.hammer).end_of_tokenizing_thread();
        }
        Ok(())
    } 
    
    fn curse(&self, current_node: &Node, chars: &mut Peekable<Chars>) -> Result<(), i8> {
        if !current_node.is_leaf() {
            loop {
                let mut retry = false;
                if !current_node.consider_garbage {
                    self.skip_garbage(chars); 
                }
                if chars.peek().is_some() {
                    let mut paths_vec = self.get_son_array(current_node);
                    let save = chars.clone();
                    match self.get_next_token(&mut paths_vec, chars) {
                        Ok(token_string) => {
                            match self.filter_nodes(&mut paths_vec, &token_string) {
                                Some(path) => {
                                    path.proke_travel_functions(self, &token_string);
                                    for node in path.path.iter() {
                                        match self.curse(node, chars) {
                                            Ok(()) => {
                                                if node.travel_react == Some(Tokenizer::push_group) {
                                                    self.end_group(node.type_token, &token_string)
                                                }
                                            },
                                            Err(depth) => {
                                                if current_node.retry != depth {
                                                    return Err(depth + 1)
                                                } 
                                                retry = true;
                                                break;
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    if !current_node.can_end {
                                        return Err(0)
                                    }
                                    *chars = save;
                                }
                            }
                        },
                        Err(_) => {
                            if !current_node.can_end {
                                return Err(0)
                            }
                            *chars = save;
                        }
                    }
                }else if !current_node.can_end {
                    return Err(0);
                }
                if !retry {
                    break;
                }
            }
        }
        Ok(())
    }

    fn get_next_token(&self, path_vec: &mut VecDeque<Path>, chars: &mut Peekable<Chars>) -> Result<String, String> {
        //println!("{:?}\n\n", path_vec);
        let c = chars.peek().unwrap();
        if self.detect_char_token(path_vec, &c.to_string()) {
            return Ok(chars.next().unwrap().to_string()) 
        }
        let mut current_token = String::new();
        for (cond_stop, author_type) in self.identity_map.iter() {
            if cond_stop(*c) {
                if self.clean_son_vec(path_vec, author_type) {
                    self.next_char_while(&mut current_token, chars, *cond_stop);
                    if *cond_stop == is_letter as fn(char)->bool && is_number(*chars.peek().unwrap()) && self.clean_son_vec(path_vec, &vec!(TokenType::Ident)) {  // If we are looking for an ident
                        self.next_char_while(&mut current_token, chars, |c: char| {is_letter(c) || is_number(c)});
                    }
                    return Ok(current_token)
                }else{
                    return Err(format!("FAILED TO TOKENIZE"))
                }
            }
        }
        Ok(current_token)
    }

    fn detect_char_token(&self, path_vec: &mut VecDeque<Path>, c: &str) -> bool {
        let mut i = 0;
        while i < path_vec.len() {
            if path_vec[i].p_node().type_token == TokenType::Symbol && path_vec[i].p_node().constraint_satisfied(c){
                while path_vec.len() - 1 > i {
                    path_vec.pop_back();
                }
                while path_vec.len() != 1 {
                    path_vec.pop_front();
                }
                return true 
            }  
            i += 1;
        }
        false
    }

    fn clean_son_vec(&self, path_vec: &mut VecDeque<Path>, author_type: &Vec<TokenType>) -> bool {
        let mut i = 0;
        while i < path_vec.len() {
            if !author_type.contains(&path_vec[i].p_node().type_token) {
                path_vec.remove(i);
            }else{
                i += 1;
            }
        }
        !path_vec.is_empty()
    }

    fn next_char_while(&self, current_token: &mut String, chars: &mut Peekable<Chars>, continue_cond: fn(char)->bool) {
        current_token.push(chars.next().unwrap());
        if continue_cond != is_sign as fn(char) -> bool {
            while let Some(c) = chars.peek() {
                if continue_cond(*c) {    
                    current_token.push(chars.next().unwrap());
                }else{
                    break;
                }
            }   
        }
    }

    fn get_son_array(&'a self, node: &'a Node) -> VecDeque<Path> {
        let mut res = VecDeque::<Path>::new();
        for son in node.sons.iter() {
            res.push_back(Path::init(son));
        }
        for group in node.groups.iter() {
            let mut paths = self.get_son_array(self.group_map.get(&group.type_token).unwrap());
            if group.travel_react.is_some() || !group.is_leaf() {
                //println!("{:?}", group.type_token);
                for p in paths.iter_mut() {
                    p.path.push(group);
                }
            }
            res.append(&mut paths);
        }
        res
    }

    fn filter_nodes(&'a self, paths: &'a mut VecDeque::<Path>, token: &str) -> Option<&Path>{
        if token.is_empty() {
            return None
        }
        let mut i = 0;
        let mut res: Option<&Path> = None;
        while i < paths.len() {
            let node = paths[i].p_node();
            if node.constraint_satisfied(token) && (!res.is_some() || 
                self.priority_map.get(&res.unwrap().p_node().type_token) < self.priority_map.get(&node.type_token)){
                        res = Some(&paths[i])
            }
            i += 1;
        }
        res
    }

    fn skip_garbage(&self, chars: &mut Peekable<Chars>) {
        while let Some(c) = chars.peek() {
            if !DEFAULT_GARBAGE_CHARACTER.contains(c) {    
                break;
            }
            chars.next();
        }
    }

    pub fn push_token(&self, token_type: TokenType, content: &String) {
        unsafe{(**&self.hammer).new_token(Token::new(token_type, content.clone()));}
    }

    pub fn push_group(&self, token_type: TokenType, _content: &String) {
        unsafe{(**&self.hammer).new_group(token_type);}
    }

    pub fn end_group(&self, _token_type: TokenType, _content: &String) {
        unsafe{(**&self.hammer).end_group();}
    }

    pub fn push_once(&self, token_type: TokenType, content: &String) {
        self.push_group(token_type, content)
    }
}

fn is_sign(c: char) -> bool {
    !is_number(c) && !is_letter(c) && !DEFAULT_GARBAGE_CHARACTER.contains(&c) && !OPERATOR_COMPONENT.contains(&c)
}

fn is_number(c: char) -> bool {
    (c as u8) < 58 && (c as u8) >= 48
}

fn is_letter(c: char) -> bool {
    (c as u8) >= 65 && (c as u8) <= 122 && !((c as u8) >= 91 && (c as u8) <= 96) || c == '_'
}

fn is_operator(c: char) -> bool {
    OPERATOR_COMPONENT.contains(&c)
}

