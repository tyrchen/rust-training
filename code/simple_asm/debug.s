	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 7
	.p2align	4, 0x90
__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h5472c873967227b0E:
Lfunc_begin0:
	.file	1 "/Users/tchen/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs"
	.loc	1 121 0
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	movq	%rdi, -24(%rbp)
Ltmp3:
	.loc	1 125 18 prologue_end
	callq	__ZN4core3ops8function6FnOnce9call_once17h683c3ede57be420eE
Ltmp0:
Ltmp4:
	.loc	1 128 5
	callq	__ZN4core4hint9black_box17h3f2328ddbd61b373E
Ltmp1:
	jmp	LBB0_2
Ltmp5:
LBB0_2:
	.loc	1 131 2
	addq	$48, %rsp
	popq	%rbp
	retq
LBB0_3:
	.loc	1 131 1 is_stmt 0
	jmp	LBB0_4
LBB0_4:
	.loc	1 121 1 is_stmt 1
	movq	-16(%rbp), %rdi
	callq	__Unwind_Resume
	ud2
Ltmp6:
LBB0_5:
Ltmp2:
	.loc	1 0 1 is_stmt 0
	movq	%rax, -16(%rbp)
	movl	%edx, -8(%rbp)
	jmp	LBB0_3
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table0:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Lfunc_begin0-Lfunc_begin0
	.uleb128 Ltmp0-Lfunc_begin0
	.byte	0
	.byte	0
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp1
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2

	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__ZN3std2rt10lang_start17h4caa46d899550d98E
	.globl	__ZN3std2rt10lang_start17h4caa46d899550d98E
	.p2align	4, 0x90
__ZN3std2rt10lang_start17h4caa46d899550d98E:
Lfunc_begin1:
	.file	2 "/Users/tchen/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/rt.rs"
	.loc	2 60 0 is_stmt 1
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$64, %rsp
	leaq	l___unnamed_1(%rip), %rax
	movq	%rdi, -24(%rbp)
	movq	%rsi, -16(%rbp)
	movq	%rdx, -8(%rbp)
Ltmp7:
	.loc	2 66 10 prologue_end
	movq	%rdi, -32(%rbp)
	.loc	2 66 9 is_stmt 0
	leaq	-32(%rbp), %rcx
	.loc	2 65 5 is_stmt 1
	movq	%rcx, %rdi
	movq	%rsi, -40(%rbp)
	movq	%rax, %rsi
	movq	-40(%rbp), %rax
	movq	%rdx, -48(%rbp)
	movq	%rax, %rdx
	movq	-48(%rbp), %rcx
	callq	__ZN3std2rt19lang_start_internal17h86f505dc7de50d93E
	movq	%rax, -56(%rbp)
	.loc	2 0 5 is_stmt 0
	movq	-56(%rbp), %rax
	.loc	2 70 2 is_stmt 1
	addq	$64, %rsp
	popq	%rbp
	retq
Ltmp8:
Lfunc_end1:
	.cfi_endproc

	.p2align	4, 0x90
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h39df29d0fee81e1eE:
Lfunc_begin2:
	.loc	2 66 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
Ltmp9:
	.loc	2 66 77 prologue_end
	movq	(%rdi), %rdi
	.loc	2 66 18 is_stmt 0
	callq	__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h5472c873967227b0E
	callq	__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hba63e1989e7bb043E
	movl	%eax, -12(%rbp)
	.loc	2 0 18
	movl	-12(%rbp), %eax
	.loc	2 66 91
	addq	$16, %rsp
	popq	%rbp
	retq
Ltmp10:
Lfunc_end2:
	.cfi_endproc

	.p2align	4, 0x90
__ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h5193a79422012651E:
Lfunc_begin3:
	.file	3 "/Users/tchen/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/unix/process/process_common.rs"
	.loc	3 438 0 is_stmt 1
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%rax
	movq	%rdi, -8(%rbp)
Ltmp11:
	.loc	3 439 9 prologue_end
	movzbl	(%rdi), %eax
	.loc	3 440 6
	addq	$8, %rsp
	popq	%rbp
	retq
Ltmp12:
Lfunc_end3:
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3fmt10ArgumentV13new17h19c824d6b33ce84eE:
Lfunc_begin4:
	.file	4 "/Users/tchen/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/fmt/mod.rs"
	.loc	4 267 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$72, %rsp
	movq	%rdi, -32(%rbp)
	movq	%rsi, -24(%rbp)
Ltmp13:
	.loc	4 276 42 prologue_end
	movq	%rsi, -16(%rbp)
	movq	-16(%rbp), %rax
	movq	%rdi, -56(%rbp)
	movq	%rax, -64(%rbp)
	.loc	4 0 42 is_stmt 0
	movq	-56(%rbp), %rax
	.loc	4 276 68
	movq	%rax, -8(%rbp)
	movq	-8(%rbp), %rax
	movq	%rax, -72(%rbp)
	.loc	4 0 68
	movq	-72(%rbp), %rax
	.loc	4 276 18
	movq	%rax, -48(%rbp)
	movq	-64(%rbp), %rcx
	movq	%rcx, -40(%rbp)
	.loc	4 277 6 is_stmt 1
	movq	-48(%rbp), %rax
	movq	-40(%rbp), %rdx
	addq	$72, %rsp
	popq	%rbp
	retq
Ltmp14:
Lfunc_end4:
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h40a2eb713cf4428eE:
Lfunc_begin5:
	.file	5 "/Users/tchen/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/fmt/num.rs"
	.loc	5 185 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	movq	%rdi, -16(%rbp)
	movq	%rsi, -8(%rbp)
	movq	%rdi, -32(%rbp)
Ltmp15:
	.loc	5 186 20 prologue_end
	movq	%rsi, %rdi
	movq	%rsi, -40(%rbp)
	callq	__ZN4core3fmt9Formatter15debug_lower_hex17h711d6e847f2fb68dE
	movb	%al, -41(%rbp)
	.loc	5 0 20 is_stmt 0
	movb	-41(%rbp), %al
	.loc	5 186 17
	testb	$1, %al
	jne	LBB5_2
	jmp	LBB5_3
LBB5_2:
	.loc	5 0 17
	movq	-32(%rbp), %rdi
	movq	-40(%rbp), %rsi
	.loc	5 187 21 is_stmt 1
	callq	__ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17hd85567a5614f1f15E
	andb	$1, %al
	movb	%al, -17(%rbp)
	jmp	LBB5_4
LBB5_3:
	.loc	5 0 21 is_stmt 0
	movq	-40(%rbp), %rdi
	.loc	5 188 27 is_stmt 1
	callq	__ZN4core3fmt9Formatter15debug_upper_hex17h699081381c0e441bE
	movb	%al, -42(%rbp)
	jmp	LBB5_5
LBB5_4:
	.loc	5 186 17
	jmp	LBB5_11
LBB5_5:
	.loc	5 0 17 is_stmt 0
	movb	-42(%rbp), %al
	.loc	5 188 24 is_stmt 1
	testb	$1, %al
	jne	LBB5_6
	jmp	LBB5_7
LBB5_6:
	.loc	5 0 24 is_stmt 0
	movq	-32(%rbp), %rdi
	movq	-40(%rbp), %rsi
	.loc	5 189 21 is_stmt 1
	callq	__ZN4core3fmt3num55_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$usize$GT$3fmt17h16e9342ec3f451c5E
	andb	$1, %al
	movb	%al, -17(%rbp)
	jmp	LBB5_8
LBB5_7:
	.loc	5 0 21 is_stmt 0
	movq	-32(%rbp), %rdi
	movq	-40(%rbp), %rsi
	.loc	5 191 21 is_stmt 1
	callq	__ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h5e5dc183b103235dE
	andb	$1, %al
	movb	%al, -17(%rbp)
	jmp	LBB5_9
LBB5_8:
	.loc	5 188 24
	jmp	LBB5_10
LBB5_9:
	jmp	LBB5_10
LBB5_10:
	.loc	5 186 17
	jmp	LBB5_11
LBB5_11:
	.loc	5 193 14
	movb	-17(%rbp), %al
	andb	$1, %al
	movzbl	%al, %eax
	addq	$48, %rsp
	popq	%rbp
	retq
Ltmp16:
Lfunc_end5:
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3fmt9Arguments6new_v117h8a0fa4a3ae033e15E:
Lfunc_begin6:
	.loc	4 313 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	movq	%rdi, %rax
	movq	%rsi, -32(%rbp)
	movq	%rdx, -24(%rbp)
	movq	%rcx, -16(%rbp)
	movq	%r8, -8(%rbp)
