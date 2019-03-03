//! bare1.rs
//!
//! Inspecting the generated assembly
//!
//! What it covers
//! - tracing over semihosting and ITM
//! - assembly calls and inline assembly
//! - more on arithmetics

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::{iprintln, Peripherals};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
#[inline(never)]
fn main() -> ! {
    // Prepend by `x` by _ to avoid warning (never used).
    // The compiler is smart enough to figure out that
    // `x` is not used in any menaningful way.

    let mut _x:i32 = 0;
    loop {
        let mut p = Peripherals::take().unwrap();
        _x += 1;
        let stim = &mut p.ITM.stim[0];

        iprintln!(stim, "{}", _x);
        hprintln!("{}", _x).unwrap();

        cortex_m::asm::nop();
        cortex_m::asm::bkpt();
        _x -= 1;
        
    }
}

// 0. Setup
//    For this example we will use the `nightly` compiler
//    to get inline assembly.
//    (Inline assembly is currently not stabelized.)
//
//    > rustup override set nightly
//
//    In the `Corgo.toml` file, uncomment
//    # features = ["inline-asm"] # <- currently requires nightly compiler
//
//    You may need/want to install addititonal components also,
//    to that end look at the install section in the README.md.
//    If you change toolchain, exit and re-start `vscode`.
//
// 1. Build and run the application
//
//    > cargo build --example bare1
//    (or use the vscode build task)
//
//    Look at the `hello.rs` and `itm.rs` examples to setup the tracing.
//
//    When debugging the application it should get stuck in the
//    loop, (press pause/suspend to verify this).
//    what is the output in the ITM console
//
//    The output is 1.
//
//    What is the output in the semihosting (openocd) console
//    The output is also 1.
//
//    Commit your answers (bare1_1)
//
// 2. Inspecting the generated assembly code
//    If in `vcsode` the gdb console in DEBUG CONSOLE
//
//    What is the output of:
//    (gdb) disassemble
//
//{"token":40,"outOfBandRecord":[],"resultRecords":{"resultClass":"done","results":[]}}
//Dump of assembler code for function rust_begin_unwind:
//   0x08003420 <+0>:	sub	sp, #16
//   0x08003422 <+2>:	mov	r1, r0
//  0x08003424 <+4>:	str	r0, [sp, #8]
//   0x08003426 <+6>:	str	r1, [sp, #4]
//   0x08003428 <+8>:	b.n	0x800342a <rust_begin_unwind+10>
//   0x0800342a <+10>:	movs	r0, #4
//   0x0800342c <+12>:	strb.w	r0, [sp, #15]
//   0x08003430 <+16>:	ldrb.w	r0, [sp, #15]
//   0x08003434 <+20>:	bl	0x800343c <core::sync::atomic::compiler_fence>
//=> 0x08003438 <+24>:	b.n	0x800343a <rust_begin_unwind+26>
//   0x0800343a <+26>:	b.n	0x800342a <rust_begin_unwind+10>
End of assembler dump.
//
//    Commit your answers (bare1_2)
//
// 3. Now remove the comment for `cortex_m::asm::nop()`.
//    Rebuild and debug, pause the program.
//
//    What is the output of:
//    (gdb) disassemble
//
//    {"token":54,"outOfBandRecord":[],"resultRecords":{"resultClass":"done","results":[]}}
//Dump of assembler code for function rust_begin_unwind:
//   0x0800342a <+0>:	sub	sp, #16
//   0x0800342c <+2>:	mov	r1, r0
//   0x0800342e <+4>:	str	r0, [sp, #8]
//   0x08003430 <+6>:	str	r1, [sp, #4]
//   0x08003432 <+8>:	b.n	0x8003434 <rust_begin_unwind+10>
//   0x08003434 <+10>:	movs	r0, #4
//   0x08003436 <+12>:	strb.w	r0, [sp, #15]
//   0x0800343a <+16>:	ldrb.w	r0, [sp, #15]
//   0x0800343e <+20>:	bl	0x8003448 <core::sync::atomic::compiler_fence>
//=> 0x08003442 <+24>:	b.n	0x8003444 <rust_begin_unwind+26>
//   0x08003444 <+26>:	b.n	0x8003434 <rust_begin_unwind+10>
//  End of assembler dump.
//
//    Commit your answers (bare1_3)
//
// 4. Now remove the comment for `cortex_m::asm::bkpt()`
//    Rebuild and debug, let the program run until it halts.
//
//    What is the output of:
//    (gdb) disassemble
//
//    {"token":43,"outOfBandRecord":[],"resultRecords":{"resultClass":"done","results":[]}}
//Dump of assembler code for function main:
//   0x08000464 <+0>:	sub	sp, #136	; 0x88
//   0x08000466 <+2>:	movs	r0, #0
//  0x08000468 <+4>:	str	r0, [sp, #44]	; 0x2c
//  0x0800046a <+6>:	b.n	0x800046c <main+8>
//   0x0800046c <+8>:	bl	0x8000400 <cortex_m::peripheral::Peripherals::take>
//   0x08000470 <+12>:	str	r0, [sp, #40]	; 0x28
//   0x08000472 <+14>:	b.n	0x8000474 <main+16>
//   0x08000474 <+16>:	ldr	r0, [sp, #40]	; 0x28
//   0x08000476 <+18>:	and.w	r0, r0, #1
//   0x0800047a <+22>:	bl	0x800057e <<core::option::Option<T>>::unwrap>
//   0x080004aa <+70>:	str	r1, [sp, #92]	; 0x5c
//   0x080004ac <+72>:	ldr	r1, [sp, #92]	; 0x5c
//   0x080004ae <+74>:	movw	r2, #13325	; 0x340d
//   0x080004b2 <+78>:	movt	r2, #2048	; 0x800
//   0x080004b6 <+82>:	str	r0, [sp, #28]
//   0x080004b8 <+84>:	mov	r0, r1
//   0x080004ba <+86>:	mov	r1, r2
//   0x080004bc <+88>:	bl	0x80007bc <core::fmt::ArgumentV1::new>
//   0x080004c0 <+92>:	str	r0, [sp, #24]
//   0x080004c2 <+94>:	str	r1, [sp, #20]
//   0x080004c4 <+96>:	b.n	0x80004c6 <main+98>
//   0x080004c6 <+98>:	ldr	r0, [sp, #24]
//   0x080004c8 <+100>:	str	r0, [sp, #80]	; 0x50
//   0x080004ca <+102>:	ldr	r1, [sp, #20]
//   0x080004cc <+104>:	str	r1, [sp, #84]	; 0x54
//   0x080004ce <+106>:	mov	r2, sp
//   0x080004d0 <+108>:	movs	r3, #1
//   0x080004d2 <+110>:	str	r3, [r2, #0]
//   0x080004d4 <+112>:	movw	r1, #14424	; 0x3858
//   0x080004d8 <+116>:	movt	r1, #2048	; 0x800
//   0x080004dc <+120>:	add	r0, sp, #56	; 0x38
//   0x080004de <+122>:	movs	r2, #2
//   0x080004e0 <+124>:	add	r3, sp, #80	; 0x50
//   0x080004e2 <+126>:	bl	0x800062e <core::fmt::Arguments::new_v1>
//   0x080004e6 <+130>:	b.n	0x80004e8 <main+132>
//   0x080004e8 <+132>:	add	r1, sp, #56	; 0x38
//   0x080004ea <+134>:	ldr	r0, [sp, #28]
//   0x080004ec <+136>:	bl	0x8001e36 <cortex_m::itm::write_fmt>
//   0x080004f0 <+140>:	b.n	0x80004f2 <main+142>
//   0x080004f2 <+142>:	add	r0, sp, #44	; 0x2c
//   0x080004f4 <+144>:	str	r0, [sp, #128]	; 0x80
//   0x080004f6 <+146>:	ldr	r0, [sp, #128]	; 0x80
//   0x080004f8 <+148>:	str	r0, [sp, #132]	; 0x84
//   0x080004fa <+150>:	ldr	r0, [sp, #132]	; 0x84
//   0x080004fc <+152>:	movw	r1, #13325	; 0x340d
//   0x08000500 <+156>:	movt	r1, #2048	; 0x800
//   0x08000504 <+160>:	bl	0x80007bc <core::fmt::ArgumentV1::new>
//   0x08000508 <+164>:	str	r0, [sp, #16]
//   0x0800050a <+166>:	str	r1, [sp, #12]
//   0x0800050c <+168>:	b.n	0x800050e <main+170>
//   0x0800050e <+170>:	ldr	r0, [sp, #16]
//   0x08000510 <+172>:	str	r0, [sp, #120]	; 0x78
//   0x08000512 <+174>:	ldr	r1, [sp, #12]
//   0x08000514 <+176>:	str	r1, [sp, #124]	; 0x7c
//   0x08000516 <+178>:	mov	r2, sp
//   0x08000518 <+180>:	movs	r3, #1
//   0x0800051a <+182>:	str	r3, [r2, #0]
//   0x0800051c <+184>:	movw	r1, #14424	; 0x3858
//   0x08000520 <+188>:	movt	r1, #2048	; 0x800
//   0x08000524 <+192>:	add	r0, sp, #96	; 0x60
//   0x08000526 <+194>:	movs	r2, #2
//   0x08000528 <+196>:	add	r3, sp, #120	; 0x78
//   0x0800052a <+198>:	bl	0x800062e <core::fmt::Arguments::new_v1>
//   0x0800052e <+202>:	b.n	0x8000530 <main+204>
//   0x08000530 <+204>:	add	r0, sp, #96	; 0x60
//   0x08000532 <+206>:	bl	0x8000852 <cortex_m_semihosting::export::hstdout_fmt>
//   0x08000536 <+210>:	str	r0, [sp, #8]
//   0x08000538 <+212>:	b.n	0x800053a <main+214>
//   0x0800053a <+214>:	ldr	r0, [sp, #8]
//   0x0800053c <+216>:	and.w	r0, r0, #1
//   0x08000540 <+220>:	bl	0x80006c0 <<core::result::Result<T, E>>::unwrap>
//   0x08000544 <+224>:	b.n	0x8000546 <main+226>
//   0x08000546 <+226>:	bl	0x80006bc <cortex_m::asm::nop>
//   0x0800054a <+230>:	b.n	0x800054c <main+232>
//=> 0x0800054c <+232>:	bkpt	0x0000
//   0x0800054e <+234>:	b.n	0x8000550 <main+236>
//   0x08000550 <+236>:	ldr	r0, [sp, #44]	; 0x2c
//   0x08000552 <+238>:	subs	r1, r0, #1
//   0x08000554 <+240>:	cmp	r0, #1
//   0x08000556 <+242>:	str	r1, [sp, #4]
//   0x08000558 <+244>:	bvs.n	0x8000570 <main+268>
//   0x0800055a <+246>:	b.n	0x800055c <main+248>
//   0x0800055c <+248>:	ldr	r0, [sp, #4]
//   0x0800055e <+250>:	str	r0, [sp, #44]	; 0x2c
//   0x08000560 <+252>:	b.n	0x800046c <main+8>
//   0x08000562 <+254>:	movw	r0, #14396	; 0x383c
//   0x08000566 <+258>:	movt	r0, #2048	; 0x800
//   0x0800056a <+262>:	bl	0x80029ca <panic>
//   0x0800056e <+266>:	udf	#254	; 0xfe
//   0x08000570 <+268>:	movw	r0, #14484	; 0x3894
//   0x08000574 <+272>:	movt	r0, #2048	; 0x800
//   0x08000578 <+276>:	bl	0x80029ca <panic>
//   0x0800057c <+280>:	udf	#254	; 0xfe
// End of assembler dump.
//
//    Commit your answers (bare1_4)
//
// 5. Release mode (optimized builds).
//    Rebuild `bare1.rs` in release (optimized mode).
//  
//    > cargo build --example bare1 --release
//    (or using the vscode build task)
//
//    Compare the generated assembly for the loop
//    between the dev (unoptimized) and release (optimized) build.
//
//    ** your answer here **
//
//    commit your answers (bare1_5)
//
//    Tips: The optimized build should have 3 instructions
//    while the debug (dev) build should have > 20 instructions
//    (both counting the inner loop only). The debug build
//    should have additional code that call panic if the additon
//    wraps (and in such case call panic).
//
//    Discussion:
//    In release (optimized) mode the addition is unchecked,
//    so there is a semantic difference here in between
//    the dev and release modes. This is motivited by:
//    1) efficiency, unchecked is faster
//    2) convenience, it would be inconvenient to explicitly use
//    wrapping arithmetics, and wrapping is what the programmer
//    typically would expect in any case. So the check
//    in dev/debug mode is just there for some extra safety
//    if your intention is NON-wrapping arithmetics.
//
// 6. *Optional
//    You can pass additional flags to the Rust `rustc` compiler.
//
//    `-Z force-overflow-checks=off`
//
//    Under this flag, code is never generated for oveflow checking.
//    You can enable this flag (uncomment the corresponding flag in
//    the `.cargo/config` file.)
//
//    What is now the disassembly of the loop (in debug mode):
//
//    ** your answer here **
//
//    commit your answers (bare1_6)
//
//    Now restore the `.cargo/config` to its original state.
//
// 7. *Optional
//    There is another way to conveniently use wrapping arithmetics
//    without passing flags to the compiler.
//
//    https://doc.rust-lang.org/std/num/struct.Wrapping.html
//
//    Rewrite the code using this approach.
//
//    What is now the disassembly of the code in dev mode?
//
//    ** your answer here **
//
//    What is now the disassembly of the code in release mode?
//
//    ** your answer here **
//
//    commit your answers (bare1_7)
//
//    Final discussion:
//
//    Embedded code typically is performance sensitve, hence
//    it is important to understand how code is generated
//    to achieve efficient implementations.
//
//    Moreover, arithmetics are key to processing of data,
//    so its important that we are in control over the
//    computations. E.g. comupting checksums, hashes, cryptos etc.
//    all require precise control over wrapping vs. overflow behaviour.
//
//    If you write a library depending on wrapping arithmetics
//    do NOT rely on a compiler flag. (The end user might compile
//    it without this flag enabled, and thus get erronous results.)
//
//    NOTICE:
//    ------
//    You are now on a `nightly` release of the compiler for good and bad.
//    You can chose to switch back to the stable channel. If so you must
//    restore the `Cargo.toml` (comment out the `features = ["inline-asm"]`)
//
//    Pros and cons of nightly:
//    + Acccess to new Rust features (such as inline assembly)
//    - No guarantee these features will work, they might change semantics,
//      or even be revoked.
//
//    The compiler itself is the same, the stable release is just a snapchot
//    of the nightly (released each 6 week). It is the latest nightly
//    that passed some additional regression test, not a different compiler.
//    And of course, the stable has the experimental features disabled.
//
//    So its up to you to decide if you want to use the stable or nightly.
