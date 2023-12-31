
section .data:
    chiffres: db '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
    newline: db 10
    next: db 44, 32

section .text

%macro print_char 1
    
    mov rax, 1
    mov rdi, 1
    mov rsi, %1
    mov rdx, 1
    syscall

%endmacro



%macro display 1

    mov rax, %1
    xor r10, r10    
    stock:
        inc r10
        xor rdx, rdx          
        mov rcx, 10         
        idiv rcx
        push rdx
        cmp rax, 0
        jne stock
    mov rax, 1  
    mov rdx, 1
    display:
        pop rbx        
        cmp r10, 0  
        je end_loop_display_number
        mov rsi, chiffres
        add rsi, rbx 
        print_char rsi
        dec r10
        jmp display
    end_loop_display_number:
        print_char newline 
        
%endmacro

%macro exit 0

    mov rax, 60
    xor rdx, rdx
    syscall

%endmacro   