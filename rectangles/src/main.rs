#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn from(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle::from(30, 50);
    println!("The area of the rectangle is {:#?} square pixels.", rect1.area());

    //let five = Some(5);
    //let six = plus_one(five);
    //let _none = plus_one(None);

    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(x) => println!("The third element is {}", x),
        None => println!("There is no third element"),
    }

    for i in &v {
        println!("{}", i)
    }
}

//fn plus_one(x: Option<i32>) -> Option<i32> {
//    match x {
//        None => None,
//        Some(i) => Some(i + 1),
//    }
//}
