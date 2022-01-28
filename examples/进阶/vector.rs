fn fn main() {
    println("fuck ");
}

let v:Vec<i32>=Vec::new();


vec!可以自动推断vector中元素的类型
let v=vec![1,2,3];  //直接包含初值，类型说明并不是必须的

//如任何变量一样，如果想要能够改变它的值，必须使用 mut 关键字使其可变。
//放入其中的所有值都是 i32 类型的，而且 Rust 也根据数据做出如此判断，所以不需要 Vec<i32> 注解。
let mut v=Vec::new();   
v.push(5);

v.push(6);

//丢弃 vector 时也会丢弃其所有元素
//当 vector 被丢弃时，所有其内容也会被丢弃，这意味着这里它包含的整数将被清理。
//一旦开始使用 vector 元素的引用，情况就变得有些复杂了

//访问 vector 中一个值的两种方式，索引语法或者 get 方法
let v = vec![1, 2, 3, 4, 5];

//使用 & 和 [] 返回一个引用；
//或者使用 get 方法以索引作为参数来返回一个 Option<&T>。
//Rust 有两个引用元素的方法的原因是程序可以选择如何处理当索引值在 vector 中没有对应值的情况
let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
                None => println!("There is no third element."),
}


//这两个不同的获取第三个元素的方式分别为：
//使用 & 和 [] 返回一个引用；
//或者使用 get 方法以索引作为参数来返回一个 Option<&T>。
//Rust 有两个引用元素的方法的原因是程序可以选择如何处理当索引值在 vector 中没有对应值的情况

//有一个有五个元素的 vector 接着尝试访问索引为 100 的元素时程序会如何处理

//ctrl+~,隐藏显示终端

遍历vector
let v=vec![100,32,57];
for i in &v
{
    println!("{}",i)
}

//调用vector中每个元素的可变引用
let mut v=vec![100,32,57];
for i in &mut v{
    *i+=50;
}
//为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值。
//枚举的成员都被定义为相同的枚举类型，所以当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举！

enum SpreadsheetCell
{
    Int(i32),
    Float(f64),
    Text(String), //即使是最后一个元素，也要加上逗号
}

let row=vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];//语句都要加逗号

//定义一个枚举，以便能在 vector 中存放不同类型的数据

let mut s=String::new();
let data = "initial contents";

let s = data.to_string();

// 该方法也可直接用于字符串字面值：
let s = "initial contents".to_string();

//两条语句，效果等价
let s = String::from("initial contents");

//可以方便的使用 + 运算符或 format! 宏来拼接 String 值。

let mut s = String::from("foo");
s.push_str("bar");
//push_str 方法获取字符串 slice，因为我们并不需要获取参数的所有权

//一旦获取到了所有权，自身就会无法使用
//push追加单个字符
s.push('l');

//let s3 = s1 + &s2; 看起来就像它会复制两个字符串并创建一个新的字符串，而实际上这个语句会获取 s1 的所有权，附加上从 s2 中拷贝的内容，并返回结果的所有权。
//换句话说，它看起来好像生成了很多拷贝不过实际上并没有：这个实现比拷贝要更高效

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);


//这些代码也会将 s 设置为 “tic-tac-toe”。format! 与 println! 的工作原理相同，不过不同于将输出打印到屏幕上，它返回一个带有结果内容的 String。
//这个版本就好理解的多，并且不会获取任何参数的所有权。

//rust 中的字符串不支持索引
//最后一个 Rust 不允许使用索引获取 String 字符的原因是索引操作预期总是需要常数时间 (O(1))。
//但是对于 String 不可能保证这样的性能，因为 Rust 不得不检查从字符串的开头到索引位置的内容来确定这里有多少有效的字符。