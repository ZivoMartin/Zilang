use std::collections::HashMap;
use super::include::{Node, TokenType, AFFECT_OPERATOR};
use super::tokenizer::{
    push_token,
    push_group,
    end_group,
    push_once,
    push_ending_group,
    push_ending_once,
    push_ending_token,
    push_token_and_end,
};

pub fn build_grammar_tree() -> HashMap<TokenType, Node> {
    let mut group_map = HashMap::new();
    group_map.insert(
        TokenType::DeclarationTuple,
        Node::new(
            TokenType::DeclarationTuple,
            vec!(),
            vec!(
                Node::new_c(
                    TokenType::Symbol, // ( 
                    vec!(
                        Node::new(
                            TokenType::SerieDeclaration,
                            vec!(),
                            vec!(
                                Node::leaf_c(TokenType::Symbol, vec!(")")).react(end_group)
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

    group_map.insert(
        TokenType::ExpressionTuple,
        Node::new(
            TokenType::ExpressionTuple,
            vec!(),
            vec!(
                Node::new_c(
                    TokenType::Symbol, // ( 
                    vec!(
                        Node::new(
                            TokenType::SerieExpression,
                            vec!(),
                            vec!(
                                Node::leaf_c(TokenType::Symbol, vec!(")")).react(end_group)
                            ),
                        )
                    ), 
                    vec!(
                        Node::leaf_c(TokenType::Symbol, vec!(")")) // )
                    ),
                    vec!("(")
                )
            )
        )
    );

    
    group_map.insert(
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
                        ).react(end_group)
                    )
                ).react(push_once)
            ),
            vec!()
        )
    );

    group_map.insert(
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
                        ).react(end_group)
                    )
                ).react(push_once)
            ),
            vec!()
        )
    );

    group_map.insert(
        TokenType::ComplexIdent,
        Node::new(
            TokenType::ComplexIdent,
            vec!(),
            vec!(
                Node::new_end(
                    TokenType::Ident,
                    vec!(
                        Node::leaf(TokenType::BrackTuple)
                    ),
                    vec!()
                ).react(push_token),
                Node::new_c(
                    TokenType::Symbol,
                    vec!(
                        Node::leaf(TokenType::ComplexIdent)
                    ),
                    vec!(),
                    vec!("&", "*")
                ).react(push_token),
            )
        )
    );

    group_map.insert(
        TokenType::BrackTuple,
        Node::new(
            TokenType::BrackTuple,
            vec!(
                Node::new_end(
                    TokenType::Brackets,
                    vec!(
                        Node::leaf(TokenType::BrackTuple).react(push_token)
                    ),
                    vec!()
                ).react(push_token),
                Node::new_end(
                    TokenType::ExpressionTuple,
                    vec!(
                        Node::leaf(TokenType::BrackTuple).react(push_token)
                    ),
                    vec!()
                ).react(push_token)
            ),
            vec!()
        )
    );

    group_map.insert(
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
                                ).react(end_group)
                            )
                        ).react(push_once)
                    ),
                    vec!(),
                    vec!("[")
                )
            )
        )
    );

    group_map.insert(
        TokenType::ComplexChar,
        Node::new(
            TokenType::ComplexChar,
            vec!(),
            vec!(
                Node::leaf_c(TokenType::Symbol, vec!("\\", "\"", "\'")).priv_const().react(push_token_and_end), // N'importe quoi sauf la contrainte
                Node::new_c(
                    TokenType::Symbol,
                    vec!(),
                    vec!(
                        Node::leaf(TokenType::Symbol).react(push_token_and_end)
                    ),
                    vec!("\\")
                ).react(push_token)
            )
        )
    );

    group_map.insert(
        TokenType::DirectChar,
        Node::new(
            TokenType::DirectChar,
            vec!(),
            vec!(
                Node::new_c(
                    TokenType::Symbol,
                    vec!(
                        Node::new(
                            TokenType::ComplexChar,
                            vec!(),
                            vec!(
                                Node::leaf_c(TokenType::Symbol, vec!("\'"))
                            ),
                        ).react(push_once)
                    ),
                    vec!(
                    ),
                    vec!("\'")
                ).consider_garbage()
            )
        ).react(push_group)
    );

    group_map.insert(
        TokenType::SerieChar,
        Node::new(
            TokenType::SerieChar,
            vec!(
                Node::new(
                    TokenType::ComplexChar,
                    vec!(),
                    vec!(
                        Node::leaf_c(TokenType::Symbol, vec!("\""))
                    )
                ).react(push_once).consider_garbage()
            ),
            vec!()
        )
    );
    
    group_map.insert(
        TokenType::Value,
        Node::new(
            TokenType::Value,
            vec!(
                Node::leaf(TokenType::ComplexIdent).react(push_group),
                Node::leaf(TokenType::DirectChar)
            ),
            vec!(
                Node::leaf(
                    TokenType::Number
                ).react(push_token),
                Node::new_c(
                    TokenType::Symbol,
                    vec!(
                        Node::leaf(TokenType::Value)
                    ),
                    vec!(),
                    vec!("-")
                )
            )
        ).react(push_group)
    );

    group_map.insert(
        TokenType::SerieSerieExpression,
        Node::new(
            TokenType::SerieSerieExpression,
            vec!(
                Node::new(
                    TokenType::SerieExpression,
                    vec!(),
                    vec!(
                        Node::new_end_c(
                            TokenType::Symbol,
                            vec!(),
                            vec!(
                                Node::new_c(
                                    TokenType::Symbol,
                                    vec!(),
                                    vec!(
                                        Node::new_c(
                                            TokenType::Symbol,
                                            vec!(
                                                Node::leaf(TokenType::SerieSerieExpression)
                                            ),
                                            vec!(),
                                            vec!("{")
                                        )
                                    ),
                                    vec!(",")
                                )
                            ),
                            vec!("}")
                        ).react(end_group)
                    )
                )
            ),
            vec!()
        )
    );

    group_map.insert(
        TokenType::SerieDTab,
        Node::new(
            TokenType::SerieDTab,
            vec!(),
            vec!(
                Node::new_c(
                    TokenType::Symbol,
                    vec!(
                        Node::leaf(TokenType::SerieSerieExpression),
                        Node::new(
                            TokenType::SerieDTab,
                            vec!(),
                            vec!(
                                Node::new_end_c(
                                    TokenType::Symbol,
                                    vec!(),
                                    vec!(
                                        Node::new_c(
                                            TokenType::Symbol,
                                            vec!(
                                                Node::leaf(TokenType::SerieDTab)
                                            ),
                                            vec!(),
                                            vec!(",")
                                        )
                                    ),
                                    vec!("}")
                                )
                            )
                        )
                    ),
                    vec!(),
                    vec!("{")
                )
            )
        )
    );

    group_map.insert(
        TokenType::DirectTab,
        Node::new(
            TokenType::DirectTab,
            vec!(),
            vec!(
                Node::new_c(
                    TokenType::Symbol,
                    vec!(
                        Node::leaf(TokenType::SerieSerieExpression),
                        Node::new(
                            TokenType::SerieDTab,
                            vec!(),
                            vec!(
                                Node::leaf_c(TokenType::Symbol, vec!("}")).react(end_group)
                            )
                        )
                    ),
                    vec!(),
                    vec!("{")
                )
            )
        )
    );

    group_map.insert(
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
                        ).react(push_token)
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
                                        ).react(push_token)
                                    ), 
                                    vec!(")") 
                                ).react(push_token)
                            )
                        )
                    ),
                    vec!(),
                    vec!("(")
                ).react(push_token)
            )
        )
    );

    group_map.insert(
        TokenType::String,
        Node::new(
            TokenType::String,
            vec!(),
            vec!(
                Node::new_c_r(
                    TokenType::Symbol,
                    vec!(
                        Node::leaf(TokenType::SerieChar)
                    ),
                    vec!(),
                    vec!("\""),
                    0
                ).consider_garbage()
            )
        ).react(push_group)
    );

    group_map.insert(
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
                                    TokenType::DeclBracket,
                                    vec!(
                                        Node::leaf(TokenType::Affectation)
                                    ),
                                    vec!()
                                )
                            ),
                            vec!()   
                        ).react(push_ending_token)
                    )   
                ).react(push_once)
            ),
            vec!()   
        )
    );

    group_map.insert(
        TokenType::DeclBracket,
        Node::new(
            TokenType::DeclBracket,
            vec!(),
            vec!(
                Node::new_c(
                    TokenType::Symbol,
                    vec!(),
                    vec!(
                        Node::new(
                            TokenType::Number,
                            vec!(),
                            vec!(
                                Node::new_end_c(
                                    TokenType::Symbol,
                                    vec!(
                                        Node::leaf(TokenType::DeclBracket)
                                    ),
                                    vec!(),
                                    vec!("]")
                                )
                            )
                        ).react(push_token)
                    ),
                    vec!("[")
                )
            )
        )
    );

    group_map.insert(
        TokenType::Affectation,
        Node::new(
            TokenType::Affectation,
            vec!(),
            vec!(
                Node::new_c(
                    TokenType::Operator, // =
                    vec!(
                        Node::leaf(TokenType::Expression).react(push_group),
                        Node::leaf(TokenType::DirectTab),
                        Node::leaf(TokenType::String)
                    ),
                    vec!(),
                    Vec::from(AFFECT_OPERATOR)
                ).react(push_token)
            )
        )
    );

    
    group_map.insert(
        TokenType::Instruction,
        Node::new(
            TokenType::Instruction,
            vec!(
                Node::leaf(TokenType::KeywordInstruction).react(push_group),
                Node::new_end(
                    TokenType::ComplexIdent,
                    vec!(
                        Node::leaf(TokenType::Affectation),
                    ),
                    vec!()
                ).react(push_group),
                Node::leaf(TokenType::Declaration).react(push_group),
                Node::leaf(TokenType::MacroCall).react(push_group)
            ),
            vec!(
                Node::leaf_c(TokenType::Keyword, vec!("break", "continue")).react(push_token),
            )
        )
    );

    group_map.insert(
        TokenType::MacroCall,
        Node::new(
            TokenType::MacroCall,
            vec!(),
            vec!(
                Node::new_c(
                    TokenType::Symbol,
                    vec!(),
                    vec!(
                        Node::new(
                            TokenType::Ident,
                            vec!(
                                Node::leaf(TokenType::ExpressionTuple).react(push_token)
                            ),
                            vec!()
                        ).react(push_token)
                    ),
                    vec!("!")
                )
            )
        ).react(push_group)
    );

    group_map.insert(
        TokenType::Program,
        Node::new(
            TokenType::Program, 
            vec!(
                Node::new(
                    TokenType::Instruction,
                    vec!(),
                    vec!(
                        Node::leaf_c(TokenType::Symbol, vec!(";")).react(end_group)
                    )
                ).react(push_once)
            ), 
            vec!()      
        )
    );
    
    group_map.insert(
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
                            vec!(),
                            vec!(
                                Node::leaf_c(TokenType::Symbol, vec!("}")).react(end_group)
                            ),
                            vec!(";")
                        ).react(end_group)
                    )
                ).react(push_once)
            ), 
            vec!(),
        )
    );

    group_map.insert(
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
                ).react(push_token)
            )
        ).react(push_group)
    );

    group_map.insert(
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
                ).react(push_token)
            )
        )
    );

    group_map.insert(
        TokenType::KeywordInstruction,
        Node::new(
            TokenType::KeywordInstruction,
            vec!(
                Node::leaf(TokenType::IfKeyword).react(push_group),
                Node::leaf(TokenType::ForKeyword).react(push_group),
                Node::leaf(TokenType::WhileKeyword).react(push_group),
                Node::leaf(TokenType::FuncKeyword).react(push_group),
                Node::leaf(TokenType::DoKeyWord).react(push_once),
                Node::leaf(TokenType::ReturnKeyword).react(push_group),
            ),
            vec!()
        )
    );

    group_map.insert(
        TokenType::Bloc,
        Node::new(
            TokenType::Bloc,
            vec!(
            //    Node::leaf(TokenType::Instruction).react(push_group)
            ),
            vec!(
                Node::new_c_r(
                    TokenType::Symbol,
                    vec!(
                        Node::leaf(TokenType::BlocProgram)
                    ),
                    vec!(Node::leaf_c(TokenType::Symbol, vec!("}")).react(end_group)),
                    vec!("{"),
                    1
                )
            )
        )
    );

    group_map.insert(
        TokenType::ReturnKeyword,
        Node::new(
            TokenType::ReturnKeyword,
            vec!(),
            vec!(
                Node::new_end_c(
                    TokenType::Keyword,
                    vec!(
                        Node::leaf(TokenType::Expression).react(push_group)
                    ),
                    vec!(),
                    vec!("return")
                )
            )   
        )
    );

    
    group_map.insert(
        TokenType::ForKeyword,
        Node::new(
            TokenType::ForKeyword,
            vec!(),
            vec!(
                Node::new_c(
                    TokenType::Keyword,
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
                                                                                Node::leaf(TokenType::Bloc).react(push_once)
                                                                            ),
                                                                            vec!(),
                                                                            vec!(")")
                                                                        ).react(end_group)
                                                                    )
                                                                ).react(push_once)
                                                            ),
                                                            vec!(),
                                                            vec!(";")
                                                        ).react(end_group)
                                                    )
                                                ).react(push_once)
                                            ),
                                            vec!(),
                                            vec!(";")
                                        ).react(end_group)
                                    )
                                ).react(push_once)
                            ),
                            vec!(),
                            vec!("(")
                        )
                    ),
                    vec!("for")
                ).react(push_token)
            )
        )
    );

    group_map.insert(
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
                                                Node::leaf(TokenType::Bloc).react(push_once),
                                                Node::leaf(TokenType::IfKeyword).react(push_group)
                                            ),
                                            vec!(),
                                            vec!("else")
                                        ).react(push_token)
                                    )
                                ).react(push_ending_group)
                            ),
                            vec!()
                        ).react(push_once)
                    ),
                    vec!(),
                    vec!("if")
                )
            )
        )
    );

    group_map.insert(
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
                                    TokenType::DeclarationTuple,
                                    vec!(
                                        Node::new(
                                            TokenType::ComplexType,
                                            vec!(
                                                Node::leaf(TokenType::Bloc).react(push_ending_once)
                                            ),
                                            vec!()
                                        ).react(push_once)
                                    ),
                                    vec!()
                                )
                            ),
                            vec!()
                        ).react(push_token)
                    ),
                    vec!("func")
                )
            ),
        )
    );

    group_map.insert(
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
                            vec!(Node::leaf(TokenType::Bloc).react(push_ending_group)),
                            vec!()
                        ).react(push_once)
                    ),
                    vec!(),
                    vec!("while")
                ).react(push_token)
            ),
        )
    );

    group_map.insert(
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
                                        Node::leaf(TokenType::Expression).react(push_ending_group)
                                    ),
                                    vec!(),
                                    vec!("while")
                                )
                            )
                        ).react(push_group)
                    ),
                    vec!(),
                    vec!("do")
                ).react(push_token)
            )
        )
    );
    group_map
}


