section .text
    global print_decimal ; Export the function that prints value in decimal numbers

print_decimal:
    ; Save the registers
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push rbp
    mov rbp, rsp

    sub rsp, 32  ; reserve a buffer of 32 characters, should be enough to print any int64 value as string, max value for int64 only takes around 20 chars
    mov rsi, rsp

    test rax, rax
    jnz not_zero

    mov byte [rsi], '0'
    mov rcx, 1
    jmp print_buffer

not_zero:
    mov rbx, rax
    mov rcx, 0

    cmp rax, 0
    jge pos_number

    neg rax ; number is negative, make it positive and remember the sign
    mov byte [rsi], '-'
    inc rsi
    mov rcx, 1


pos_number:
    push 10             ; Add newline after the number
    add rcx, 4

    mov rdi, 10         ; divisor for base 10

convert_loop:
    xor rdx, rdx        ; clear rdx for division
    div rdi             ; rax = quotient, rdx = remainer
    add dl, '0'         ; convert remainder to ASCII
    push dx             ; Store digit on stack
    inc rcx             ; inc digit count

    test rax, rax       ; check if quotient is 0, if so, then our last division was done on a number <10 (e.g. 4 / 10 = 0 (quotient), 4 (remainder))
    jnz convert_loop

    mov rdi, rsi        ; rdi points where to store digits, rsi was pointing to the memory location after the '-' sign on the buffer (in case of negative number)

pop_digits:
    pop dx
    mov [rdi], dl
    inc rdi
    loop pop_digits

    ; Calculate the buffer length
    mov rcx, rdi
    sub rcx, rsi

    cmp rbx, 0
    jge print_buffer

    ; If negative, then push the buf pointer back by 1 to include the minus sign
    dec rsi
    inc rcx

print_buffer:  
    mov rax, 1
    mov rdi, 1
    ; rsi already points to the buffer
    ; rcx already has the length
    mov rdx, rcx
    syscall

    ; Clean up stack and restore registers
    mov rsp, rbp
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    ret