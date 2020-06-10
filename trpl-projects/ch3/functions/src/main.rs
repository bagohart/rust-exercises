fn main() {
    println!("Hello, world!");

    another_function();
    yet_another_function(7);

    let x = 1337;
    let y = {
        let x = 3;
        x + 1
    };
    println!("y={}",y);
    println!("x={}",x);
}

fn another_function() {
    println!("Another function.");
}

fn yet_another_function(x: i32) {
    println!("x={}", x);
    // let x = (let y = 7); // nope.
}

fn five() -> i32 {
    5
}

fn six() -> i32 {
    return 6;
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // return ();
}


