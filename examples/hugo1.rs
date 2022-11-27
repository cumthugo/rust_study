//! hugo first try

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
//use cortex_m_semihosting::{debug, hprintln};
use stm32f1xx_hal::{pac, prelude::*};


#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();

    gpiob.pb5.into_push_pull_output(&mut gpiob.crl).set_low();

    gpiob.pb0.into_push_pull_output(&mut gpiob.crl).set_low();


    let mut delay = cp.SYST.delay(&clocks);

    let mut abc = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
    loop {

        abc.set_low();
        delay.delay_ms(1_u16);
        abc.set_high();
        //delay.delay(1.secs());
        delay.delay_ms(10_u16);
    }

}
