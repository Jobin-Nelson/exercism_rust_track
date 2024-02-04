#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut tmp_scores = self.scores.to_vec();
        tmp_scores.sort_by(|a, b| b.cmp(a));
        tmp_scores.into_iter().take(3).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn list_of_scores() {
        let expected = [30, 50, 20, 70];
        let high_scores = HighScores::new(&expected);
        assert_eq!(high_scores.scores(), &expected);
    }

    #[test]
    fn latest_score() {
        let high_scores = HighScores::new(&[100, 0, 90, 30]);
        assert_eq!(high_scores.latest(), Some(30));
    }

    #[test]
    fn latest_score_empty() {
        let high_scores  = HighScores::new(&[]);
        assert_eq!(high_scores.latest(), None);
    }

    #[test]
    fn personal_best() {
        let high_scores = HighScores::new(&[40, 100, 70]);
        assert_eq!(high_scores.personal_best(), Some(100));
    }
}
