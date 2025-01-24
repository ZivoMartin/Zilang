# Welcome to Zilang!

**First things first:** this project is **absolutely not stable**.  
Zilang is a toy programming language I created out of curiosity about how compilation works.  

## Getting Started

The source code of the compiler, written entirely in Rust, is located in the `/zipiler` directory. To compile a `.zi` file, you can run:  
```bash
./zipiler file.zi -o file_exe
```

Here:

    ./zipiler is the path to the compiler.
    file.zi is a file containing valid Zilang code.

**Example**

You can find example .zi files in the /zipiler/testing directory.
What is Zilang?

Zilang is a compiled, object-oriented programming language (OOP). You can:

    Declare classes,
    Add methods and attributes to those classes, etc.

**Target**

    The compiler generates NASM native executables for Linux.
    No LLVM is used in the process.

**Limitations**

    Inheritance is not supported yet (but it will be in the future—when I feel motivated enough).
    The compiler has several bugs and is not production-ready.

**Why Zilang?**

Don't use it ! It is useless XD
I made it purely to learn more about compilation. That said, if you’re curious or want to tinker with it, feel free to dive in!
