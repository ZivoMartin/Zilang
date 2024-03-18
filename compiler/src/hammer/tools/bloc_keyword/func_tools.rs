use crate::hammer::tools::include::*;

pub struct FuncTools {
    name: String,
    type_args: Vec<Type>,
    return_type: Type
}

impl Tool for FuncTools {

    fn new(memory: &mut Memory) -> Box<dyn Tool> where Self: Sized {
        memory.in_func();
        Box::from(
            FuncTools{
                name: String::new(),
                type_args: Vec::new(),
                return_type: Type{name: String::new(), size: 0, stars: 0}
            }
        )
    }

    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<String, String> {
        let mut res = String::new();
        match token.token_type {
            TokenType::Declaration => self.new_arg(memory, token.content),
            TokenType::Ident => res = self.set_ident(token.content),
            TokenType::Type => self.set_type_name(memory, token.content),
            TokenType::Symbol => self.new_star(),
            TokenType::Bloc => (),
            _ => panic_bad_token("func keyword", token)
        }
        Ok(res)
    }

    
    fn end(&mut self, memory: &mut Memory) -> Result<(Token, String), String> {
        memory.out_func();
        Ok((Token::new(TokenType::FuncKeyword, String::new()), String::new()))
    }
}

impl FuncTools {

    fn new_arg(&mut self, memory: &mut Memory, dec_data: String) {
        let var_def = memory.get_var_def(&str::parse::<usize>(&dec_data).unwrap()).unwrap();
        self.type_args.push(var_def.type_var.clone())
    }

    fn set_ident(&mut self, name: String) -> String {
        self.name = name;
        format!("{}:", self.name)
    } 

    fn new_star(&mut self) {
        self.return_type.size = 4;
        self.return_type.stars += 1;
    }

    fn set_type_name(&mut self, memory: &Memory, name: String) {
        self.return_type.size = memory.get_type_size(0, &name);
        self.return_type.name = name;
    }

}