pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=u8::MAX => 0.77,
    };
    221.0 * (speed as f64) * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let cars = production_rate_per_hour(speed);
    cars as u32 / 60
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn production_rate_per_hour_at_speed_four() {
        assert_eq!(working_items_per_minute(4 as u8), 14);
    }

    #[test]
    fn production_rate_per_hour_at_speed_nine() {
        assert_eq!(working_items_per_minute(9 as u8), 25);
    }
}
