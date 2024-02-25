use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
enum TokenType {
    // Primitive Token
    Ident,
    Number,
    Type,
    Symbol,
    // Keyword,

    // Token group
    Program,
    Instruction,
    Value,              
    ComplexIdent,       
    Expression,    
    Brackets,     
    Tuple,              // (Expression, Expression, ... , Expression)
    SerieExpression,    
    Affectation        // = Expression
}


impl Copy for TokenType {}

impl Clone for TokenType {
    fn clone(&self) -> TokenType {
        *self
    }
}


#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    string: String
}

impl Token {
    fn new(token_type: TokenType, string: String) -> Token {
        Token{token_type, string}
    }
}

struct Path<'a> {
    path: Vec<&'a Node>,
}

impl<'a> Path<'a> {
    fn init(node: &'a Node) -> Path {
        Path{path: vec!(node)}
    }

    fn p_node(&self) -> &'a Node {
        self.path[0]
    } 
}

struct Node {
    type_token: TokenType,
    groups: Vec<Node>, 
    sons: Vec<Node>,
    can_end: bool
}


impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        other.type_token == self.type_token
    }
}

impl Node {

    /// Build a new node wich has to be builded.
    fn new(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>) -> Node {
        Node{type_token, groups, sons, can_end: true}
    }

    /// Build a leaf, a leaf has to be builded
    fn leaf(type_token: TokenType) -> Node {
        Node{type_token, sons: vec!(), groups: vec!(), can_end: false}
    }

    /// Build a new node wich can end the building of the group.
    fn new_end(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>) -> Node {
        Node{type_token, groups, sons, can_end: true}
    }

    fn end_inst() -> Node {
        Node::leaf(TokenType::Symbol)
    }
}

pub struct Tokenizer {
    group_map: HashMap<TokenType, Node>,
    priority_map: HashMap<TokenType, u8>
}

fn build_priority_map() -> HashMap<TokenType, u8> {
    let mut priority_map = HashMap::<TokenType, u8>::new();
    priority_map.insert(TokenType::Ident, 1);
    priority_map.insert(TokenType::Number, 1);
    priority_map.insert(TokenType::Type, 2);
    //priority_map.insert(TokenType::Keyword, 3);
    priority_map
}

