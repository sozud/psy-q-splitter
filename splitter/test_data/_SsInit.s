.set noat      /* allow manual use of $at */
.set noreorder /* don't insert nops after branches */

glabel _SsInit
/* 00000000 27BDFFE8 */ addiu       $sp, $sp, -0x18
/* 00000004 AFB00010 */ sw          $s0, 0x10($sp)
/* 00000008 AFBF0014 */ sw          $ra, 0x14($sp)
/* 0000000C 0C000000 */ jal         func_80000000
/* 00000010 00808021 */ addu        $s0, $a0, $zero
/* 00000014 16000005 */ bnez        $s0, .L0000002C
/* 00000018 00000000 */ nop
/* 0000001C 0C000000 */ jal         SpuInit
/* 00000020 00000000 */ nop
/* 00000024 08000000 */ j           .L00000038
/* 00000028 3C061F80 */ lui         $a2, 0x1F80
.L0000002C:
/* 0000002C 0C000000 */ jal         SpuInitHot
/* 00000030 00000000 */ nop
/* 00000034 3C061F80 */ lui         $a2, 0x1F80
.L00000038:
/* 00000038 34C61C00 */ ori         $a2, $a2, 0x1C00
/* 0000003C 00002021 */ addu        $a0, $zero, $zero
/* 00000040 3C070000 */ lui         $a3, %hi(D_00000000)
/* 00000044 24E70000 */ addiu       $a3, $a3, %lo(D_00000000)
.L00000048:
/* 00000048 00002821 */ addu        $a1, $zero, $zero
/* 0000004C 00E01821 */ addu        $v1, $a3, $zero
.L00000050:
/* 00000050 94620000 */ lhu         $v0, 0x0($v1)
/* 00000054 24630002 */ addiu       $v1, $v1, 0x2
/* 00000058 24A50001 */ addiu       $a1, $a1, 0x1
/* 0000005C A4C20000 */ sh          $v0, 0x0($a2)
/* 00000060 28A20008 */ slti        $v0, $a1, 0x8
/* 00000064 1440FFFA */ bnez        $v0, .L00000050
/* 00000068 24C60002 */ addiu       $a2, $a2, 0x2
/* 0000006C 24840001 */ addiu       $a0, $a0, 0x1
/* 00000070 28820018 */ slti        $v0, $a0, 0x18
/* 00000074 1440FFF4 */ bnez        $v0, .L00000048
/* 00000078 00000000 */ nop
/* 0000007C 3C061F80 */ lui         $a2, 0x1F80
/* 00000080 34C61D80 */ ori         $a2, $a2, 0x1D80
/* 00000084 00002021 */ addu        $a0, $zero, $zero
/* 00000088 3C030000 */ lui         $v1, %hi(D_00000010)
/* 0000008C 24630000 */ addiu       $v1, $v1, %lo(D_00000010)
.L00000090:
/* 00000090 94620000 */ lhu         $v0, 0x0($v1)
/* 00000094 24630002 */ addiu       $v1, $v1, 0x2
/* 00000098 24840001 */ addiu       $a0, $a0, 0x1
/* 0000009C A4C20000 */ sh          $v0, 0x0($a2)
/* 000000A0 28820010 */ slti        $v0, $a0, 0x10
/* 000000A4 1440FFFA */ bnez        $v0, .L00000090
/* 000000A8 24C60002 */ addiu       $a2, $a2, 0x2
/* 000000AC 0C000000 */ jal         SpuVmInit
/* 000000B0 34040018 */ ori         $a0, $zero, 0x18
/* 000000B4 00002821 */ addu        $a1, $zero, $zero
/* 000000B8 3C030000 */ lui         $v1, 0x0
/* 000000BC 24630000 */ addiu       $v1, $v1, 0x0
.L000000C0:
/* 000000C0 3404000F */ ori         $a0, $zero, 0xF
/* 000000C4 2462003C */ addiu       $v0, $v1, 0x3C
.L000000C8:
/* 000000C8 AC400000 */ sw          $zero, 0x0($v0)
/* 000000CC 2484FFFF */ addiu       $a0, $a0, -0x1
/* 000000D0 0481FFFD */ bgez        $a0, .L000000C8
/* 000000D4 2442FFFC */ addiu       $v0, $v0, -0x4
/* 000000D8 24A50001 */ addiu       $a1, $a1, 0x1
/* 000000DC 28A20020 */ slti        $v0, $a1, 0x20
/* 000000E0 1440FFF7 */ bnez        $v0, .L000000C0
/* 000000E4 24630040 */ addiu       $v1, $v1, 0x40
/* 000000E8 3402003C */ ori         $v0, $zero, 0x3C
/* 000000EC 3C010000 */ lui         $at, 0x0
/* 000000F0 AC220000 */ sw          $v0, 0x0($at)
/* 000000F4 2402FFFF */ addiu       $v0, $zero, -0x1
/* 000000F8 3C010000 */ lui         $at, 0x0
/* 000000FC AC200000 */ sw          $zero, 0x0($at)
/* 00000100 3C010000 */ lui         $at, %hi(_snd_use_vsync_cb)
/* 00000104 AC200000 */ sw          $zero, %lo(_snd_use_vsync_cb)($at)
/* 00000108 3C010000 */ lui         $at, %hi(_snd_use_interrupt_id)
/* 0000010C AC220000 */ sw          $v0, %lo(_snd_use_interrupt_id)($at)
/* 00000110 3C010000 */ lui         $at, %hi(_snd_use_event)
/* 00000114 AC200000 */ sw          $zero, %lo(_snd_use_event)($at)
/* 00000118 3C010000 */ lui         $at, %hi(_snd_1per2)
/* 0000011C AC200000 */ sw          $zero, %lo(_snd_1per2)($at)
/* 00000120 3C010000 */ lui         $at, %hi(_snd_vsync_cb)
/* 00000124 AC200000 */ sw          $zero, %lo(_snd_vsync_cb)($at)
/* 00000128 0C000000 */ jal         GetVideoMode
/* 0000012C 00000000 */ nop
/* 00000130 3C010000 */ lui         $at, %hi(_snd_video_mode)
/* 00000134 AC220000 */ sw          $v0, %lo(_snd_video_mode)($at)
/* 00000138 3C010000 */ lui         $at, 0x0
/* 0000013C AC200000 */ sw          $zero, 0x0($at)
/* 00000140 8FBF0014 */ lw          $ra, 0x14($sp)
/* 00000144 8FB00010 */ lw          $s0, 0x10($sp)
/* 00000148 27BD0018 */ addiu       $sp, $sp, 0x18
/* 0000014C 03E00008 */ jr          $ra
/* 00000150 00000000 */ nop
.size _SsInit, . - _SsInit
