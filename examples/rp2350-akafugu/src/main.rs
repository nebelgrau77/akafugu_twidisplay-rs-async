// Made for Seeedstudio Xiao RP2350
#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio;
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};
use embassy_rp::i2c::{I2c, Config as I2cConfig, InterruptHandler, Async};
use embassy_rp::peripherals::I2C1;

use embassy_sync::{
    signal::Signal,
    blocking_mutex::raw::CriticalSectionRawMutex,
};

embassy_rp::bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<embassy_rp::peripherals::I2C1>;
});

use akafugu_twidisplay_async::*;

static TIMESIGNAL: Signal<CriticalSectionRawMutex, TimeDigits> = Signal::new();

#[derive(Clone, Copy)]
/// struct to hold hours and minutes from some clock
struct TimeDigits {
    hours: u8,
    minutes: u8,
    seconds: u8
}

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();



#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    let sda_pin =p.PIN_6;
    let scl_pin = p.PIN_7;

    let i2c_config = I2cConfig::default();
    let i2c_bus = I2c::new_async(p.I2C1, scl_pin, sda_pin, Irqs, i2c_config);

    let mut akafugu = TWIDisplay::new(i2c_bus, DEFAULT_ADDRESS);

    akafugu.clear_display().await.unwrap();
    akafugu.set_brightness(200).await.unwrap();    

    spawner.spawn(display_clock(akafugu)).ok();
    spawner.spawn(fake_time(TimeDigits { hours: 0, minutes: 0, seconds: 0 })).ok();

    spawner.spawn(blink(led)).ok();


    loop {
        info!("hello!");
        Timer::after(Duration::from_secs(2)).await;
    }
}

#[embassy_executor::task]
async fn blink(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(250)).await;
    }
}

#[embassy_executor::task]
/// display time (minutes and seconds), blinking the dot every other time
async fn display_clock(mut akafugu: TWIDisplay<I2c<'static, I2C1, Async>>) {

    let mut dot: bool = false;

    loop {        
        let time = TIMESIGNAL.wait().await;        
        info!("time read: {}:{}:{}", time.hours, time.minutes, time.seconds);
        akafugu.display_time(time.minutes,time.seconds, dot).await.unwrap();
        dot = !dot;
    }

}

#[embassy_executor::task]
/// update time signal with fake time count
async fn fake_time(start_time: TimeDigits) {    
    let mut time: TimeDigits = TimeDigits { hours: start_time.hours, minutes: start_time.minutes, seconds: start_time.seconds };
    loop {
        if time.seconds >= 59 {
            time.seconds = 0;
            time.minutes += 1;
        } else {
            time.seconds += 1;
        }
        if time.minutes >= 59 {
            time.minutes = 0;
            time.hours += 1;
        }
        if time.hours >= 23 {
            time.hours = 0
        }
        Timer::after_secs(1).await;
        TIMESIGNAL.signal(time);
    }

}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Blinky Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico on board LED, connected to gpio 25"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];


// End of file