Ltmp17:
	.loc	4 314 34 prologue_end
	movq	$0, -48(%rbp)
	.loc	4 314 9 is_stmt 0
	movq	%rsi, (%rdi)
	movq	%rdx, 8(%rdi)
	movq	-48(%rbp), %rdx
	movq	-40(%rbp), %rsi
	movq	%rdx, 16(%rdi)
	movq	%rsi, 24(%rdi)
	movq	%rcx, 32(%rdi)
	movq	%r8, 40(%rdi)
	.loc	4 315 6 is_stmt 1
	addq	$48, %rsp
	popq	%rbp
	retq
Ltmp18:
Lfunc_end6:
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h080ec6d6def2a24cE:
Lfunc_begin7:
	.file	6 "/Users/tchen/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs"
	.loc	6 227 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$32, %rsp
	movq	%rdi, -8(%rbp)
Ltmp19:
	.loc	6 227 5 prologue_end
	movq	(%rdi), %rdi
	callq	__ZN4core3ops8function6FnOnce9call_once17hd751dbd7b8922d0fE
	movl	%eax, -20(%rbp)
	.loc	6 0 5 is_stmt 0
	movl	-20(%rbp), %eax
	.loc	6 227 5
	addq	$32, %rsp
	popq	%rbp
	retq
Ltmp20:
Lfunc_end7:
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ops8function6FnOnce9call_once17h683c3ede57be420eE:
Lfunc_begin8:
	.loc	6 227 0 is_stmt 1
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
Ltmp21:
	.loc	6 227 5 prologue_end
	callq	*%rdi
	addq	$16, %rsp
	popq	%rbp
	retq
Ltmp22:
Lfunc_end8:
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ops8function6FnOnce9call_once17hd751dbd7b8922d0fE:
Lfunc_begin9:
	.loc	6 227 0
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception1
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	movq	%rdi, -32(%rbp)
Ltmp23:
	leaq	-32(%rbp), %rdi
Ltmp26:
	.loc	6 227 5 prologue_end
	callq	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h39df29d0fee81e1eE
Ltmp24:
	movl	%eax, -36(%rbp)
	jmp	LBB9_1
LBB9_1:
	jmp	LBB9_2
LBB9_2:
	.loc	6 0 5 is_stmt 0
	movl	-36(%rbp), %eax
	.loc	6 227 5
	addq	$48, %rsp
	popq	%rbp
	retq
LBB9_3:
	jmp	LBB9_4
LBB9_4:
	movq	-16(%rbp), %rdi
	callq	__Unwind_Resume
	ud2
Ltmp27:
LBB9_5:
Ltmp25:
	.loc	6 0 5
	movq	%rax, -16(%rbp)
	movl	%edx, -8(%rbp)
	jmp	LBB9_3
Lfunc_end9:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table9:
Lexception1:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp23-Lfunc_begin9
	.uleb128 Ltmp24-Ltmp23
	.uleb128 Ltmp25-Lfunc_begin9
	.byte	0
	.uleb128 Ltmp24-Lfunc_begin9
	.uleb128 Lfunc_end9-Ltmp24
	.byte	0
	.byte	0
Lcst_end1:
	.p2align	2

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	4, 0x90
__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hcd5aba0e684c50bdE:
Lfunc_begin10:
	.file	7 "/Users/tchen/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs"
	.loc	7 179 0 is_stmt 1
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
Ltmp28:
	.loc	7 179 1 prologue_end
	addq	$16, %rsp
	popq	%rbp
	retq
Ltmp29:
Lfunc_end10:
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core4hint9black_box17h3f2328ddbd61b373E:
Lfunc_begin11:
	.file	8 "/Users/tchen/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/hint.rs"
	.loc	8 159 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%rax
	leaq	-8(%rbp), %rax
Ltmp30:
	.loc	8 170 9 prologue_end
	## InlineAsm Start
	## InlineAsm End
	.loc	8 174 2
	addq	$8, %rsp
	popq	%rbp
	retq
Ltmp31:
Lfunc_end11:
	.cfi_endproc

	.p2align	4, 0x90
__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hba63e1989e7bb043E:
Lfunc_begin12:
	.file	9 "/Users/tchen/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/process.rs"
	.loc	9 1828 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	xorl	%edi, %edi
Ltmp32:
	.loc	9 1829 9 prologue_end
	callq	__ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h7735134e327cd395E
	movl	%eax, -12(%rbp)
	.loc	9 0 9 is_stmt 0
	movl	-12(%rbp), %eax
	.loc	9 1830 6 is_stmt 1
	addq	$16, %rsp
	popq	%rbp
	retq
Ltmp33:
Lfunc_end12:
	.cfi_endproc

	.p2align	4, 0x90
__ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h7735134e327cd395E:
Lfunc_begin13:
	.loc	9 1862 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movb	%dil, -1(%rbp)
Ltmp34:
	.loc	9 1863 9 prologue_end
	leaq	-1(%rbp), %rdi
Ltmp35:
	callq	__ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h5193a79422012651E
Ltmp36:
	.loc	9 0 9 is_stmt 0
	movl	%eax, -8(%rbp)
	movl	-8(%rbp), %eax
	.loc	9 1864 6 is_stmt 1
	addq	$16, %rsp
	popq	%rbp
	retq
Ltmp37:
Lfunc_end13:
	.cfi_endproc

	.p2align	4, 0x90
__ZN10simple_asm4main17hc4bb5d0ec919f26cE:
Lfunc_begin14:
	.file	10 "/Users/tchen/projects/mycode/rust/rust-training/code/simple_asm/src/main.rs"
	.loc	10 1 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$128, %rsp
Ltmp38:
	.loc	10 2 24 prologue_end
	movl	$3735928559, %eax
	movq	%rax, -16(%rbp)
Ltmp39:
	.loc	10 3 30
	movq	%rax, %rdi
	callq	__ZN10simple_asm7process17ha5bf2b7c755a17d1E
	movq	%rax, -24(%rbp)
	.loc	10 3 5 is_stmt 0
	leaq	-24(%rbp), %rax
	movq	%rax, -32(%rbp)
	movq	-32(%rbp), %rax
	movq	%rax, -8(%rbp)
Ltmp40:
	.loc	10 3 5
	movq	%rax, %rdi
	leaq	__ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h40a2eb713cf4428eE(%rip), %rsi
	callq	__ZN4core3fmt10ArgumentV13new17h19c824d6b33ce84eE
	movq	%rax, -104(%rbp)
	movq	%rdx, -112(%rbp)
	.loc	10 0 5
	leaq	l___unnamed_2(%rip), %rax
	movq	-104(%rbp), %rcx
	.loc	10 3 5
	movq	%rcx, -48(%rbp)
	movq	-112(%rbp), %rdx
	movq	%rdx, -40(%rbp)
Ltmp41:
	.loc	10 3 5
	leaq	-48(%rbp), %rsi
	leaq	-96(%rbp), %rdi
	movq	%rsi, -120(%rbp)
	movq	%rax, %rsi
	movl	$2, %edx
	movq	-120(%rbp), %rcx
	movl	$1, %r8d
	callq	__ZN4core3fmt9Arguments6new_v117h8a0fa4a3ae033e15E
	leaq	-96(%rbp), %rdi
	callq	__ZN3std2io5stdio6_print17hea90da9dad6f182dE
Ltmp42:
	.loc	10 4 2 is_stmt 1
	addq	$128, %rsp
	popq	%rbp
	retq
Ltmp43:
Lfunc_end14:
	.cfi_endproc

	.p2align	4, 0x90
__ZN10simple_asm7process17ha5bf2b7c755a17d1E:
Lfunc_begin15:
	.loc	10 6 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$32, %rsp
	movq	%rdi, -8(%rbp)
Ltmp44:
	.loc	10 7 9 prologue_end
	movq	%rdi, %rax
	shrq	$16, %rax
	movq	%rdi, -16(%rbp)
	movq	%rax, -24(%rbp)
	.loc	10 0 9 is_stmt 0
	movq	-24(%rbp), %rax
	.loc	10 7 5
	cmpq	$0, %rax
	sete	%cl
	testb	$1, %cl
	jne	LBB15_3
	.loc	10 0 5
	movq	-16(%rbp), %rax
	.loc	10 7 5
	xorl	%ecx, %ecx
	movl	%ecx, %edx
	movq	-24(%rbp), %rsi
	divq	%rsi
	.loc	10 8 2 is_stmt 1
	movq	%rdx, %rax
	addq	$32, %rsp
	popq	%rbp
	retq
