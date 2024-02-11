.set noat      /* allow manual use of $at */
.set noreorder /* don't insert nops after branches */

glabel _SsSeqPlay
/* E064 8001D864 C8FFBD27 */  addiu      $sp, $sp, -0x38
/* E068 8001D868 003C0400 */  sll        $a3, $a0, 16
/* E06C 8001D86C 83230700 */  sra        $a0, $a3, 14
/* E070 8001D870 002C0500 */  sll        $a1, $a1, 16
/* E074 8001D874 031C0500 */  sra        $v1, $a1, 16
/* E078 8001D878 40100300 */  sll        $v0, $v1, 1
/* E07C 8001D87C 21104300 */  addu       $v0, $v0, $v1
/* E080 8001D880 80100200 */  sll        $v0, $v0, 2
/* E084 8001D884 23104300 */  subu       $v0, $v0, $v1
/* E088 8001D888 80100200 */  sll        $v0, $v0, 2
/* E08C 8001D88C 23104300 */  subu       $v0, $v0, $v1
/* E090 8001D890 3000BFAF */  sw         $ra, 0x30($sp)
/* E094 8001D894 2C00B3AF */  sw         $s3, 0x2C($sp)
/* E098 8001D898 2800B2AF */  sw         $s2, 0x28($sp)
/* E09C 8001D89C 2400B1AF */  sw         $s1, 0x24($sp)
/* E0A0 8001D8A0 2000B0AF */  sw         $s0, 0x20($sp)
/* E0A4 8001D8A4 0980013C */  lui        $at, %hi(_ss_score)
/* E0A8 8001D8A8 21082400 */  addu       $at, $at, $a0
/* E0AC 8001D8AC 9C7C238C */  lw         $v1, %lo(_ss_score)($at)
/* E0B0 8001D8B0 80100200 */  sll        $v0, $v0, 2
/* E0B4 8001D8B4 21884300 */  addu       $s1, $v0, $v1
/* E0B8 8001D8B8 70002286 */  lh         $v0, 0x70($s1)
/* E0BC 8001D8BC 8800238E */  lw         $v1, 0x88($s1)
/* E0C0 8001D8C0 00000000 */  nop
/* E0C4 8001D8C4 23206200 */  subu       $a0, $v1, $v0
/* E0C8 8001D8C8 10008018 */  blez       $a0, .L8001D90C
/* E0CC 8001D8CC 21304000 */   addu      $a2, $v0, $zero
/* E0D0 8001D8D0 6E002386 */  lh         $v1, 0x6E($s1)
/* E0D4 8001D8D4 00000000 */  nop
/* E0D8 8001D8D8 04006018 */  blez       $v1, .L8001D8EC
/* E0DC 8001D8DC 21106000 */   addu      $v0, $v1, $zero
/* E0E0 8001D8E0 FFFF4224 */  addiu      $v0, $v0, -0x1
/* E0E4 8001D8E4 55760008 */  j          .L8001D954
/* E0E8 8001D8E8 6E0022A6 */   sh        $v0, 0x6E($s1)
.L8001D8EC:
/* E0EC 8001D8EC 05006014 */  bnez       $v1, .L8001D904
/* E0F0 8001D8F0 00000000 */   nop
/* E0F4 8001D8F4 8800228E */  lw         $v0, 0x88($s1)
/* E0F8 8001D8F8 6E0026A6 */  sh         $a2, 0x6E($s1)
/* E0FC 8001D8FC 54760008 */  j          .L8001D950
/* E100 8001D900 FFFF4224 */   addiu     $v0, $v0, -0x1
.L8001D904:
/* E104 8001D904 55760008 */  j          .L8001D954
/* E108 8001D908 880024AE */   sw        $a0, 0x88($s1)
.L8001D90C:
/* E10C 8001D90C 2A104300 */  slt        $v0, $v0, $v1
/* E110 8001D910 10004014 */  bnez       $v0, .L8001D954
/* E114 8001D914 21806000 */   addu      $s0, $v1, $zero
/* E118 8001D918 2198E000 */  addu       $s3, $a3, $zero
/* E11C 8001D91C 2190A000 */  addu       $s2, $a1, $zero
/* E120 8001D920 03241300 */  sra        $a0, $s3, 16
.L8001D924:
/* E124 8001D924 5D76000C */  jal        _SsGetSeqData
/* E128 8001D928 032C1200 */   sra       $a1, $s2, 16
/* E12C 8001D92C 8800228E */  lw         $v0, 0x88($s1)
/* E130 8001D930 00000000 */  nop
/* E134 8001D934 FBFF4010 */  beqz       $v0, .L8001D924
/* E138 8001D938 03241300 */   sra       $a0, $s3, 16
/* E13C 8001D93C 70002386 */  lh         $v1, 0x70($s1)
/* E140 8001D940 21800202 */  addu       $s0, $s0, $v0
/* E144 8001D944 2A100302 */  slt        $v0, $s0, $v1
/* E148 8001D948 F6FF4014 */  bnez       $v0, .L8001D924
/* E14C 8001D94C 23100302 */   subu      $v0, $s0, $v1
.L8001D950:
/* E150 8001D950 880022AE */  sw         $v0, 0x88($s1)
.L8001D954:
/* E154 8001D954 3000BF8F */  lw         $ra, 0x30($sp)
/* E158 8001D958 2C00B38F */  lw         $s3, 0x2C($sp)
/* E15C 8001D95C 2800B28F */  lw         $s2, 0x28($sp)
/* E160 8001D960 2400B18F */  lw         $s1, 0x24($sp)
/* E164 8001D964 2000B08F */  lw         $s0, 0x20($sp)
/* E168 8001D968 3800BD27 */  addiu      $sp, $sp, 0x38
/* E16C 8001D96C 0800E003 */  jr         $ra
/* E170 8001D970 00000000 */   nop
.size _SsSeqPlay, . - _SsSeqPlay
