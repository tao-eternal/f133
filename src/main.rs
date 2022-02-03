#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![allow(dead_code)]

use riscv_rt::{entry, pre_init};

#[macro_use]
mod console;
mod panic;
mod uart;

#[entry]
fn main() -> ! {
    let mut count = 0;
    let mut fib = (0usize, 1usize);
    let start = counter();
    loop {
        fib = (fib.1, fib.0 + fib.1);
        println!("{}: {}", count, fib.0);
        count += 1;
        assert!(
            fib.0 <= fib.1,
            "count: {}, tick: {}",
            count,
            counter() - start
        );
        // sdelay(1 * 1000);
    }
}

#[pre_init]
unsafe fn my_pre_init() {
    use riscv::register::mstatus;
    /* Enable FPU and accelerator if present */
    mstatus::set_fs(mstatus::FS::Dirty);
    println!("启动中！");
}

fn counter() -> u64 {
    let cnt: u64;
    unsafe {
        core::arch::asm!("csrr {}, time\n", out(reg) cnt);
    }
    cnt
}

fn sdelay(us: u64) {
    let end = counter() + us * 24;
    while counter() <= end {}
}
