use std::fmt;

const DAY: i64 = 24 * 60;
const HOUR: i64 = 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i64,
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        Clock {
            minutes: (((hours * HOUR) + minutes % DAY) + DAY) % DAY
        }
    }

    pub fn add_minutes(self, minutes: i64) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_on_the_hour() {
        assert_eq!(Clock::new(8, 0).to_string(), "08:00");
    }

    #[test]
    fn test_add_no_minutes() {
        let clock = Clock::new(6, 41).add_minutes(0);
        assert_eq!(clock.to_string(), "06:41");
    }

    #[test]
    fn test_add_to_next_hour() {
        let clock = Clock::new(0, 45).add_minutes(40);
        assert_eq!(clock.to_string(), "01:25");
    }

    #[test]
    fn test_add_more_than_two_hours_with_carry() {
        let clock = Clock::new(0, 45).add_minutes(160);
        assert_eq!(clock.to_string(), "03:25");
    }

    #[test]
    fn test_add_more_than_one_day() {
        let clock = Clock::new(5, 32).add_minutes(1500);
        assert_eq!(clock.to_string(), "06:32");
    }

    #[test]
    fn test_subtract_across_midnight() {
        let clock = Clock::new(0, 3).add_minutes(-4);
        assert_eq!(clock.to_string(), "23:59");
    }

    #[test]
    fn test_subtract_more_than_two_days() {
        let clock = Clock::new(2, 20).add_minutes(-3000);
        assert_eq!(clock.to_string(), "00:20");
    }

    #[test]
    fn test_compare_clocks_with_negative_hour() {
        assert_eq!(Clock::new(-2, 40), Clock::new(22, 40));
    }

    #[test]
    fn test_compare_clocks_with_negative_minute_that_wraps_multiple() {
        assert_eq!(Clock::new(6, -4305), Clock::new(6, 15));
    }
}

