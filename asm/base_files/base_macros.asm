%include "asm/data.asm"

section .text

%macro _deref 1
    xor r11, r11
    xor r10, r10
    mov r11, %1
    dec r11
    %%_deref_loop:
        cmp r10, r11
        jne %%_deref_end_loop
        inc r10
        movsx rax, dword[stack+rax]
        jmp %%_deref_loop 
    %%_deref_end_loop:    
%endmacro

%macro print_char 1
    
    mov rax, 1
    mov rdi, 1
    mov rsi, %1
    mov rdx, 1
    syscall

%endmacro

%macro dn 1
    mov rax, %1
    xor r10, r10    
    %%_local_label_stock_loop:
        inc r10
        xor rdx, rdx          
        mov rcx, 10         
        idiv rcx
        push rdx
        cmp rax, 0
        jne %%_local_label_stock_loop

    mov rax, 1  
    mov rdx, 1
    %%_local_label_display:
        pop rbx        
        cmp r10, 0  
        je %%_local_label_end_loop_display_number
        mov rsi, _chiffres
        add rsi, rbx 
        print_char rsi
        dec r10
        jmp %%_local_label_display

    %%_local_label_end_loop_display_number:
        print_char _newline 
%endmacro


%macro exit 1

    mov rax, 60
    mov rdi, %1
    syscall

%endmacro   

