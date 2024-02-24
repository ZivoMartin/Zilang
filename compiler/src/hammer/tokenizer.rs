use std::collections::HashMap;

#[derive(Debug)]
enum PrimitiveTokenType {
    Ident,
    Number,
    Type,
    Symbol,
    Comma,
    Keyword
}

#[derive(Eq, Hash, PartialEq)]
enum TypeTokenGroup {
    ComplexIdent,
    Expression,
    Tuple,
    Instruction
}

enum CharType {
    Number,
    Letter,
    Symbol,
    Unknow
}



#[derive(Debug)]
pub struct Token {
    token_type: PrimitiveTokenType,
    string: String
}

struct Node {
    type_token: PrimitiveTokenType,
    groups: Vec<GroupNode>,
    sons: Vec<Node>
}

impl Node {

    fn new(
        type_token: PrimitiveTokenType,
        groups: Vec<GroupNode>,
        sons: Vec<Node>
        ) -> Node {Node{type_token, groups, sons}}

}

struct GroupNode {
    type_group: TypeTokenGroup,
    sons: Vec<Node>,
    groups: Vec<GroupNode>
}

impl GroupNode {
    fn new(
        type_group: TypeTokenGroup,
        groups: Vec<GroupNode>,
        sons: Vec<Node>
        ) -> GroupNode {GroupNode{type_group, groups, sons}}}

struct TokenGroup {
    group_type: TypeTokenGroup,
    sons: Vec<Node>
}

impl TokenGroup {
    fn new(
        group_type: TypeTokenGroup,
        sons: Vec<Node>
    ) -> TokenGroup {TokenGroup{group_type, sons}}
}


pub struct Tokenizer {
    group_map: HashMap<TypeTokenGroup, TokenGroup>
}

impl Tokenizer {

    pub fn new() -> Tokenizer {
        let mut res = Tokenizer{group_map: HashMap::<TypeTokenGroup, TokenGroup>::new()};
        res.init_token_groups();
        res
    }

    fn init_token_groups(&mut self) {
        self.group_map.insert(
            TypeTokenGroup::Tuple,
            TokenGroup::new(
                TypeTokenGroup::Tuple,
                vec!(
                    Node::new(
                        PrimitiveTokenType::Symbol,
                        vec!(
                            GroupNode::new(
                                TypeTokenGroup::Expression,
                                vec!(),
                                vec!()
                            )
                        ),
                        vec!(
                            Node::new(
                                PrimitiveTokenType::Symbol,
                                vec!(),
                                vec!()
                            )
                        )
                    )
                )
            )
        );
    }

    pub fn tokenize(&self, input: String) -> Result<Vec<Token>, String> {
        let mut _current_token = String::new();
        for c in input.chars() {
            if is_number(c) {

            }else if is_letter(c) {

            }else if is_unknow(c) {
                return Err(format!("You can't put the character {} here", c))
            }else{

            }
        }
        Ok(Vec::new())
    } 

}

fn is_unknow(c: char) -> bool {
    (c as u8) < 33 || "#.?@\\^~".contains(c)
}

fn is_number(c: char) -> bool {
    (c as u8) < 58 && (c as u8) >= 48
}

fn is_letter(c: char) -> bool {
    (c as u8) >= 65 && (c as u8) <= 122 && !((c as u8) >= 91 && (c as u8) <= 96)  
}