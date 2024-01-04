%include "asm/macros.asm"
%include "asm/data.asm"




global _start


section .data
    multip: db '*'
    soustr: db '-'
    addi: db '+'
    divis: db '/'


section .text

_operation:
    cmp r12b, byte[addi]
    je _addi
    cmp r12b, byte[soustr]
    je _soustr
    cmp r12b, byte[multip]
    je _multip
    cmp r12b, byte[divis]
    je _divis

    _addi:
        add r10, r11
        mov rax, r10
        ret
    _soustr:
        sub r10, r11
        mov rax, r10
        ret
    _multip:
        mov rax, r10
        mul r11
        ret
    _divis:
        xor rcx, rcx
        mov rax, r10
        idiv r11
        ret

_start:

