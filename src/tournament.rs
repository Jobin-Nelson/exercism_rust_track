use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut results: Vec<(_, _)> = match_results
        .split('\n')
        .map(|l| l.split(';').collect::<Vec<_>>())
        .filter(|l| l.len() > 2)
        .flat_map(|v| {
            let (t1, t2) = match v[2] {
                "win" => ((1, 0, 0), (0, 0, 1)),
                "draw" => ((0, 1, 0), (0, 1, 0)),
                "loss" => ((0, 0, 1), (1, 0, 0)),
                _ => unimplemented!(),
            };

            vec![(v[0], t1), (v[1], t2)]
        })
        .fold(HashMap::new(), |mut map, (n, (w, d, l))| {
            let e = map.entry(n).or_insert((0, 0, 0, 0, 0));
            *e = (e.0 + 1, e.1 + w, e.2 + d, e.3 + l, e.4 + (w * 3 + d));
            map
        })
        .into_iter()
        .collect();
    results.sort_by(|(na, a), (nb, b)| b.4.cmp(&a.4).then_with(|| na.cmp(nb)));
    ["Team                           | MP |  W |  D |  L |  P".to_owned()]
        .into_iter()
        .chain(results.into_iter().map(|(n, (m, w, d, l, p))| {
            format!(
                "{:030} | {:2} | {:2} | {:2} | {:2} | {:2}",
                n, m, w, d, l, p
            )
        }))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn just_header_if_no_input() {
        let input: &[&str] = &[];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = ["Team                           | MP |  W |  D |  L |  P"].join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn a_win_is_three_points_a_loss_is_zero_points() {
        let input: &[&str] = &["Allegoric Alaskans;Blithering Badgers;win"];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3",
            "Blithering Badgers             |  1 |  0 |  0 |  1 |  0",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn a_win_can_also_be_expressed_as_a_loss() {
        let input: &[&str] = &["Blithering Badgers;Allegoric Alaskans;loss"];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3",
            "Blithering Badgers             |  1 |  0 |  0 |  1 |  0",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn a_different_team_can_win() {
        let input: &[&str] = &["Blithering Badgers;Allegoric Alaskans;win"];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Blithering Badgers             |  1 |  1 |  0 |  0 |  3",
            "Allegoric Alaskans             |  1 |  0 |  0 |  1 |  0",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn a_draw_is_one_point_each() {
        let input: &[&str] = &["Allegoric Alaskans;Blithering Badgers;draw"];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  1 |  0 |  1 |  0 |  1",
            "Blithering Badgers             |  1 |  0 |  1 |  0 |  1",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn there_can_be_more_than_one_match() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;win",
            "Allegoric Alaskans;Blithering Badgers;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6",
            "Blithering Badgers             |  2 |  0 |  0 |  2 |  0",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn there_can_be_more_than_one_winner() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;loss",
            "Allegoric Alaskans;Blithering Badgers;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  2 |  1 |  0 |  1 |  3",
            "Blithering Badgers             |  2 |  1 |  0 |  1 |  3",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn there_can_be_more_than_two_teams() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;win",
            "Blithering Badgers;Courageous Californians;win",
            "Courageous Californians;Allegoric Alaskans;loss",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6",
            "Blithering Badgers             |  2 |  1 |  0 |  1 |  3",
            "Courageous Californians        |  2 |  0 |  0 |  2 |  0",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn typical_input() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;win",
            "Devastating Donkeys;Courageous Californians;draw",
            "Devastating Donkeys;Allegoric Alaskans;win",
            "Courageous Californians;Blithering Badgers;loss",
            "Blithering Badgers;Devastating Donkeys;loss",
            "Allegoric Alaskans;Courageous Californians;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Devastating Donkeys            |  3 |  2 |  1 |  0 |  7",
            "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6",
            "Blithering Badgers             |  3 |  1 |  0 |  2 |  3",
            "Courageous Californians        |  3 |  0 |  1 |  2 |  1",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn incomplete_competition_not_all_pairs_have_played() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;loss",
            "Devastating Donkeys;Allegoric Alaskans;loss",
            "Courageous Californians;Blithering Badgers;draw",
            "Allegoric Alaskans;Courageous Californians;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6",
            "Blithering Badgers             |  2 |  1 |  1 |  0 |  4",
            "Courageous Californians        |  2 |  0 |  1 |  1 |  1",
            "Devastating Donkeys            |  1 |  0 |  0 |  1 |  0",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn ties_broken_alphabetically() {
        let input: &[&str] = &[
            "Courageous Californians;Devastating Donkeys;win",
            "Allegoric Alaskans;Blithering Badgers;win",
            "Devastating Donkeys;Allegoric Alaskans;loss",
            "Courageous Californians;Blithering Badgers;win",
            "Blithering Badgers;Devastating Donkeys;draw",
            "Allegoric Alaskans;Courageous Californians;draw",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  3 |  2 |  1 |  0 |  7",
            "Courageous Californians        |  3 |  2 |  1 |  0 |  7",
            "Blithering Badgers             |  3 |  0 |  1 |  2 |  1",
            "Devastating Donkeys            |  3 |  0 |  1 |  2 |  1",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn ensure_points_sorted_numerically() {
        let input: &[&str] = &[
            "Devastating Donkeys;Blithering Badgers;win",
            "Devastating Donkeys;Blithering Badgers;win",
            "Devastating Donkeys;Blithering Badgers;win",
            "Devastating Donkeys;Blithering Badgers;win",
            "Blithering Badgers;Devastating Donkeys;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Devastating Donkeys            |  5 |  4 |  0 |  1 | 12",
            "Blithering Badgers             |  5 |  1 |  0 |  4 |  3",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
}
