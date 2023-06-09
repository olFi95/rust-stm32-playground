#![no_main]
#![no_std]


use stm32h7xx_hal::{block, pac};
use cortex_m_rt::entry;

use panic_halt as _;
use rtt_target::{rprintln, rtt_init, rtt_init_print};

use stm32h7xx_hal::prelude::*;
use stm32h7xx_hal::time::MilliSeconds;


#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();
    let pwr = dp.PWR.constrain();

    let pwrcfg = pwr.vos1().freeze();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc
        .sys_ck(192.MHz())
        .pclk1(48.MHz())
        .freeze(pwrcfg, &dp.SYSCFG);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    let mut ledGreen = gpiob.pb0.into_push_pull_output();
    let mut ledYellow = gpioe.pe1.into_push_pull_output();
    let mut ledRed = gpiob.pb14.into_push_pull_output();
    let mut timer = dp.TIM2.timer(1.Hz(), ccdr.peripheral.TIM2, &ccdr.clocks);

    rprintln!("Initialize all LEDs with low");
    ledGreen.set_low();
    ledYellow.set_low();
    ledRed.set_low();

    let timeout = MilliSeconds::from_ticks(200).into_rate();
    loop {
        ledGreen.set_high();
        timer.start(timeout);
        block!(timer.wait()).ok();
        ledGreen.set_low();
        ledYellow.set_high();
        timer.start(timeout);
        block!(timer.wait()).ok();
        ledYellow.set_low();
        ledRed.set_high();
        timer.start(timeout);
        block!(timer.wait()).ok();
        ledRed.set_low();
    }
}
