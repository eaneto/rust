const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    // let x = 5;
    // x = 6;
    // cannot assign twice to immutable variable
    x = 6;
    println!("The value of x is: {}", x);

    println!("Constant: {}", THREE_HOURS_IN_SECONDS);

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_, y, _) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;

    let _six_point_four = tup.1;

    let _one = tup.2;

    let _a = (500, 6.4, 1).0;

    some_function();

    another_function(five_hundred);

    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    let x = plus_one(five());

    println!("The value of x is: {}", x);
}

fn some_function() {
    println!("This functions does something")
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
