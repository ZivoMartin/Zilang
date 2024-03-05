
pub struct Type {
    pub name: String,
    pub size: u8,
    pub stars: u32
}

pub struct Function {
    pub args: Vec::<VariableDefinition>,
    pub return_type: Type,
    name: String
}


#[allow(dead_code)]
pub struct AsmType{
    pub long: &'static str,
    pub short: &'static str,
    pub register: &'static str,
    pub mov: &'static str
}

pub struct VariableDefinition{
    pub name: String,
    pub type_var: Type,
}