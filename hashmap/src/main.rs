fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn insert() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        assert_eq!(scores.len(), 2);
        assert_eq!(scores.get("Blue"), Some(&10));
        assert_eq!(scores.get("Yellow"), Some(&50));
    }

    #[test]
    fn iterator() {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

        assert_eq!(scores.len(), 2);
        assert_eq!(scores.get("Blue"), Some(&10));
        assert_eq!(scores.get("Yellow"), Some(&50));

        for (key, value) in &scores {
            assert_eq!(scores.get(key), Some(value));
        }
    }

    #[test]
    fn ownership() {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid now.

        assert_eq!(map.len(), 1);
        assert_eq!(map.get("Favorite color"), Some(&String::from("Blue")));
    }

    #[test]
    fn update() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        assert_eq!(scores.len(), 1);
        assert_eq!(scores.get("Blue"), Some(&25));

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        // Because Blue alterady exists, the value won't be updated.
        scores.entry(String::from("Blue")).or_insert(50);

        assert_eq!(scores.len(), 2);
        assert_eq!(scores.get("Blue"), Some(&10));
        assert_eq!(scores.get("Yellow"), Some(&50));

        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            // or_insert returns a mutable reference to the value.
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        assert_eq!(map.get("world"), Some(&2));
        assert_eq!(map.get("hello"), Some(&1));
        assert_eq!(map.get("wonderful"), Some(&1));
    }
}
