use std::io;

fn main() {
    println!("Enter the Fahrenheit temperature:");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Error reading input.");

    let temperature: f32 = temperature
        .trim()
        .parse()
        .expect("Error converting input to float");

    let celsius = (temperature - 32.0) * (5.0 / 9.0);

    println!("{}ºF -> {}ºC", temperature, celsius)
}
