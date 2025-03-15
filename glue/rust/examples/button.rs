// config::BUTTON demo app
use warduino::{
    delay, digital_read, digital_write, pin_mode, sub_interrupt, InterruptMode, PinMode, PinVoltage,
};

mod config;

fn callback(
    _topic: *const u8,
    _topic_length: usize,
    _payload: *const u8,
    _payload_length: usize,
    _length: u32,
) {
    let voltage = digital_read(config::LED);
    match voltage {
        PinVoltage::HIGH => digital_write(config::LED, PinVoltage::LOW),
        PinVoltage::LOW => digital_write(config::LED, PinVoltage::HIGH),
    }
}

#[no_mangle]
pub fn main() {
    pin_mode(config::BUTTON, PinMode::INPUT);
    pin_mode(config::LED, PinMode::OUTPUT);

    sub_interrupt(config::BUTTON, InterruptMode::FALLING, callback);

    loop {
        delay(1000);
    }
}
