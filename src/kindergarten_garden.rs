pub enum Students {
    Alice,
    Bob,
    Charlie,
    David,
    Eve,
    Fred,
    Ginny,
    Harriet,
    Ileana,
    Joseph,
    Kincaid,
    Larry,
}

impl Students {
    pub fn get_plants(&self, plants: &str) -> Vec<&'static str> {
        let rows = plants.split_whitespace();
        let iter_plants = |i: usize| {
            rows.into_iter()
                .flat_map(|r| r.chars().skip(i * 2).take(2))
                .map(|p| match p {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    'V' => "violets",
                    s => panic!("Unknown plant {s}"),
                })
                .collect()
        };
        match self {
            Students::Alice => iter_plants(0),
            Students::Bob => iter_plants(1),
            Students::Charlie => iter_plants(2),
            Students::David => iter_plants(3),
            Students::Eve => iter_plants(4),
            Students::Fred => iter_plants(5),
            Students::Ginny => iter_plants(6),
            Students::Harriet => iter_plants(7),
            Students::Ileana => iter_plants(8),
            Students::Joseph => iter_plants(9),
            Students::Kincaid => iter_plants(10),
            Students::Larry => iter_plants(11),
        }
    }

    pub fn from(student: &str) -> Self {
        match student {
            "Alice" => Self::Alice,
            "Bob" => Self::Bob,
            "Charlie" => Self::Charlie,
            "David" => Self::David,
            "Eve" => Self::Eve,
            "Fred" => Self::Fred,
            "Ginny" => Self::Ginny,
            "Harriet" => Self::Harriet,
            "Ileana" => Self::Ileana,
            "Joseph" => Self::Joseph,
            "Kincaid" => Self::Kincaid,
            "Larry" => Self::Larry,
            s => panic!("No student with name {s}"),
        }
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student = Students::from(student);
    student.get_plants(diagram)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn garden_with_single_student() {
        let diagram = "RC
        GG";
        let student = "Alice";
        let expected = vec!["radishes", "clover", "grass", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn different_garden_with_single_student() {
        let diagram = "VC
    RC";
        let student = "Alice";
        let expected = vec!["violets", "clover", "radishes", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn garden_with_two_students() {
        let diagram = "VVCG
    VVRC";
        let student = "Bob";
        let expected = vec!["clover", "grass", "radishes", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn second_students_garden() {
        let diagram = "VVCCGG
    VVCCGG";
        let student = "Bob";
        let expected = vec!["clover", "clover", "clover", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn third_students_garden() {
        let diagram = "VVCCGG
    VVCCGG";
        let student = "Charlie";
        let expected = vec!["grass", "grass", "grass", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_alice_first_students_garden() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
    VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Alice";
        let expected = vec!["violets", "radishes", "violets", "radishes"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_bob_second_students_garden() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
    VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Bob";
        let expected = vec!["clover", "grass", "clover", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_charlie() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
    VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Charlie";
        let expected = vec!["violets", "violets", "clover", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_david() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
    VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "David";
        let expected = vec!["radishes", "violets", "clover", "radishes"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_eve() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
    VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Eve";
        let expected = vec!["clover", "grass", "radishes", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_fred() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
    VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Fred";
        let expected = vec!["grass", "clover", "violets", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_ginny() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
    VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Ginny";
        let expected = vec!["clover", "grass", "grass", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_harriet() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
    VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Harriet";
        let expected = vec!["violets", "radishes", "radishes", "violets"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_ileana() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
    VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Ileana";
        let expected = vec!["grass", "clover", "violets", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_joseph() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
    VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Joseph";
        let expected = vec!["violets", "clover", "violets", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_kincaid_second_to_last_students_garden() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
    VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Kincaid";
        let expected = vec!["grass", "clover", "clover", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_larry_last_students_garden() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
    VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Larry";
        let expected = vec!["grass", "violets", "clover", "violets"];
        assert_eq!(plants(diagram, student), expected);
    }
}
