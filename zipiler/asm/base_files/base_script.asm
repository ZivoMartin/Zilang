%include "./asm/base_files/base_macros.asm"

global _start

section .data
    _operation_tab: dq _addi, _soustr, _multip, _divis, _modulo, _inf, _sup, _inf_equal, _sup_equal, _equal, _diff, _or, _and, true, false

section .text

_operation:

    jmp [_operation_tab+r12*8]

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
        xor rdx, rdx
        mov rax, r10
        idiv r11
        ret

    _modulo:
        xor rdx, rdx
        mov rax, r10
        idiv r11
        mov rax, rdx
        ret 
    
    _inf:
        cmp r10, r11
        jl true
        jmp false

    _sup:
        cmp r10, r11
        jg true
        jmp false

    _inf_equal:
        cmp r10, r11
        jl true
        je true
        jmp false

    _sup_equal:
        cmp r10, r11
        jg true
        je true
        jmp false
    

    _equal:
        cmp r10, r11
        je true
        jmp false

    _diff:
        cmp r10, r11
        jne true
        jmp false

    _or:
        and r10, r10
        jne true
        and r11, r11
        jne true
        jmp false

    _and:
        and r10, r10
        je false
        and r10, r10
        je false
        jmp true

    true:
        mov rax, 1
        ret

    false:
        mov rax, 0
        ret
        
_invalid_address:
    mov rax, 1
    mov rdi, 1
    mov rsi, _seg_fault_msg
    mov rdx, _size_seg_fault_msg
    syscall
    exit 1

_start:
xor r15, r15
mov r14, 4