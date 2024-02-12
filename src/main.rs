use std::env;
use hammer::hammer::compile_txt;
use tools::tools::{file_exists, TextFile};
mod tools;
mod hammer;
mod stack;
mod tracker;
use std::process::{Command, exit, ExitCode};

static OK: i8 = 0;
static COMPILATION_ERROR: i8 = 1;
static BAD_OPERATOR: i8 = 2;
static BAD_PARAMETER: i8 = 3;
static INPUT_FILE_MISSING: i8 = 4;
static OUTPUT_FILE_MISSING: i8 = 5;
static CONVERT_OPERATOR_MISSING: i8 = 6;
static FILE_DOESNT_EXISTS: i8 = 7;


fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let operations: Vec<&str> = vec!("-o");
    let parameters: Vec<&str> = vec!("-t");
    
    let mut operation: Option<&str> = None;
    let mut debug = true;
    let mut input: Option<&str> = None;
    let mut output: Option<&str> = None;
    for elt in args.iter().skip(1) {
        if operations.contains(&(elt as &str)) {
            if operation.is_some() {
                eprintln!("There is two convert operator in the command line.");
                exit(BAD_OPERATOR as i32)
            }
            operation = Some(elt);
        }else if parameters.contains(&(elt as &str)) {
            match elt as &str {
                "-t" => debug = false,
                _ => {
                    eprintln!("Unknow parameter in the command line: {}", elt);
                    exit(BAD_PARAMETER as i32)
                }
            }
        }else if input.is_none() {
            input = Some(elt)
        }else{
            if output.is_some() {
                eprintln!("We found an invalid parameter in the command line: {}", elt);
                exit(BAD_PARAMETER as i32)
            }
            output = Some(elt)
        }
    } 
    if input.is_none() {
        eprintln!("Expected a file name to compile...");
        exit(INPUT_FILE_MISSING as i32)
    }else if output.is_none() {
        ("Expected a file name for the output...");
        exit(OUTPUT_FILE_MISSING as i32)
    }else if operation.is_none() {
        eprintln!("You didn't indicate the convert operator...");
        exit(CONVERT_OPERATOR_MISSING as i32);
    }
    compile(input.unwrap(), output.unwrap(), debug)
}


fn compile(input: &str, output: &str, debug: bool) -> ExitCode {
    if !file_exists(&input){
        eprintln!("File {} don't exist.", input);
        exit(FILE_DOESNT_EXISTS as i32)
    }
    let mut input_file = TextFile::new(String::from(input)).unwrap();
    compile_txt(input.to_string(), String::from(input_file.get_text()), debug).unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(COMPILATION_ERROR as i32);
    });
    compile_asm_to_executable("asm/script.asm", output);
    ExitCode::from(OK as u8)
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