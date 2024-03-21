mod hammer; 
// mod structs;
// mod tracker;
mod tokenizer;
mod tools;
mod prog_manager;
mod program;
mod collections;
use hammer::Hammer;

/// This function takes as parameters the program name, the text you want to compile and a Boolean indicating whether you want the 
/// the compiler to optimize for you. The function places the asm text in the files in the asm directory and simply returns a result.
pub fn compile_txt(_prog_name: String, input: std::fs::File, _debug: bool) -> Result<(), String>{
    Hammer::new().compile(input);
    Ok(())
}
