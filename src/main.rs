use doorsensor::gpio::GPIO;
use rppal::system::DeviceInfo;
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Door sensor on {}.", DeviceInfo::new()?.model());

    let mut gpio = GPIO::new().expect("error initializing GPIO");

    // let gpio = conn.execute(
    //     "create table if not exists cat_colors (
    //          id integer primary key,
    //          name text not null unique
    //      )",
    //     NO_PARAMS,
    // )?;

    loop {
        thread::sleep(Duration::from_secs(1));

        if gpio.is_door_open() {
            println!("Door is opened!");
            gpio.turn_on_led();
        } else {
            println!("Door is closed!");
            gpio.turn_off_led();
        }
    }

    Ok(())
}
