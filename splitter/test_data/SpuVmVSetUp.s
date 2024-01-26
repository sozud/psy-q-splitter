.set noat      /* allow manual use of $at */
.set noreorder /* don't insert nops after branches */

glabel SpuVmVSetUp
/* 12338 80021B38 21308000 */  addu       $a2, $a0, $zero
/* 1233C 80021B3C FFFFC230 */  andi       $v0, $a2, 0xFFFF
/* 12340 80021B40 1000422C */  sltiu      $v0, $v0, 0x10
/* 12344 80021B44 10004010 */  beqz       $v0, .L80021B88
/* 12348 80021B48 2138A000 */   addu      $a3, $a1, $zero
/* 1234C 80021B4C 00140400 */  sll        $v0, $a0, 16
/* 12350 80021B50 03240200 */  sra        $a0, $v0, 16
/* 12354 80021B54 0980013C */  lui        $at, %hi(_svm_vab_used)
/* 12358 80021B58 21082400 */  addu       $at, $at, $a0
/* 1235C 80021B5C E8782390 */  lbu        $v1, %lo(_svm_vab_used)($at)
/* 12360 80021B60 01000234 */  ori        $v0, $zero, 0x1
/* 12364 80021B64 23006214 */  bne        $v1, $v0, .L80021BF4
/* 12368 80021B68 FFFF0224 */   addiu     $v0, $zero, -0x1
/* 1236C 80021B6C 001C0500 */  sll        $v1, $a1, 16
/* 12370 80021B70 0780023C */  lui        $v0, %hi(kMaxPrograms)
/* 12374 80021B74 94C34284 */  lh         $v0, %lo(kMaxPrograms)($v0)
/* 12378 80021B78 032C0300 */  sra        $a1, $v1, 16
/* 1237C 80021B7C 2A10A200 */  slt        $v0, $a1, $v0
/* 12380 80021B80 03004014 */  bnez       $v0, .L80021B90
/* 12384 80021B84 80100400 */   sll       $v0, $a0, 2
.L80021B88:
/* 12388 80021B88 FD860008 */  j          .L80021BF4
/* 1238C 80021B8C FFFF0224 */   addiu     $v0, $zero, -0x1
.L80021B90:
/* 12390 80021B90 0480013C */  lui        $at, %hi(_svm_vab_vh)
/* 12394 80021B94 21082200 */  addu       $at, $at, $v0
/* 12398 80021B98 14C9238C */  lw         $v1, %lo(_svm_vab_vh)($at)
/* 1239C 80021B9C 0480013C */  lui        $at, %hi(_svm_vab_pg)
/* 123A0 80021BA0 21082200 */  addu       $at, $at, $v0
/* 123A4 80021BA4 C8C8248C */  lw         $a0, %lo(_svm_vab_pg)($at)
/* 123A8 80021BA8 0480013C */  lui        $at, %hi(_svm_vab_tn)
/* 123AC 80021BAC 21082200 */  addu       $at, $at, $v0
/* 123B0 80021BB0 58C9228C */  lw         $v0, %lo(_svm_vab_tn)($at)
/* 123B4 80021BB4 0980013C */  lui        $at, %hi(_svm_cur+1)
/* 123B8 80021BB8 C97826A0 */  sb         $a2, %lo(_svm_cur+1)($at)
/* 123BC 80021BBC 0980013C */  lui        $at, %hi(_svm_cur+6)
/* 123C0 80021BC0 CE7827A0 */  sb         $a3, %lo(_svm_cur+6)($at)
/* 123C4 80021BC4 0780013C */  lui        $at, %hi(_svm_tn)
/* 123C8 80021BC8 C8CB22AC */  sw         $v0, %lo(_svm_tn)($at)
/* 123CC 80021BCC 00110500 */  sll        $v0, $a1, 4
/* 123D0 80021BD0 21104400 */  addu       $v0, $v0, $a0
/* 123D4 80021BD4 0780013C */  lui        $at, %hi(_svm_vh)
/* 123D8 80021BD8 C0C323AC */  sw         $v1, %lo(_svm_vh)($at)
/* 123DC 80021BDC 0780013C */  lui        $at, %hi(_svm_pg)
/* 123E0 80021BE0 B4C324AC */  sw         $a0, %lo(_svm_pg)($at)
/* 123E4 80021BE4 08004390 */  lbu        $v1, 0x8($v0)
/* 123E8 80021BE8 21100000 */  addu       $v0, $zero, $zero
/* 123EC 80021BEC 0980013C */  lui        $at, %hi(_svm_cur+7)
/* 123F0 80021BF0 CF7823A0 */  sb         $v1, %lo(_svm_cur+7)($at)
.L80021BF4:
/* 123F4 80021BF4 0800E003 */  jr         $ra
/* 123F8 80021BF8 00000000 */   nop
.size SpuVmVSetUp, . - SpuVmVSetUp
