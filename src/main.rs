use doorsensor::db::DB;
use doorsensor::gpio::GPIO;
use rppal::system::DeviceInfo;
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Door sensor on {}.", DeviceInfo::new()?.model());

    let mut door_is_open = true;
    let mut state = true;

    let mut db = DB::init().expect("error initializing DB instance");
    let mut gpio = GPIO::new().expect("error initializing GPIO");

    loop {
        thread::sleep(Duration::from_millis(500));

        if gpio.is_door_open() {
            println!("Door is opened!");
            gpio.turn_on_led();
            state = true;
        } else {
            println!("Door is closed!");
            gpio.turn_off_led();
            state = false;
        }

        // on state change
        if door_is_open != state {
            door_is_open = state;

            // TODO: write to DB on state change
        }
    }

    Ok(())
}
