fn main() {
    let s = "hello";
    println!("{}.len = {}", s, s.len());

    let s2 = "안녕";
    println!("{}.len = {}", s2, s2.len());

    println!("{}", s2.chars().count());

    let bananas = "bananas";

    assert!(bananas.contains("nana"));
    assert!(!bananas.contains("apples"));

    let hello = String::from("Hello, world!");
    println!("{}",hello);

    let s1 = "Hello, ".to_owned();
    let s2 = "world!".to_string();

    let mut s = String::from("foo");
    s.push_str("bar");
    assert_eq!("foobar", s);
    
    let a: String = String::from("foo");
    let b: String = String::from("bar");
    let c = a + &b;

    println!("c={}",c); // "foobar"
}
