use cortex_m::asm;
use cortex_m_rt::exception;

use panic_probe as _;

use semihosting;

#[defmt::panic_handler]
fn panic() -> ! {
    asm::udf()
}

#[allow(dead_code)]
pub fn exit() -> ! {
    semihosting::process::exit(0);
}

#[exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    semihosting::process::exit(1);
}
