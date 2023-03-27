use anyhow::Result;
use rppal::gpio::Gpio;

// Gpio uses BCM pin numbering. BCM GPIO 18 is tied to physical pin 12. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;
const GPIO_DOORSENSOR: u8 = 18;

pub struct GPIO {
    led: rppal::gpio::OutputPin,
    doorsensor: rppal::gpio::InputPin,
}

impl GPIO {
    pub fn new() -> Result<GPIO> {
        let gpio = Gpio::new()?;

        let led = gpio.get(GPIO_LED)?.into_output();
        let doorsensor = gpio.get(GPIO_DOORSENSOR)?.into_input_pullup();

        Ok(GPIO { led, doorsensor })
    }

    pub fn is_door_open(&self) -> bool {
        *(&self.doorsensor.is_high())
    }

    pub fn turn_on_led(&mut self) {
        let _ = &self.led.set_high();
    }

    pub fn turn_off_led(&mut self) {
        let _ = &self.led.set_low();
    }
}
