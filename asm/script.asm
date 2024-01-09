%include "asm/macros.asm"

global _start


section .text

_operation:
    cmp r12b, byte[_addi]
    je _addi_op
    cmp r12b, byte[_soustr]
    je _soustr_op
    cmp r12b, byte[_multip]
    je _multip_op
    cmp r12b, byte[_divis]
    je _divis_op

    _addi_op:
        add r10, r11
        mov rax, r10
        ret
    _soustr_op:
        sub r10, r11
        mov rax, r10
        ret
    _multip_op:
        mov rax, r10
        mul r11
        ret
    _divis_op:
        xor rcx, rcx
        mov rax, r10
        idiv r11
        ret

_start:

mov rax, 50
mov dword[_stack + 0 + 0*4], eax
xor rax, rax
movsx rax, dword[_stack + 0 + 0*4]
mov dword[_stack + 8 + 0*4], eax
movsx rax, dword[_stack + 8 + 0*4]
dn rax

mov rax, 0
exit rax

