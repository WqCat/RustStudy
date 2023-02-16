mod back_of_bouse {
    //pub struct 
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,  //私有
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    //pub enum
    pub enum Appetizer {
        Soup,
        Salad,    //都是公有的
    }

}

pub fn eat_at_restaurant() {
    let mut meal = back_of_bouse::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    meal.seasonal_fruit=String::from("s: &str");   //seasonal_fruit为私有的会报错 
}