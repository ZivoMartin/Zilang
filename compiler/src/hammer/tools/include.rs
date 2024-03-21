pub use crate::hammer::{
                        prog_manager::{prog_manager::ProgManager, include::{
                            VariableDefinition, 
                            Type
                        }},
                        program::{Tool, panic_bad_token},
                        tokenizer::include::{TokenType, Token, OPERATORS}
                    };
pub use crate::hammer::collections::Stack;
pub use std::collections::HashMap;
