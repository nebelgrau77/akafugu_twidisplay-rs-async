#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::time::Rate;
use esp_hal::Async;

use embassy_sync::{
    signal::Signal,
    blocking_mutex::{
        raw::{
        CriticalSectionRawMutex
        },    
    },
};

use akafugu_twidisplay_async::*;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();


#[derive(Clone, Copy)]
/// struct to hold hours and minutes from some clock
struct TimeDigits {
    hours: u8,
    minutes: u8,
    seconds: u8
}

// Signal to pass data between tasks
static TIMESIGNAL: Signal<CriticalSectionRawMutex, TimeDigits> = Signal::new();


#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.2.0

    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let i2c_bus = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(100)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO6)
    .with_sda(peripherals.GPIO5)
    .into_async()
    ;

    let mut akafugu = TWIDisplay::new(i2c_bus, DEFAULT_ADDRESS);

    akafugu.clear_display().await.unwrap();
    akafugu.set_brightness(200).await.unwrap();    

    spawner.spawn(display_clock(akafugu)).ok();
    spawner.spawn(fake_time(TimeDigits { hours: 12, minutes: 7, seconds: 0 })).ok();

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(5)).await;
    }
}


#[embassy_executor::task]
/// display time (minutes and seconds), blinking the dot every other time
async fn display_clock(mut akafugu: TWIDisplay<I2c<'static, Async>>) {

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