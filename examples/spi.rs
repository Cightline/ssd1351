//! Interfacing the on-board L3GD20 (gyroscope)
#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate panic_semihosting;
extern crate embedded_hal as ehal;
extern crate stm32l432xx_hal as hal;
extern crate ssd1351;

use cortex_m::asm;
use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32l4::stm32l4x2;
use rt::ExceptionFrame;
use ehal::spi::{FullDuplex, Mode, Phase, Polarity};
use ssd1351::builder::Builder;

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
};

entry!(main);

fn main() -> ! {
    let p = stm32l4x2::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // TRY the other clock configuration
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb2);

    let mut nss = gpiob
        .pb0
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut dc = gpiob
        .pb7
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    // nss.set_high();
    let mut spi = Spi::spi1(
        p.SPI1,
        (sck, miso, mosi),
        MODE,
        // 1.mhz(),
        100.khz(),
        clocks,
        &mut rcc.apb2,
    );

    nss.set_low(); // only one device, always select
    
    // TODO
    let display = Builder::new().connect_spi(spi, dc);

    // when you reach this breakpoint you'll be able to inspect the variable `_m` which contains the
    // gyroscope and the temperature sensor readings
    asm::bkpt();

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
