use std::collections::HashMap;

pub struct PersonContainer {
    people: Vec<Person>,
    peopleMap: HashMap<u128, Person>
}

pub struct Person{
    id: u128,
    name: String,
}

impl Person {
    pub fn say_hi(self) {
        println!("Hi, my name is {} my id is {}", self.name, self.id)
    }
}

impl PersonContainer {
    // Searches for a person in the container.
    pub fn find_by_id(self, id: u128) -> Option<Person> {
        for person in self.people {
            if person.id == id {
                return Some(person)
            }
        }
        return None
    }

    pub fn find_by_id_on_map(self, id: u128) -> Option<Person> {
        return match self.peopleMap.get(&id) {
            Some(person) => Some(person.clone()),
            None => None
        }
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

    let container = PersonContainer{
        people,
        peopleMap: HashMap::new()
    };

    match container.find_by_id(id) {
        Some(person) => person.say_hi(),
        None => println!("not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let edison = Person{
            id: 123,
            name: String::from("Edison")
        };
        let id = edison.id;
        let mut people = Vec::new();
        people.push(edison);

        let container = PersonContainer{
            people,
            peopleMap: HashMap::new()
        };

        let found_id = match container.find_by_id(id) {
            Some(person) => person.id,
            None => panic!("fail")
        };
        assert_eq!(found_id, id)
    }
}
