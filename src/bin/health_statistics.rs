struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    fn new(name: String, age: u32, weight: f32) -> Self {
        Self { name, age, weight }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn weight(&self) -> f32 {
        self.weight
    }

    fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}

fn main() {
    let mut jobin = User::new("Jobin".to_string(), 26, 101.0);

    println!("my name is {}", jobin.name);
    println!("my age is {}", jobin.age);
    println!("my weight is {}", jobin.weight);
    jobin.set_age(30);
    println!("changing my age, now it is {}", jobin.age);
    jobin.set_weight(120.0);
    println!("changing my weight, now it is {}", jobin.weight);
}
