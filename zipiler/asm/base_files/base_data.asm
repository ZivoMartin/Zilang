section .data
    _ascii db 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, " !", 34, "#$%&", 39, "()*+,-./0123456789:", 59, "<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"
    _newline: db 10
    _seg_fault_msg: db 'Segmentation fault', 10
    _size_seg_fault_msg: equ $-_seg_fault_msg

section .bss
     _stack: resb 300000
     _heap: resb 300000
     _progmem: resb 3000