use super::tokenizer::Tokenizer;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum TokenType {
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
    ComplexIdent,              
    Expression,    
    Brackets, 
    DirectTab, 
    String,   
    DeclarationTuple,              // (Expression, Expression, ... , Expression)
    ExpressionTuple,
    BrackTuple,
    SerieExpression,
    SerieSerieExpression,
    SerieDTab,
    SerieDeclaration,
    SerieChar,
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
    PointerSymbolSerie,
    ComplexChar,

    EndToken
}


static TYPE_LIST: &[&'static str; 3] = &["int", "char", "void"];
pub static OPERATORS: &[&'static str; 13] = &["+", "-", "%", "*", "/", "<", "<=", ">", ">=", "==", "!=", "||", "&&"];
pub static AFFECT_OPERATOR: &[&'static str; 5] = &["=", "+=", "-=", "*=", "/="];
static KEYWORD: &[&'static str; 9] = &["if", "else", "for", "while", "return", "continue", "break", "func", "do"];
pub static OPERATOR_COMPONENT: &[char; 9] = &['+', '%', '/', '<', '>', '=', '|', '&', '!'];
pub static DEFAULT_GARBAGE_CHARACTER: &[char; 3] = &[' ', '\n', '\t'];
static PRIMITIVE_TOKENTYPE: &[TokenType; 6] = &[TokenType::Ident, TokenType::Type, TokenType::Symbol, TokenType::Number, TokenType::Operator, TokenType::Keyword];

impl Copy for TokenType {}

impl Clone for TokenType {
    fn clone(&self) -> TokenType {
        *self
    }
}


pub struct Token {
    pub token_type: TokenType,
    pub content: String
}

impl Token {
    pub fn new(token_type: TokenType, content: String) -> Token {
        Token{token_type, content}
    }
}


#[derive(Debug)]
#[derive(PartialEq)]
pub struct Path<'a> {
    pub path: Vec<&'a Node>,
}

impl<'a> Path<'a> {
    pub fn init(node: &'a Node) -> Path {
        Path{path: vec!(node)}
    }

    pub fn p_node(&self) -> &'a Node {
        self.path[0]
    }

    pub fn proke_travel_functions(&self, tokenizer: &Tokenizer, token_string: &String) {
        for node in self.path.iter().rev() {
            if node.travel_react.is_some() {
                (node.travel_react.unwrap())(tokenizer, node.type_token, token_string)
            }
        }
    } 
}

#[derive(Debug)]
pub struct Node {
    pub type_token: TokenType,
    pub groups: Vec<Node>, 
    pub sons: Vec<Node>,
    pub can_end: bool,
    pub constraints: (Vec::<&'static str>, bool),
    pub consider_garbage: bool,
    pub retry: i8,
    pub travel_react: Option::<fn(&Tokenizer, TokenType, &String)>
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
    pub fn new(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>) -> Node {
        Node::new_c(type_token, groups, sons, get_default_constraint(type_token))
    }

    pub fn new_c_r(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>, constraints: Vec<&'static str>, depth: i8) -> Node {
        Node{type_token, groups, sons, can_end: true, constraints: (constraints, true), consider_garbage: false, retry: depth, travel_react: None}.check_son()        
    }

    /// Build a leaf, a leaf has to be builded
    pub fn leaf(type_token: TokenType) -> Node {
        Node::new_end(type_token, Vec::new(), Vec::new())
    }

    /// Build a new node wich can end the building of the group.
    pub fn new_end(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>) -> Node {
        Node::new_end_c(type_token, groups, sons, get_default_constraint(type_token))
    }

    pub fn new_c(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>, constraints: Vec<&'static str>) -> Node {
        Node{type_token, groups, sons, can_end: false, constraints: (constraints, true), consider_garbage: false, retry: -1, travel_react: None}.check_son()
    }

    pub fn leaf_c(type_token: TokenType, constraints: Vec<&'static str>) -> Node {
        Node::new_end_c(type_token, Vec::new(), Vec::new(), constraints)
    }

    pub fn new_end_c(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>, constraints: Vec<&'static str>) -> Node {
        Node{type_token, groups, sons, can_end: true, constraints: (constraints, true), consider_garbage: false, retry: -1, travel_react: None}.check_son()
    }

    pub fn is_leaf(&self) -> bool {
        self.sons.is_empty() && self.groups.is_empty()
    }

    pub fn priv_const(mut self) -> Node {
        self.constraints.1 = false;
        self
    }

    pub fn constraint_satisfied(&self, c: &str) -> bool {
        let contains = self.constraints.0.contains(&c);
        self.constraints.0.is_empty() || contains && self.constraints.1 || !contains && !self.constraints.1
    }

    pub fn react(mut self, r: fn(&Tokenizer, TokenType, &String)) -> Node {
        self.travel_react = Some(r);
        self
    }
}