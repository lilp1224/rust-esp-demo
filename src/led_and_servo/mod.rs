use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::ledc::{config::{Resolution, TimerConfig}, LedcDriver, LedcTimerDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::FromValueType;

/* Wokwi Project code 350684622891254355 */

const SERVO_MIN_PULSEWIDTH_US: u32 = 374;  // 最小脉冲
const SERVO_MAX_PULSEWIDTH_US: u32 = 2038; // 最大脉冲
const SERVO_MAX_ANGLE: f32 = 180.0;        // 最大角度
const PWM_PERIOD: u32 = 50;// 频率
const TIMER_RESOLUTION: u32 = 16383;       // 2^Bits14 - 1

pub fn start() {
    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::input_output(peripherals.pins.gpio2).unwrap();
    let button = PinDriver::input(peripherals.pins.gpio3).unwrap();

    // resolution 2的N次方+1
    let config = TimerConfig::default().frequency(PWM_PERIOD.Hz()).resolution(Resolution::Bits14);

    let timer = LedcTimerDriver::new(peripherals.ledc.timer1, &config).unwrap();
    let mut channel = LedcDriver::new(peripherals.ledc.channel1,
                                      timer, peripherals.pins.gpio10,
                                      &config).unwrap();
    loop {
        if button.is_low() {
            led.set_high().unwrap();
            channel.set_duty(2038).unwrap();
            FreeRtos::delay_ms(4000);
        } else {
            led.set_low().unwrap();
            channel.set_duty(374).unwrap();
        }
    }
}
