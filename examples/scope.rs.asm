            Add fp, zero, sp# move sp to frame pointer
            Blez zero, zero, main_GLOBAL_SCOPE# call main
            Halt# Main exit
other_tmp_GLOBAL_SCOPEAddiu sp, sp, -4 (0xfffc)# enter frame 'fn other_tmp', push ra
            Sw ra, 0[sp] (0x0000)
            Addiu sp, sp, -4 (0xfffc)# push fp
            Sw fp, 0[sp] (0x0000)
            Add fp, zero, sp
            Ori t0, zero, 0 (0x0000)# empty block, () return value, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit frame 'fn other_tmp', pop return value, pop t0
            Addiu sp, sp, 4 (0x0004)
            Add sp, zero, fp
            Lw fp, 0[sp] (0x0000)# pop fp
            Addiu sp, sp, 4 (0x0004)
            Lw ra, 0[sp] (0x0000)# pop ra
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, -4 (0xfffc)# push back return value, push t0
            Sw t0, 0[sp] (0x0000)
            Jr ra
a_main_GLOBAL_SCOPEAddiu sp, sp, -4 (0xfffc)# enter frame 'fn a', push ra
            Sw ra, 0[sp] (0x0000)
            Addiu sp, sp, -4 (0xfffc)# push fp
            Sw fp, 0[sp] (0x0000)
            Add fp, zero, sp
            Addiu sp, sp, -4 (0xfffc)# allocate 'a'
            Ori t0, zero, 1 (0x0001)# 'a = 1 + 2', op +, integer constant 1, 16 bit constant
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
            Ori t0, zero, 2 (0x0002)# 'a = 2 + a', op +, integer constant 2, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, -4[fp] (0xfffc)# load 'a' at offset -4
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
            Ori t0, zero, 1 (0x0001)# if true {
     a = a - 1;
     let mut a : i32 = 0;
     a = a + 1
} else {
     a = a - 1
}, condition, boolean constant true, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Beq zero, t0, 48 (0x0030)
            Lw t0, -4[fp] (0xfffc)# then arm, 'a = a - 1', op -, load 'a' at offset -4
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Ori t0, zero, 1 (0x0001)# integer constant 1, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t1, 0[sp] (0x0000)# pop t1
            Addiu sp, sp, 4 (0x0004)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Subu t0, t0, t1
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Sw t0, -4[fp] (0xfffc)# store 'a' at offset -4
            Addiu sp, sp, -4 (0xfffc)# allocate 'a'
            Ori t0, zero, 0 (0x0000)# 'a = 0', integer constant 0, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Sw t0, -8[fp] (0xfff8)# store 'a' at offset -8
            Lw t0, -8[fp] (0xfff8)# 'a = a + 1', op +, load 'a' at offset -8
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
            Sw t0, -8[fp] (0xfff8)# store 'a' at offset -8
            Ori t0, zero, 0 (0x0000)# exit block semi, () return value, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit block, pop block result, pop t0
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, 4 (0x0004)# exit block, remove locals
            Addiu sp, sp, -4 (0xfffc)# exit block, push back block result, push t0
            Sw t0, 0[sp] (0x0000)
            Beq zero, zero, 19 (0x0013)
            Lw t0, -4[fp] (0xfffc)# else arm, 'a = a - 1', op -, load 'a' at offset -4
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Ori t0, zero, 1 (0x0001)# integer constant 1, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t1, 0[sp] (0x0000)# pop t1
            Addiu sp, sp, 4 (0x0004)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Subu t0, t0, t1
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Sw t0, -4[fp] (0xfffc)# store 'a' at offset -4
            Ori t0, zero, 0 (0x0000)# exit block semi, () return value, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# pop non-last expression, pop t0
            Addiu sp, sp, 4 (0x0004)
            Lw t0, -4[fp] (0xfffc)# a, load 'a' at offset -4
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit block, pop block result, pop t0
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, 4 (0x0004)# exit block, remove locals
            Addiu sp, sp, -4 (0xfffc)# exit block, push back block result, push t0
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
