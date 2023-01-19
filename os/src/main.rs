//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![allow(unused)]

use core::{arch::{global_asm, asm}, panic::{self, PanicInfo}};

use crate::sbi::{console_putchar, console_getchar};
use crate::console::print;

#[macro_use]
mod console;
mod lang_items;
mod sbi;

#[path = "boards/qemu.rs"]
mod board;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(
        |a| unsafe { (a as *mut u8).write_volatile(0); }
    );
}

// rust macro is like some kind of patern match
macro_rules! add {
    ($a: expr, $b: expr) => {
        $a + $b
    };
    ($a: expr) => {
        $a
    }
}

fn welcome() {
    // here we are "cheating" the compiler. We ask it to help us find functions.
    // While in fact we treated these "functions" as values
    extern "C" {
        fn stext();     // begin addr of text segment
        fn etext();     // end addr of text segment
        fn srodata();   // start addr of Read-Only data segment
        fn erodata();   // end addr of Read-Only data ssegment
        fn sdata();     // start addr of data segment
        fn edata();     // end addr of data segment
        fn sbss();      // start addr of BSS segment
        fn ebss();      // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    console_putchar('\n' as usize);
    info!("memory layout:");
    info!("rust-sbi  [0x80000000, 0x80200000]");
    info!(".text     [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata   [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data     [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(
        ".stack    [{:#x}, {:#x}]",
        boot_stack_lower_bound as usize, boot_stack_top as usize
    );
    info!(".bss      [{:#x}, {:#x})", sbss as usize, ebss as usize);
}

/// the rust entry-point of os, .org 0x80200000
#[no_mangle]
pub fn rust_main() -> ! {
    unsafe{ asm!("li t0, 0x114","li t1, 0x514");};
    clear_bss();
    welcome();
    // how to display line numbers, file names and function names?
    warn!("hello world");
    info!("hello world");
    debug!("hello world");
    error!("hello world");
    trace!("hello world");
    use crate::board::QEMUExit;
    crate::board::QEMU_EXIT_HANDLE.exit_success(); // CI autotest success
                                                   //crate::board::QEMU_EXIT_HANDLE.exit_failure(); // CI autoest failed

}
