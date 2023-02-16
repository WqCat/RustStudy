mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
} 

use crate::front_of_house::hosting;

use std::fmt::Result;
use std::io::Result as IoResult;   //as的用法，类似重命名

use rand::Rng;   //外部的包 Cargo.toml有rand
use std::collections::HashMap;  //配置文件不需要引用

use std::io::{self,Write};   //一个是本身，一个是下面的函数   *的为全部，一般不用

fn main() {
    hosting::add_to_waitlist();
}
