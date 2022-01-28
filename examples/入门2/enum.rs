#[derive(Debug)]
enum IPAddrKind {   //枚举类型，枚举的成员位于其标识符的命名空间中
    V4,
    V6,
}

let four=IPAddrKind::V4;    //应该加分号，代表一个语句
let six=IPAddrKind::V6;     // 成员均为IpaddrKind类型

fn route(ip_type:IPAddrKind){}

//可以使用任一成员来调用这个函数
route(IPAddrKind::V4);

//结构体也是语句的一种，所以应该加分号
struct IPAddrKind
{
    king :IPAddrKind,
    address:String,
};


let home=IPAddrKind
{
    king:IPAddrKind::V4,
    address:String::from("127.0.0.1"),
}

let loop_back=IPAddrKind
{
    kind:IPAddrKind::V6,
    address:String::from("::1"),
}

//更简洁的写法 仅仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分
//新定义表示v4和v6都关联了string值
//可以直接加数据附加到枚举的每个成员上，这样就不用额外定义一个结构体

enum IPAddrKind
{
    v4(String),
    v6(String),
}

let home =IPAddrKind::V4(String::from("127.0.01"));
let loop_back=IPAddrKind::V6(String::from("::1"));


//使用枚举代替结构体的优势还有，每个成员可以处理不同类型和数量的数据
enum IPAddrKind
{
    v4(u8,u8,u8,u8),        //使用元组结构体？
    v6(String),
}

//如果我们使用不同的结构体，由于它们都有不同的类型，
//我们将不能像使用枚举那样，轻易的定义一个能够处理这些不同类型的结构体的函数，因为枚举是单独一个类型

//也可以改枚举定义方法
enum message
{
    quit,
    move(x:i32,y:i32),
    write(String),
    change_color(i32,i32,i32),
}       //枚举不用写分号，结构体需要写

impl message {
    fn call(&self)
    {}
}

//option的使用
//Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。这个枚举是 Option<T>，而且它定义于标准库中
// Option<T> 也仍是常规的枚举，Some(T) 和 None 仍是 Option<T> 的成员。
enum Option<T>  //T为泛型
{
    Some(T),
    None,   //即使是最后一个，也需要加逗号
}

//如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。
//当有一个 Some 值时，我们就知道存在一个值，而这个值保存在 Some 中。
//当有个 None 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值。


//from my desktop