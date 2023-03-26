use doorsensor::db::DB;
use doorsensor::environment::EnvironmentVariables;
use doorsensor::gpio::GPIO;
use email::{Email, Relay};
use rppal::system::DeviceInfo;
use std::error::Error;
use std::thread;
use std::time::Duration;

const EMAIL_FROM: &str = "DoorSensor <bhargav.lakkur@gmail.com>";
const TO_ADDRESS: &str = "Bhargav Lakkur <lkbhargav9@gmail.com>";

fn main() -> Result<(), Box<dyn Error>> {
    println!("Door sensor on {}.", DeviceInfo::new()?.model());

    let mut door_is_open = true;
    let mut state = true;

    let vars = EnvironmentVariables::init().expect("error initializing environment variables");
    let mut db = DB::init(&vars.db_path).expect("error initializing DB instance");
    let mut gpio = GPIO::new().expect("error initializing GPIO");
    let gmail = Email::new(
        EMAIL_FROM,
        EMAIL_FROM,
        &vars.gmail.username,
        &vars.gmail.password,
        Relay::Gmail,
    )
    .expect("error initializing email service");

    loop {
        thread::sleep(Duration::from_millis(vars.ping_interval));

        if gpio.is_door_open() {
            gpio.turn_on_led();
            state = true;
        } else {
            gpio.turn_off_led();
            state = false;
        }

        // on state change
        if door_is_open != state {
            door_is_open = state;

            db.log(state);

            if vars.email_alert {
                let mut message = "";
                if state {
                    message = "Room door OPENED";
                } else {
                    message = "Room door CLOSED";
                }

                let system_time = SystemTime::now();
                let datetime: DateTime<Utc> = system_time.into();
                let datetime = format!("{}", datetime.format("%m/%d/%Y %T"));

                gmail.send(
                    TO_ADDRESS,
                    "Room door alert",
                    format!("{message} @ {datetime} (mm/dd/yyyy)!").as_str(),
                )
            }
        }
    }
}
