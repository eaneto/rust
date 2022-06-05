fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s)
}

// a reference is not a pointer because a reference is always safe.
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but because it was a reference it didn't
  // have ownership of what s refers to.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
