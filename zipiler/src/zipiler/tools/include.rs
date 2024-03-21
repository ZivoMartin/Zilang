pub use crate::zipiler::{
                        prog_manager::{prog_manager::ProgManager, include::{
                            VariableDefinition, 
                            Type
                        }},
                        program::{Tool, panic_bad_token},
                        tokenizer::include::{TokenType, Token, OPERATORS}
                    };
pub use crate::zipiler::collections::Stack;
pub use std::collections::HashMap;
