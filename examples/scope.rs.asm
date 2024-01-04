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
            Addiu sp, sp, -4 (0xfffc)# allocate '>2#4!1_a'
            Ori t0, zero, 1 (0x0001)# '>2#4!1_a = 1 + 2', op +, integer constant 1, 16 bit constant
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
            Sw t0, -4[fp] (0xfffc)# store '>2#4!1_a' at offset -4
            Addiu sp, sp, -4 (0xfffc)# allocate '>2#4!2_a'
            Ori t0, zero, 2 (0x0002)# '>2#4!2_a = 2 + >2#4!1_a', op +, integer constant 2, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, -4[fp] (0xfffc)# load '>2#4!1_a' at offset -4
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
            Sw t0, -8[fp] (0xfff8)# store '>2#4!2_a' at offset -8
            Ori t0, zero, 1 (0x0001)# if true {
     >2#4!2_a = >2#4!2_a - 1;
     let mut >3#5!1_a : i32 = 0;
     >3#5!1_a = >3#5!1_a + 1
} else {
     >2#4!2_a = >2#4!2_a - 1
}, condition, boolean constant true, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Beq zero, t0, 48 (0x0030)
            Lw t0, -8[fp] (0xfff8)# then arm, '>2#4!2_a = >2#4!2_a - 1', op -, load '>2#4!2_a' at offset -8
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
            Sw t0, -8[fp] (0xfff8)# store '>2#4!2_a' at offset -8
            Addiu sp, sp, -4 (0xfffc)# allocate '>3#5!1_a'
            Ori t0, zero, 0 (0x0000)# '>3#5!1_a = 0', integer constant 0, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# pop t0
            Addiu sp, sp, 4 (0x0004)
            Sw t0, -12[fp] (0xfff4)# store '>3#5!1_a' at offset -12
            Lw t0, -12[fp] (0xfff4)# '>3#5!1_a = >3#5!1_a + 1', op +, load '>3#5!1_a' at offset -12
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
            Sw t0, -12[fp] (0xfff4)# store '>3#5!1_a' at offset -12
            Ori t0, zero, 0 (0x0000)# exit block semi, () return value, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit block, pop block result, pop t0
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, 4 (0x0004)# exit block, remove locals
            Addiu sp, sp, -4 (0xfffc)# exit block, push back block result, push t0
            Sw t0, 0[sp] (0x0000)
            Beq zero, zero, 19 (0x0013)
            Lw t0, -8[fp] (0xfff8)# else arm, '>2#4!2_a = >2#4!2_a - 1', op -, load '>2#4!2_a' at offset -8
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
            Sw t0, -8[fp] (0xfff8)# store '>2#4!2_a' at offset -8
            Ori t0, zero, 0 (0x0000)# exit block semi, () return value, 16 bit constant
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# pop non-last expression, pop t0
            Addiu sp, sp, 4 (0x0004)
            Lw t0, -8[fp] (0xfff8)# >2#4!2_a, load '>2#4!2_a' at offset -8
            Addiu sp, sp, -4 (0xfffc)# push t0
            Sw t0, 0[sp] (0x0000)
            Lw t0, 0[sp] (0x0000)# exit block, pop block result, pop t0
            Addiu sp, sp, 4 (0x0004)
            Addiu sp, sp, 8 (0x0008)# exit block, remove locals
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
