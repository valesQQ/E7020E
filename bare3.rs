//! bare3.rs
//!
//! String types in Rust
//!
//! What it covers:
//! - Types, str, arrays ([u8;uszie]), slices (&[u8])
//! - Iteration, copy
//! - Semihosting (tracing)

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("bare3").unwrap();
    let s: &'static str = "ABCD";
    let bs: &[u8] = s.as_bytes();
    let _c: &[u8];
    let _i: i32;

    hprintln!("s = {}", s).unwrap();
    hprintln!("bs = {:?}", bs).unwrap();

    hprintln!("iterate over slice").unwrap();
    for _c in bs {
        hprint!("{},", _c).unwrap();
    }

    hprintln!("iterate iterate using (raw) indexing").unwrap();
    for _i in 0..s.len() {
        hprintln!("{},", bs[_i]).unwrap();
    }

    hprintln!("").unwrap();

    let a: [u8; 4] = [65u8; 4];
    let mut a = [0u8; 4];

    hprintln!("").unwrap();
    hprintln!("a = {}", core::str::from_utf8(&a).unwrap()).unwrap();

    loop {}
}

// 0. Build and run the application (debug build).
// 
//    > cargo build --example bare3
//    (or use the vscode build task)
//
// 1. What is the output in the `openocd` (Adapter Output) console?
//
//  The output is:
//
//  bare3
//  s = ABCD
//  bs = [65, 66, 67, 68]
//  iterate over slice
//  65,66,67,68,iterate iterate using (raw) indexing
//  65,
//  66,
//  67,
//  68,
//
//  a = AAAA
//
//    What is the type of `s`?
//
//    's' is a String
//
//    What is the type of `bs`?
//      'bs' is an array
//    
//
//    What is the type of `c`?
//
//      'c' is a slice
//
//    What is the type of `a`?
//
//      'a' is also an array   
//
//    What is the type of `i`?
//
//      'i' is an integer
//
//    Commit your answers (bare3_1)
//
// 2. Make types of `s`, `bs`, `c`, `a`, `i` explicit.
//
//    Commit your answers (bare3_2)
//
// 3. Uncomment line `let mut a = [0u8; 4];
//`
//    Run the program, what happens and why?
//
//    a is full of 0. It is due to the "0u8", which is the number 0 as an unsigned 8-bit integer
//
//    Commit your answers (bare3_3)
//
// 4. Alter the program so that the data from `bs` is copied byte by byte into `a`.
//
//    Test that it works as intended.
//
//    Commit your answers (bare3_4)
//
// 5. Look for a way to make this copy done without a loop.
//    https://doc.rust-lang.org/std/primitive.slice.html
//
//    Implement and test your solution.
//
//    Commit your answers (bare3_5)
