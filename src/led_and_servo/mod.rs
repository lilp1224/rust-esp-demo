use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::ledc::{config::{Resolution, TimerConfig}, LedcDriver, LedcTimerDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::FromValueType;

/* Wokwi Project code 350684622891254355 */

pub fn start() {
    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::input_output(peripherals.pins.gpio2).unwrap();
    let button = PinDriver::input(peripherals.pins.gpio3).unwrap();

    let config = TimerConfig::default().frequency(50.Hz()).resolution(Resolution::Bits16);

    let timer = LedcTimerDriver::new(peripherals.ledc.timer1, &config).unwrap();
    let mut channel = LedcDriver::new(peripherals.ledc.channel1,
                                      timer, peripherals.pins.gpio10,
                                      &config).unwrap();
    loop {
        if button.is_low() {
            led.set_high().unwrap();
            // Bits14  0度 374    180度 2038
            channel.set_duty(2038).unwrap();
            FreeRtos::delay_ms(4000);
        } else {
            led.set_low().unwrap();
            channel.set_duty(374).unwrap();
        }
    }
}
