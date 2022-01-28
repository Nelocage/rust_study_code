use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    //println!("Please input your guess.");

    // io::stdin().read_line(&mut guess)
    //     .expect("Failed to read line");

    loop {
        println!("please input your guess");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to .read_line(&mut gueread line");

        //let guess: u32 = guess.trim().parse().expect("please type a number"); //允许复用和隐藏
        //将 expect 调用换成 match 语句，是从遇到错误就崩溃转换到真正处理错误的惯用方法
        let guess :u32=match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,  //忽略非字符的错误
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}

//语句执行操作但不返回值
//表达式计算并且产生一个值，x=y=6不可以这样写
//表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。两者之间就差一个分号

fn another_fn(x :i32)   //必须声明每个函数参数的类型
{
    let y=five();   //使用函数的返回值初始化一个值

    println!("this is {}",x);
}   //函数后面没有分号


fn five()->i32{
    5   //没有分号，因为这是一个表达式，想要返回他的值
}

//hello