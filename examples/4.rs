use std::io;

fn main() {
    println!("guess the number!");
    println!("please in your guess ");
    

    let mut guess =String::new();   //注意这是一个 let 语句，用来创建 变量

    io::stdin().read_line(&mut guess).expect("failed to read line");

    //println!("you guessed:{}",guess);
    
    println!("You guessed:{}", guess);
}
