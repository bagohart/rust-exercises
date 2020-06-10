// const MAX_POINTS: u32 = 100_000; // this works

fn main() {
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);
    let x = x+1;
    println!("x = {}", x);
    let x = x*2;
    println!("x = {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);
    // This doesn't work:
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // let num = "42".parse().expect("Not a number!"); // this doesn't work

    let _num = 0xffu8;
    let _byte = b'A';

    // don't work
    // let falscher_bool: u8 = true;
    // let falscher_bool: u8 = 1==1;

    let _c: char = 'L';

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("y={}", y);
    println!("z={}", tup.2);

    let arr = [1,2,3,4,5];
    // let arr: [i32;5] = [1,2,3,4]; // this doesn't work.
    let arr = [1; 5];
    let first = arr[0];
    let wrong_index = 5;
    let nope = arr[wrong_index];

}
