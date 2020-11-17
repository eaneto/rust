struct PersonContainer {
    people: Vec<Person>
}
struct Person{
    id: u128,
    name: String,
}

impl Person {
    pub fn say_hi(self) {
        println!("Hi, my name is {} my id is {}", self.name, self.id)
    }
}

impl PersonContainer {
    pub fn find_by_id(self, id: u128) -> Option<Person> {
        for person in self.people {
            if person.id == id {
                return Some(person)
            }
        }
        return None
    }
}

fn main() {
    let edison = Person{
        id: 123,
        name: String::from("Edison")
    };
    let id = edison.id;
    let mut people = Vec::new();
    people.push(edison);

    let container = PersonContainer{people};

    match container.find_by_id(id) {
        Some(person) => person.say_hi(),
        None => println!("not found")
    }
}
