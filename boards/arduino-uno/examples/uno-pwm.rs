#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use arduino_uno::pwm;
use panic_halt as _;

const SERVO_CENTER: u8 = 23;
const SERVO_RIGHT: u8 = 15;
const SERVO_LEFT: u8 = 31;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    let mut timer2 = pwm::Timer1Pwm::new(dp.TC1, pwm::Prescaler::Prescale1024);

    let mut pin = pins.d3.into_output(&mut pins.ddr).into_pwm(&mut timer2);

    let mut delay = arduino_uno::Delay::new();

    pin.enable();

    loop {
        ufmt::uwriteln!(&mut serial, "Turning right").void_unwrap();
        pin.set_duty(SERVO_RIGHT as u8);
        arduino_uno::delay_ms(100);
        pin.set_duty(SERVO_CENTER as u8);
        arduino_uno::delay_ms(100);
        pin.set_duty(SERVO_LEFT as u8);
        arduino_uno::delay_ms(100);

    }
}
