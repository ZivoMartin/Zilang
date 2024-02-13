mod hammer; 
mod structs;
mod tracker;

use hammer::{Hammer, instruct_loop};

/// This function takes as parameters the program name, the text you want to compile and a Boolean indicating whether you want the 
/// the compiler to optimize for you. The function places the asm text in the files in the asm directory and simply returns a result.
pub fn compile_txt(prog_name: String, input: String, debug: bool) -> Result<(), String>{
    let mut hammer: Hammer = Hammer::new(prog_name, input, debug);
    instruct_loop(&mut hammer)?;
    Ok(())
}