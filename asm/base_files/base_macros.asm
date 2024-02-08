%include "asm/data.asm"

section .text

%macro _deref 1
    xor r11, r11
    xor r10, r10
    mov r11, %1
    %%_deref_loop:
        cmp r10, r11
        je %%_deref_end_loop
        inc r10
        movsx rbx, dword[_stack+rbx]
        jmp %%_deref_loop 
    %%_deref_end_loop:    
%endmacro

%macro print_char 1
    
    mov rax, 1
    mov rdi, 1
    mov rsi, _ascii
    mov rbx, %1
    add rsi, rbx
    mov rdx, 1
    syscall

%endmacro

_back_line:
mov rax, 1
mov rdi, 1
mov rsi, _newline
mov rdx, 1
syscall
ret

%macro dn 1
    mov rax, %1
    and rax, rax    ; For an unknow reason the previous mov don't proke the flags
    jnl %%_not_neg
    neg rax 
    push rax    ; print_char use rax, we have to save it
    print_char '-'
    pop rax
    %%_not_neg:
    mov rcx, 10    
    push rcx ; This gonna be the back to line char at the end of the loop
    mov r10, 1         
    %%_local_label_stock_loop:
        inc r10
        xor rdx, rdx          
        idiv rcx    ; The rest of the operation rax/rcx go in rdx
        add rdx, 48 ; We condider rdx as a number 
        push rdx
        and rax, rax    
        jne %%_local_label_stock_loop
    %%_local_label_display:
        pop rbx        
        print_char rbx
        dec r10
        jg %%_local_label_display
%endmacro


%macro exit 1

    mov rax, 60
    mov rdi, %1
    syscall

%endmacro   

