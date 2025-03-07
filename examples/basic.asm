.global _start
.section .text
test:
mov rdi, 5
mov rsi, 4
mov rdx, 3
mov rcx, 2
mov r8, 1
sub rcx, r8
add rdx, rcx
sub rsi, rdx
add rdi, rsi
ret
_start:
call test
mov rax, 60
syscall
