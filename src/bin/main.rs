//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO8)

//% CHIPS: esp32c3 esp32c6

#![no_std]
#![no_main]

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::{Rgb565};
use embedded_graphics::prelude::{Point, Primitive, RgbColor};
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Triangle};
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::Drawable;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{clock::CpuClock, delay::Delay, entry, rmt::Rmt};

use esp_backtrace as _;
use esp_hal::gpio::{GpioPin, Level, Output};
use esp_hal::rtc_cntl::Rtc;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::spi::{SpiBitOrder, SpiMode};
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;

use esp_hal_smartled::{smartLedBuffer, SmartLedsAdapter};
use smart_leds::{colors::*, SmartLedsWrite};

use fugit::{ExtU64, RateExtU32};
use mipidsi::interface::SpiInterface;
use mipidsi::models::ST7789;
use mipidsi::Builder;

#[entry]
fn main() -> ! {
    #[cfg(feature = "log")]
    {
        // The default log level can be specified here.
        // You can see the esp-println documentation： https://docs.rs/esp-println
        esp_println::logger::init_logger(log::LevelFilter::Info);
    }

    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        // Configure the CPU to run at the maximum frequency.
        config.cpu_clock = CpuClock::max();
        config
    });

    // use log
    #[cfg(feature = "log")]
    {
        log::error!("this is error message");
        log::warn!("this is warn message");
        log::info!("this is info message");
        log::debug!("this is debug message");
        log::trace!("this is trace message");
    }

    let rmt = Rmt::new(peripherals.RMT, 80.MHz()).unwrap();

    let rmt_buffer = smartLedBuffer!(1);
    let mut led = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO8, rmt_buffer);

    let colors = [WHITE, RED, YELLOW, LIME, GREEN, AQUA, BLUE];

    // Define the delay struct, needed for the display driver
    let mut delay = Delay::new();

    // Define the Data/Command select pin as a digital output
    let dc = Output::new(GpioPin::<15>, Level::Low);
    // Define the reset pin as digital outputs and make it high
    let mut rst = Output::new(GpioPin::<21>, Level::Low);
    rst.set_high();

    // Turn on the backlight
    let mut back_light = Output::new(GpioPin::<22>, Level::Low);
    back_light.set_high();

    // Define the SPI pins and create the SPI interface
    let sck = GpioPin::<7>;
    let miso = GpioPin::<5>;
    let mosi = GpioPin::<6>;
    let cs = GpioPin::<14>;
    let spi = Spi::new_with_config(
        peripherals.SPI2,
        Config {
            frequency: 100_u32.kHz(),
            mode: SpiMode::Mode0,
            read_bit_order: SpiBitOrder::MSBFirst,
            write_bit_order: SpiBitOrder::MSBFirst,
        },
    )
    .with_miso(miso)
    .with_mosi(mosi)
    .with_sck(sck)
    .with_cs(cs);

    let cs = GpioPin::<14>;
    let cs_output = Output::new(cs, Level::High);
    let spi_device = ExclusiveDevice::new_no_delay(spi, cs_output).unwrap();

    let mut buffer = [0_u8; 512];

    // Define the display interface with no chip select
    let di = SpiInterface::new(spi_device, dc, &mut buffer);

    // Define the display from the display interface and initialize it
    let mut display = Builder::new(ST7789, di)
        .reset_pin(rst)
        .init(&mut delay)
        .unwrap();

    // Make the display all black
    display.clear(Rgb565::BLACK).unwrap();
    // Draw a smiley face with white eyes and a red mouth

    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    let mut i = 0;
    let mut style = PrimitiveStyle::with_fill(Rgb565::WHITE);
    loop {
        println!("{} loop!", i);

        for color in colors {
            let data = [color; 1];
            led.write(data).unwrap();

            if i == 0 {
                style = PrimitiveStyle::with_fill(Rgb565::GREEN);
            } else if i == 1 {
                style = PrimitiveStyle::with_fill(Rgb565::BLUE);
            } else if i == 2 {
                style = PrimitiveStyle::with_fill(Rgb565::RED);
            } else if i == 3 {
                style = PrimitiveStyle::with_fill(Rgb565::WHITE);
            }
            i = (i + 1) % 4;
            draw_smiley(&mut display, style).unwrap();
        }
    }
}

fn draw_smiley<T: DrawTarget<Color = Rgb565>>(
    display: &mut T,
    style: PrimitiveStyle<Rgb565>,
) -> Result<(), T::Error> {
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(style.fill_color.unwrap())
        .build();

    Text::with_baseline(
        "Hello Kuttus!",
        Point::new(100, 100),
        text_style,
        Baseline::Top,
    )
    .draw(display)?;

    // Draw the left eye as a circle located at (50, 100), with a diameter of 40, filled with white
    Circle::new(Point::new(50, 100), 40)
        .into_styled(style)
        .draw(display)?;

    // Draw the right eye as a circle located at (50, 200), with a diameter of 40, filled with white
    Circle::new(Point::new(50, 200), 40)
        .into_styled(style)
        .draw(display)?;

    // Draw an upside down red triangle to represent a smiling mouth
    Triangle::new(
        Point::new(130, 140),
        Point::new(130, 200),
        Point::new(160, 170),
    )
    .into_styled(style)
    .draw(display)?;

    // Cover the top part of the mouth with a black triangle so it looks closed instead of open
    Triangle::new(
        Point::new(130, 150),
        Point::new(130, 190),
        Point::new(150, 170),
    )
    .into_styled(style)
    .draw(display)?;

    Ok(())
}
