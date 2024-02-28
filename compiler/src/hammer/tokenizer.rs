use std::collections::HashMap;
use std::str::Chars;
use std::iter::Peekable;

#[derive(Eq, Hash, PartialEq, Debug)]
enum TokenType {
    // Primitive Token
    Ident,
    Number,
    Type,
    Symbol,
    Operator,
    Keyword,

    // Token group
    Program,
    BlocProgram,
    Instruction,
    Value,
    ComplexType,
    ExpIdent,              
    MemorySpot,       
    Expression,    
    Brackets,     
    Tuple,              // (Expression, Expression, ... , Expression)
    BrackTuple,
    SerieExpression,
    SerieDeclaration,
    Declaration,    
    Affectation,        // = Expression
    Bloc,
    KeywordInstruction,
    IfKeyword,
    ForKeyword,
    WhileKeyword,
    FuncKeyword,
    DoKeyWord,
    MacroCall,
    DirectChar,
    PointerSymbolSerie
}


static TYPE_LIST: &[&'static str; 3] = &["int", "char", "void"];
static OPERATORS: &[&'static str; 13] = &["+", "-", "%", "*", "/", "<", "<=", ">", ">=", "==", "!=", "||", "&&"];
static AFFECT_OPERATOR: &[&'static str; 5] = &["=", "+=", "-=", "*=", "/="];
static KEYWORD: &[&'static str; 9] = &["if", "else", "for", "while", "return", "continue", "break", "func", "do"];
static OPERATOR_COMPONENT: &[char; 8] = &['+', '%', '/', '<', '>', '=', '|', '&'];
static DEFAULT_GARBAGE_CHARACTER: &[char; 3] = &[' ', '\n', '\t'];
static PRIMITIVE_TOKENTYPE: &[TokenType; 6] = &[TokenType::Ident, TokenType::Type, TokenType::Symbol, TokenType::Number, TokenType::Operator, TokenType::Keyword];

impl Copy for TokenType {}

impl Clone for TokenType {
    fn clone(&self) -> TokenType {
        *self
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct Token {
    token_type: TokenType,
    string: String
}

impl Token {
    fn new(token_type: TokenType, string: String) -> Token {
        Token{token_type, string}
    }
}
#[derive(Debug)]
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

#[derive(Debug)]
struct Node {
    type_token: TokenType,
    groups: Vec<Node>, 
    sons: Vec<Node>,
    can_end: bool,
    constraints: Vec::<&'static str>,
    consider_garbage: bool,
}


impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        other.type_token == self.type_token
    }
}

fn get_default_constraint(token_type: TokenType ) -> Vec<&'static str> {
    match token_type {
        TokenType::Type => Vec::from(TYPE_LIST),
        TokenType::Operator => Vec::from(OPERATORS),
        TokenType::Keyword => Vec::from(KEYWORD),
        _ => Vec::new()
    }
}

impl Node {

    fn check_son(self) -> Node{
        for son in self.sons.iter() {
            if !PRIMITIVE_TOKENTYPE.contains(&son.type_token) {
                println!("ERROR DURING THE BUILDING OF THE TREE:");
                panic!("{:?} was found on a branch of a {:?} when a primitive type was expected", son.type_token, self.type_token);
            }
        }
        for group in self.groups.iter() {
            if PRIMITIVE_TOKENTYPE.contains(&group.type_token) {
                println!("ERROR DURING THE BUILDING OF THE TREE:");
                panic!("{:?} was found on a branch of a {:?} when a group type was expected", group.type_token, self.type_token);
            }
        }
        self
    }

    /// Build a new node wich has to be builded.
    fn new(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>) -> Node {
        Node{type_token, groups, sons, can_end: false, constraints: get_default_constraint(type_token), consider_garbage: false}.check_son()
    }

    /// Build a leaf, a leaf has to be builded
    fn leaf(type_token: TokenType) -> Node {
        Node{type_token, sons: vec!(), groups: vec!(), can_end: true, constraints: get_default_constraint(type_token), consider_garbage: false}
    }

    /// Build a new node wich can end the building of the group.
    fn new_end(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>) -> Node {
        Node{type_token, groups, sons, can_end: true, constraints: get_default_constraint(type_token), consider_garbage: false}.check_son()
    }

    fn new_c(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>, constraints: Vec<&'static str>) -> Node {
        Node{type_token, groups, sons, can_end: false, constraints, consider_garbage: false}.check_son()
    }

    fn leaf_c(type_token: TokenType, constraints: Vec<&'static str>) -> Node {
        Node{type_token, sons: vec!(), groups: vec!(), can_end: true, constraints, consider_garbage: false}
    }

    fn new_end_c(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>, constraints: Vec<&'static str>) -> Node {
        Node{type_token, groups, sons, can_end: true, constraints, consider_garbage: true}.check_son()
    }

    fn is_leaf(&self) -> bool {
        self.sons.is_empty() && self.groups.is_empty()
    }

}

pub struct Tokenizer {
    group_map: HashMap<TokenType, Node>,
    priority_map: HashMap<TokenType, u8>,
    identity_map: HashMap<fn(char)->bool, Vec<TokenType>>
}

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

    pub fn new() -> Tokenizer {
        let mut res = Tokenizer{
            group_map: HashMap::<TokenType, Node>::new(),
            priority_map: build_priority_map(),
            identity_map: build_identity_map()
        };
        res.init_token_groups();
        res
    }

    pub fn tokenize(&mut self, input: String) -> Result<Vec<Token>, String> {
        let mut result = Vec::<Token>::new();
        let first_node = self.group_map.get(&TokenType::Program).unwrap();
        let mut chars = input.chars().peekable();
        while chars.peek().is_some() {  
            self.curse(first_node, &mut result, &mut chars)?;
            self.skip_garbage(&mut chars); 
        }   
        
        Ok(result)
    } 
    
    fn curse(&self, current_node: &Node, res: &mut Vec<Token>, chars: &mut Peekable<Chars>) -> Result<(), String> {
        if !current_node.is_leaf() {
            if !current_node.consider_garbage {
                self.skip_garbage(chars); 
            }
            if !chars.peek().is_some() {
                return Ok(())
            }
            let mut paths_vec = self.get_son_array(current_node);
            let save = chars.clone();
            match self.get_next_token(&mut paths_vec, chars) {
                Ok(token_string) => {
                    match self.filter_nodes(&mut paths_vec, &token_string) {
                        Some(path) => {
                            println!("PUSHED: {:?}: {token_string}", path.p_node().type_token);
                            res.push(Token::new(path.p_node().type_token, token_string));
                            for node in path.path.iter() {
                                self.curse(node, res, chars)?;
                            }
                        }
                        _ => {
                            if !current_node.can_end {
                                return Err(format!("CAN T BUILD THE TREE"))
                            }
                            *chars = save;
                        }
                    }
                },
                Err(e) => {
                    if !current_node.can_end {
                        return Err(e)
                    }
                    *chars = save;
                }
            }
        }
        Ok(())
    }

    fn get_next_token(&self, path_vec: &mut Vec<Path>, chars: &mut Peekable<Chars>) -> Result<String, String> {
        let mut current_token = String::new();
        let c: char = *chars.peek().unwrap();
        for (cond_stop, author_type) in self.identity_map.iter() {
            if cond_stop(c) {
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

    fn clean_son_vec(&self, path_vec: &mut Vec<Path>, author_type: &Vec<TokenType>) -> bool {
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
        current_token.push(chars.nth(0).unwrap());
        if continue_cond != is_sign as fn(char) -> bool {
            while let Some(c) = chars.peek() {
                if continue_cond(*c) {    
                    current_token.push(chars.nth(0).unwrap());
                }else{
                    break;
                }
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
            if !group.is_leaf() {
                for p in paths.iter_mut() {
                    p.path.push(group);
                }
            }
            res.append(&mut paths);
        }
        res
    }

    fn filter_nodes(&'a self, paths: &'a mut Vec::<Path>, token: &str) -> Option<&Path>{
        if token.is_empty() {
            return None
        }
        let mut i = 0;
        let mut res: Option<&Path> = None;
        while i < paths.len() {
            let node = paths[i].p_node();
            if node.constraints.is_empty() || node.constraints.contains(&token) {
                if !res.is_some() || self.priority_map.get(&res.unwrap().p_node().type_token) < self.priority_map.get(&node.type_token){
                    res = Some(&paths[i])
                }
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

    fn init_token_groups(&mut self) {
        self.group_map.insert(
            TokenType::Tuple,
            Node::new(
                TokenType::Tuple,
                vec!(),
                vec!(
                    Node::new_c(
                        TokenType::Symbol, // ( 
                        vec!(
                            Node::new(
                                TokenType::SerieExpression,
                                vec!(),
                                vec!(
                                    Node::leaf_c(TokenType::Symbol, vec!(")"))
                                )
                            ),
                            Node::new(
                                TokenType::SerieDeclaration,
                                vec!(),
                                vec!(
                                    Node::leaf_c(TokenType::Symbol, vec!(")"))
                                )
                            ),
                        ), 
                        vec!(
                            Node::leaf_c(TokenType::Symbol, vec!(")")) // )
                        ),
                        vec!("(")
                    )
                )
            )
        );
        
        self.group_map.insert(
            TokenType::SerieExpression,
            Node::new(
                TokenType::SerieExpression,
                vec!(
                    Node::new_end(
                        TokenType::Expression,
                        vec!(),
                        vec!(
                            Node::leaf_c(TokenType::Symbol, vec!("(")),
                            Node::new_c(
                                TokenType::Symbol, // ,
                                vec!(
                                    Node::leaf(TokenType::SerieExpression)
                                ),
                                vec!(),
                                vec!(",")
                            )
                        )
                    )
                ),
                vec!()
            )
        );

        self.group_map.insert(
            TokenType::SerieDeclaration,
            Node::new(
                TokenType::SerieDeclaration,
                vec!(
                    Node::new_end(
                        TokenType::Declaration,
                        vec!(),
                        vec!(
                            Node::new_c(
                                TokenType::Symbol, // ,
                                vec!(
                                    Node::leaf(TokenType::SerieDeclaration)
                                ),
                                vec!(),
                                vec!(",")
                            )
                        )
                    )
                ),
                vec!()
            )
        );

        self.group_map.insert(
            TokenType::ExpIdent,
            Node::new(
                TokenType::ExpIdent,
                vec!(),
                vec!(
                    Node::new_end(
                        TokenType::Ident,
                        vec!(
                            Node::leaf(TokenType::BrackTuple)
                        ),
                        vec!()
                    ),
                    Node::new_c(
                        TokenType::Symbol,
                        vec!(
                            Node::leaf(TokenType::ExpIdent)
                        ),
                        vec!(),
                        vec!("&", "*")
                    ),
                )
            )
        );

        self.group_map.insert(
            TokenType::MemorySpot,
            Node::new(
                TokenType::MemorySpot,
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
                    Node::new_c(
                        TokenType::Symbol, // $
                        vec!(
                            Node::leaf(
                                TokenType::MemorySpot
                            )
                        ),
                        vec!(),
                        vec!("$")
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::BrackTuple,
            Node::new(
                TokenType::BrackTuple,
                vec!(
                    Node::new_end(
                        TokenType::Brackets,
                        vec!(
                            Node::leaf(TokenType::BrackTuple)
                        ),
                        vec!()
                    ),
                    Node::new_end(
                        TokenType::Tuple,
                        vec!(
                            Node::leaf(TokenType::BrackTuple)
                        ),
                        vec!()
                    )
                ),
                vec!()
            )
        );

        self.group_map.insert(
            TokenType::Brackets,
            Node::new(
                TokenType::Brackets,
                vec!(),
                vec!(
                    Node::new_c(
                       TokenType::Symbol, // [
                       vec!(
                           Node::new(
                                TokenType::Expression,
                                vec!(),
                                vec!(
                                    Node::new_end_c(
                                        TokenType::Symbol, // ]
                                        vec!(
                                            Node::leaf(
                                                TokenType::Brackets
                                            )
                                        ),
                                        vec!(),
                                        vec!("]")
                                    )
                                )
                           )
                       ),
                       vec!(),
                       vec!("[")
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::DirectChar,
            Node::new(
                TokenType::DirectChar,
                vec!(),
                vec!(
                    Node::new_c(
                        TokenType::Symbol,
                        vec!(),
                        vec!(
                            Node::new(
                                TokenType::Symbol,
                                vec!(),
                                vec!(
                                    Node::new_c(
                                        TokenType::Symbol,
                                        vec!(),
                                        vec!(),
                                        vec!("\'")
                                    )
                                ),
                            )
                        ),
                        vec!("\'")
                    )
                )
            )
        );
        
        self.group_map.insert(
            TokenType::Value,
            Node::new(
                TokenType::Value,
                vec!(
                    Node::leaf(TokenType::ExpIdent),
                    Node::leaf(TokenType::DirectChar)
                ),
                vec!(
                    Node::leaf(
                        TokenType::Number
                    ),
                    Node::new_c(
                        TokenType::Symbol,
                        vec!(
                            Node::leaf(TokenType::Value)
                        ),
                        vec!(),
                        vec!("-")
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::Expression,
            Node::new(
                TokenType::Expression,
                vec!(
                    Node::new_end(
                        TokenType::Value,
                        vec!(),
                        vec!(
                            Node::new(
                                TokenType::Operator,  // Operateur
                                vec!(
                                    Node::leaf(
                                        TokenType::Expression
                                    )
                                ),
                                vec!()
                            )
                        )
                    )
                ),
                vec!(
                    Node::new_c(
                        TokenType::Symbol,  //(
                        vec!(
                            Node::new(
                                TokenType::Expression,
                                vec!(),
                                vec!(
                                    Node::new_end_c(
                                        TokenType::Symbol, // )
                                        vec!(),
                                        vec!(
                                            Node::new(
                                                TokenType::Operator,
                                                vec!(
                                                    Node::leaf(TokenType::Expression)
                                                ),
                                                vec!()
                                            )
                                        ), 
                                        vec!(")") 
                                    )
                                )
                            )
                        ),
                        vec!(),
                        vec!("(")
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::Declaration,
            Node::new(
                TokenType::Declaration,
                vec!(
                    Node::new(
                        TokenType::ComplexType,
                        vec!(),
                        vec!(
                            Node::new_end(
                                TokenType::Ident,
                                vec!(
                                    Node::leaf(TokenType::Affectation),
                                    Node::new_end(
                                        TokenType::Brackets,
                                        vec!(
                                            Node::leaf(TokenType::Affectation)
                                        ),
                                        vec!()
                                    )
                                ),
                                vec!()   
                            )
                        )   
                    )
                ),
                vec!()   
            )
        );

        self.group_map.insert(
            TokenType::Affectation,
            Node::new(
                TokenType::Affectation,
                vec!(),
                vec!(
                    Node::new_c(
                        TokenType::Operator, // =
                        vec!(
                            Node::leaf(TokenType::Expression)
                        ),
                        vec!(),
                        Vec::from(AFFECT_OPERATOR)
                    )
                )
            )
        );

        
        self.group_map.insert(
            TokenType::Instruction,
            Node::new(
                TokenType::Instruction,
                vec!(
                    Node::leaf(TokenType::KeywordInstruction),
                    Node::new(
                        TokenType::MemorySpot,
                        vec!(
                            Node::leaf(TokenType::Affectation),
                        ),
                        vec!()
                    ),
                    Node::leaf(TokenType::Declaration),
                    Node::leaf(TokenType::MacroCall)
                ),
                vec!(
                    Node::leaf_c(TokenType::Keyword, vec!("break", "continue")),
                    Node::new_end_c(
                        TokenType::Keyword,
                        vec!(Node::leaf(TokenType::Expression)),
                        vec!(),
                        vec!("return")
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::MacroCall,
            Node::new(
                TokenType::MacroCall,
                vec!(),
                vec!(
                    Node::new_c(
                        TokenType::Operator,
                        vec!(),
                        vec!(
                            Node::new(
                                TokenType::Ident,
                                vec!(
                                    Node::leaf(TokenType::Tuple)
                                ),
                                vec!()
                            )
                        ),
                        vec!("!")
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
                        vec!(),
                        vec!(
                            Node::leaf_c(TokenType::Symbol, vec!(";"))
                        )
                    )
                ), 
                vec!()      
            )
        );
        
        self.group_map.insert(
            TokenType::BlocProgram,
            Node::new_end(
                TokenType::BlocProgram, 
                vec!(
                    Node::new(
                        TokenType::Instruction,
                        vec!(),
                        vec!(
                            Node::new_c(
                                TokenType::Symbol,
                                vec!(
                                    Node::leaf(TokenType::BlocProgram)
                                ),
                                vec!(
                                    Node::leaf_c(TokenType::Symbol, vec!("}"))
                                ),
                                vec!(";")
                            )
                        )
                    )
                ), 
                vec!()      
            )
        );

        self.group_map.insert(
            TokenType::ComplexType,
            Node::new(
                TokenType::ComplexType,
                vec!(),
                vec!(
                    Node::new_end(
                        TokenType::Type,
                        vec!(
                            Node::leaf(TokenType::PointerSymbolSerie)
                        ),
                        vec!()
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::PointerSymbolSerie,
            Node::new(
                TokenType::PointerSymbolSerie,
                vec!(),
                vec!(
                    Node::new_end_c(
                        TokenType::Symbol,
                        vec!(
                            Node::leaf(TokenType::PointerSymbolSerie)
                        ),
                        vec!(),
                        vec!("*")
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::KeywordInstruction,
            Node::new(
                TokenType::KeywordInstruction,
                vec!(
                    Node::leaf(TokenType::IfKeyword),
                    Node::leaf(TokenType::ForKeyword),
                    Node::leaf(TokenType::WhileKeyword),
                    Node::leaf(TokenType::FuncKeyword)
                ),
                vec!()
            )
        );

        self.group_map.insert(
            TokenType::Bloc,
            Node::new(
                TokenType::Bloc,
                vec!(
                    Node::leaf(TokenType::Instruction)
                ),
                vec!(
                    Node::new_c(
                        TokenType::Symbol,
                        vec!(
                            Node::leaf(TokenType::BlocProgram)
                        ),
                        vec!(Node::leaf_c(TokenType::Symbol, vec!("}"))),
                        vec!("{")
                    )
                )
            )
        );

        
        self.group_map.insert(
            TokenType::ForKeyword,
            Node::new(
                TokenType::ForKeyword,
                vec!(),
                vec!(
                    Node::new_c(
                        TokenType::Keyword,
                        vec!(
                            
                        ),
                        vec!(
                            Node::new_c(
                                TokenType::Symbol,
                                vec!(
                                    Node::new(
                                        TokenType::Instruction,
                                        vec!(),
                                        vec!(
                                            Node::new_c(
                                                TokenType::Symbol,
                                                vec!(
                                                    Node::new(
                                                        TokenType::Expression,
                                                        vec!(),
                                                        vec!(
                                                            Node::new_c(
                                                                TokenType::Symbol,
                                                                vec!(
                                                                    Node::new(
                                                                        TokenType::Instruction,
                                                                        vec!(),
                                                                        vec!(
                                                                            Node::new_c(
                                                                                TokenType::Symbol,
                                                                                vec!(
                                                                                    Node::leaf(TokenType::Bloc)
                                                                                ),
                                                                                vec!(),
                                                                                vec!(")")
                                                                            )
                                                                        )
                                                                    )
                                                                ),
                                                                vec!(),
                                                                vec!(";")
                                                            )
                                                        )
                                                    )
                                                ),
                                                vec!(),
                                                vec!(";")
                                            )
                                        )
                                    )
                                ),
                                vec!(),
                                vec!("(")
                            )
                        ),
                        vec!("for")
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::IfKeyword,
            Node::new(
                TokenType::IfKeyword,
                vec!(),
                vec!(
                    Node::new_c(
                        TokenType::Keyword,
                        vec!(
                            Node::new(
                                TokenType::Expression,
                                vec!(
                                    Node::new_end(
                                        TokenType::Bloc,
                                        vec!(),
                                        vec!(
                                            Node::new_c(
                                                TokenType::Keyword, 
                                                vec!(
                                                    Node::leaf(TokenType::Bloc),
                                                    Node::leaf(TokenType::IfKeyword)
                                                ),
                                                vec!(),
                                                vec!("else")

                                            )
                                        )
                                    )
                                ),
                                vec!()
                            )
                        ),
                        vec!(),
                        vec!("if")
                    )
                )
            )
        );

        self.group_map.insert(
            TokenType::FuncKeyword,
            Node::new(
                TokenType::FuncKeyword,
                vec!(),
                vec!(
                    Node::new_c(
                        TokenType::Keyword,
                        vec!(),
                        vec!(
                            Node::new(
                                TokenType::Ident,
                                vec!(
                                    Node::new(
                                        TokenType::Tuple,
                                        vec!(
                                            Node::new(
                                                TokenType::ComplexType,
                                                vec!(
                                                    Node::leaf(TokenType::Bloc)
                                                ),
                                                vec!()
                                            )
                                        ),
                                        vec!()
                                    )
                                ),
                                vec!()
                            )
                        ),
                        vec!("func")
                    )
                ),
            )
        );

        self.group_map.insert(
            TokenType::WhileKeyword,
            Node::new(
                TokenType::WhileKeyword,
                vec!(),
                vec!(
                    Node::new_c(
                        TokenType::Keyword,
                        vec!(
                            Node::new(
                                TokenType::Expression,
                                vec!(Node::leaf(TokenType::Bloc)),
                                vec!()
                            )
                        ),
                        vec!(),
                        vec!("while")
                    )
                ),
            )
        );

        self.group_map.insert(
            TokenType::DoKeyWord,
            Node::new(
                TokenType::DoKeyWord,
                vec!(),
                vec!(
                    Node::new_c(
                        TokenType::Keyword,
                        vec!(
                            Node::new_end(
                                TokenType::Bloc,
                                vec!(),
                                vec!(
                                    Node::new_c(
                                        TokenType::Keyword,
                                        vec!(
                                            Node::leaf(TokenType::Expression)
                                        ),
                                        vec!(),
                                        vec!("while")
                                    )
                                )
                            )
                        ),
                        vec!(),
                        vec!("do")
                    )
                )
            )
        );
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
