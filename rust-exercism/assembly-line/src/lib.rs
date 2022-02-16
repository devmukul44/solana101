// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/*
analyze the production of an assembly line in a car factory.
The assembly line's speed can range from 0 (off) to 10 (maximum).

At its lowest speed (1), 221 cars are produced each hour.
The production increases linearly with the speed.
So with the speed set to 4, it should produce 4 * 221 = 884 cars per hour.
However, higher speeds increase the likelihood that faulty cars are produced, which then have to be discarded.

The following table shows how speed influences the success rate:
    1 to 4: 100% success rate.
    5 to 8: 90% success rate.
    9 and 10: 77% success rate.
 */

const CARS_PER_HOUR: f64 = 221 as f64;

fn success_rate(speed: u8) -> f64 {
    match speed {
        0..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    }
}

/// calculate the assembly line's production rate per hour, taking into account its success rate
pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = success_rate(speed);
    CARS_PER_HOUR * (speed as f64) * rate
}

/// calculate how many working cars are produced per minute
pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed)/60.0) as u32
}
