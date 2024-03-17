#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: u16,
    pins_left: u16,
    first: bool,
    frames: u16,
    perks: u16,
    tenth: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            score: 0,
            pins_left: 10,
            first: true,
            frames: 0,
            perks: 0,
            tenth: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.pins_left.checked_sub(pins).is_none() {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.frames == 10 {
            return Err(Error::GameComplete);
        }

        self.pins_left -= pins;

        self.score += pins;

        if self.perks > 0 {
            self.score += pins;
            self.perks -= 1;
        }

        match (self.pins_left, self.first) {
            (0, true) => {
                self.perks += 2;
                self.frames += 1;
                if !self.tenth && self.frames == 10 {
                    self.frames -= 2;
                    self.perks -= 2;
                    self.tenth = true;
                }
                self.pins_left = 10;
            },
            (0, false) => {
                self.perks += 1;
                self.frames += 1;
                if !self.tenth && self.frames == 10 {
                    self.frames -= 1;
                    self.perks -= 1;
                    self.tenth = true;
                }
                self.first = true;
                self.pins_left = 10;
            },
            (_, false) => {
                self.frames += 1;
                self.first = true;
                self.pins_left = 10;
            }
            (_, true) => {
                self.first = false;
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.tenth && self.frames != 10 {
            return None;
        }
        Some(self.score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roll_returns_a_result() {
        let mut game = BowlingGame::new();
        assert!(game.roll(0).is_ok());
    }

    #[test]
    fn you_cannot_roll_more_than_ten_pins_in_a_single_roll() {
        let mut game = BowlingGame::new();
        assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
    }
    #[test]
    fn a_game_score_is_some_if_ten_frames_have_been_rolled() {
        let mut game = BowlingGame::new();
        for _ in 0..10 {
            let _ = game.roll(0);
            let _ = game.roll(0);
        }
        assert!(game.score().is_some());
    }
    #[test]
    fn you_cannot_score_a_game_with_no_rolls() {
        let game = BowlingGame::new();
        assert_eq!(game.score(), None);
    }
    #[test]
    fn a_game_score_is_none_if_fewer_than_ten_frames_have_been_rolled() {
        let mut game = BowlingGame::new();
        for _ in 0..9 {
            let _ = game.roll(0);
            let _ = game.roll(0);
        }
        assert_eq!(game.score(), None);
    }
    #[test]
    fn a_roll_is_err_if_the_game_is_done() {
        let mut game = BowlingGame::new();
        for _ in 0..10 {
            let _ = game.roll(0);
            let _ = game.roll(0);
        }
        assert_eq!(game.roll(0), Err(Error::GameComplete));
    }
    #[test]
    fn twenty_zero_pin_rolls_scores_zero() {
        let mut game = BowlingGame::new();
        for _ in 0..20 {
            let _ = game.roll(0);
        }
        assert_eq!(game.score(), Some(0));
    }
    #[test]
    fn ten_frames_without_a_strike_or_spare() {
        let mut game = BowlingGame::new();
        for _ in 0..10 {
            let _ = game.roll(3);
            let _ = game.roll(6);
        }
        assert_eq!(game.score(), Some(90));
    }
    #[test]
    fn spare_in_the_first_frame_followed_by_zeros() {
        let mut game = BowlingGame::new();
        let _ = game.roll(6);
        let _ = game.roll(4);
        for _ in 0..18 {
            let _ = game.roll(0);
        }
        assert_eq!(game.score(), Some(10));
    }
    #[test]
    fn points_scored_in_the_roll_after_a_spare_are_counted_twice_as_a_bonus() {
        let mut game = BowlingGame::new();
        let _ = game.roll(6);
        let _ = game.roll(4);
        let _ = game.roll(3);
        for _ in 0..17 {
            let _ = game.roll(0);
        }
        assert_eq!(game.score(), Some(16));
    }
    #[test]
    fn consecutive_spares_each_get_a_one_roll_bonus() {
        let mut game = BowlingGame::new();
        let _ = game.roll(5);
        let _ = game.roll(5);
        let _ = game.roll(3);
        let _ = game.roll(7);
        let _ = game.roll(4);
        for _ in 0..15 {
            let _ = game.roll(0);
        }
        assert_eq!(game.score(), Some(31));
    }
    #[test]
    fn if_the_last_frame_is_a_spare_you_get_one_extra_roll_that_is_scored_once() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }
        let _ = game.roll(5);
        let _ = game.roll(5);
        let _ = game.roll(7);
        assert_eq!(game.score(), Some(17));
    }
    #[test]
    fn a_strike_earns_ten_points_in_a_frame_with_a_single_roll() {
        let mut game = BowlingGame::new();
        let _ = game.roll(10);
        for _ in 0..18 {
            let _ = game.roll(0);
        }
        assert_eq!(game.score(), Some(10));
    }
    #[test]
    fn points_scored_in_the_two_rolls_after_a_strike_are_counted_twice_as_a_bonus() {
        let mut game = BowlingGame::new();
        let _ = game.roll(10);
        let _ = game.roll(5);
        let _ = game.roll(3);
        for _ in 0..16 {
            let _ = game.roll(0);
        }
        assert_eq!(game.score(), Some(26));
    }
    #[test]
    fn consecutive_strikes_each_get_the_two_roll_bonus() {
        let mut game = BowlingGame::new();
        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(5);
        let _ = game.roll(3);
        for _ in 0..12 {
            let _ = game.roll(0);
        }
        assert_eq!(game.score(), Some(81));
    }
    #[test]
    fn a_strike_in_the_last_frame_earns_a_two_roll_bonus_that_is_counted_once() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }
        let _ = game.roll(10);
        let _ = game.roll(7);
        let _ = game.roll(1);
        assert_eq!(game.score(), Some(18));
    }
    #[test]
    fn a_spare_with_the_two_roll_bonus_does_not_get_a_bonus_roll() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }
        let _ = game.roll(10);
        let _ = game.roll(7);
        let _ = game.roll(3);
        assert_eq!(game.score(), Some(20));
    }
    #[test]
    fn strikes_with_the_two_roll_bonus_do_not_get_a_bonus_roll() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }
        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(10);
        assert_eq!(game.score(), Some(30));
    }
    #[test]
    fn a_strike_with_the_one_roll_bonus_after_a_spare_in_the_last_frame_does_not_get_a_bonus() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }
        let _ = game.roll(7);
        let _ = game.roll(3);
        let _ = game.roll(10);
        assert_eq!(game.score(), Some(20));
    }
    #[test]
    fn all_strikes_is_a_perfect_score_of_300() {
        let mut game = BowlingGame::new();
        for _ in 0..12 {
            let _ = game.roll(10);
        }
        assert_eq!(game.score(), Some(300));
    }
    #[test]
    fn you_cannot_roll_more_than_ten_pins_in_a_single_frame() {
        let mut game = BowlingGame::new();
        assert!(game.roll(5).is_ok());
        assert_eq!(game.roll(6), Err(Error::NotEnoughPinsLeft));
    }
    #[test]
    fn first_bonus_ball_after_a_final_strike_cannot_score_an_invalid_number_of_pins() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }
        let _ = game.roll(10);
        assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
    }
    #[test]
    fn the_two_balls_after_a_final_strike_cannot_score_an_invalid_number_of_pins() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }
        let _ = game.roll(10);
        assert!(game.roll(5).is_ok());
        assert_eq!(game.roll(6), Err(Error::NotEnoughPinsLeft));
    }
    #[test]
    fn the_two_balls_after_a_final_strike_can_be_a_strike_and_non_strike() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }
        let _ = game.roll(10);
        assert!(game.roll(10).is_ok());
        assert!(game.roll(6).is_ok());
    }
    #[test]
    fn the_two_balls_after_a_final_strike_cannot_be_a_non_strike_followed_by_a_strike() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }
        let _ = game.roll(10);
        assert!(game.roll(6).is_ok());
        assert_eq!(game.roll(10), Err(Error::NotEnoughPinsLeft));
    }
}
