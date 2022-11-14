pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        Self { name, age, weight }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_age() {
        let mut jobin = User::new("Jobin".to_string(), 26, 101.0);
        jobin.set_age(30);
        assert_eq!(jobin.age, 30);
    }

    #[test]
    fn test_weight() {
        let jobin = User::new("Jobin".to_string(), 26, 101.0);
        assert_eq!(jobin.weight, 101.0);
    }

}
