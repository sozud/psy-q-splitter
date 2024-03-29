.set noat      /* allow manual use of $at */
.set noreorder /* don't insert nops after branches */

glabel SpuVmAlloc
/* 12A0C 8002220C D8FFBD27 */  addiu      $sp, $sp, -0x28
/* 12A10 80022210 2000B0AF */  sw         $s0, 0x20($sp)
/* 12A14 80022214 63001034 */  ori        $s0, $zero, 0x63
/* 12A18 80022218 FFFF0B34 */  ori        $t3, $zero, 0xFFFF
/* 12A1C 8002221C 21500000 */  addu       $t2, $zero, $zero
/* 12A20 80022220 21400000 */  addu       $t0, $zero, $zero
/* 12A24 80022224 63000934 */  ori        $t1, $zero, 0x63
/* 12A28 80022228 0880023C */  lui        $v0, %hi(spuVmMaxVoice)
/* 12A2C 8002222C E86F4290 */  lbu        $v0, %lo(spuVmMaxVoice)($v0)
/* 12A30 80022230 09800C3C */  lui        $t4, %hi(_svm_cur+15)
/* 12A34 80022234 D7788C91 */  lbu        $t4, %lo(_svm_cur+15)($t4)
/* 12A38 80022238 21380000 */  addu       $a3, $zero, $zero
/* 12A3C 8002223C 4C004010 */  beqz       $v0, .L80022370
/* 12A40 80022240 2400BFAF */   sw        $ra, 0x24($sp)
/* 12A44 80022244 FF00E230 */  andi       $v0, $a3, 0xFF
.L80022248:
/* 12A48 80022248 40180200 */  sll        $v1, $v0, 1
/* 12A4C 8002224C 21186200 */  addu       $v1, $v1, $v0
/* 12A50 80022250 80180300 */  sll        $v1, $v1, 2
/* 12A54 80022254 21186200 */  addu       $v1, $v1, $v0
/* 12A58 80022258 80180300 */  sll        $v1, $v1, 2
/* 12A5C 8002225C 0480013C */  lui        $at, %hi(_svm_voice+27)
/* 12A60 80022260 21082300 */  addu       $at, $at, $v1
/* 12A64 80022264 43B82290 */  lbu        $v0, %lo(_svm_voice+27)($at)
/* 12A68 80022268 00000000 */  nop
/* 12A6C 8002226C 09004014 */  bnez       $v0, .L80022294
/* 12A70 80022270 FF00E230 */   andi      $v0, $a3, 0xFF
/* 12A74 80022274 0480013C */  lui        $at, %hi(_svm_voice+6)
/* 12A78 80022278 21082300 */  addu       $at, $at, $v1
/* 12A7C 8002227C 2EB82294 */  lhu        $v0, %lo(_svm_voice+6)($at)
/* 12A80 80022280 00000000 */  nop
/* 12A84 80022284 03004014 */  bnez       $v0, .L80022294
/* 12A88 80022288 FF00E230 */   andi      $v0, $a3, 0xFF
/* 12A8C 8002228C DC880008 */  j          .L80022370
/* 12A90 80022290 2180E000 */   addu      $s0, $a3, $zero
.L80022294:
/* 12A94 80022294 40180200 */  sll        $v1, $v0, 1
/* 12A98 80022298 21186200 */  addu       $v1, $v1, $v0
/* 12A9C 8002229C 80180300 */  sll        $v1, $v1, 2
/* 12AA0 800222A0 21186200 */  addu       $v1, $v1, $v0
/* 12AA4 800222A4 80180300 */  sll        $v1, $v1, 2
/* 12AA8 800222A8 0480013C */  lui        $at, %hi(_svm_voice+24)
/* 12AAC 800222AC 21082300 */  addu       $at, $at, $v1
/* 12AB0 800222B0 40B82684 */  lh         $a2, %lo(_svm_voice+24)($at)
/* 12AB4 800222B4 FFFF8431 */  andi       $a0, $t4, 0xFFFF
/* 12AB8 800222B8 2A10C400 */  slt        $v0, $a2, $a0
/* 12ABC 800222BC 0B004010 */  beqz       $v0, .L800222EC
/* 12AC0 800222C0 2128C000 */   addu      $a1, $a2, $zero
/* 12AC4 800222C4 2160A000 */  addu       $t4, $a1, $zero
/* 12AC8 800222C8 2148E000 */  addu       $t1, $a3, $zero
/* 12ACC 800222CC 0480013C */  lui        $at, %hi(_svm_voice+6)
/* 12AD0 800222D0 21082300 */  addu       $at, $at, $v1
/* 12AD4 800222D4 2EB82B94 */  lhu        $t3, %lo(_svm_voice+6)($at)
/* 12AD8 800222D8 0480013C */  lui        $at, %hi(_svm_voice+2)
/* 12ADC 800222DC 21082300 */  addu       $at, $at, $v1
/* 12AE0 800222E0 2AB82894 */  lhu        $t0, %lo(_svm_voice+2)($at)
/* 12AE4 800222E4 D5880008 */  j          .L80022354
/* 12AE8 800222E8 01000A34 */   ori       $t2, $zero, 0x1
.L800222EC:
/* 12AEC 800222EC 1900C414 */  bne        $a2, $a0, .L80022354
/* 12AF0 800222F0 FFFF6531 */   andi      $a1, $t3, 0xFFFF
/* 12AF4 800222F4 0480013C */  lui        $at, %hi(_svm_voice+6)
/* 12AF8 800222F8 21082300 */  addu       $at, $at, $v1
/* 12AFC 800222FC 2EB82494 */  lhu        $a0, %lo(_svm_voice+6)($at)
/* 12B00 80022300 00000000 */  nop
/* 12B04 80022304 2B108500 */  sltu       $v0, $a0, $a1
/* 12B08 80022308 06004010 */  beqz       $v0, .L80022324
/* 12B0C 8002230C 01004A25 */   addiu     $t2, $t2, 0x1
/* 12B10 80022310 0480013C */  lui        $at, %hi(_svm_voice+2)
/* 12B14 80022314 21082300 */  addu       $at, $at, $v1
/* 12B18 80022318 2AB82894 */  lhu        $t0, %lo(_svm_voice+2)($at)
/* 12B1C 8002231C D4880008 */  j          .L80022350
/* 12B20 80022320 21588000 */   addu      $t3, $a0, $zero
.L80022324:
/* 12B24 80022324 0B008514 */  bne        $a0, $a1, .L80022354
/* 12B28 80022328 00000000 */   nop
/* 12B2C 8002232C 0480013C */  lui        $at, %hi(_svm_voice+2)
/* 12B30 80022330 21082300 */  addu       $at, $at, $v1
/* 12B34 80022334 2AB82284 */  lh         $v0, %lo(_svm_voice+2)($at)
/* 12B38 80022338 00000000 */  nop
/* 12B3C 8002233C 21184000 */  addu       $v1, $v0, $zero
/* 12B40 80022340 2A100201 */  slt        $v0, $t0, $v0
/* 12B44 80022344 03004010 */  beqz       $v0, .L80022354
/* 12B48 80022348 00000000 */   nop
/* 12B4C 8002234C 21406000 */  addu       $t0, $v1, $zero
.L80022350:
/* 12B50 80022350 2148E000 */  addu       $t1, $a3, $zero
.L80022354:
/* 12B54 80022354 0100E724 */  addiu      $a3, $a3, 0x1
/* 12B58 80022358 0880033C */  lui        $v1, %hi(spuVmMaxVoice)
/* 12B5C 8002235C E86F6390 */  lbu        $v1, %lo(spuVmMaxVoice)($v1)
/* 12B60 80022360 FF00E230 */  andi       $v0, $a3, 0xFF
/* 12B64 80022364 2B104300 */  sltu       $v0, $v0, $v1
/* 12B68 80022368 B7FF4014 */  bnez       $v0, .L80022248
/* 12B6C 8002236C FF00E230 */   andi      $v0, $a3, 0xFF
.L80022370:
/* 12B70 80022370 FF000332 */  andi       $v1, $s0, 0xFF
/* 12B74 80022374 63000234 */  ori        $v0, $zero, 0x63
/* 12B78 80022378 05006214 */  bne        $v1, $v0, .L80022390
/* 12B7C 8002237C FF004231 */   andi      $v0, $t2, 0xFF
/* 12B80 80022380 03004014 */  bnez       $v0, .L80022390
/* 12B84 80022384 21802001 */   addu      $s0, $t1, $zero
/* 12B88 80022388 0880103C */  lui        $s0, %hi(spuVmMaxVoice)
/* 12B8C 8002238C E86F1092 */  lbu        $s0, %lo(spuVmMaxVoice)($s0)
.L80022390:
/* 12B90 80022390 0880043C */  lui        $a0, %hi(spuVmMaxVoice)
/* 12B94 80022394 E86F8490 */  lbu        $a0, %lo(spuVmMaxVoice)($a0)
/* 12B98 80022398 FF000232 */  andi       $v0, $s0, 0xFF
/* 12B9C 8002239C 2B104400 */  sltu       $v0, $v0, $a0
/* 12BA0 800223A0 2E004010 */  beqz       $v0, .L8002245C
/* 12BA4 800223A4 00000000 */   nop
/* 12BA8 800223A8 14008010 */  beqz       $a0, .L800223FC
/* 12BAC 800223AC 21380000 */   addu      $a3, $zero, $zero
/* 12BB0 800223B0 0480053C */  lui        $a1, %hi(_svm_voice)
/* 12BB4 800223B4 28B8A524 */  addiu      $a1, $a1, %lo(_svm_voice)
/* 12BB8 800223B8 FF00E330 */  andi       $v1, $a3, 0xFF
.L800223BC:
/* 12BBC 800223BC 40100300 */  sll        $v0, $v1, 1
/* 12BC0 800223C0 21104300 */  addu       $v0, $v0, $v1
/* 12BC4 800223C4 80100200 */  sll        $v0, $v0, 2
/* 12BC8 800223C8 21104300 */  addu       $v0, $v0, $v1
/* 12BCC 800223CC 80100200 */  sll        $v0, $v0, 2
/* 12BD0 800223D0 0100E724 */  addiu      $a3, $a3, 0x1
/* 12BD4 800223D4 0480013C */  lui        $at, %hi(_svm_voice+2)
/* 12BD8 800223D8 21082200 */  addu       $at, $at, $v0
/* 12BDC 800223DC 2AB82394 */  lhu        $v1, %lo(_svm_voice+2)($at)
/* 12BE0 800223E0 21104500 */  addu       $v0, $v0, $a1
/* 12BE4 800223E4 01006324 */  addiu      $v1, $v1, 0x1
/* 12BE8 800223E8 020043A4 */  sh         $v1, 0x2($v0)
/* 12BEC 800223EC FF00E230 */  andi       $v0, $a3, 0xFF
/* 12BF0 800223F0 2B104400 */  sltu       $v0, $v0, $a0
/* 12BF4 800223F4 F1FF4014 */  bnez       $v0, .L800223BC
/* 12BF8 800223F8 FF00E330 */   andi      $v1, $a3, 0xFF
.L800223FC:
/* 12BFC 800223FC FF000332 */  andi       $v1, $s0, 0xFF
/* 12C00 80022400 40100300 */  sll        $v0, $v1, 1
/* 12C04 80022404 21104300 */  addu       $v0, $v0, $v1
/* 12C08 80022408 80100200 */  sll        $v0, $v0, 2
/* 12C0C 8002240C 21104300 */  addu       $v0, $v0, $v1
/* 12C10 80022410 80100200 */  sll        $v0, $v0, 2
/* 12C14 80022414 0480013C */  lui        $at, %hi(_svm_voice+2)
/* 12C18 80022418 21082200 */  addu       $at, $at, $v0
/* 12C1C 8002241C 2AB820A4 */  sh         $zero, %lo(_svm_voice+2)($at)
/* 12C20 80022420 0980033C */  lui        $v1, %hi(_svm_cur+15)
/* 12C24 80022424 D7786390 */  lbu        $v1, %lo(_svm_cur+15)($v1)
/* 12C28 80022428 0480013C */  lui        $at, %hi(_svm_voice+24)
/* 12C2C 8002242C 21082200 */  addu       $at, $at, $v0
/* 12C30 80022430 40B823A4 */  sh         $v1, %lo(_svm_voice+24)($at)
/* 12C34 80022434 0480013C */  lui        $at, %hi(_svm_voice+27)
/* 12C38 80022438 21082200 */  addu       $at, $at, $v0
/* 12C3C 8002243C 43B82390 */  lbu        $v1, %lo(_svm_voice+27)($at)
/* 12C40 80022440 02000234 */  ori        $v0, $zero, 0x2
/* 12C44 80022444 06006214 */  bne        $v1, $v0, .L80022460
/* 12C48 80022448 FF000232 */   andi      $v0, $s0, 0xFF
/* 12C4C 8002244C FF00053C */  lui        $a1, 0xFF
/* 12C50 80022450 FFFFA534 */  ori        $a1, $a1, 0xFFFF
/* 12C54 80022454 1CA4000C */  jal        SpuSetNoiseVoice
/* 12C58 80022458 21200000 */   addu      $a0, $zero, $zero
.L8002245C:
/* 12C5C 8002245C FF000232 */  andi       $v0, $s0, 0xFF
.L80022460:
/* 12C60 80022460 2400BF8F */  lw         $ra, 0x24($sp)
/* 12C64 80022464 2000B08F */  lw         $s0, 0x20($sp)
/* 12C68 80022468 2800BD27 */  addiu      $sp, $sp, 0x28
/* 12C6C 8002246C 0800E003 */  jr         $ra
/* 12C70 80022470 00000000 */   nop
.size SpuVmAlloc, . - SpuVmAlloc