LBB15_3:
	.loc	10 7 5
	leaq	_str.0(%rip), %rdi
	leaq	l___unnamed_3(%rip), %rdx
	movl	$57, %esi
	callq	__ZN4core9panicking5panic17hfa08580418a71d7fE
Ltmp45:
Lfunc_end15:
	.cfi_endproc

	.globl	_main
	.p2align	4, 0x90
_main:
Lfunc_begin16:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movslq	%edi, %rax
	leaq	__ZN10simple_asm4main17hc4bb5d0ec919f26cE(%rip), %rdi
	movq	%rsi, -8(%rbp)
	movq	%rax, %rsi
	movq	-8(%rbp), %rdx
	callq	__ZN3std2rt10lang_start17h4caa46d899550d98E
	addq	$16, %rsp
	popq	%rbp
	retq
Lfunc_end16:
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3
l___unnamed_1:
	.quad	__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hcd5aba0e684c50bdE
	.quad	8
	.quad	8
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h39df29d0fee81e1eE
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h39df29d0fee81e1eE
	.quad	__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h080ec6d6def2a24cE

	.section	__TEXT,__literal8,8byte_literals
L___unnamed_4:
	.ascii	"result: "

	.section	__TEXT,__const
l___unnamed_5:
	.byte	10

	.section	__DATA,__const
	.p2align	3
l___unnamed_2:
	.quad	L___unnamed_4
	.asciz	"\b\000\000\000\000\000\000"
	.quad	l___unnamed_5
	.asciz	"\001\000\000\000\000\000\000"

	.section	__TEXT,__const
l___unnamed_6:
	.ascii	"src/main.rs"

	.section	__DATA,__const
	.p2align	3
l___unnamed_3:
	.quad	l___unnamed_6
	.asciz	"\013\000\000\000\000\000\000\000\007\000\000\000\005\000\000"

	.section	__TEXT,__const
	.p2align	4
_str.0:
	.ascii	"attempt to calculate the remainder with a divisor of zero"

	.section	__DWARF,__debug_abbrev,regular,debug
Lsection_abbrev:
	.byte	1
	.byte	17
	.byte	1
	.byte	37
	.byte	14
	.byte	19
	.byte	5
	.byte	3
	.byte	14
	.byte	16
	.byte	6
	.byte	27
	.byte	14
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	0
	.byte	0
	.byte	2
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.byte	2
	.byte	10
	.byte	0
	.byte	0
	.byte	3
	.byte	19
	.byte	0
	.byte	29
	.byte	19
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	4
	.byte	57
	.byte	1
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	5
	.byte	19
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	6
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	10
	.byte	0
	.byte	0
	.byte	7
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	64
	.byte	10
	.ascii	"\207@"
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	8
	.byte	52
	.byte	0
	.byte	2
	.byte	10
	.byte	3
	.byte	14
	.ascii	"\210\001"
	.byte	15
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	9
	.byte	47
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	10
	.byte	5
	.byte	0
	.byte	2
	.byte	10
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	11
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	64
	.byte	10
	.ascii	"\207@"
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	0
	.byte	0
	.byte	12
	.byte	11
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	0
	.byte	0
	.byte	13
	.byte	52
	.byte	0
	.byte	2
	.byte	10
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	14
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	64
	.byte	10
	.ascii	"\207@"
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	15
	.byte	5
	.byte	0
	.byte	2
	.byte	10
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	16
	.byte	15
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	51
	.byte	6
	.byte	0
	.byte	0
	.byte	17
	.byte	21
	.byte	0
	.byte	0
	.byte	0
	.byte	18
	.byte	4
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	19
	.byte	40
	.byte	0
	.byte	3
	.byte	14
	.byte	28
	.byte	15
	.byte	0
	.byte	0
	.byte	20
	.byte	51
	.byte	1
	.byte	21
	.byte	19
	.byte	0
	.byte	0
	.byte	21
	.byte	13
	.byte	0
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	10
	.byte	52
	.byte	12
	.byte	0
	.byte	0
	.byte	22
	.byte	25
	.byte	1
	.byte	22
	.byte	11
	.byte	0
	.byte	0
	.byte	23
	.byte	19
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	24
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	10
	.byte	52
	.byte	12
	.byte	0
	.byte	0
	.byte	25
	.byte	25
	.byte	1
	.byte	0
	.byte	0
	.byte	26
	.byte	5
	.byte	0
	.byte	2
	.byte	10
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	27
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	62
	.byte	11
	.byte	11
	.byte	11
	.byte	0
	.byte	0
	.byte	28
	.byte	21
	.byte	1
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	29
	.byte	5
	.byte	0
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	30
	.byte	1
	.byte	1
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	31
	.byte	33
	.byte	0
	.byte	73
	.byte	19
	.byte	34
	.byte	13
	.byte	55
	.byte	11
	.byte	0
	.byte	0
	.byte	32
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	62
	.byte	11
	.byte	0
	.byte	0
	.byte	33
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	64
	.byte	10
	.ascii	"\207@"
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	106
	.byte	12
	.byte	0
	.byte	0
	.byte	0
	.section	__DWARF,__debug_info,regular,debug
Lsection_info:
Lcu_begin0:
.set Lset0, Ldebug_info_end0-Ldebug_info_start0
	.long	Lset0
Ldebug_info_start0:
	.short	2
.set Lset1, Lsection_abbrev-Lsection_abbrev
	.long	Lset1
	.byte	8
	.byte	1
	.long	0
	.short	28
	.long	57
