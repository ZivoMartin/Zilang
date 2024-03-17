pub use crate::hammer::{
                        {memory::Memory, include::{
                            VariableDefinition, 
                            Type
                        }},
                        program::{Tool, panic_bad_token},
                        tokenizer::include::{TokenType, Token, OPERATORS}
                    };
pub use crate::hammer::collections::Stack;
pub use std::collections::HashMap;

/// (deref_time, stars, size)
pub fn extract_cident_data(d: &str) -> (i8, i32, u32) {
    let mut split = d.split_whitespace();
    (
        str::parse::<i8>(split.next().unwrap()).unwrap(), 
        str::parse::<i32>(split.next().unwrap()).unwrap(), 
        str::parse::<u32>(split.next().unwrap()).unwrap()
    )
}