fn main() {
    println!("Hello, world!");
    let s = String::from("  ahah lhal lol ");
    let f = first_word(&s);
    println!("[?]={}", f);

    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello={}, world={}", hello, world);

    let slice = first_word_better(&s);
    // s.clear();
    println!("[?]={}", slice);

    let sl = "Hello, world!";
    let x = sl;
    let y = sl;
    println!("[?]={}, <??>={}", x, y);
    let sli = &sl[2..7];
    println!("[?]={}", sli);

    let lit = "lolol trollol lol";

    let f = first_word_better(&lit);
    println!("[?]={}", f);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // for (i, item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         println!( "defuq");
    //     }
    //     println!("[?]={}", item);
    // }
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_better(s: &str) -> &str {
    println!("first word better!");
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

