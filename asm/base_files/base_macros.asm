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
    sub rbx, 32
    add rsi, rbx
    mov rdx, 1
    syscall

%endmacro

%macro dn 1
    mov rax, %1
    xor r10, r10    
    and rax, rax
    jl %%_neg
    %%_local_label_stock_loop:
        inc r10
        xor rdx, rdx          
        mov rcx, 10         
        idiv rcx
        push rdx
        and rax, rax
        jne %%_local_label_stock_loop

    %%_local_label_display:
        and r10, r10  
        je %%_local_label_end_loop_display_number
        pop rbx        
        add rbx, 48
        print_char rbx
        dec r10
        jmp %%_local_label_display

    %%_neg:
        neg rax
        push rax
        print_char _soustr
        pop rax
        jmp %%_local_label_stock_loop

    %%_local_label_end_loop_display_number:
        print_char _newline 
    
    
%endmacro


%macro exit 1

    mov rax, 60
    mov rdi, %1
    syscall

%endmacro   

