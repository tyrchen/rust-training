	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 7
	.p2align	4, 0x90
__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h5472c873967227b0E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	callq	*%rdi
	leaq	-8(%rbp), %rax
	## InlineAsm Start
	## InlineAsm End
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.private_extern	__ZN3std2rt10lang_start17h4caa46d899550d98E
	.globl	__ZN3std2rt10lang_start17h4caa46d899550d98E
	.p2align	4, 0x90
__ZN3std2rt10lang_start17h4caa46d899550d98E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, -8(%rbp)
	leaq	l___unnamed_1(%rip), %rsi
	leaq	-8(%rbp), %rdi
	callq	__ZN3std2rt19lang_start_internal17h86f505dc7de50d93E
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h39df29d0fee81e1eE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movq	(%rdi), %rdi
	callq	__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h5472c873967227b0E
	xorl	%eax, %eax
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h9c086c8a73faafe5E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r14
	pushq	%rbx
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	movq	%rsi, %rbx
	movq	%rdi, %r14
	movq	%rsi, %rdi
	callq	__ZN4core3fmt9Formatter15debug_lower_hex17h711d6e847f2fb68dE
	testb	%al, %al
	je	LBB3_1
	movq	%r14, %rdi
	movq	%rbx, %rsi
	popq	%rbx
	popq	%r14
	popq	%rbp
	jmp	__ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17hd85567a5614f1f15E
LBB3_1:
	movq	%rbx, %rdi
	callq	__ZN4core3fmt9Formatter15debug_upper_hex17h699081381c0e441bE
	movq	%r14, %rdi
	movq	%rbx, %rsi
	testb	%al, %al
	je	LBB3_2
	popq	%rbx
	popq	%r14
	popq	%rbp
	jmp	__ZN4core3fmt3num55_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$usize$GT$3fmt17h16e9342ec3f451c5E
LBB3_2:
	popq	%rbx
	popq	%r14
	popq	%rbp
	jmp	__ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h5e5dc183b103235dE
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h080ec6d6def2a24cE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movq	(%rdi), %rdi
	callq	__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h5472c873967227b0E
	xorl	%eax, %eax
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hcd5aba0e684c50bdE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN10simple_asm4main17hc4bb5d0ec919f26cE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$80, %rsp
	movq	$48879, -8(%rbp)
	leaq	-8(%rbp), %rax
	movq	%rax, -24(%rbp)
	leaq	__ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h9c086c8a73faafe5E(%rip), %rax
	movq	%rax, -16(%rbp)
	leaq	l___unnamed_2(%rip), %rax
	movq	%rax, -72(%rbp)
	movq	$2, -64(%rbp)
	movq	$0, -56(%rbp)
	leaq	-24(%rbp), %rax
	movq	%rax, -40(%rbp)
	movq	$1, -32(%rbp)
	leaq	-72(%rbp), %rdi
	callq	__ZN3std2io5stdio6_print17hea90da9dad6f182dE
	addq	$80, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.globl	_process
	.p2align	4, 0x90
_process:
	.cfi_startproc
	movq	%rdi, %rcx
	shrq	$16, %rcx
	je	LBB7_2
	movq	%rdi, %rax
	xorl	%edx, %edx
	divq	%rcx
	movq	%rdx, %rax
	retq
LBB7_2:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	leaq	_str.0(%rip), %rdi
	leaq	l___unnamed_3(%rip), %rdx
	movl	$57, %esi
	callq	__ZN4core9panicking5panic17hfa08580418a71d7fE
	.cfi_endproc

	.globl	_main
	.p2align	4, 0x90
_main:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rsi, %rcx
	movslq	%edi, %rdx
	leaq	__ZN10simple_asm4main17hc4bb5d0ec919f26cE(%rip), %rax
	movq	%rax, -8(%rbp)
	leaq	l___unnamed_1(%rip), %rsi
	leaq	-8(%rbp), %rdi
	callq	__ZN3std2rt19lang_start_internal17h86f505dc7de50d93E
	addq	$16, %rsp
	popq	%rbp
	retq
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
	.asciz	"\013\000\000\000\000\000\000\000\b\000\000\000\005\000\000"

	.section	__TEXT,__const
	.p2align	4
_str.0:
	.ascii	"attempt to calculate the remainder with a divisor of zero"

.subsections_via_symbols
