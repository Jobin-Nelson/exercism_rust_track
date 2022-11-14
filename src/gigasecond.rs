use time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration;

pub fn after(start: DateTime) -> DateTime {
    start + 1e9.seconds()
}

#[cfg(test)]
mod tests {
    use super::*;
    use time_macros::{date, time, datetime};

    #[test]
    fn gigasecond_nov_12() {
        let nov_12 = DateTime::new(date!(2022-11-01), time!(0:00));
        let nov_12_plus_gigasecond = datetime!(2054-07-10 1:46:40.0);
        assert_eq!(after(nov_12), nov_12_plus_gigasecond);
    }
}
