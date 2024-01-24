%include "asm/functions.asm"

global _start


section .text

_operation:
    xor rcx, rcx

    cmp r12b, byte[_addi]
    je _addi_op
    cmp r12b, byte[_soustr]
    je _soustr_op
    cmp r12b, byte[_multip]
    je _multip_op
    cmp r12b, byte[_divis]
    je _divis_op
    cmp r12b, byte[_modulo]
    je _modulo_op

    cmp r12b, byte[_inf]
    je _inf_op
    cmp r12b, byte[_sup]
    je _sup_op
    cmp r12b, byte[_equal]
    je _equal_op
    cmp r12b, byte[_inf_equal]
    je _inf_equal_op
    cmp r12b, byte[_sup_equal]
    je _sup_equal_op
    cmp r12b, byte[_and]
    je _and_op
    cmp r12b, byte[_or]
    je _or_op
    cmp r12b, byte[_diff]
    je _diff_op

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
        mov rax, r10
        idiv r11
        ret

    _modulo_op:
        mov rax, r10
        idiv r11
        mov rax, rdi
        ret 
    
    _inf_op:
        cmp r10, r11
        jl true
        jmp false

    _sup_op:
        cmp r10, r11
        jg true
        jmp false

    _inf_equal_op:
        cmp r10, r11
        jl true
        je true
        jmp false

    _sup_equal_op:
        cmp r10, r11
        jg true
        je true
        jmp false
    

    _equal_op:
        cmp r10, r11
        je true
        jmp false

    _diff_op:
        cmp r10, r11
        jne true
        jmp false

    _or_op:
        cmp r10, rcx
        jne true
        cmp r11, rcx
        jne true
        jmp false

    _and_op:
        cmp r10, rcx
        je false
        cmp r11, rcx
        je false
        jmp true


    true:
        mov rax, 1
        ret

    false:
        mov rax, 0
        ret

_start:

