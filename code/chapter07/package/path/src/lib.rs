mod front_of_house {
    pub mod housing {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::housing::add_to_waitlist();  //绝对路径，都是跟所以能访问到front_of_house
    front_of_house::housing::add_to_waitlist();
}