.set Lset2, Lline_table_start0-Lsection_line
	.long	Lset2
	.long	88
	.quad	Lfunc_begin0
	.quad	Lfunc_end15
	.byte	2
	.long	120
	.long	65
	.byte	9
	.byte	3
	.quad	l___unnamed_1
	.byte	3
	.long	91
	.long	120
	.byte	0
	.byte	8
	.byte	4
	.long	127
	.byte	4
	.long	131
	.byte	4
	.long	134
	.byte	5
	.long	145
	.byte	8
	.byte	8
	.byte	6
	.long	155
	.long	593
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	0
	.byte	7
	.quad	Lfunc_begin2
	.quad	Lfunc_end2
	.byte	1
	.byte	86
	.long	446
	.long	430
	.byte	2
	.byte	66
	.long	2384
	.byte	8
	.byte	3
	.byte	145
	.byte	120
	.byte	6
	.long	2190
	.byte	1
	.byte	2
	.byte	61
	.long	593
	.byte	9
	.long	2370
	.long	904
	.byte	0
	.byte	0
	.byte	7
	.quad	Lfunc_begin1
	.quad	Lfunc_end1
	.byte	1
	.byte	86
	.long	387
	.long	372
	.byte	2
	.byte	60
	.long	2377
	.byte	10
	.byte	2
	.byte	145
	.byte	104
	.long	2190
	.byte	2
	.byte	61
	.long	593
	.byte	10
	.byte	2
	.byte	145
	.byte	112
	.long	2307
	.byte	2
	.byte	62
	.long	2377
	.byte	10
	.byte	2
	.byte	145
	.byte	120
	.long	2312
	.byte	2
	.byte	63
	.long	2391
	.byte	9
	.long	2370
	.long	904
	.byte	0
	.byte	0
	.byte	4
	.long	233
	.byte	4
	.long	244
	.byte	11
	.quad	Lfunc_begin0
	.quad	Lfunc_end0
	.byte	1
	.byte	86
	.long	292
	.long	254
	.byte	1
	.byte	121
	.byte	10
	.byte	2
	.byte	145
	.byte	104
	.long	2305
	.byte	1
	.byte	121
	.long	593
	.byte	12
	.quad	Ltmp4
	.quad	Ltmp5
	.byte	13
	.byte	2
	.byte	145
	.byte	96
	.long	169
	.byte	1
	.byte	125
	.long	2370
	.byte	0
	.byte	9
	.long	593
	.long	2280
	.byte	9
	.long	2370
	.long	904
	.byte	0
	.byte	0
	.byte	0
	.byte	4
	.long	519
	.byte	4
	.long	523
	.byte	4
	.long	528
	.byte	4
	.long	536
	.byte	5
	.long	551
	.byte	1
	.byte	1
	.byte	6
	.long	155
	.long	1888
	.byte	1
	.byte	2
	.byte	35
	.byte	0
	.byte	14
	.quad	Lfunc_begin3
	.quad	Lfunc_end3
	.byte	1
	.byte	86
	.long	570
	.long	563
	.byte	3
	.short	438
	.long	2384
	.byte	15
	.byte	2
	.byte	145
	.byte	120
	.long	2334
	.byte	3
	.short	438
	.long	2404
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	4
	.long	528
	.byte	4
	.long	1048
	.byte	14
	.quad	Lfunc_begin12
	.quad	Lfunc_end12
	.byte	1
	.byte	86
	.long	1991
	.long	1984
	.byte	9
	.short	1828
	.long	2384
	.byte	15
	.byte	2
	.byte	145
	.byte	120
	.long	2334
	.byte	9
	.short	1828
	.long	2370
	.byte	0
	.byte	14
	.quad	Lfunc_begin13
	.quad	Lfunc_end13
	.byte	1
	.byte	86
	.long	2078
	.long	1984
	.byte	9
	.short	1862
	.long	2384
	.byte	15
	.byte	2
	.byte	145
	.byte	127
	.long	2334
	.byte	9
	.short	1862
	.long	570
	.byte	0
	.byte	0
	.byte	5
	.long	551
	.byte	1
	.byte	1
	.byte	6
	.long	155
	.long	384
	.byte	1
	.byte	2
	.byte	35
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.long	606
	.long	159
	.long	0
	.byte	17
	.byte	4
	.long	164
	.byte	4
	.long	169
	.byte	18
	.long	176
	.byte	1
	.byte	1
	.byte	19
	.long	183
	.byte	0
	.byte	19
	.long	186
	.byte	1
	.byte	0
	.byte	0
	.byte	4
	.long	190
	.byte	4
	.long	131
	.byte	4
	.long	194
	.byte	18
	.long	197
	.byte	1
	.byte	1
	.byte	19
	.long	207
	.byte	0
	.byte	19
	.long	212
	.byte	1
	.byte	19
	.long	218
	.byte	2
	.byte	19
	.long	225
	.byte	3
	.byte	0
	.byte	5
	.long	1325
	.byte	56
	.byte	8
	.byte	6
	.long	1334
	.long	1978
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	6
	.long	1343
	.long	719
	.byte	8
	.byte	2
	.byte	35
	.byte	8
	.byte	0
	.byte	5
	.long	1350
	.byte	48
	.byte	8
	.byte	6
	.long	846
	.long	1964
	.byte	4
	.byte	2
	.byte	35
	.byte	32
	.byte	6
	.long	856
	.long	653
	.byte	1
	.byte	2
	.byte	35
	.byte	40
	.byte	6
	.long	836
	.long	1957
	.byte	4
	.byte	2
	.byte	35
	.byte	36
	.byte	6
	.long	911
	.long	792
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	6
	.long	862
	.long	792
	.byte	8
	.byte	2
	.byte	35
	.byte	16
	.byte	0
	.byte	5
	.long	1361
	.byte	16
	.byte	8
	.byte	20
	.long	804
	.byte	21
	.long	1971
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	1
	.byte	22
	.byte	0
	.byte	6
	.long	1367
	.long	863
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	0
	.byte	22
	.byte	1
	.byte	6
	.long	1370
	.long	884
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	0
	.byte	22
	.byte	2
	.byte	6
	.long	1376
	.long	905
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	1367
	.byte	16
	.byte	8
	.byte	6
	.long	155
	.long	1978
	.byte	8
	.byte	2
	.byte	35
	.byte	8
	.byte	0
	.byte	5
	.long	1370
	.byte	16
	.byte	8
	.byte	6
	.long	155
	.long	1978
	.byte	8
	.byte	2
	.byte	35
	.byte	8
	.byte	0
	.byte	23
	.long	1376
	.byte	16
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	647
	.byte	16
	.byte	8
	.byte	6
	.long	658
	.long	1895
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	6
	.long	692
	.long	1915
	.byte	8
	.byte	2
	.byte	35
	.byte	8
	.byte	14
	.quad	Lfunc_begin4
	.quad	Lfunc_end4
	.byte	1
	.byte	86
	.long	995
	.long	984
	.byte	4
	.short	267
	.long	915
	.byte	15
	.byte	2
	.byte	145
	.byte	96
	.long	2390
	.byte	4
	.short	267
	.long	2417
	.byte	15
	.byte	2
	.byte	145
	.byte	104
	.long	2305
	.byte	4
	.short	267
	.long	2430
	.byte	9
	.long	1978
	.long	904
	.byte	0
	.byte	0
	.byte	5
	.long	826
	.byte	64
	.byte	8
	.byte	6
	.long	836
	.long	1957
	.byte	4
	.byte	2
	.byte	35
	.byte	48
	.byte	6
	.long	846
	.long	1964
	.byte	4
	.byte	2
	.byte	35
	.byte	52
	.byte	6
	.long	856
	.long	653
	.byte	1
	.byte	2
	.byte	35
	.byte	56
	.byte	6
	.long	862
	.long	1337
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	6
	.long	911
	.long	1337
	.byte	8
	.byte	2
	.byte	35
	.byte	16
	.byte	6
	.long	921
	.long	1109
	.byte	8
	.byte	2
	.byte	35
	.byte	32
	.byte	0
	.byte	5
	.long	925
	.byte	16
	.byte	8
	.byte	24
	.long	936
	.long	1985
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	1
	.byte	24
	.long	120
	.long	1998
	.byte	8
	.byte	2
	.byte	35
	.byte	8
	.byte	1
	.byte	0
	.byte	4
	.long	1044
	.byte	4
	.long	1048
	.byte	7
	.quad	Lfunc_begin5
	.quad	Lfunc_end5
	.byte	1
	.byte	86
	.long	1057
	.long	190
	.byte	5
	.byte	185
	.long	617
	.byte	10
	.byte	2
	.byte	145
	.byte	112
	.long	2334
	.byte	5
	.byte	185
	.long	2417
	.byte	10
	.byte	2
	.byte	145
	.byte	120
	.long	2305
	.byte	5
	.byte	185
	.long	1944
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	1152
	.byte	48
	.byte	8
	.byte	6
	.long	1162
	.long	2031
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	6
	.long	190
	.long	1440
	.byte	8
	.byte	2
	.byte	35
	.byte	16
	.byte	6
	.long	1384
	.long	2172
	.byte	8
	.byte	2
	.byte	35
	.byte	32
	.byte	14
	.quad	Lfunc_begin6
	.quad	Lfunc_end6
	.byte	1
	.byte	86
	.long	1450
	.long	1443
	.byte	4
	.short	313
	.long	1219
	.byte	15
	.byte	2
	.byte	145
	.byte	96
	.long	1162
	.byte	4
	.short	313
	.long	2031
	.byte	15
	.byte	2
	.byte	145
	.byte	112
	.long	1384
	.byte	4
	.short	313
	.long	2172
	.byte	0
	.byte	0
	.byte	0
	.byte	4
	.long	868
	.byte	5
	.long	875
	.byte	16
	.byte	8
	.byte	20
	.long	1349
	.byte	21
	.long	1971
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	1
	.byte	22
	.byte	0
	.byte	6
	.long	893
	.long	1392
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	0
	.byte	22
	.byte	1
	.byte	6
	.long	906
	.long	1409
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	893
	.byte	16
	.byte	8
	.byte	9
	.long	1978
	.long	904
	.byte	0
	.byte	5
	.long	906
	.byte	16
	.byte	8
	.byte	6
	.long	155
	.long	1978
	.byte	8
	.byte	2
	.byte	35
	.byte	8
	.byte	9
	.long	1978
	.long	904
	.byte	0
	.byte	0
	.byte	5
	.long	1220
	.byte	16
	.byte	8
	.byte	20
	.long	1452
	.byte	21
	.long	1971
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	1
	.byte	22
	.byte	0
	.byte	6
	.long	893
	.long	1494
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	0
	.byte	25
	.byte	6
	.long	906
	.long	1511
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	893
	.byte	16
	.byte	8
	.byte	9
	.long	2125
	.long	904
	.byte	0
	.byte	5
	.long	906
	.byte	16
	.byte	8
	.byte	6
	.long	155
	.long	2125
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	9
	.long	2125
	.long	904
	.byte	0
	.byte	0
	.byte	0
	.byte	4
	.long	1500
	.byte	4
	.long	1504
	.byte	4
	.long	1513
	.byte	7
	.quad	Lfunc_begin7
	.quad	Lfunc_end7
	.byte	1
	.byte	86
	.long	1544
	.long	1520
	.byte	6
	.byte	227
	.long	2384
	.byte	26
	.byte	2
	.byte	145
	.byte	120
	.byte	6
	.byte	227
	.long	2459
	.byte	26
	.byte	2
	.byte	145
	.byte	112
	.byte	6
	.byte	227
	.long	2370
	.byte	9
	.long	91
	.long	2295
	.byte	9
	.long	2370
	.long	2300
	.byte	0
	.byte	11
	.quad	Lfunc_begin8
	.quad	Lfunc_end8
	.byte	1
	.byte	86
	.long	1654
	.long	1635
	.byte	6
	.byte	227
	.byte	26
	.byte	2
	.byte	145
	.byte	120
	.byte	6
	.byte	227
	.long	593
	.byte	26
	.byte	2
	.byte	145
	.byte	112
	.byte	6
	.byte	227
	.long	2370
	.byte	9
	.long	593
	.long	2295
	.byte	9
	.long	2370
	.long	2300
	.byte	0
	.byte	7
	.quad	Lfunc_begin9
	.quad	Lfunc_end9
	.byte	1
	.byte	86
	.long	1713
	.long	1520
	.byte	6
	.byte	227
	.long	2384
	.byte	26
	.byte	2
	.byte	145
	.byte	96
	.byte	6
	.byte	227
	.long	91
	.byte	26
	.byte	2
	.byte	145
	.byte	104
	.byte	6
	.byte	227
	.long	2370
	.byte	9
	.long	91
	.long	2295
	.byte	9
	.long	2370
	.long	2300
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	4
	.long	1772
	.byte	11
	.quad	Lfunc_begin10
	.quad	Lfunc_end10
	.byte	1
	.byte	86
	.long	1801
	.long	1776
	.byte	7
	.byte	179
	.byte	26
	.byte	2
	.byte	145
	.byte	120
	.byte	7
	.byte	179
	.long	2459
	.byte	9
	.long	91
	.long	904
	.byte	0
	.byte	0
	.byte	4
	.long	1921
	.byte	11
	.quad	Lfunc_begin11
	.quad	Lfunc_end11
	.byte	1
	.byte	86
	.long	1940
	.long	1926
	.byte	8
	.byte	159
	.byte	10
	.byte	2
	.byte	145
	.byte	120
	.long	2498
	.byte	8
	.byte	159
	.long	2370
	.byte	9
	.long	2370
	.long	904
	.byte	0
	.byte	0
	.byte	0
	.byte	27
	.long	560
	.byte	7
	.byte	1
	.byte	16
	.long	1908
	.long	664
	.long	0
	.byte	23
	.long	685
	.byte	0
	.byte	1
	.byte	16
	.long	1928
	.long	702
	.long	0
	.byte	28
	.long	617
	.byte	29
	.long	1895
	.byte	29
	.long	1944
	.byte	0
	.byte	16
	.long	1023
	.long	800
	.long	0
	.byte	27
	.long	842
	.byte	7
	.byte	4
	.byte	27
	.long	851
	.byte	8
	.byte	4
	.byte	27
	.long	889
	.byte	7
	.byte	8
	.byte	27
	.long	898
	.byte	7
	.byte	8
	.byte	16
	.long	1888
	.long	944
	.long	0
	.byte	16
	.long	2011
	.long	952
	.long	0
	.byte	30
	.long	1978
	.byte	31
	.long	2024
	.byte	0
	.byte	3
	.byte	0
	.byte	32
	.long	964
	.byte	8
	.byte	7
	.byte	5
	.long	1169
	.byte	16
	.byte	8
	.byte	6
	.long	1177
	.long	2065
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	6
	.long	1213
	.long	1978
	.byte	8
	.byte	2
	.byte	35
	.byte	8
	.byte	0
	.byte	16
	.long	2078
	.long	1186
	.long	0
	.byte	5
	.long	1198
	.byte	16
	.byte	8
	.byte	6
	.long	1177
	.long	2112
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	6
	.long	1213
	.long	1978
	.byte	8
	.byte	2
	.byte	35
	.byte	8
	.byte	0
	.byte	16
	.long	1888
	.long	1203
	.long	0
	.byte	5
	.long	1259
	.byte	16
	.byte	8
	.byte	6
	.long	1177
	.long	2159
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	6
	.long	1213
	.long	1978
	.byte	8
	.byte	2
	.byte	35
	.byte	8
	.byte	0
	.byte	16
	.long	685
	.long	1290
	.long	0
	.byte	5
	.long	1389
	.byte	16
	.byte	8
	.byte	6
	.long	1177
	.long	2206
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	6
	.long	1213
	.long	1978
	.byte	8
	.byte	2
	.byte	35
	.byte	8
	.byte	0
	.byte	16
	.long	915
	.long	1414
	.long	0
	.byte	4
	.long	2179
	.byte	33
	.quad	Lfunc_begin14
	.quad	Lfunc_end14
	.byte	1
	.byte	86
	.long	2195
	.long	2190
	.byte	10
	.byte	1
	.byte	1
	.byte	12
	.quad	Ltmp39
	.quad	Ltmp42
	.byte	8
	.byte	2
	.byte	145
	.byte	112
	.long	658
	.byte	1
	.byte	10
	.byte	2
	.long	1978
	.byte	12
	.quad	Ltmp40
	.quad	Ltmp41
	.byte	8
	.byte	2
	.byte	145
	.byte	120
	.long	2504
	.byte	1
	.byte	10
	.byte	3
	.long	2417
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.quad	Lfunc_begin15
	.quad	Lfunc_end15
	.byte	1
	.byte	86
	.long	2236
	.long	528
	.byte	10
	.byte	6
	.long	1978
	.byte	10
	.byte	2
	.byte	145
	.byte	120
	.long	2509
	.byte	10
	.byte	6
	.long	1978
	.byte	0
	.byte	0
	.byte	27
	.long	2282
	.byte	7
	.byte	0
	.byte	27
	.long	2285
	.byte	5
	.byte	8
	.byte	27
	.long	2291
	.byte	5
	.byte	4
	.byte	16
	.long	2112
	.long	2317
	.long	0
	.byte	16
	.long	384
	.long	2339
	.long	0
	.byte	16
	.long	1978
	.long	2392
	.long	0
	.byte	16
	.long	2443
	.long	2399
	.long	0
	.byte	28
	.long	617
	.byte	29
	.long	2417
	.byte	29
	.long	1944
	.byte	0
	.byte	16
	.long	91
	.long	2483
	.long	0
	.byte	0
