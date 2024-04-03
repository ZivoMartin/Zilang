pub use crate::zipiler::{
                        prog_manager::{prog_manager::ProgManager, include::{
                            VariableDefinition, 
                            Class,
                            Type,
                            POINTER_SIZE,
                            MUL_REGISTER
                        }},
                        program::Tool,
                        tokenizer::include::{TokenType, Token, OPERATORS}
                    };
pub use crate::zipiler::collections::Stack;
pub use std::collections::HashMap;
