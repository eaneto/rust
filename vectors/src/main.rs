enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let _v = vec![1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![100, 32, 57];
    print_vector(&v);

    mutate_vector(&mut v);

    assert_eq!(v.pop(), Some(107));

    print_vector(&v);

    v.sort();

    assert_eq!(v.is_empty(), false);

    print_vector(&v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    assert_eq!(row.len(), 3);
}

fn print_vector(vec: &Vec<i32>) {
    // Immutable reference
    for i in vec {
        println!("{}", i);
    }
}

fn mutate_vector(vec: &mut Vec<i32>) {
    // Mutable reference
    for i in vec {
        *i += 50;
    }
}
