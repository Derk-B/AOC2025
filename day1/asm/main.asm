section .data
    filename db "input.txt", 0    ; File to read from
    error_msg db "File error!", 10
    error_len equ $ - error_msg
    newline db 10                 ; Newline character

section .bss
    fd resq 1                     ; File descriptor
    buffer resb 5                 ; Buffer for reading (4 chars + null)
    bytes_read resq 1             ; Store bytes read count

section .text
    extern print_decimal
    global _start

_start:
    ; Open the file
    mov rax, 2
    mov rdi, filename 
    mov rsi, 0                    ; O_RDONLY
    syscall
    
    ; Check if open succeeded
    cmp rax, 0
    jl exit_error                 ; If negative, error occurred
    
    ; Store file descriptor
    mov [fd], rax

    ; Init dial at 50
    push 50

read_loop:
    ; Read from file (max 4 characters)
    mov rax, 0                    ; syscall: read
    mov rdi, [fd]                 ; file descriptor
    mov rsi, buffer               ; buffer
    mov rdx, 4                    ; max bytes to read
    syscall
    
    ; Check if read succeeded
    cmp rax, 0
    jl close_and_exit_error       ; If negative, error occurred
    
    ; Check for EOF (rax = 0)
    cmp rax, 0
    je close_file                 ; If 0 bytes read, we're at EOF
    
    ; ; Store bytes read
    ; mov [bytes_read], rax
    
    ; ; Print the buffer contents
    ; mov rax, 1 
    ; mov rdi, 1 
    ; mov rsi, buffer
    ; mov rdx, [bytes_read]
    ; syscall
    
    mov al, [buffer] ; Load first byte (char) from buffer

    cmp al, 'L'
    je found_L

    cmp al, 'R'
    je found_R

found_L:
    sub qword [rsp], 1
    jmp continue_reading

found_R:
    add qword [rsp], 1

continue_reading:
    jmp read_loop

close_file:
    ; Close the file
    mov rax, 3
    mov rdi, [fd]
    syscall

    ; Print the result (part 1)
    mov rax, 1
    mov rdi, 1
    mov rsi, rsp
    mov rdx, 8
    syscall

    mov rax, [rsp]
    call print_decimal

    ; Exit successfully
    mov rax, 60 
    xor rdi, rdi 
    syscall

close_and_exit_error:
    ; Close file before exiting with error
    mov rax, 3
    mov rdi, [fd]
    syscall

exit_error:
    ; Print error message
    mov rax, 1
    mov rdi, 1
    mov rsi, error_msg
    mov rdx, error_len
    syscall
    
    ; Exit with error code
    mov rax, 60
    mov rdi, 1
    syscall
