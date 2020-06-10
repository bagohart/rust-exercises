fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("s2={}, len={}", s2, len);
    let len = calculate_length_borrowing(&s2);
    println!("s2={}, len={}", s2, len);
    let mut s3 = s2.clone();
    mutate_borrowed(&mut s3);
    {
    let s4 = &mut s3;
    }
    let s5 = &mut s3;
    // println!("s4={}, s5={}", s4, s5);
    // let s5 = & s3; // doesn't work either
    // println!("s4={}, s5={}", s4, s5);
    let mut x = 5;
    let y = &x;
    let z = &x;
    println!("[?]={}, <??>={}, (??)={}", x, y, z);
    let w = &mut x;

    // let ref_to_nowhere = dangle();
    let real_ref = no_dangle();
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_borrowing(s: &String) -> usize {
    s.len()
}

fn mutate_borrowed(s: &mut String) {
    s.push_str(" ahaha");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
