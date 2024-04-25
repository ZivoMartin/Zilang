mod hammer;

use std::env;
use std::fs;
use hammer::compile_txt;

use std::process::{Command, exit, ExitCode};
use std::time::Instant;

static OK: i8 = 0;
static COMPILATION_ERROR: i8 = 1;
static BAD_OPERATOR: i8 = 2;
static BAD_PARAMETER: i8 = 3;
static INPUT_FILE_MISSING: i8 = 4;
static OUTPUT_FILE_MISSING: i8 = 5;
static CONVERT_OPERATOR_MISSING: i8 = 6;


fn main() -> ExitCode {
    let debut = Instant::now();
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        args = vec!("compiler".to_string(), "testing/tokenizer/first.vu".to_string(), "-o".to_string(), "exe".to_string());
    }
    let operations: Vec<&str> = vec!("-o");
    let parameters: Vec<&str> = vec!("-opt");
    
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
                "-opt" => debug = false,
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
    }else if output.is_none() && operation == Some("-o") {
        ("Expected a file name for the output...");
        exit(OUTPUT_FILE_MISSING as i32)
    }else if operation.is_none() {
        eprintln!("You didn't indicate the convert operator...");
        exit(CONVERT_OPERATOR_MISSING as i32);
    }
    match operation.unwrap() {
        "-o" => {compile(input.unwrap(), output.unwrap(), debug).unwrap_or_else(|e| {
            eprintln!("{e}");
        })},
        _ => panic!("Impossible")
    }
    println!("\n Succès: {:?}", debut.elapsed());
    ExitCode::from(OK as u8)
}


fn compile(input: &str, output: &str, debug: bool) ->  Result<(), std::io::Error> {
    let input_file: fs::File = fs::File::open(String::from(input))?;
    compile_txt(input.to_string(), input_file, debug).unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(COMPILATION_ERROR as i32);
    });
    compile_asm_to_executable("asm/script.asm", output);
    Ok(())
}

#[allow(dead_code)]
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