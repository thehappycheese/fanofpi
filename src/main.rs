// Note: `dtoverlay=pwm` must be added somewhere in /boot/config.txt for this to work.

use rppal::pwm::{Channel, Polarity, Pwm};
use std::error::Error;
use std::fs;
use std::thread;
use std::time::Duration;

const MIN_TEMP: f64 = 30.0;
const MAX_TEMP: f64 = 60.0;
const TEMP_RANGE: f64 = MAX_TEMP - MIN_TEMP;

const FAN_LOW: f64 = 0.2f64;

fn main() -> Result<(), Box<dyn Error>> {
    let p = Pwm::with_frequency(Channel::Pwm0, 25000.0, 0.5, Polarity::Normal, true)?;

    p.set_duty_cycle(0.0)?;

    let mut avg_temp = get_temp()?;

    loop {
        avg_temp = match get_temp() {
            Ok(temp) => (temp + avg_temp * 3.0) / 4.0,
            Err(_) => {
                break;
            }
        };

        let force = ((avg_temp - MIN_TEMP) / TEMP_RANGE).clamp(0.0, 1.0);
        let real_duty = set_fan(&p, force)?;
        println!("{:>5.2}C  {:>5.0}%", avg_temp, real_duty * 100.0);
        thread::sleep(Duration::from_millis(2000));
    }

    p.set_duty_cycle(0.0)?;
    let mut p = p;
    p.set_reset_on_drop(false);

    Ok(())
}

fn get_temp() -> Result<f64, Box<dyn Error>> {
    Ok(fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?
        .trim()
        .parse::<f64>()?
        / 1000.0)
}

fn set_fan(p: &Pwm, duty: f64) -> Result<f64, Box<dyn Error>> {
    if duty < 0.01 {
        p.set_duty_cycle(0.0)?;
        Ok(0.0)
    } else {
        let real_duty = FAN_LOW + (1.0 - FAN_LOW) * duty;
        p.set_duty_cycle(real_duty)?;
        Ok(real_duty)
    }
}
