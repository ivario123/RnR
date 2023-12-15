            Add fp, zero, sp# move sp to frame pointer
            Blez zero, zero, main_GLOBAL_SCOPE# call main
            Halt# Main exit
c_a_GLOBAL_SCOPEAddiu sp, sp, -4 (0xfffc)# enter frame 'fn c', push ra
            Sw ra, 0[sp] (0x0000)
            Addiu sp, sp, -4 (0xfffc)# push fp
            Sw fp, 0[sp] (0x0000)
            Add fp, zero, sp
            Ori t0, zero, 0 (0x0000)# false, boolean constant false, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit frame 'fn c', pop return value, pop t0
            Addiu sp, sp, 4 (0x0004)
            Add sp, zero, fp
            Lw fp, 0[sp] (0x0000)# pop fp
            Addiu sp, sp, 4 (0x0004)
            Lw ra, 0[sp] (0x0000)# pop ra
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, -4 (0xfffc)# push back return value, push t0
            Sw t0, 0[sp] (0x0000)
            Jr ra
b_a_GLOBAL_SCOPEAddiu sp, sp, -4 (0xfffc)# enter frame 'fn bj: i32', push ra
            Sw ra, 0[sp] (0x0000)
            Addiu sp, sp, -4 (0xfffc)# push fp
            Sw fp, 0[sp] (0x0000)
            Add fp, zero, sp
            Lw t0, 8[fp] (0x0008)# a(j,c()), arg j, load 'j' at offset 8
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Blez zero, zero, c_a_GLOBAL_SCOPE# arg c(), call c
            Blez zero, zero, a_GLOBAL_SCOPE# call a
            Lw t0, 0[sp] (0x0000)# pop result, pop t0
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, 8 (0x0008)# remove arguments
            Addiu sp, sp, -4 (0xfffc)# push back result, push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit frame 'fn b', pop return value, pop t0
            Addiu sp, sp, 4 (0x0004)
            Add sp, zero, fp
            Lw fp, 0[sp] (0x0000)# pop fp
            Addiu sp, sp, 4 (0x0004)
            Lw ra, 0[sp] (0x0000)# pop ra
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, -4 (0xfffc)# push back return value, push t0
            Sw t0, 0[sp] (0x0000)
            Jr ra
a_GLOBAL_SCOPEAddiu sp, sp, -4 (0xfffc)# enter frame 'fn ai: i32,bo: bool', push ra
            Sw ra, 0[sp] (0x0000)
            Addiu sp, sp, -4 (0xfffc)# push fp
            Sw fp, 0[sp] (0x0000)
            Add fp, zero, sp
            Ori t0, zero, 1 (0x0001)# b(1 + i), arg 1 + i, op +, integer constant 1, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 12[fp] (0x000c)# load 'i' at offset 12
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t1, 0[sp] (0x0000)# pop t1
            Addiu sp, sp, 4 (0x0004)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Addu t0, t0, t1
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Blez zero, zero, b_a_GLOBAL_SCOPE# call b
            Lw t0, 0[sp] (0x0000)# pop result, pop t0
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, 4 (0x0004)# remove arguments
            Addiu sp, sp, -4 (0xfffc)# push back result, push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# pop non-last expression, pop t0
            Addiu sp, sp, 4 (0x0004)
            Lw t0, 12[fp] (0x000c)# a(i,bo), arg i, load 'i' at offset 12
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 8[fp] (0x0008)# arg bo, load 'bo' at offset 8
            Addiu sp, sp, -4 (0xfffc)# push t0
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
