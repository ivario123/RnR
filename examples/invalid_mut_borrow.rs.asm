            Add fp, zero, sp# move sp to frame pointer
            Blez zero, zero, main_GLOBAL_SCOPE# call main
            Halt# Main exit
main_GLOBAL_SCOPEAddiu sp, sp, -4 (0xfffc)# enter frame 'fn main', push ra
            Sw ra, 0[sp] (0x0000)
            Addiu sp, sp, -4 (0xfffc)# push fp
            Sw fp, 0[sp] (0x0000)
            Add fp, zero, sp
            Addiu sp, sp, -4 (0xfffc)# allocate '_a'
            Ori t0, zero, 0 (0x0000)# '_a = 0', integer constant 0, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Sw t0, -4[fp] (0xfffc)# store '_a' at offset -4
            Ori t0, zero, 0 (0x0000)# exit block semi, () return value, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit block, pop block result, pop t0
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, 4 (0x0004)# exit block, remove locals
            Addiu sp, sp, -4 (0xfffc)# exit block, push back block result, push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit frame 'fn main', pop return value, pop t0
            Addiu sp, sp, 4 (0x0004)
            Add sp, zero, fp
            Lw fp, 0[sp] (0x0000)# pop fp
            Addiu sp, sp, 4 (0x0004)
            Lw ra, 0[sp] (0x0000)# pop ra
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, -4 (0xfffc)# push back return value, push t0
            Sw t0, 0[sp] (0x0000)
            Jr ra
            Halt