Ldebug_info_end0:
	.section	__DATA,__const
Lsec_end0:
	.section	__TEXT,__text,regular,pure_instructions
Lsec_end1:
	.section	__DWARF,__debug_aranges,regular,debug
	.long	60
	.short	2
.set Lset3, Lcu_begin0-Lsection_info
	.long	Lset3
	.byte	8
	.byte	0
	.space	4,255
	.quad	l___unnamed_1
.set Lset4, Lsec_end0-l___unnamed_1
	.quad	Lset4
	.quad	Lfunc_begin0
.set Lset5, Lsec_end1-Lfunc_begin0
	.quad	Lset5
	.quad	0
	.quad	0
	.section	__DWARF,__debug_str,regular,debug
Linfo_string:
	.asciz	"clang LLVM (rustc version 1.51.0 (2fd73fabe 2021-03-23))"
	.asciz	"src/main.rs/@/5fcdu18hy1z7nw4b"
	.asciz	"/Users/tchen/.target/debug/deps"
	.asciz	"vtable"
	.asciz	"std"
	.asciz	"rt"
	.asciz	"lang_start"
	.asciz	"closure-0"
	.asciz	"__0"
	.asciz	"fn()"
	.asciz	"core"
	.asciz	"result"
	.asciz	"Result"
	.asciz	"Ok"
	.asciz	"Err"
	.asciz	"fmt"
	.asciz	"v1"
	.asciz	"Alignment"
	.asciz	"Left"
	.asciz	"Right"
	.asciz	"Center"
	.asciz	"Unknown"
	.asciz	"sys_common"
	.asciz	"backtrace"
	.asciz	"__rust_begin_short_backtrace<fn(),()>"
	.asciz	"_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h5472c873967227b0E"
	.asciz	"lang_start<()>"
	.asciz	"_ZN3std2rt10lang_start17h4caa46d899550d98E"
	.asciz	"{{closure}}<()>"
	.asciz	"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h39df29d0fee81e1eE"
	.asciz	"sys"
	.asciz	"unix"
	.asciz	"process"
	.asciz	"process_common"
	.asciz	"ExitCode"
	.asciz	"u8"
	.asciz	"as_i32"
	.asciz	"_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h5193a79422012651E"
	.asciz	"ArgumentV1"
	.asciz	"value"
	.asciz	"&core::fmt::::Opaque"
	.asciz	"Opaque"
	.asciz	"formatter"
	.asciz	"fn(&core::fmt::::Opaque, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>"
	.asciz	"&mut core::fmt::Formatter"
	.asciz	"Formatter"
	.asciz	"flags"
	.asciz	"u32"
	.asciz	"fill"
	.asciz	"char"
	.asciz	"align"
	.asciz	"width"
	.asciz	"option"
	.asciz	"Option<usize>"
	.asciz	"u64"
	.asciz	"None"
	.asciz	"usize"
	.asciz	"T"
	.asciz	"Some"
	.asciz	"precision"
	.asciz	"buf"
	.asciz	"&mut Write"
	.asciz	"pointer"
	.asciz	"*mut u8"
	.asciz	"&[usize; 3]"
	.asciz	"__ARRAY_SIZE_TYPE__"
	.asciz	"new<usize>"
	.asciz	"_ZN4core3fmt10ArgumentV13new17h19c824d6b33ce84eE"
	.asciz	"num"
	.asciz	"{{impl}}"
	.asciz	"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h40a2eb713cf4428eE"
	.asciz	"Arguments"
	.asciz	"pieces"
	.asciz	"&[&str]"
	.asciz	"data_ptr"
	.asciz	"*const &str"
	.asciz	"&str"
	.asciz	"*const u8"
	.asciz	"length"
	.asciz	"Option<&[core::fmt::rt::v1::Argument]>"
	.asciz	"&[core::fmt::rt::v1::Argument]"
	.asciz	"*const core::fmt::rt::v1::Argument"
	.asciz	"Argument"
	.asciz	"position"
	.asciz	"format"
	.asciz	"FormatSpec"
	.asciz	"Count"
	.asciz	"Is"
	.asciz	"Param"
	.asciz	"Implied"
	.asciz	"args"
	.asciz	"&[core::fmt::ArgumentV1]"
	.asciz	"*const core::fmt::ArgumentV1"
	.asciz	"new_v1"
	.asciz	"_ZN4core3fmt9Arguments6new_v117h8a0fa4a3ae033e15E"
	.asciz	"ops"
	.asciz	"function"
	.asciz	"FnOnce"
	.asciz	"call_once<closure-0,()>"
	.asciz	"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h080ec6d6def2a24cE"
	.asciz	"call_once<fn(),()>"
	.asciz	"_ZN4core3ops8function6FnOnce9call_once17h683c3ede57be420eE"
	.asciz	"_ZN4core3ops8function6FnOnce9call_once17hd751dbd7b8922d0fE"
	.asciz	"ptr"
	.asciz	"drop_in_place<closure-0>"
	.asciz	"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hcd5aba0e684c50bdE"
	.asciz	"hint"
	.asciz	"black_box<()>"
	.asciz	"_ZN4core4hint9black_box17h3f2328ddbd61b373E"
	.asciz	"report"
	.asciz	"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hba63e1989e7bb043E"
	.asciz	"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17h7735134e327cd395E"
	.asciz	"simple_asm"
	.asciz	"main"
	.asciz	"_ZN10simple_asm4main17hc4bb5d0ec919f26cE"
	.asciz	"_ZN10simple_asm7process17ha5bf2b7c755a17d1E"
	.asciz	"F"
	.asciz	"()"
	.asciz	"isize"
	.asciz	"i32"
	.asciz	"Self"
	.asciz	"Args"
	.asciz	"f"
	.asciz	"argc"
	.asciz	"argv"
	.asciz	"*const *const u8"
	.asciz	"self"
	.asciz	"&std::sys::unix::process::process_common::ExitCode"
	.asciz	"x"
	.asciz	"&usize"
	.asciz	"fn(&usize, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>"
	.asciz	"*mut closure-0"
	.asciz	"dummy"
	.asciz	"arg0"
	.asciz	"v"
	.section	__DWARF,__apple_names,regular,debug
