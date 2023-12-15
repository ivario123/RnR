            Add fp, zero, sp# move sp to frame pointer
            Blez zero, zero, main_GLOBAL_SCOPE# call main
            Halt# Main exit
a_GLOBAL_SCOPEAddiu sp, sp, -4 (0xfffc)# enter frame 'fn a', push ra
            Sw ra, 0[sp] (0x0000)
            Addiu sp, sp, -4 (0xfffc)# push fp
            Sw fp, 0[sp] (0x0000)
            Add fp, zero, sp
            Ori t0, zero, 2 (0x0002)# 2, integer constant 2, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit frame 'fn a', pop return value, pop t0
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
            Addiu sp, sp, -4 (0xfffc)# allocate 'a'
            Ori t0, zero, 5 (0x0005)# 'a = 5 + 2', op +, integer constant 5, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Ori t0, zero, 2 (0x0002)# integer constant 2, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t1, 0[sp] (0x0000)# pop t1
            Addiu sp, sp, 4 (0x0004)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Addu t0, t0, t1
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Sw t0, -4[fp] (0xfffc)# store 'a' at offset -4
            Lw t0, -4[fp] (0xfffc)# 'a = a + 1', op +, load 'a' at offset -4
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Ori t0, zero, 1 (0x0001)# integer constant 1, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t1, 0[sp] (0x0000)# pop t1
            Addiu sp, sp, 4 (0x0004)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Addu t0, t0, t1
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Sw t0, -4[fp] (0xfffc)# store 'a' at offset -4
            Lw t0, -4[fp] (0xfffc)# a, load 'a' at offset -4
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
    Jr ra
            Halt
sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Blez zero, zero, a_GLOBAL_SCOPE# call a
            Lw t0, 0[sp] (0x0000)# pop result, pop t0
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, 8 (0x0008)# remove arguments
            Addiu sp, sp, -4 (0xfffc)# push back result, push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit frame 'fn a', pop return value, pop t0
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
            Ori t0, zero, 1 (0x0001)# a(1,false), arg 1, integer constant 1, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Ori t0, zero, 0 (0x0000)# arg false, boolean constant false, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Blez zero, zero, a_GLOBAL_SCOPE# call a
            Lw t0, 0[sp] (0x0000)# pop result, pop t0
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, 8 (0x0008)# remove arguments
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
