use rand::Rng;
use std::io;
use mini_redis::{client,Result};

/*   两者等价
#[tokio::main]
 async fn main() {
     println!("hello");
 }


 fn main() {
     let mut rt = tokio::runtime::Runtime::new().unwrap();
     rt.block_on(async {
         println!("hello");
     })
 }
*/ 


#[tokio::main]
async fn main()->Result<()>{

    let mut client=client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;


    let result=client.get("hello").await?;


    println!("got value from the server; result={:?}",result);

    Ok(())
}

fn test() {
    println!("**************");
    println!("guess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("随机数为{}", secret_number);
    loop {
        println!("**************");
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        //guess 进行类型转换
        println!("you guessed :{}", guess);

        //trim 用于消除回车
        //expect 调用换成 match 语句，以从遇到错误就崩溃转换为处理错误
        // let guess: u32 = guess.trim().parse().expect("please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //字符串比较
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("samll"),
            std::cmp::Ordering::Greater => println!("big"),
            std::cmp::Ordering::Equal => {
                println!("bingo");
                break;
            }
        }
    }

    //let 变量默认不可变  可变变量 和不可变变量   可以进行变量推导
    //const 常量   不允许对常量使用mut 常量不光默认不可变 他总是不能变  必须指定类型，无法进行变量推导
    //常量只能被设置为常量表达式，而不可以是其他任何只能在运行时计算出的值。
    //Rust 对常量的命名约定是在单词之间使用全大写加下划线

    //元组长度是固定的，类型可以是不同的
    let tup = (500, 6.4, 1);

    //可以使用pattern matching 解构元组
    let (x, y, z) = tup;

    //也可以使用点号（.）后跟值的索引来直接访问它们
    let p = tup.0;

    //数组中元素类型必须相同，长度是固定的，相当于长度也算进type

    // println!("Hello, world!");
}

fn fn1(x: i32) {}