Lnames_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	15
	.long	31
	.long	12
	.long	0
	.long	1
	.short	1
	.short	6
	.long	0
	.long	1
	.long	3
	.long	6
	.long	11
	.long	13
	.long	16
	.long	17
	.long	19
	.long	21
	.long	23
	.long	25
	.long	29
	.long	30
	.long	-1
	.long	159079455
	.long	2090499946
	.long	-76791450
	.long	266144117
	.long	282823052
	.long	-572965319
	.long	1845844053
	.long	-1752772603
	.long	-1609817953
	.long	-1257172633
	.long	-300363073
	.long	1707704404
	.long	-1992886392
	.long	590015090
	.long	1516626455
	.long	-667561031
	.long	596228451
	.long	-1709124714
	.long	-1674711999
	.long	193491788
	.long	-277830788
	.long	422451489
	.long	-459238702
	.long	2116503325
	.long	-226866906
	.long	1613641256
	.long	1694291036
	.long	-1019809820
	.long	-717901355
	.long	-2065484179
	.long	-115583943
.set Lset6, LNames26-Lnames_begin
	.long	Lset6
.set Lset7, LNames14-Lnames_begin
	.long	Lset7
.set Lset8, LNames13-Lnames_begin
	.long	Lset8
.set Lset9, LNames19-Lnames_begin
	.long	Lset9
.set Lset10, LNames16-Lnames_begin
	.long	Lset10
.set Lset11, LNames23-Lnames_begin
	.long	Lset11
.set Lset12, LNames21-Lnames_begin
	.long	Lset12
.set Lset13, LNames0-Lnames_begin
	.long	Lset13
.set Lset14, LNames27-Lnames_begin
	.long	Lset14
.set Lset15, LNames11-Lnames_begin
	.long	Lset15
.set Lset16, LNames29-Lnames_begin
	.long	Lset16
.set Lset17, LNames3-Lnames_begin
	.long	Lset17
.set Lset18, LNames1-Lnames_begin
	.long	Lset18
.set Lset19, LNames17-Lnames_begin
	.long	Lset19
.set Lset20, LNames5-Lnames_begin
	.long	Lset20
.set Lset21, LNames22-Lnames_begin
	.long	Lset21
.set Lset22, LNames9-Lnames_begin
	.long	Lset22
.set Lset23, LNames20-Lnames_begin
	.long	Lset23
.set Lset24, LNames28-Lnames_begin
	.long	Lset24
.set Lset25, LNames15-Lnames_begin
	.long	Lset25
.set Lset26, LNames24-Lnames_begin
	.long	Lset26
.set Lset27, LNames8-Lnames_begin
	.long	Lset27
.set Lset28, LNames18-Lnames_begin
	.long	Lset28
.set Lset29, LNames25-Lnames_begin
	.long	Lset29
.set Lset30, LNames10-Lnames_begin
	.long	Lset30
.set Lset31, LNames12-Lnames_begin
	.long	Lset31
.set Lset32, LNames7-Lnames_begin
	.long	Lset32
.set Lset33, LNames30-Lnames_begin
	.long	Lset33
.set Lset34, LNames4-Lnames_begin
	.long	Lset34
.set Lset35, LNames2-Lnames_begin
	.long	Lset35
.set Lset36, LNames6-Lnames_begin
	.long	Lset36
LNames26:
	.long	1940
	.long	1
	.long	1833
	.long	0
LNames14:
	.long	2190
	.long	1
	.long	2224
	.long	0
LNames13:
	.long	995
	.long	1
	.long	948
	.long	0
LNames19:
	.long	1443
	.long	1
	.long	1265
	.long	0
LNames16:
	.long	1450
	.long	1
	.long	1265
	.long	0
LNames23:
	.long	984
	.long	1
	.long	948
	.long	0
LNames21:
	.long	1926
	.long	1
	.long	1833
	.long	0
LNames0:
	.long	2195
	.long	1
	.long	2224
	.long	0
LNames27:
	.long	254
	.long	1
	.long	268
	.long	0
LNames11:
	.long	387
	.long	1
	.long	172
	.long	0
LNames29:
	.long	372
	.long	1
	.long	172
	.long	0
LNames3:
	.long	1057
	.long	1
	.long	1155
	.long	0
LNames1:
	.long	1544
	.long	1
	.long	1558
	.long	0
LNames17:
	.long	446
	.long	1
	.long	112
	.long	0
LNames5:
	.long	2078
	.long	1
	.long	519
	.long	0
LNames22:
	.long	570
	.long	1
	.long	404
	.long	0
LNames9:
	.long	120
	.long	1
	.long	46
	.long	0