impl<'a> Tokenizer {

    pub fn new() -> Tokenizer {
        let mut res = Tokenizer{
            group_map: HashMap::<TokenType, Node>::new(),
            priority_map: build_priority_map()
        };
        res.init_token_groups();
        res
    }

    pub fn tokenize(&mut self, input: String) -> Result<Vec<Token>, String> {
        let mut result = Vec::<Token>::new();
        let first_node = self.group_map.get(&TokenType::Program).unwrap();
        let mut chars = input.chars();
        result = self.curse(first_node, result, &mut chars)?;
        for t in result.iter() {
            println!("type: {:?}, ", t.token_type);
            println!("string: {}", t.string)
        }
        Ok(result)
    } 

    fn curse(&self, current_node: &Node, mut res: Vec<Token>, chars: &mut std::str::Chars) -> Result<Vec<Token>, String> {
        let mut paths_vec = self.get_son_array(current_node);
        match self.get_next_token(&mut paths_vec, chars) {
            Ok(token) => {
                let mut path_prio = &paths_vec[0];
                for p in paths_vec.iter() {
                    if self.priority_map.get(&p.p_node().type_token) > self.priority_map.get(&path_prio.p_node().type_token){
                        path_prio = p;
                    };
                };
                res.push(Token::new(path_prio.p_node().type_token, token));
                for node in path_prio.path.iter() {
                    res = self.curse(node, res, chars)?;
                }
            },
            Err(e) => {
                if !current_node.can_end {
                    return Err(e)
                }
            }
        }
        Ok(res)
    }

    fn get_next_token(&self, path_vec: &mut Vec<Path>, chars: &mut std::str::Chars) -> Result<String, String> {
        let mut current_token = String::new();
        let c = chars.nth(0).unwrap();
        if is_number(c) {
            if self.clean_son_vec(path_vec, vec!(TokenType::Number)) {  
                self.next_char_while(&mut current_token, chars, is_number)
            }else{
                return Err(format!("You can't put a number here."))
            }
        }else if is_letter(c) {
            if self.clean_son_vec(path_vec, vec!(TokenType::Type, TokenType::Ident)) {
                self.next_char_while(&mut current_token, chars, is_letter);
                if is_number(chars.nth(0).unwrap()) && self.clean_son_vec(path_vec, vec!(TokenType::Ident)) {
                    self.next_char_while(&mut current_token, chars, |c: char| {is_letter(c) || is_number(c)});
                }
            }else{
                return Err(format!("You can't put a letter here"))
            }
        }else{
            if self.clean_son_vec(path_vec, vec!(TokenType::Symbol)) {
                self.next_char_while(&mut current_token, chars, is_sign);
            }else{
                return Err(format!("You can't put a symbol here."))
            }
        }
        Ok(current_token)
    }

    fn clean_son_vec(&self, path_vec: &mut Vec<Path>, author_type: Vec<TokenType>) -> bool {
        let mut i = 0;
        while i < path_vec.len() {
            if !author_type.contains(&path_vec[0].p_node().type_token) {
                path_vec.remove(i);
            }else{
                i += 1;
            }
        }
        !path_vec.is_empty()
    }

    fn next_char_while(&self, current_token: &mut String, chars: &mut std::str::Chars, continue_cond: fn(char)->bool) {
        while let Some(c) = chars.nth(0) {
            if continue_cond(c) {    
                current_token.push(c);
                chars.next();
            }
        }   
    }

    fn get_son_array(&'a self, node: &'a Node) -> Vec<Path> {
        let mut res = Vec::<Path>::new();
        for son in node.sons.iter() {
            res.push(Path::init(son));
        }
        for group in node.groups.iter() {
            let mut paths = self.get_son_array(self.group_map.get(&group.type_token).unwrap());
            for p in paths.iter_mut() {
                p.path.push(group);
            }
            res.append(&mut paths);
        }
        res
    }

    fn init_token_groups(&mut self) {
        self.group_map.insert(
            TokenType::Tuple,
            Node::new(
                TokenType::Tuple,
                vec!(),
                vec!(
                    Node::new(
                        TokenType::Symbol, // ( 
                        vec!(Node::new(
                            TokenType::SerieExpression,
                                vec!(),
                                vec!(
                                    Node::leaf(TokenType::Symbol) // )
                                )
                            )
                        ), 
                        vec!()
                    )
                )
            )
        );

        
        self.group_map.insert(
            TokenType::SerieExpression,
            Node::new_end(
                TokenType::SerieExpression,
                vec!(
                    Node::new(
                        TokenType::Expression,
                        vec!(),
                        vec!(
                            Node::leaf(
                                TokenType::Symbol, // ,
                            )
                        )
                    )
                ),
                vec!()
            )
        );

        self.group_map.insert(
            TokenType::ComplexIdent,
            Node::new(
                TokenType::ComplexIdent,
                vec!(),
                vec!(
                    Node::new_end(
                        TokenType::Ident,
                        vec!(
                            Node::leaf(
                                TokenType::Brackets,
                            ),
                            Node::leaf(
                                TokenType::Tuple,
                            )
                        ),
                        vec!()
                    ),
                    Node::new(
                        TokenType::Symbol, // $
                        vec!(
                            Node::leaf(
                                TokenType::ComplexIdent
                            )
                        ),
                        vec!()
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::Brackets,
            Node::new(
                TokenType::Brackets,
                vec!(),
                vec!(
                    Node::new_end(
                       TokenType::Symbol, // [
                       vec!(
                           Node::new(
                                TokenType::Expression,
                                vec!(),
                                vec!(
                                    Node::new(
                                        TokenType::Symbol, // ]
                                        vec!(
                                            Node::leaf(
                                                TokenType::Brackets
                                            )
                                        ),
                                        vec!()
                                    )
                                )
                           )
                       ),
                       vec!()
                    )
                )
            )
        );
        
        self.group_map.insert(
            TokenType::Value,
            Node::new(
                TokenType::Value,
                vec!(
                    Node::leaf(
                        TokenType::ComplexIdent
                    ),
                    Node::leaf(
                        TokenType::Number
                    )
                ),
                vec!()
            )
        );

        self.group_map.insert(
            TokenType::Expression,
            Node::new(
                TokenType::Expression,
                vec!(
                    Node::new_end(
                        TokenType::Value,
                        vec!(
                            Node::new(
                                TokenType::Symbol,  // Operateur
                                vec!(
                                    Node::leaf(
                                        TokenType::Expression
                                    )
                                ),
                                vec!()
                            )
                        ),
                        vec!()
                    )
                ),
                vec!(
                    Node::new(
                        TokenType::Symbol,  //(
                        vec!(
                            Node::new(
                                TokenType::Expression,
                                vec!(),
                                vec!(
                                    Node::leaf(
                                        TokenType::Symbol // )
                                    )
                                )
                            )
                        ),
                        vec!()
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::Affectation,
            Node::new(
                TokenType::Affectation,
                vec!(
                    Node::new(
                        TokenType::Symbol, // =
                        vec!(
                            Node::leaf(
                                TokenType::Expression 
                            )
                        ),
                        vec!()
                    )
                ),
                vec!()
            )
        );

        self.group_map.insert(
            TokenType::Instruction,
            Node::new(
                TokenType::Instruction,
                vec!(),
                vec!(
                    Node::leaf(
                        TokenType::Symbol, // }
                    ),
                    Node::new(
                        TokenType::Type, 
                        vec!(
                            Node::new(
                                TokenType::Ident,
                                vec!(
                                    Node::new(
                                        TokenType::Affectation,
                                        vec!(),
                                        vec!(Node::end_inst())
                                    )
                                ),
                                vec!(Node::end_inst())
                            )
                        ),
                        vec!()
                    ),
                    Node::new(
                        TokenType::Ident,
                        vec!(
                            Node::new(
                                TokenType::Affectation,
                                vec!(),
                                vec!(Node::end_inst())
                            ),
                            Node::new(
                                TokenType::Tuple,
                                vec!(),
                                vec!(Node::end_inst())
                            ),
                        ),
                        vec!()
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::Program,
            Node::new(
                TokenType::Program, 
                vec!(
                    Node::new(
                        TokenType::Instruction,
                        vec!(
                            Node::leaf(
                                TokenType::Program,
                            )
                        ),
                        vec!()
                    )
                ), 
                vec!(
                    Node::new(
                        TokenType::Symbol,  // \n
                        vec!(
                            Node::leaf(
                                TokenType::Program,
                            )
                        ),
                        vec!()
                    )
                )
            )
        );
    }
}



fn is_sign(c: char) -> bool {
    !is_number(c) && !is_letter(c)
}

fn is_number(c: char) -> bool {
    (c as u8) < 58 && (c as u8) >= 48
}

fn is_letter(c: char) -> bool {
    (c as u8) >= 65 && (c as u8) <= 122 && !((c as u8) >= 91 && (c as u8) <= 96)  
}