use std::env;
use hammer::hammer::compile_txt;
use tools::tools::{file_exists, TextFile};
mod tools;
mod hammer;
mod stack;
mod tracker;
use std::process::{Command, exit};





fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let operations: Vec<&str> = vec!("-o");
    let parameters: Vec<&str> = vec!("-g");
    
    let mut operation: Option<&str> = None;
    let mut debug = false;
    let mut input: Option<&str> = None;
    let mut output: Option<&str> = None;
    for elt in args.iter().skip(1) {
        if operations.contains(&(elt as &str)) {
            if operation.is_some() {
                return Err(String::from("There is two convert operator in the command line."))
            }
            operation = Some(elt);
        }else if parameters.contains(&(elt as &str)) {
            match elt as &str {
                "-g" => debug = true,
                _ => return Err(format!("Unknow parameter in the command line: {}", elt))
            }
        }else if input.is_none() {
            input = Some(elt)
        }else{
            if output.is_some() {
                return Err(format!("We found an invalid parameter in the command line: {}", elt));
            }
            output = Some(elt)
        }
    } 
    if input.is_none() {
        return Err(String::from("Expected a file name to compile..."));
    }else if output.is_none() {
        return Err(String::from("Expected a file name for the output..."));
    }else if operation.is_none() {
        return Err(String::from("You didn't indicate the convert operator..."));
    }
    compile(input.unwrap(), output.unwrap(), debug)?;
    Ok(())
}


fn compile(input: &str, output: &str, debug: bool) -> Result<(), String>{
    if !file_exists(&input){
        return Err(format!("File {} don't exist.", input));
    }
    let mut input_file = TextFile::new(String::from(input))?; 
    compile_txt(input.to_string(), String::from(input_file.get_text()), debug)?;
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