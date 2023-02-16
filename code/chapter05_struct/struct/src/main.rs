#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32, 
}

//struct方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other : &Rectangle) -> bool {
        self.height >= other.height && self.width >= other.width
    }

    //关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    //一个struct可以有多个方法，一个imple可以拆成多个
}

fn main() {
    let s = Rectangle::square(20); 
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 30,
        height: 55,
    };


    println!("{}", rect1.area());

    println!("{}",rect1.can_hold(&rect2));

    println!("{}",rect1.can_hold(&rect3));

    println!("{:#?}", rect1);
}


