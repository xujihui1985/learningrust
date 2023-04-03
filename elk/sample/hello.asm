global _start
section .text

_start: mov rdi, 1
        mov rsi, msg
        mov rdx, 9
        mov rax, 1
        syscall

        xor rdi, rdi
        mov rax, 60
        syscall

        section .data


msg: db "hi there", 10