use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;
use std::error::Error;
use std::thread;
use std::time::Duration;

// Gpio uses BCM pin numbering. BCM GPIO 18 is tied to physical pin 12.
const GPIO_LED: u8 = 18;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_high();
    thread::sleep(Duration::from_millis(500));
    pin.set_low();

    Ok(())
}
