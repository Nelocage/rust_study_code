fn fn main() {
    println("fuck ");
}

let v:Vec<i32>=Vec::new();

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