LNames20:
	.long	1520
	.long	2
	.long	1558
	.long	1698
	.long	0
LNames28:
	.long	1635
	.long	1
	.long	1630
	.long	0
LNames15:
	.long	190
	.long	1
	.long	1155
	.long	0
LNames24:
	.long	1713
	.long	1
	.long	1698
	.long	0
LNames8:
	.long	1984
	.long	2
	.long	469
	.long	519
	.long	0
LNames18:
	.long	292
	.long	1
	.long	268
	.long	0
LNames25:
	.long	430
	.long	1
	.long	112
	.long	0
LNames10:
	.long	563
	.long	1
	.long	404
	.long	0
LNames12:
	.long	1776
	.long	1
	.long	1778
	.long	0
LNames7:
	.long	1654
	.long	1
	.long	1630
	.long	0
LNames30:
	.long	528
	.long	1
	.long	2321
	.long	0
LNames4:
	.long	1991
	.long	1
	.long	469
	.long	0
LNames2:
	.long	2236
	.long	1
	.long	2321
	.long	0
LNames6:
	.long	1801
	.long	1
	.long	1778
	.long	0
	.section	__DWARF,__apple_objc,regular,debug
Lobjc_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	1
	.long	0
	.long	12
	.long	0
	.long	1
	.short	1
	.short	6
	.long	-1
	.section	__DWARF,__apple_namespac,regular,debug
Lnamespac_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	11
	.long	22
	.long	12
	.long	0
	.long	1
	.short	1
	.short	6
	.long	-1
	.long	0
	.long	1
	.long	3
	.long	5
	.long	6
	.long	10
	.long	-1
	.long	13
	.long	14
	.long	17
	.long	193506160
	.long	2090329144
	.long	-712886363
	.long	318227550
	.long	-1229807316
	.long	193502907
	.long	5863852
	.long	193501687
	.long	193506340
	.long	-1430835988
	.long	5863787
	.long	193491788
	.long	2090801609
	.long	-1019809820
	.long	422565636
	.long	2090156110
	.long	-1290020034
	.long	193500757
	.long	373306735
	.long	1883124308
	.long	-735823797
	.long	-126803385
.set Lset37, Lnamespac10-Lnamespac_begin
	.long	Lset37
.set Lset38, Lnamespac16-Lnamespac_begin
	.long	Lset38
.set Lset39, Lnamespac0-Lnamespac_begin
	.long	Lset39
.set Lset40, Lnamespac19-Lnamespac_begin
	.long	Lset40
.set Lset41, Lnamespac9-Lnamespac_begin
	.long	Lset41
.set Lset42, Lnamespac18-Lnamespac_begin
	.long	Lset42
.set Lset43, Lnamespac7-Lnamespac_begin
	.long	Lset43
.set Lset44, Lnamespac15-Lnamespac_begin
	.long	Lset44
.set Lset45, Lnamespac1-Lnamespac_begin
	.long	Lset45
.set Lset46, Lnamespac12-Lnamespac_begin
	.long	Lset46
.set Lset47, Lnamespac5-Lnamespac_begin
	.long	Lset47
.set Lset48, Lnamespac6-Lnamespac_begin
	.long	Lset48
.set Lset49, Lnamespac3-Lnamespac_begin
	.long	Lset49
.set Lset50, Lnamespac4-Lnamespac_begin
	.long	Lset50
.set Lset51, Lnamespac21-Lnamespac_begin
	.long	Lset51
.set Lset52, Lnamespac8-Lnamespac_begin
	.long	Lset52
.set Lset53, Lnamespac20-Lnamespac_begin
	.long	Lset53
.set Lset54, Lnamespac14-Lnamespac_begin
	.long	Lset54
.set Lset55, Lnamespac17-Lnamespac_begin
	.long	Lset55
.set Lset56, Lnamespac13-Lnamespac_begin
	.long	Lset56
.set Lset57, Lnamespac11-Lnamespac_begin
	.long	Lset57
.set Lset58, Lnamespac2-Lnamespac_begin
	.long	Lset58
Lnamespac10:
	.long	127
	.long	1
	.long	76
	.long	0
Lnamespac16:
	.long	1921
	.long	1
	.long	1828
	.long	0
Lnamespac0:
	.long	244
	.long	1
	.long	263
	.long	0
Lnamespac19:
	.long	868
	.long	1
	.long	1332
	.long	0
Lnamespac9:
	.long	233
	.long	1
	.long	258
	.long	0
Lnamespac18:
	.long	1772
	.long	1
	.long	1773
	.long	0
Lnamespac7:
	.long	194
	.long	1
	.long	648
	.long	0
Lnamespac15:
	.long	1500
	.long	1
	.long	1543
	.long	0
Lnamespac1:
	.long	519
	.long	1
	.long	364
	.long	0
Lnamespac12:
	.long	536
	.long	1
	.long	379
	.long	0
Lnamespac5:
	.long	131
	.long	2
	.long	81
	.long	643
	.long	0
Lnamespac6:
	.long	190
	.long	1
	.long	638
	.long	0
Lnamespac3:
	.long	523
	.long	1
	.long	369
	.long	0
Lnamespac4:
	.long	528
	.long	2
	.long	374
	.long	459
	.long	0
Lnamespac21:
	.long	169
	.long	1
	.long	612
	.long	0
Lnamespac8:
	.long	164
	.long	1
	.long	607
	.long	0
Lnamespac20:
	.long	1513
	.long	1
	.long	1553
	.long	0
Lnamespac14:
	.long	1044
	.long	1
	.long	1145
	.long	0
Lnamespac17:
	.long	2179
	.long	1
	.long	2219
	.long	0
Lnamespac13:
	.long	134
	.long	1
	.long	86
	.long	0
Lnamespac11:
	.long	1504
	.long	1
	.long	1548
	.long	0
Lnamespac2:
	.long	1048
	.long	2
	.long	464
	.long	1150
	.long	0
	.section	__DWARF,__apple_types,regular,debug
Ltypes_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	24
	.long	48
	.long	20
	.long	0
	.long	3
	.short	1
	.short	6
	.short	3
	.short	5
	.short	4
	.short	11
	.long	0
	.long	1
	.long	6
	.long	8
	.long	9
	.long	-1
	.long	11
	.long	12
	.long	14
	.long	-1
	.long	16
	.long	18
	.long	23
	.long	25
	.long	29
	.long	31
	.long	32
	.long	34
	.long	36
	.long	38
	.long	40
	.long	42
	.long	43
	.long	46
	.long	-1197510040
	.long	5862433
	.long	262925161
	.long	2089580953
	.long	-1988298567
	.long	-1190517543
	.long	5863826
	.long	1209713282
	.long	596228451
	.long	193506244
	.long	-829766940
	.long	232639254
	.long	203485471
	.long	1150367335
	.long	511671320
	.long	1811514104
	.long	545374306
	.long	2090260330
	.long	193493075
	.long	553511219
	.long	2090147939
	.long	-863125541
	.long	-594775205
	.long	2127712596
	.long	-1799286004
	.long	139308853
	.long	277156213
	.long	707679685
	.long	2089401301
	.long	5861270
	.long	1005944462
	.long	524881599
	.long	-1773357240
	.long	-934778928
	.long	1004366081
	.long	-41616791
	.long	1006996602
	.long	-1416280078
	.long	1762205179
	.long	-713725437
	.long	2087968388
	.long	-1134209084
	.long	-1449878611
	.long	217729102
	.long	1413919846
	.long	-1535681082
	.long	193506143
	.long	1581627311
.set Lset59, Ltypes31-Ltypes_begin
	.long	Lset59
.set Lset60, Ltypes39-Ltypes_begin
	.long	Lset60
.set Lset61, Ltypes4-Ltypes_begin
	.long	Lset61
.set Lset62, Ltypes16-Ltypes_begin
	.long	Lset62
.set Lset63, Ltypes15-Ltypes_begin
	.long	Lset63
.set Lset64, Ltypes18-Ltypes_begin
	.long	Lset64
.set Lset65, Ltypes8-Ltypes_begin
	.long	Lset65
.set Lset66, Ltypes32-Ltypes_begin
	.long	Lset66
.set Lset67, Ltypes26-Ltypes_begin
	.long	Lset67
.set Lset68, Ltypes25-Ltypes_begin
	.long	Lset68
.set Lset69, Ltypes24-Ltypes_begin
	.long	Lset69
.set Lset70, Ltypes40-Ltypes_begin
	.long	Lset70
.set Lset71, Ltypes20-Ltypes_begin
	.long	Lset71
