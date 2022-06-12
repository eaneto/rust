use std::collections::HashMap;

fn main() {
    let mut list = vec![5, 9, 1, 3, 0, 3, 5, 8, 2, 4, 5, 3, 2];
    list.sort();
    let median = list[list.len() / 2];
    println!("Median {}", median);

    let mut mode = HashMap::new();
    let mut highest = 0;
    let mut most_common = 0;
    for number in &list {
        let count = mode.entry(number).or_insert(0);
        *count += 1;
        if *count > highest {
            highest = *count;
            most_common = *number;
        }
    }
    println!("Mode {}", most_common);
}
