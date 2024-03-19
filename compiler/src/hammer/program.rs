use crate::hammer::tokenizer::include::{Token, TokenType};
use super::collections::Stack;
use std::collections::HashMap;
use crate::hammer::memory::Memory;
use super::tools::{
            exp_tools::ExpTools,
            decl_tools::DeclTools,
            cident_tools::CIdentTools,
            macrocall_tools::MacroCallTools,
            complexchar_tools::ComplexCharTools,
            instructions_tools::InstructionTools,
            keyword_tools::KeyWordTools,
            bloc_tools::BlocTools,
            complex_type_tools::ComplexTypeTools,
            bloc_keyword::{
                if_tools::IfTools,
                while_tools::WhileTools,
                for_tools::ForTools,
                do_tools::DoTools,
                func_tools::FuncTools
            }
            
        };

pub trait Tool {

    fn new(memory: &mut Memory) -> Box<dyn Tool> where Self: Sized;

    fn end(&mut self, memory: &mut Memory) -> Result<(Token, String), String>;
    
    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<String, String>;

}

fn build_constructor_map() -> HashMap<TokenType, fn(&mut Memory) -> Box<dyn Tool>> {
    let mut res = HashMap::<TokenType, fn(memory: &mut Memory) -> Box<dyn Tool>>::new();
    res.insert(TokenType::Instruction, InstructionTools::new);
    res.insert(TokenType::Declaration, DeclTools::new);
    res.insert(TokenType::Expression, ExpTools::new);
    res.insert(TokenType::ComplexIdent, CIdentTools::new);
    res.insert(TokenType::MacroCall, MacroCallTools::new);
    res.insert(TokenType::ComplexType, ComplexTypeTools::new);
    res.insert(TokenType::ComplexChar, ComplexCharTools::new);
    res.insert(TokenType::Bloc, BlocTools::new);
    res.insert(TokenType::KeywordInstruction, KeyWordTools::new);
    res.insert(TokenType::IfKeyword, IfTools::new);
    res.insert(TokenType::ForKeyword, ForTools::new);
    res.insert(TokenType::WhileKeyword, WhileTools::new);
    res.insert(TokenType::FuncKeyword, FuncTools::new);
    res.insert(TokenType::DoKeyWord, DoTools::new);
    res
}

pub struct Program {
    memory: Memory,
    tools_stack: Stack<Box<dyn Tool>>,
    constructor_map: HashMap<TokenType, fn(memory: &mut Memory) -> Box<dyn Tool>>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            memory: Memory::new(),
            tools_stack: Stack::new(),
            constructor_map: build_constructor_map(),
        }
    }

    pub fn tokenize(&mut self, token: Token) -> Result<(String, usize), String> {
        Ok((self.tools_stack.val_mut().unwrap().new_token(token, &mut self.memory)?, self.memory.current_file))
    }

    pub fn new_group(&mut self, type_token: TokenType) {
        println!("new           {type_token:?}");
        self.tools_stack.push((self.constructor_map.get(&type_token).unwrap())(&mut self.memory));
    }

    pub fn end_group(&mut self) -> Result<(String, usize), String>{
        let (token_to_raise, mut end_txt) = self.tools_stack.pop().unwrap().end(&mut self.memory)?;
        println!("end           {:?}", token_to_raise.token_type);
        let asm = if !self.tools_stack.is_empty() {
            self.tokenize(token_to_raise)?.0
        }else{
            String::new()
        };
        end_txt.push_str(&asm);
        Ok((end_txt, self.memory.current_file))
    }

    
}


pub fn panic_bad_token(receiver: &str, token: Token) {
    panic!("Unknow token type for a {receiver}: {:?}    {}", token.token_type, token.content)
}
