#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{DrawTarget, Primitive, RgbColor};
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Triangle};
use embedded_graphics::text::{Baseline, Text};
use embedded_hal_bus::spi::ExclusiveDevice;
use smart_leds::SmartLedsWrite;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::rmt::Rmt;
use esp_hal::time::{Rate};
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::spi::master::{Config, Spi};
use esp_hal_smartled::{smart_led_buffer, SmartLedsAdapter};
use mipidsi::Builder;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ST7789;
use smart_leds::colors::{GREEN, RED, BLUE, YELLOW};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.1.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).unwrap();

    let mut rmt_buffer = smart_led_buffer!(1);
    let mut led = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO8, &mut rmt_buffer);

    // Define the Data/Command select pin as a digital output
    let dc = Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default());
    // Define the reset pin as digital outputs and make it high
    let mut rst = Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default());
    rst.set_high();

    // Turn on the backlight
    let mut back_light = Output::new(peripherals.GPIO22, Level::Low, OutputConfig::default());
    back_light.set_high();

    // Define the SPI pins and create the SPI interface
    let sck = peripherals.GPIO7;
    let miso = peripherals.GPIO5;
    let mosi = peripherals.GPIO6;
    let cs = peripherals.GPIO14;
    let spi = Spi::new(
        peripherals.SPI2,
        Config::default().with_frequency(Rate::from_mhz(10)),
    ).unwrap()
        .with_miso(miso)
        .with_mosi(mosi)
        .with_sck(sck);

    let cs_output = Output::new(cs, Level::High, OutputConfig::default());
    let spi_device = ExclusiveDevice::new_no_delay(spi, cs_output).unwrap();

    let mut buffer = [0_u8; 512];

    // Define the display interface with no chip select
    let di = SpiInterface::new(spi_device, dc, &mut buffer);

    // Define the delay struct, needed for the display driver
    let mut delay = Delay::new();

    // Define the display from the display interface and initialize it
    let mut display = Builder::new(ST7789, di)
        .reset_pin(rst)
        .init(&mut delay)
        .unwrap();

    // Reset display
    display.clear(Rgb565::BLACK).unwrap();
    let mut i = 0;
    loop {
        for color in [BLUE, RED, YELLOW, GREEN] {
            let data = [color; 1];
            led.write(data).unwrap();

            let style = match i {
                0 => Rgb565::BLUE,
                1 => Rgb565::RED,
                2 => Rgb565::YELLOW,
                _ => Rgb565::GREEN
            };

            i = (i + 1) % 4;
            draw_smiley(&mut display,  PrimitiveStyle::with_fill(style)).unwrap();
            delay.delay_millis(1000u32);
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
        "Hello World!",
        Point::new(100, 100),
        text_style,
        Baseline::Top,
    )
        .draw(display)?;

    Circle::new(Point::new(50, 100), 40)
        .into_styled(style)
        .draw(display)?;

    Circle::new(Point::new(50, 200), 40)
        .into_styled(style)
        .draw(display)?;

    Triangle::new(
        Point::new(130, 140),
        Point::new(130, 200),
        Point::new(160, 170),
    )
        .into_styled(style)
        .draw(display)?;

    Ok(())
}
