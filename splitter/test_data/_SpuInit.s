.set noat      /* allow manual use of $at */
.set noreorder /* don't insert nops after branches */

glabel _SpuInit
/* 00000000 27BDFFE8 */ addiu       $sp, $sp, -0x18
/* 00000004 AFB00010 */ sw          $s0, 0x10($sp)
/* 00000008 AFBF0014 */ sw          $ra, 0x14($sp)
/* 0000000C 0C000000 */ jal         ResetCallback
/* 00000010 00808021 */ addu        $s0, $a0, $zero
/* 00000014 0C000000 */ jal         _spu_init
/* 00000018 02002021 */ addu        $a0, $s0, $zero
/* 0000001C 16000008 */ bnez        $s0, .L00000040
/* 00000020 3404C000 */ ori         $a0, $zero, 0xC000
/* 00000024 34030017 */ ori         $v1, $zero, 0x17
/* 00000028 3C020000 */ lui         $v0, %hi(_spu_voice_centerNote+46)
/* 0000002C 24420000 */ addiu       $v0, $v0, %lo(_spu_voice_centerNote+46)
.L00000030:
/* 00000030 A4440000 */ sh          $a0, 0x0($v0)
/* 00000034 2463FFFF */ addiu       $v1, $v1, -0x1
/* 00000038 0461FFFD */ bgez        $v1, .L00000030
/* 0000003C 2442FFFE */ addiu       $v0, $v0, -0x2
.L00000040:
/* 00000040 0C000000 */ jal         SpuStart
/* 00000044 00000000 */ nop
/* 00000048 340400D1 */ ori         $a0, $zero, 0xD1
/* 0000004C 3C050000 */ lui         $a1, %hi(_spu_rev_startaddr)
/* 00000050 8CA50000 */ lw          $a1, %lo(_spu_rev_startaddr)($a1)
/* 00000054 3C010000 */ lui         $at, %hi(_spu_rev_flag)
/* 00000058 AC200000 */ sw          $zero, %lo(_spu_rev_flag)($at)
/* 0000005C 3C010000 */ lui         $at, %hi(_spu_rev_reserve_wa)
/* 00000060 AC200000 */ sw          $zero, %lo(_spu_rev_reserve_wa)($at)
/* 00000064 3C010000 */ lui         $at, %hi(_spu_rev_attr+4)
/* 00000068 AC200000 */ sw          $zero, %lo(_spu_rev_attr+4)($at)
/* 0000006C 3C010000 */ lui         $at, %hi(_spu_rev_attr+8)
/* 00000070 A4200000 */ sh          $zero, %lo(_spu_rev_attr+8)($at)
/* 00000074 3C010000 */ lui         $at, %hi(_spu_rev_attr+10)
/* 00000078 A4200000 */ sh          $zero, %lo(_spu_rev_attr+10)($at)
/* 0000007C 3C010000 */ lui         $at, %hi(_spu_rev_attr+12)
/* 00000080 AC200000 */ sw          $zero, %lo(_spu_rev_attr+12)($at)
/* 00000084 3C010000 */ lui         $at, %hi(_spu_rev_attr+16)
/* 00000088 AC200000 */ sw          $zero, %lo(_spu_rev_attr+16)($at)
/* 0000008C 3C010000 */ lui         $at, %hi(_spu_rev_offsetaddr)
/* 00000090 AC250000 */ sw          $a1, %lo(_spu_rev_offsetaddr)($at)
/* 00000094 0C000000 */ jal         _spu_FsetRXX
/* 00000098 00003021 */ addu        $a2, $zero, $zero
/* 0000009C 3C010000 */ lui         $at, %hi(_spu_trans_mode)
/* 000000A0 AC200000 */ sw          $zero, %lo(_spu_trans_mode)($at)
/* 000000A4 3C010000 */ lui         $at, %hi(_spu_transMode)
/* 000000A8 AC200000 */ sw          $zero, %lo(_spu_transMode)($at)
/* 000000AC 3C010000 */ lui         $at, %hi(_spu_keystat)
/* 000000B0 AC200000 */ sw          $zero, %lo(_spu_keystat)($at)
/* 000000B4 8FBF0014 */ lw          $ra, 0x14($sp)
/* 000000B8 8FB00010 */ lw          $s0, 0x10($sp)
/* 000000BC 27BD0018 */ addiu       $sp, $sp, 0x18
/* 000000C0 03E00008 */ jr          $ra
/* 000000C4 00000000 */ nop
.size _SpuInit, . - _SpuInit
