base:
	cargo run testing/main.vu -o exe

nasm:
	nasm -felf64 asm/script.asm && ld asm/script.o -o exe && rm asm/script.o

clean:
	rm exe*
