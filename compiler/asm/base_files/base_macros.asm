%include "asm/base_files/base_data.asm"

section .text

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
        print_char '-'
        pop rax
        jmp %%_local_label_stock_loop

    %%_local_label_end_loop_display_number:
        call _back_line 
    
    
%endmacro


%macro exit 1
    mov rax, 60
    mov rdi, %1
    syscall

%endmacro   

%macro _deref_byte 1
    xor r11, r11
    xor r10, r10
    mov r11, %1
    %%_deref_loop:
        cmp r10, r11
        je %%_deref_end_loop
        inc r10
        movzx rax, byte[_stack+rax]
        jmp %%_deref_loop 
    %%_deref_end_loop:    
%endmacro

%macro _deref_word 1
    xor r11, r11
    xor r10, r10
    mov r11, %1
    %%_deref_loop:
        cmp r10, r11
        je %%_deref_end_loop
        inc r10
        movsx rax, word[_stack+rax]
        jmp %%_deref_loop 
    %%_deref_end_loop:    
%endmacro

%macro _deref_dword 1
    mov rcx, %1
    %%_deref_loop:
        dec rcx
        jl %%_deref_end_loop
        mov eax, dword[_stack+eax]
        jmp %%_deref_loop 
    %%_deref_end_loop:    
%endmacro

%macro _deref_qword 1
    xor r11, r11
    xor r10, r10
    mov r11, %1
    %%_deref_loop:
        cmp r10, r11
        je %%_deref_end_loop
        inc r10
        mov rax, [_stack+rax]
        jmp %%_deref_loop 
    %%_deref_end_loop:    
%endmacro