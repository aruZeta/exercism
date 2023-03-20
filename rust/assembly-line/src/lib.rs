const BASE_CAR_RATE: u32 = 221;
const SUCCESS_RATE_HIGH: f64 = 1.;
const SUCCESS_RATE_MID: f64 = 0.9;
const SUCCESS_RATE_LOW: f64 = 0.77;
const MINUTES_IN_HOUR: u8 = 60;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let cars_per_hour = (speed as u32 * BASE_CAR_RATE) as f64;
    return cars_per_hour * success_rate(speed);
}

fn success_rate(speed: u8) -> f64 {
    return match speed {
        0..=4 => SUCCESS_RATE_HIGH,
        5..=8 => SUCCESS_RATE_MID,
        9..=10 => SUCCESS_RATE_LOW,
        _ => panic!("Illegal speed value: {}", speed),
    };
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let cars_per_minute = production_rate_per_hour(speed) / MINUTES_IN_HOUR as f64;
    return cars_per_minute.trunc() as u32;
}
