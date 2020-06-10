#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn t(self) -> u32 {
        5
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) ||
            (self.width > other.height && self.height > other.width)
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    // let rect1 = (30, 50);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("rect1={:?}", rect1);
    println!("rect1={:#?}", rect1);
    println!("{:#?}", rect1);
    println!("area={}", area(&rect1));
    println!("area={}", rect1.area());
    // println!("[?]={}", t(&rect1)); // doesn't work
    //  lmao v
    // rect1.t();
    // println!("{:#?}", rect1);

    println!("can r1 hold r2? {}" , rect1.can_hold(&rect2));
    println!("can r1 hold r3? {}" , rect1.can_hold(&rect3));

    let sq = Rectangle::square(5);
    println!("{:#?}", sq);
}

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
