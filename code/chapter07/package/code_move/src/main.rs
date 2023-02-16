mod front_of_house;//在同级目录下找文件

//     {
//     pub mod hosting {
//     pub fn add_to_waitlist() {}
//     }
// }

pub use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}
