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
    _chiffres: db '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
    _newline: db 10

section .bss
     _stack: resb 300000