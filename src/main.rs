pub struct Person {
    id: u128,
    name: String,
}

pub struct PersonContainer {
    people: Vec<Person>,
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
                return Some(person);
            }
        }
        return None;
    }
}

fn main() -> std::io::Result<()> {
    use std::fs;

    let metadata = fs::metadata("sample.txt")?;

    println!("{:?}", metadata);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let edison = Person {
            id: 123,
            name: String::from("Edison"),
        };
        let id = edison.id;
        let mut people = Vec::new();
        people.push(edison);

        let container = PersonContainer { people };

        let found_id = match container.find_by_id(id) {
            Some(person) => person.id,
            None => panic!("fail"),
        };
        assert_eq!(found_id, id)
    }
}
