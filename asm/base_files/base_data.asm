section .data

    _multip: db '*'
    _soustr: db '-'
    _addi: db '+'
    _divis: db '/'
    _modulo: db '%'
    _equal: db '='
    _inf: db '<'
    _sup: db '>'
    _and: db '&'
    _or: db '|' 
    _inf_equal: db ';'
    _sup_equal: db '?'
    _diff: db '@'
    _ascii db " !", 34, "#$%&", 39, "()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"
    _newline: db 10
    _seg_fault_msg: db 'Segmentation fault', 10
    _size_seg_fault_msg: equ $-_seg_fault_msg

section .bss
     _stack: resb 300000
