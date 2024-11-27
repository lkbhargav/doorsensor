use chrono::offset::Utc;
use chrono::DateTime;
use doorsensor::db::DB;
use doorsensor::environment::EnvironmentVariables;
use doorsensor::gpio::GPIO;
use email::{Email, Relay};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use rppal::system::DeviceInfo;
use serde_json::json;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

const EMAIL_FROM: &str = "DoorSensor <bhargav.lakkur@gmail.com>";
const TO_ADDRESS: &str = "Bhargav Lakkur <lkbhargav9@gmail.com>";
const SLACK_URL: &str = "https://hooks.slack.com/services/";

fn main() {
    println!(
        "Door sensor on {}.",
        DeviceInfo::new()
            .expect("error trying to deviceInfo")
            .model()
    );

    let client = Client::new();

    let mut door_is_open = true;
    let mut state;

    let vars = EnvironmentVariables::init().expect("error initializing environment variables");
    let mut db = DB::init(&vars.db_path).expect("error initializing DB instance");
    let mut gpio = GPIO::new().expect("error initializing GPIO");
    let gmail = Email::new(
        EMAIL_FROM.to_owned(),
        EMAIL_FROM.to_owned(),
        &vars.gmail.username,
        &vars.gmail.password,
        Relay::Gmail,
    )
    .expect("error initializing email service");

    gpio.turn_off_led();

    loop {
        thread::sleep(Duration::from_millis(vars.ping_interval.into()));

        if gpio.is_door_open() {
            if vars.enable_feedback {
                gpio.turn_on_led();
            }
            state = true;
        } else {
            if vars.enable_feedback {
                gpio.turn_off_led();
            }
            state = false;
        }

        let mut message = "Room door OPENED";

        if !state {
            message = "Room door CLOSED";
        }

        notify_if_slack_notification_is_enabled(&vars, message, &client);

        // on state change
        if door_is_open != state {
            door_is_open = state;

            let res = db.log(state);

            if res.is_err() {
                println!("error trying to log a record in DB");
            }

            if vars.email_alert {
                let system_time = SystemTime::now();
                let datetime: DateTime<Utc> = system_time.into();
                let datetime = format!("{}", datetime.format("%m/%d/%Y %T"));

                let res = gmail.send(
                    TO_ADDRESS,
                    "Room door alert",
                    format!("{message} @ {datetime} (mm/dd/yyyy)!").as_str(),
                );

                if res.is_err() {
                    println!("error sending email: {}", res.err().unwrap());
                }
            }
        }
    }
}

fn notify_if_slack_notification_is_enabled(
    vars: &EnvironmentVariables,
    message: &str,
    client: &Client,
) {
    if vars.slack_token.is_empty() {
        return;
    }

    let slack_token = vars.slack_token;

    let slack_url = format!("{SLACK_URL}{slack_token}");

    let payload = json!({
        "text": message
    });

    // Make the POST request
    let response = match client
        .post(slack_url)
        .header(CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
    {
        Ok(d) => d,
        Err(e) => {
            println!("error sending slack notification");
            return;
        }
    };

    // Check if the response is successful
    if !response.status().is_success() {
        eprintln!("Failed to send message. Status: {}", response.status());
    }
}
