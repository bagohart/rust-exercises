fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    println!("{}", s1);

    let s2 = s1.clone();
    println!("{}, {}", s1, s2);

    takes(s2);
    println!("{}", s1);
    let x = 5;
    copy(x);
    println!("{}", x);

    let s3 = gives();
    let s3 = takes_and_gives(s3);
    println!("s3={}", s3);
}

fn takes(some_string: String) {
    println!("{}", some_string);
}

fn copy(some_int: i32) {
    println!("{}", some_int);
}

fn gives() -> String {
    String::from("just 4 you")
}

fn takes_and_gives(a_string : String) -> String {
    a_string
}
