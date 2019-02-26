//! bare0.rs
//!
//! Simple bare metal application
//! What it covers:
//! - constants
//! - global (static) variables
//! - checked vs. wrapping arithmetics
//! - safe and unsafe code
//! - making a safe API

// build without the Rust standard library
#![no_std]
// no standard main, we declare main using [entry]
#![no_main]

extern crate panic_halt;

// Minimal runtime / startup for Cortex-M microcontrollers
use cortex_m_rt::entry;

// a constant (cannot be changed at run-time)

// global mutabale variables (changed using unsafe co




#[entry]
fn main() -> ! {
    let mut X: i32=10;
    let mut Y: i32=0;
    // local mutabale variable (changed in safe code)
    let mut x: i32=X;

    loop {
        x = x.wrapping_add(1); // <- place breakpoint here (3)
            X = X.wrapping_add(1);
            Y = X;
        
    }
}

// 0. Compile/build the example in debug (dev) mode.
//
//    > cargo build --example bare0
//    (or use the vscode build task)
//
// 1. Run the program in the debugger, let the program run for a while and
//    then press pause. Look in the (Local -vscode) Variables view what do you find.
//
//    We find the value of x
//
//    In the Expressions (WATCH -vscode) view add X and Y
//    what do you find
//
//    We find the values of X and Y, which are often equals to x.
//
//    Step through one complete iteration of the loop
//    and see how the (Local) Variables are updated
//    can you foresee what will eventually happen?
//
// 	  The program start by incrementing x, then X and finally Y.
//    It means that these three values won't always be the same,
//    depending on when we stop.
//
//    Commit your answers (bare0_1)
//
// 2. Alter the constant X_INIT so that `x += 1` directly causes `x` to wrap
// 	  what happens when `x` wraps
//
//    When x wraps, we get a panic.
//
//    Commit your answers (bare0_2)
//
// 3. Place a breakpoint at `x += 1`
//
//    Change (both) += opertions to use wrapping_add
//    load and run the progam, what happens
//    Instead of overflow, x go back to 0.
//
//    Now continue exectution, what happens
//    Values goes into a loop from 0 up to 4294967295
//
//    Commit your answers (bare0_3)
//
//    (If the program did not succeed back to the breakpoint
//    you have some fault in the program and go back to 3.)
//
// 4. Change the asserion to `assert!(x == X && X == Y + 1)`, what happens?
//
//    Because the condition in assert! is wrong, we get a panic.
//
//    Commit your answers (bare0_4)
//
// 5. Remove the assertion and implement "safe" functions for
//    reading and writing X and Y
//    e.g. read_x, read_y, write_x, write_y
//
//    Rewrite the program to use ONLY "safe" code besides the
//    read/write functions (which are internally "unsafe")
//
//    Commit your solution (bare0_5)
//
// 6. *Optional
//    Implement a read_u32/write_u32, taking a reference to a
//    "static" variable
//
//    Rewrite the program to use this abstraction instead of "read_x", etc.
//
//    Commit your solution (bare0_6)
//
