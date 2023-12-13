use std::env;
use hammer::hammer::compile_txt;
use tools::tools::{file_exists, TextFile};
mod tools;
mod hammer;
mod stack;

fn main() -> Result<(), String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    match args.len() == 3{
        true => {
            let operation = args.remove(1);
            if operation == String::from("-o") {
                    compile(&args[0], &args[1])?;
            }else{  
                return Err(format!("{} is unknow in second argument", operation));
            }
        }
        _ => return Err(String::from("Bad arguments"))
    }
    Ok(())
}


fn compile(input: &str, _output: &str) -> Result<(), String>{
    if !file_exists(&input){
        return Err(format!("File {} don't exist.", input));
    }
    let mut input_file = TextFile::new(String::from(input))?; 
    compile_txt(String::from(input_file.get_text()))
}

