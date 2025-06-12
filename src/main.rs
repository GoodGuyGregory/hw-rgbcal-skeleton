#![no_std]
#![no_main]

mod knob;
mod rgb;
mod ui;
pub use knob::*;
pub use rgb::*;
pub use ui::*;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embassy_executor::Spawner;
use embassy_futures::join;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::Timer;
use microbit_bsp::{
    embassy_nrf::{
        bind_interrupts,
        gpio::{AnyPin, Level, Output, OutputDrive},
        saadc,
    },
    Button, Microbit,
};
use num_traits::float::FloatCore;

// RGB LEVELS: Mutex to indicate the desired level to modify
pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);
// Mutex to hold the frame rate of the UI State Component
pub static FRAME_RATE: Mutex<ThreadModeRawMutex, u64> = Mutex::new(100);
pub const LEVELS: u32 = 16;

// accessor to the RGB_LEVELS value for ui reference and access
async fn get_rgb_levels() -> [u32; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}

// setter method to set the RGB level for the UiState
async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u32; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels);
}

// accessor method to get the frame_rate from the UiState 
async fn get_frame_rate() -> u64 {
    let frame_rate = FRAME_RATE.lock().await;
    *frame_rate
}

// setter used to modify the frame rate within the UiState
async fn set_frame_rate(incoming_rate: u64)
{
    let mut frame_rate = FRAME_RATE.lock().await;
    *frame_rate = incoming_rate;
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    rtt_init_print!();
    let board = Microbit::default();

    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);
    let red = led_pin(AnyPin::from(board.p9));
    let green = led_pin(AnyPin::from(board.p8));
    let blue = led_pin(AnyPin::from(board.p16));
    let rgb: Rgb = Rgb::new([red, green, blue], 100);

    // This saadc configures the Analog to Digital Signal source flow 
    // this is needed because the knob has analog values which require 
    // digital references to be represented in our state from 0-14BIT
    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT;
    let saadc = saadc::Saadc::new(
        board.saadc,
        Irqs,
        saadc_config,
        [saadc::ChannelConfig::single_ended(board.p2)],
    );

    // configure the knob to leverage embassy style interrupts 
    // this registers the interrupt handler from embassy to work with 
    // the knob to trigger interrupts when turning 
    let knob = Knob::new(saadc).await;
    // creates the ui object we will us to represent the UserInterface 
    let mut ui = Ui::new(knob, board.btn_a, board.btn_b);

    // join allows both the RGB and UI states to run together and rely on eachother
    // for execution.
    join::join(rgb.run(), ui.run()).await;

    panic!("fell off end of main loop");
}
