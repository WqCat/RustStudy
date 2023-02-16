fn main() {
    let s1 = String::from("Hello");
    
    let len = calculate_length(&s1);  //用的地址不给权限不能修改，
    
    let mut s2 = String::from("Hello");
    let len1 = calculate_length01(&mut s2); //用的地址不给权限可以修改，

    println!("The length of '{}' is '{}'----'{}'",s1,len,len1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn calculate_length01(s: &mut String) -> usize {
    s.push_str(",World");
    s.len()
}