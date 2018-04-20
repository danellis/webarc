.text 
.code 32 

    b reset // Reset
    b . // Undefined instruction
    b . // Software interrupt
    b . // Prefetch abort
    b . // Data abort
    b . // Reserved
    b . // IRQ
    b . // FIQ

reset:
    mov r0, #1024
    subs r0, r0, #1
    bne reset
    swi 1
