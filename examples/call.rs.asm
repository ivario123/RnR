            Add fp, zero, sp# move sp to frame pointer
            Blez zero, zero, main_GLOBAL_SCOPE# call main
            Halt# Main exit
f_GLOBAL_SCOPEAddiu sp, sp, -4 (0xfffc)# enter frame 'fn fa: i32', push ra
            Sw ra, 0[sp] (0x0000)
            Addiu sp, sp, -4 (0xfffc)# push fp
            Sw fp, 0[sp] (0x0000)
            Add fp, zero, sp
            Addiu sp, sp, -4 (0xfffc)# allocate '_b'
            Lw t0, 8[fp] (0x0008)# '_b = a', load 'a' at offset 8
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Sw t0, -4[fp] (0xfffc)# store '_b' at offset -4
            Ori t0, zero, 0 (0x0000)# exit block semi, () return value, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit block, pop block result, pop t0
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, 4 (0x0004)# exit block, remove locals
            Addiu sp, sp, -4 (0xfffc)# exit block, push back block result, push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit frame 'fn f', pop return value, pop t0
            Addiu sp, sp, 4 (0x0004)
            Add sp, zero, fp
            Lw fp, 0[sp] (0x0000)# pop fp
            Addiu sp, sp, 4 (0x0004)
            Lw ra, 0[sp] (0x0000)# pop ra
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, -4 (0xfffc)# push back return value, push t0
            Sw t0, 0[sp] (0x0000)
            Jr ra
main_GLOBAL_SCOPEAddiu sp, sp, -4 (0xfffc)# enter frame 'fn main', push ra
            Sw ra, 0[sp] (0x0000)
            Addiu sp, sp, -4 (0xfffc)# push fp
            Sw fp, 0[sp] (0x0000)
            Add fp, zero, sp
            Ori t0, zero, 7 (0x0007)# f(7), arg 7, integer constant 7, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Blez zero, zero, f_GLOBAL_SCOPE# call f
            Lw t0, 0[sp] (0x0000)# pop result, pop t0
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, 4 (0x0004)# remove arguments
            Addiu sp, sp, -4 (0xfffc)# push back result, push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit block semi, pop last result, pop t0
            Addiu sp, sp, 4 (0x0004)
            Ori t0, zero, 0 (0x0000)# exit block semi, () return value, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
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
