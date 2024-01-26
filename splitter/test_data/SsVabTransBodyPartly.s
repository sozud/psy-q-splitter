.set noat      /* allow manual use of $at */
.set noreorder /* don't insert nops after branches */

glabel SsVabTransBodyPartly
/* 12080 80021880 D8FFBD27 */  addiu      $sp, $sp, -0x28
/* 12084 80021884 2000B4AF */  sw         $s4, 0x20($sp)
/* 12088 80021888 21A08000 */  addu       $s4, $a0, $zero
/* 1208C 8002188C 1C00B3AF */  sw         $s3, 0x1C($sp)
/* 12090 80021890 2198A000 */  addu       $s3, $a1, $zero
/* 12094 80021894 2128C000 */  addu       $a1, $a2, $zero
/* 12098 80021898 FFFFA230 */  andi       $v0, $a1, 0xFFFF
/* 1209C 8002189C 1100422C */  sltiu      $v0, $v0, 0x11
/* 120A0 800218A0 2400BFAF */  sw         $ra, 0x24($sp)
/* 120A4 800218A4 1800B2AF */  sw         $s2, 0x18($sp)
/* 120A8 800218A8 1400B1AF */  sw         $s1, 0x14($sp)
/* 120AC 800218AC 21004010 */  beqz       $v0, .L80021934
/* 120B0 800218B0 1000B0AF */   sw        $s0, 0x10($sp)
/* 120B4 800218B4 00140600 */  sll        $v0, $a2, 16
/* 120B8 800218B8 038C0200 */  sra        $s1, $v0, 16
/* 120BC 800218BC 0980013C */  lui        $at, %hi(_svm_vab_used)
/* 120C0 800218C0 21083100 */  addu       $at, $at, $s1
/* 120C4 800218C4 E8782390 */  lbu        $v1, %lo(_svm_vab_used)($at)
/* 120C8 800218C8 02000234 */  ori        $v0, $zero, 0x2
/* 120CC 800218CC 19006214 */  bne        $v1, $v0, .L80021934
/* 120D0 800218D0 00000000 */   nop
/* 120D4 800218D4 0380023C */  lui        $v0, %hi(D_00000000)
/* 120D8 800218D8 082F428C */  lw         $v0, %lo(D_00000000)($v0)
/* 120DC 800218DC 00000000 */  nop
/* 120E0 800218E0 0F004014 */  bnez       $v0, .L80021920
/* 120E4 800218E4 80801100 */   sll       $s0, $s1, 2
/* 120E8 800218E8 0A80013C */  lui        $at, %hi(_svm_vab_total)
/* 120EC 800218EC 21083000 */  addu       $at, $at, $s0
/* 120F0 800218F0 CC87228C */  lw         $v0, %lo(_svm_vab_total)($at)
/* 120F4 800218F4 0380013C */  lui        $at, %hi(D_00000004)
/* 120F8 800218F8 0C2F25A4 */  sh         $a1, %lo(D_00000004)($at)
/* 120FC 800218FC 0380013C */  lui        $at, %hi(D_00000000)
/* 12100 80021900 082F22AC */  sw         $v0, %lo(D_00000000)($at)
/* 12104 80021904 FDAA000C */  jal        SpuSetTransferMode
/* 12108 80021908 21200000 */   addu      $a0, $zero, $zero
/* 1210C 8002190C 0A80013C */  lui        $at, %hi(_svm_vab_start)
/* 12110 80021910 21083000 */  addu       $at, $at, $s0
/* 12114 80021914 1088248C */  lw         $a0, %lo(_svm_vab_start)($at)
/* 12118 80021918 EEAA000C */  jal        SpuSetTransferStartAddr
/* 1211C 8002191C 00000000 */   nop
.L80021920:
/* 12120 80021920 0380123C */  lui        $s2, %hi(D_00000004)
/* 12124 80021924 0C2F5286 */  lh         $s2, %lo(D_00000004)($s2)
/* 12128 80021928 00000000 */  nop
/* 1212C 8002192C 05005112 */  beq        $s2, $s1, .L80021944
/* 12130 80021930 21806002 */   addu      $s0, $s3, $zero
.L80021934:
/* 12134 80021934 5BAB000C */  jal        _spu_setInTransfer
/* 12138 80021938 21200000 */   addu      $a0, $zero, $zero
/* 1213C 8002193C 6F860008 */  j          .L800219BC
/* 12140 80021940 FFFF0224 */   addiu     $v0, $zero, -0x1
.L80021944:
/* 12144 80021944 0380033C */  lui        $v1, %hi(D_00000000)
/* 12148 80021948 082F638C */  lw         $v1, %lo(D_00000000)($v1)
/* 1214C 8002194C 00000000 */  nop
/* 12150 80021950 2B107000 */  sltu       $v0, $v1, $s0
/* 12154 80021954 02004010 */  beqz       $v0, .L80021960
/* 12158 80021958 00000000 */   nop
/* 1215C 8002195C 21806000 */  addu       $s0, $v1, $zero
.L80021960:
/* 12160 80021960 5BAB000C */  jal        _spu_setInTransfer
/* 12164 80021964 01000434 */   ori       $a0, $zero, 0x1
/* 12168 80021968 21208002 */  addu       $a0, $s4, $zero
/* 1216C 8002196C 0AAB000C */  jal        SpuWritePartly
/* 12170 80021970 21280002 */   addu      $a1, $s0, $zero
/* 12174 80021974 0380023C */  lui        $v0, %hi(D_00000000)
/* 12178 80021978 082F428C */  lw         $v0, %lo(D_00000000)($v0)
/* 1217C 8002197C 00000000 */  nop
/* 12180 80021980 23105000 */  subu       $v0, $v0, $s0
/* 12184 80021984 0380013C */  lui        $at, %hi(D_00000000)
/* 12188 80021988 082F22AC */  sw         $v0, %lo(D_00000000)($at)
/* 1218C 8002198C 0B004014 */  bnez       $v0, .L800219BC
/* 12190 80021990 FEFF0224 */   addiu     $v0, $zero, -0x2
/* 12194 80021994 21104002 */  addu       $v0, $s2, $zero
/* 12198 80021998 FFFF0324 */  addiu      $v1, $zero, -0x1
/* 1219C 8002199C 0380013C */  lui        $at, %hi(D_00000004)
/* 121A0 800219A0 0C2F23A4 */  sh         $v1, %lo(D_00000004)($at)
/* 121A4 800219A4 01000334 */  ori        $v1, $zero, 0x1
/* 121A8 800219A8 0380013C */  lui        $at, %hi(D_00000000)
/* 121AC 800219AC 082F20AC */  sw         $zero, %lo(D_00000000)($at)
/* 121B0 800219B0 0980013C */  lui        $at, %hi(_svm_vab_used)
/* 121B4 800219B4 21082200 */  addu       $at, $at, $v0
/* 121B8 800219B8 E87823A0 */  sb         $v1, %lo(_svm_vab_used)($at)
.L800219BC:
/* 121BC 800219BC 2400BF8F */  lw         $ra, 0x24($sp)
/* 121C0 800219C0 2000B48F */  lw         $s4, 0x20($sp)
/* 121C4 800219C4 1C00B38F */  lw         $s3, 0x1C($sp)
/* 121C8 800219C8 1800B28F */  lw         $s2, 0x18($sp)
/* 121CC 800219CC 1400B18F */  lw         $s1, 0x14($sp)
/* 121D0 800219D0 1000B08F */  lw         $s0, 0x10($sp)
/* 121D4 800219D4 2800BD27 */  addiu      $sp, $sp, 0x28
/* 121D8 800219D8 0800E003 */  jr         $ra
/* 121DC 800219DC 00000000 */   nop
.size SsVabTransBodyPartly, . - SsVabTransBodyPartly
