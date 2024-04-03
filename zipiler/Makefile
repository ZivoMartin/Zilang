base:
	cargo run testing/main.zi -o exe

nasm:
	nasm -felf64 asm/script.asm && ld asm/script.o -o exe && rm asm/script.o

clean:
	rm exe*

stdio:
	cargo run stdlib/io.vu -o io

old:
	./compiler_save testing/main.zi -o exe