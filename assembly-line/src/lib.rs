const STANDARD_PRODUCE_LOW_SPEED: f64 = 221.0;
const ONE_HOUR: f64 = 60.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (match speed {
       spd if  spd <= 4 => spd as f64,
       spd if spd <= 8 => spd as f64 * 0.9,
       spd  => spd as f64 * 0.77,
    }) * STANDARD_PRODUCE_LOW_SPEED
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed)  / ONE_HOUR) as u32
}

