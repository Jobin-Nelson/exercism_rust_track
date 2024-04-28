use std::collections::{BTreeMap, BinaryHeap};

pub struct School {
    roaster: BTreeMap<u32, BinaryHeap<String>>,
}

impl School {
    pub fn new() -> Self {
        Self {
            roaster: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roaster
            .entry(grade)
            .or_insert(BinaryHeap::new())
            .push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roaster.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roaster
            .get(&grade)
            .cloned()
            .unwrap_or(BinaryHeap::new())
            .into_sorted_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_owned(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }
    #[test]
    fn grades_for_empty_school() {
        let s = School::new();
        assert_eq!(s.grades(), vec![]);
    }
    #[test]
    fn grades_for_one_student() {
        let mut s = School::new();
        s.add(2, "Aimee");
        assert_eq!(s.grades(), vec![2]);
    }
    #[test]
    fn grades_for_several_students_are_sorted() {
        let mut s = School::new();
        s.add(2, "Aimee");
        s.add(7, "Logan");
        s.add(4, "Blair");
        assert_eq!(s.grades(), vec![2, 4, 7]);
    }
    #[test]
    fn grades_when_several_students_have_the_same_grade() {
        let mut s = School::new();
        s.add(2, "Aimee");
        s.add(2, "Logan");
        s.add(2, "Blair");
        assert_eq!(s.grades(), vec![2]);
    }
    #[test]
    fn grade_for_empty_school() {
        let s = School::new();
        assert_eq!(s.grade(1), Vec::<String>::new());
    }
    #[test]
    fn grade_when_no_students_have_that_grade() {
        let mut s = School::new();
        s.add(7, "Logan");
        assert_eq!(s.grade(1), Vec::<String>::new());
    }
    #[test]
    fn grade_for_one_student() {
        let mut s = School::new();
        s.add(2, "Aimee");
        assert_eq!(s.grade(2), to_owned(&["Aimee"]));
    }
    #[test]
    fn grade_returns_students_sorted_by_name() {
        let mut s = School::new();
        s.add(2, "James");
        s.add(2, "Blair");
        s.add(2, "Paul");
        assert_eq!(s.grade(2), to_owned(&["Blair", "James", "Paul"]));
    }
    #[test]
    fn add_students_to_different_grades() {
        let mut s = School::new();
        s.add(3, "Chelsea");
        s.add(7, "Logan");
        assert_eq!(s.grades(), vec![3, 7]);
        assert_eq!(s.grade(3), to_owned(&["Chelsea"]));
        assert_eq!(s.grade(7), to_owned(&["Logan"]));
    }
}
