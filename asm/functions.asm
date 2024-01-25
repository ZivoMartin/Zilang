%include "asm/macros.asm"

section .text

merge:
mov rbx, 16
add rbx, r15
mov rax, rbx
push r15
add r15, 20
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 12
add rbx, r15
mov rax, rbx
push r15
add r15, 20
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rbx, 4
add rbx, r15
mov rax, rbx
push r15
add r15, 20
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
pop r11
pop r10
mov r12, 45
call _operation
pop rbx
mov dword[_stack + rbx], eax
mov dword[_stack + r15 + 20 + 4*0], 20
mov rdx, 20
add rdx, r15
mov eax, 60
mov rcx, r15
mov dword[_stack + ecx + eax], edx
mov rbx, 64
add rbx, r15
mov rax, rbx
push r15
add r15, 68
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 4
add rbx, r15
mov rax, rbx
push r15
add r15, 68
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
mov dword[_stack + rbx], eax
mov rbx, 68
add rbx, r15
mov rax, rbx
push r15
add r15, 72
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 8
add rbx, r15
mov rax, rbx
push r15
add r15, 72
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rax, 1
push rax
pop r11
pop r10
mov r12, 43
call _operation
pop rbx
mov dword[_stack + rbx], eax
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rax, 0
pop rbx
mov dword[_stack + rbx], eax
_loop_2:
mov rbx, 64
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rbx, 8
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
pop r11
pop r10
mov r12, 59
call _operation
push rax
mov rbx, 68
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rbx, 12
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
pop r11
pop r10
mov r12, 60
call _operation
push rax
pop r11
pop r10
mov r12, 38
call _operation
cmp rax, 0
je _end_loop_2
mov rbx, 0
add rbx, r15
push rbx
mov rbx, 64
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
push rax
mov rax, rbx
_deref 1
mov rbx, rax
pop rax
mov rcx, 4
mul rcx
add rbx, rax
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rbx, 0
add rbx, r15
push rbx
mov rbx, 68
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
push rax
mov rax, rbx
_deref 1
mov rbx, rax
pop rax
mov rcx, 4
mul rcx
add rbx, rax
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
pop r11
pop r10
mov r12, 62
call _operation
cmp rax, 0
je _end_condition_3
mov rbx, 60
add rbx, r15
push rbx
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
push rax
mov rax, rbx
_deref 1
mov rbx, rax
pop rax
mov rcx, 4
mul rcx
add rbx, rax
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 0
add rbx, r15
push rbx
mov rbx, 68
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
push rax
mov rax, rbx
_deref 1
mov rbx, rax
pop rax
mov rcx, 4
mul rcx
add rbx, rax
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
mov dword[_stack + rbx], eax
mov rbx, 68
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 68
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rax, 1
push rax
pop r11
pop r10
mov r12, 43
call _operation
pop rbx
mov dword[_stack + rbx], eax
jmp _real_end_condition_3
_end_condition_3:
mov rbx, 60
add rbx, r15
push rbx
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
push rax
mov rax, rbx
_deref 1
mov rbx, rax
pop rax
mov rcx, 4
mul rcx
add rbx, rax
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 0
add rbx, r15
push rbx
mov rbx, 64
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
push rax
mov rax, rbx
_deref 1
mov rbx, rax
pop rax
mov rcx, 4
mul rcx
add rbx, rax
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
mov dword[_stack + rbx], eax
mov rbx, 64
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 64
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rax, 1
push rax
pop r11
pop r10
mov r12, 43
call _operation
pop rbx
mov dword[_stack + rbx], eax
_real_end_condition_3:
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rax, 1
push rax
pop r11
pop r10
mov r12, 43
call _operation
pop rbx
mov dword[_stack + rbx], eax
jmp _loop_2
_end_loop_2:
_loop_5:
mov rbx, 64
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rbx, 8
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
pop r11
pop r10
mov r12, 59
call _operation
cmp rax, 0
je _end_loop_5
mov rbx, 60
add rbx, r15
push rbx
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
push rax
mov rax, rbx
_deref 1
mov rbx, rax
pop rax
mov rcx, 4
mul rcx
add rbx, rax
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 0
add rbx, r15
push rbx
mov rbx, 64
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
push rax
mov rax, rbx
_deref 1
mov rbx, rax
pop rax
mov rcx, 4
mul rcx
add rbx, rax
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
mov dword[_stack + rbx], eax
mov rbx, 64
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 64
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rax, 1
push rax
pop r11
pop r10
mov r12, 43
call _operation
pop rbx
mov dword[_stack + rbx], eax
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rax, 1
push rax
pop r11
pop r10
mov r12, 43
call _operation
pop rbx
mov dword[_stack + rbx], eax
jmp _loop_5
_end_loop_5:
_loop_6:
mov rbx, 68
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rbx, 12
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
pop r11
pop r10
mov r12, 60
call _operation
cmp rax, 0
je _end_loop_6
mov rbx, 60
add rbx, r15
push rbx
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
push rax
mov rax, rbx
_deref 1
mov rbx, rax
pop rax
mov rcx, 4
mul rcx
add rbx, rax
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 0
add rbx, r15
push rbx
mov rbx, 68
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
push rax
mov rax, rbx
_deref 1
mov rbx, rax
pop rax
mov rcx, 4
mul rcx
add rbx, rax
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
mov dword[_stack + rbx], eax
mov rbx, 68
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 68
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rax, 1
push rax
pop r11
pop r10
mov r12, 43
call _operation
pop rbx
mov dword[_stack + rbx], eax
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rax, 1
push rax
pop r11
pop r10
mov r12, 43
call _operation
pop rbx
mov dword[_stack + rbx], eax
jmp _loop_6
_end_loop_6:
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 4
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
mov dword[_stack + rbx], eax
_loop_7:
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rbx, 16
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
pop r11
pop r10
mov r12, 60
call _operation
cmp rax, 0
je _end_loop_7
mov rbx, 0
add rbx, r15
push rbx
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
push rax
mov rax, rbx
_deref 1
mov rbx, rax
pop rax
mov rcx, 4
mul rcx
add rbx, rax
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 60
add rbx, r15
push rbx
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
push rax
mov rax, rbx
_deref 1
mov rbx, rax
pop rax
mov rcx, 4
mul rcx
add rbx, rax
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
pop rbx
mov dword[_stack + rbx], eax
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 72
add rbx, r15
mov rax, rbx
push r15
add r15, 76
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rax, 1
push rax
pop r11
pop r10
mov r12, 43
call _operation
pop rbx
mov dword[_stack + rbx], eax
jmp _loop_7
_end_loop_7:
ret
merge_sort:
mov rbx, 4
add rbx, r15
mov rax, rbx
push r15
add r15, 12
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rbx, 8
add rbx, r15
mov rax, rbx
push r15
add r15, 12
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
pop r11
pop r10
mov r12, 60
call _operation
cmp rax, 0
je _end_condition_9
mov rbx, 12
add rbx, r15
mov rax, rbx
push r15
add r15, 16
cmp rax, r15
jg _invalid_address
pop r15
push rax
mov rbx, 4
add rbx, r15
mov rax, rbx
push r15
add r15, 16
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rbx, 8
add rbx, r15
mov rax, rbx
push r15
add r15, 16
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
pop r11
pop r10
mov r12, 43
call _operation
push rax
mov rax, 2
push rax
pop r11
pop r10
mov r12, 47
call _operation
pop rbx
mov dword[_stack + rbx], eax
mov rbx, 0
add rbx, r15
mov rax, rbx
push r15
add r15, 16
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
mov dword[_stack + r15 + 16 + 0], eax
mov rbx, 4
add rbx, r15
mov rax, rbx
push r15
add r15, 16
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
mov dword[_stack + r15 + 16 + 4], eax
mov rbx, 12
add rbx, r15
mov rax, rbx
push r15
add r15, 16
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
mov dword[_stack + r15 + 16 + 8], eax
push r15
add r15, 16
call merge_sort
pop r15
mov rbx, 0
add rbx, r15
mov rax, rbx
push r15
add r15, 16
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
mov dword[_stack + r15 + 16 + 0], eax
mov rbx, 12
add rbx, r15
mov rax, rbx
push r15
add r15, 16
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rax, 1
push rax
pop r11
pop r10
mov r12, 43
call _operation
mov dword[_stack + r15 + 16 + 4], eax
mov rbx, 8
add rbx, r15
mov rax, rbx
push r15
add r15, 16
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
mov dword[_stack + r15 + 16 + 8], eax
push r15
add r15, 16
call merge_sort
pop r15
mov rbx, 0
add rbx, r15
mov rax, rbx
push r15
add r15, 16
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
mov dword[_stack + r15 + 16 + 0], eax
mov rbx, 4
add rbx, r15
mov rax, rbx
push r15
add r15, 16
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
mov dword[_stack + r15 + 16 + 4], eax
mov rbx, 12
add rbx, r15
mov rax, rbx
push r15
add r15, 16
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
push rax
mov rax, 1
push rax
pop r11
pop r10
mov r12, 45
call _operation
mov dword[_stack + r15 + 16 + 8], eax
mov rbx, 8
add rbx, r15
mov rax, rbx
push r15
add r15, 16
cmp rax, r15
jg _invalid_address
pop r15
movsx rax, dword[_stack + rax]
mov dword[_stack + r15 + 16 + 12], eax
push r15
add r15, 16
call merge
pop r15
jmp _real_end_condition_9
_end_condition_9:
_real_end_condition_9:
ret
