fn main() {
    loop {
        println!("again");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result={}",result);

    loop {
        counter +=1;
        if counter == 20 {
            break 7;
        }
    };

    let mut n = 3;
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }
    println!("LIFTOFF!!!");

    let arr = [1,2,3,4,5];
    for e in arr.iter() {
        println!("v={}", e);
    }

    for n in (1..4).rev() {
        println!("{}!", n);
    }
    
}
