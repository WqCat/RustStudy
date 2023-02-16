fn main() {
   let condition = true;
   let number=if condition{ 5 } else { 6 };//前后类型一致，后面不运行，也会识别
   println!("The value of number is:{}",number);
}
