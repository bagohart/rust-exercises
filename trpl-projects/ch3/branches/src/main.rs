fn main() {
    println!("Hello, world!");

    let num = 3;

    if num < 5 {
        println!("true");
    } else {
        println!("false");
    }

    // if 3 {
    //     println!("not allowed");
    // }

    if num % 4 == 0 {
        println!("lol");
    } else if num % 3 == 0 {
        println!("3")
    } else if num % 2 == 0 {
        println!("2")
    } else {
        println!("else");
    }

    let x = if true {
        5
    } else { 7 };
}
