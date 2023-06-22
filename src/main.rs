#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

//use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    //asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    let rcc_abh1_enr_r_p = 0x40023830usize as *mut i32;
    let gpiox_moder_r_p = 0x40020C00usize as *mut i32;
    let gpiox_odr_r_p = 0x40020C14usize as *mut i32;

    unsafe{
        //1. Enable Clock for GPIOD Peripheral
        *rcc_abh1_enr_r_p |= 1 << 3;

	    //2. Set GPIOx_MODER in output mode
	     *gpiox_moder_r_p &= !(3 << 30);
	     *gpiox_moder_r_p |= 1 << 30;

         loop{

            *gpiox_odr_r_p |= 1 << 15;
            for _ in 1..30000{}
            *gpiox_odr_r_p &= !(1 << 15);
            for _ in 1..30000{}
            
         }
         
    }
}
