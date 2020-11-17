struct Person{
    name: String,
}

impl Person {
    pub fn say_hi(self) {
        println!("Hi, my name is {}", self.name)
    }
}

fn main() {
    let edison = Person{name: String::from("Edison")};
    edison.say_hi();
}
