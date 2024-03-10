use super::hammer::Hammer;

#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub size: u32,
    pub stars: u32
}

impl Type{
    pub fn clone(&self) -> Type{
        Type{
            name: self.name.clone(),
            size: self.size,
            stars: self.stars
        }
    } 
}

pub struct Function {
    pub args: Vec::<VariableDefinition>,
    pub return_type: Type
}

impl Function {

    pub fn clone(&self) -> Function {
        let mut args_cloned = Vec::<VariableDefinition>::new();
        for elt in self.args.iter() {
            args_cloned.push(elt.clone());
        }
        Function {
            args: args_cloned,
            return_type: self.return_type.clone()
        }
    }

}

#[derive(Debug)]
pub struct VariableDefinition{
    pub name: String,
    pub type_var: Type,
}

impl Clone for VariableDefinition {
    fn clone(&self) -> VariableDefinition {
        VariableDefinition {
            name: self.name.clone(),
            type_var: self.type_var.clone(),
        }
    }
}

pub struct MacroCall{
    pub macro_name: String,
    pub args: Vec::<Vec::<Token>>
}

pub struct Jump{
    pub vars: Vec::<u32>, 
    pub stack_index: u32,
    pub action: (fn(&mut Hammer, String), String),
    pub bloc_index: u32
}

impl Jump {

    pub fn new(stack_index: u32, action: (fn(&mut Hammer, String), String), bloc_index: u32) -> Self {
        Jump{
            vars: Vec::new(),
            stack_index,
            action,
            bloc_index
        }
    }

}



