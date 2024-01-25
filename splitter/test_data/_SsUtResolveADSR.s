.set noat      /* allow manual use of $at */
.set noreorder /* don't insert nops after branches */

glabel _SsUtResolveADSR
/* FDBC 8001F5BC 00808230 */  andi       $v0, $a0, 0x8000
/* FDC0 8001F5C0 0A00C2A4 */  sh         $v0, 0xA($a2)
/* FDC4 8001F5C4 0080A230 */  andi       $v0, $a1, 0x8000
/* FDC8 8001F5C8 0C00C2A4 */  sh         $v0, 0xC($a2)
/* FDCC 8001F5CC 0040A230 */  andi       $v0, $a1, 0x4000
/* FDD0 8001F5D0 1000C2A4 */  sh         $v0, 0x10($a2)
/* FDD4 8001F5D4 2000A230 */  andi       $v0, $a1, 0x20
/* FDD8 8001F5D8 FFFF8330 */  andi       $v1, $a0, 0xFFFF
/* FDDC 8001F5DC 0E00C2A4 */  sh         $v0, 0xE($a2)
/* FDE0 8001F5E0 02120300 */  srl        $v0, $v1, 8
/* FDE4 8001F5E4 7F004230 */  andi       $v0, $v0, 0x7F
/* FDE8 8001F5E8 02190300 */  srl        $v1, $v1, 4
/* FDEC 8001F5EC 0F006330 */  andi       $v1, $v1, 0xF
/* FDF0 8001F5F0 0F008430 */  andi       $a0, $a0, 0xF
/* FDF4 8001F5F4 0000C2A4 */  sh         $v0, 0x0($a2)
/* FDF8 8001F5F8 82110500 */  srl        $v0, $a1, 6
/* FDFC 8001F5FC 7F004230 */  andi       $v0, $v0, 0x7F
/* FE00 8001F600 1F00A530 */  andi       $a1, $a1, 0x1F
/* FE04 8001F604 0200C3A4 */  sh         $v1, 0x2($a2)
/* FE08 8001F608 0400C4A4 */  sh         $a0, 0x4($a2)
/* FE0C 8001F60C 0600C2A4 */  sh         $v0, 0x6($a2)
/* FE10 8001F610 0800E003 */  jr         $ra
/* FE14 8001F614 0800C5A4 */   sh        $a1, 0x8($a2)
.size _SsUtResolveADSR, . - _SsUtResolveADSR
