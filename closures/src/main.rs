use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Eq + Hash,
{
    calculation: T,
    values: HashMap<E, E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Eq + Hash + Copy,
{
    fn new(calculation: T) -> Cacher<T, E> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: E) -> E {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(&intensity));
        println!("Next, do {} situps!", expensive_closure.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(&intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_different_types() {
        let mut c1 = Cacher::new(|a| a);
        let mut c2 = Cacher::new(|a| a);

        let v1 = c1.value(1);
        let v2 = c2.value("potato");

        assert_eq!(v1, 1);
        assert_eq!(v2, "potato");
    }

    #[test]
    fn capturin_environment() {
        let x = 4;

        let equal_to_x = |z| z == x;

        let y = 4;

        assert!(equal_to_x(y));
    }
}
