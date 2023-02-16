fn main() {
    let s1 = gives_owership();
    
    let s2 = String::from("Hello World");

    let s3 = takes_and_gives_back(s2); //此时s2已经废了

    println!("{},{}",s1,s3);
}

fn gives_owership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
} 