.set Lset72, Ltypes2-Ltypes_begin
	.long	Lset72
.set Lset73, Ltypes17-Ltypes_begin
	.long	Lset73
.set Lset74, Ltypes41-Ltypes_begin
	.long	Lset74
.set Lset75, Ltypes22-Ltypes_begin
	.long	Lset75
.set Lset76, Ltypes30-Ltypes_begin
	.long	Lset76
.set Lset77, Ltypes38-Ltypes_begin
	.long	Lset77
.set Lset78, Ltypes37-Ltypes_begin
	.long	Lset78
.set Lset79, Ltypes23-Ltypes_begin
	.long	Lset79
.set Lset80, Ltypes44-Ltypes_begin
	.long	Lset80
.set Lset81, Ltypes43-Ltypes_begin
	.long	Lset81
.set Lset82, Ltypes10-Ltypes_begin
	.long	Lset82
.set Lset83, Ltypes5-Ltypes_begin
	.long	Lset83
.set Lset84, Ltypes14-Ltypes_begin
	.long	Lset84
.set Lset85, Ltypes12-Ltypes_begin
	.long	Lset85
.set Lset86, Ltypes0-Ltypes_begin
	.long	Lset86
.set Lset87, Ltypes11-Ltypes_begin
	.long	Lset87
.set Lset88, Ltypes13-Ltypes_begin
	.long	Lset88
.set Lset89, Ltypes33-Ltypes_begin
	.long	Lset89
.set Lset90, Ltypes19-Ltypes_begin
	.long	Lset90
.set Lset91, Ltypes28-Ltypes_begin
	.long	Lset91
.set Lset92, Ltypes7-Ltypes_begin
	.long	Lset92
.set Lset93, Ltypes21-Ltypes_begin
	.long	Lset93
.set Lset94, Ltypes3-Ltypes_begin
	.long	Lset94
.set Lset95, Ltypes42-Ltypes_begin
	.long	Lset95
.set Lset96, Ltypes9-Ltypes_begin
	.long	Lset96
.set Lset97, Ltypes45-Ltypes_begin
	.long	Lset97
.set Lset98, Ltypes29-Ltypes_begin
	.long	Lset98
.set Lset99, Ltypes47-Ltypes_begin
	.long	Lset99
.set Lset100, Ltypes27-Ltypes_begin
	.long	Lset100
.set Lset101, Ltypes6-Ltypes_begin
	.long	Lset101
.set Lset102, Ltypes36-Ltypes_begin
	.long	Lset102
.set Lset103, Ltypes35-Ltypes_begin
	.long	Lset103
.set Lset104, Ltypes1-Ltypes_begin
	.long	Lset104
.set Lset105, Ltypes46-Ltypes_begin
	.long	Lset105
.set Lset106, Ltypes34-Ltypes_begin
	.long	Lset106
Ltypes31:
	.long	1325
	.long	1
	.long	685
	.short	19
	.byte	0
	.long	0
Ltypes39:
	.long	1367
	.long	1
	.long	863
	.short	19
	.byte	0
	.long	0
Ltypes4:
	.long	2285
	.long	1
	.long	2377
	.short	36
	.byte	0
	.long	0
Ltypes16:
	.long	906
	.long	2
	.long	1409
	.short	19
	.byte	0
	.long	1511
	.short	19
	.byte	0
	.long	0
Ltypes15:
	.long	826
	.long	1
	.long	1023
	.short	19
	.byte	0
	.long	0
Ltypes18:
	.long	1350
	.long	1
	.long	719
	.short	19
	.byte	0
	.long	0
Ltypes8:
	.long	560
	.long	1
	.long	1888
	.short	36
	.byte	0
	.long	0
Ltypes32:
	.long	1169
	.long	1
	.long	2031
	.short	19
	.byte	0
	.long	0
Ltypes26:
	.long	120
	.long	1
	.long	65
	.short	19
	.byte	0
	.long	0
Ltypes25:
	.long	889
	.long	1
	.long	1971
	.short	36
	.byte	0
	.long	0
Ltypes24:
	.long	176
	.long	1
	.long	617
	.short	4
	.byte	0
	.long	0
Ltypes40:
	.long	1370
	.long	1
	.long	884
	.short	19
	.byte	0
	.long	0
Ltypes20:
	.long	2483
	.long	1
	.long	2459
	.short	15
	.byte	0
	.long	0
Ltypes2:
	.long	2399
	.long	1
	.long	2430
	.short	15
	.byte	0
	.long	0
Ltypes17:
	.long	1414
	.long	1
	.long	2206
	.short	15
	.byte	0
	.long	0
Ltypes41:
	.long	2339
	.long	1
	.long	2404
	.short	15
	.byte	0
	.long	0
Ltypes22:
	.long	664
	.long	1
	.long	1895
	.short	15
	.byte	0
	.long	0
Ltypes30:
	.long	159
	.long	1
	.long	593
	.short	15
	.byte	0
	.long	0
Ltypes38:
	.long	2291
	.long	1
	.long	2384
	.short	36
	.byte	0
	.long	0
Ltypes37:
	.long	1259
	.long	1
	.long	2125
	.short	19
	.byte	0
	.long	0
Ltypes23:
	.long	851
	.long	1
	.long	1964
	.short	36
	.byte	0
	.long	0
Ltypes44:
	.long	1152
	.long	1
	.long	1219
	.short	19
	.byte	0
	.long	0
Ltypes43:
	.long	964
	.long	1
	.long	2024
	.short	36
	.byte	0
	.long	0
Ltypes10:
	.long	2317
	.long	1
	.long	2391
	.short	15
	.byte	0
	.long	0
Ltypes5:
	.long	925
	.long	1
	.long	1109
	.short	19
	.byte	0
	.long	0
Ltypes14:
	.long	1186
	.long	1
	.long	2065
	.short	15
	.byte	0
	.long	0
Ltypes12:
	.long	898
	.long	1
	.long	1978
	.short	36
	.byte	0
	.long	0
Ltypes0:
	.long	1389
	.long	1
	.long	2172
	.short	19
	.byte	0
	.long	0
Ltypes11:
	.long	893
	.long	2
	.long	1392
	.short	19
	.byte	0
	.long	1494
	.short	19
	.byte	0
	.long	0
Ltypes13:
	.long	2282
	.long	1
	.long	2370
	.short	36
	.byte	0
	.long	0
Ltypes33:
	.long	702
	.long	1
	.long	1915
	.short	15
	.byte	0
	.long	0
Ltypes19:
	.long	145
	.long	1
	.long	91
	.short	19
	.byte	0
	.long	0
Ltypes28:
	.long	875
	.long	1
	.long	1337
	.short	19
	.byte	0
	.long	0
Ltypes7:
	.long	685
	.long	1
	.long	1908
	.short	19
	.byte	0
	.long	0
Ltypes21:
	.long	952
	.long	1
	.long	1998
	.short	15
	.byte	0
	.long	0
Ltypes3:
	.long	1376
	.long	1
	.long	905
	.short	19
	.byte	0
	.long	0
Ltypes42:
	.long	551
	.long	2
	.long	384
	.short	19
	.byte	0
	.long	570
	.short	19
	.byte	0
	.long	0
Ltypes9:
	.long	944
	.long	1
	.long	1985
	.short	15
	.byte	0
	.long	0
Ltypes45:
	.long	2392
	.long	1
	.long	2417
	.short	15
	.byte	0
	.long	0
Ltypes29:
	.long	1203
	.long	1
	.long	2112
	.short	15
	.byte	0
	.long	0
Ltypes47:
	.long	1198
	.long	1
	.long	2078
	.short	19
	.byte	0
	.long	0
Ltypes27:
	.long	197
	.long	1
	.long	653
	.short	4
	.byte	0
	.long	0
Ltypes6:
	.long	800
	.long	1
	.long	1944
	.short	15
	.byte	0
	.long	0
Ltypes36:
	.long	1361
	.long	1
	.long	792
	.short	19
	.byte	0
	.long	0
Ltypes35:
	.long	1220
	.long	1
	.long	1440
	.short	19
	.byte	0
	.long	0
Ltypes1:
	.long	1290
	.long	1
	.long	2159
	.short	15
	.byte	0
	.long	0
Ltypes46:
	.long	842
	.long	1
	.long	1957
	.short	36
	.byte	0
	.long	0
Ltypes34:
	.long	647
	.long	1
	.long	915
	.short	19
	.byte	0
	.long	0
.subsections_via_symbols
	.section	__DWARF,__debug_line,regular,debug
Lsection_line:
Lline_table_start0:
