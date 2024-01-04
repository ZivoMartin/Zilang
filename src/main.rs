use std::env;
use hammer::hammer::compile_txt;
use tools::tools::{file_exists, TextFile};
mod tools;
mod hammer;
mod stack;
use std::process::{Command, exit};

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


fn compile(input: &str, output: &str) -> Result<(), String>{
    if !file_exists(&input){
        return Err(format!("File {} don't exist.", input));
    }
    let mut input_file = TextFile::new(String::from(input))?; 
    compile_txt(String::from(input_file.get_text()))?;
    compile_asm_to_executable("asm/script.asm", output);
    Ok(())
}

fn compile_asm_to_executable(file_path: &str, output: &str) {

    let mut output_object = String::from(output);
    output_object.push_str(".o");

    Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg("-o")
        .arg(&output_object)
        .arg(file_path)
        .status().unwrap_or_else(|e| {
            eprintln!("ERROR: Could not call nasm: {e}");
            exit(1);
        });


    Command::new("ld")
        .arg(&output_object)
        .arg("-o")
        .arg(output)
        .status().unwrap_or_else(|e| {
            eprintln!("ERROR: Could not call ld: {e}");
            exit(1);
        });
}
