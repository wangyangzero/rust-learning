fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    // println!("s: {}", s);
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    println!("x: {}", x);
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut str1: String= String::from("hello");
    add(&mut str1);
    println!("str1: {}", str1);

    let mut str: String= String::from("hello");
    let str1 = &mut str;
    // let str2 = &mut str;
    println!("{} {}", str1, str1)
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn cal_len(s: &String) -> usize {
  s.len()
}

fn add(s: &mut String) {
  s.push_str(",world");
}