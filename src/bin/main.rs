#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::delay::Delay;
use esp_println::println;
use esp32_rust_project::blink_morse;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]

#[main]
fn main() -> ! {
    // generator version: 1.4.0
    // generator parameters: -o vscode -o esp32c3 -o wokwi
    println!("Hello, user!");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let mut led = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());
    let text = "Hello World";

    loop {
        blink_morse::blink_text(&text, &mut led, &delay);

        println!("Word finished. Pausing before next round...");
        led.set_level(Level::Low);
        delay.delay_millis(4000);
    }
}
