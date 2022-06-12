fn main() {
    let data = "initial contents";

    let s = data.to_string();

    assert_eq!(s, data);

    // the method also works on a literal directly:
    let s1 = "initial contents".to_string();

    assert_eq!(s1, data);

    let s2 = String::from("initial contents");

    assert_eq!(s1, s2);

    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    assert_eq!(s2, "bar");
    assert_eq!(s1, "foobar");

    let mut s = String::from("lo");
    s.push('l');
    assert_eq!(s, "lol");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // the compiler uses deref coersion to transform &s2 into &s2[..]
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                        // Signature of add: fn add(self, s: &str) -> String
                        // Add only takes ownership of s1

    assert_eq!(s2.is_empty(), false);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    assert_eq!(s, "tic-tac-toe");

    // or with format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    assert_eq!(s, "tic-tac-toe");

    let hello = String::from("Hola");
    assert_eq!(hello.len(), 4);
    let hello = String::from("Здравствуйте");
    assert_eq!(hello.len(), 24);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
