//trait  ----特性
//trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。
//可当做接口理解
//trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。

//可理解成抽象接口
pub trait Summary
{
    fn summarize(&self)->String;
}

//在方法签名后跟分号，而不是在大括号中提供其实现。接着每一个实现这个 trait 的类型都需要提供其自定义行为的方法体，编译器也会确保任何实现 Summary trait 的类型都拥有与这个签名的定义完全一致的 summarize 方法。

//trait 体中可以有多个方法：一行一个方法签名且都以分号结尾。


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//在类型上实现 trait 类似于实现与 trait 无关的方法。区别在于 impl 关键字之后，我们提供需要实现 trait 的名称，接着是 for 和需要实现 trait 的类型的名称。
//在 impl 块中，使用 trait 定义中的方法签名，不过不再后跟分号，而是需要在大括号中编写函数体来为特定类型实现 trait 方法所拥有的行为。
//都说rust的学习曲线高，但无所谓，C++你也不会，那个学习曲线更高，没什么负担

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")  //带有默认实现的trait

    }
}
//如果想要对 NewsArticle 实例使用这个默认实现，而不是定义一个自己的实现，则可以通过 impl Summary for NewsArticle {} 指定一个空的 impl 块。

//默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

//trait作为参数
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
//item 是实现了summary trait的某种类型

//trait bounds
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
//不需要再学习C++的任何东西，止步于算法就可以
//不同类型，使用impl
pub fn notify(item1: impl Summary, item2: impl Summary) {}
//相同类型，使用trait bounds
pub fn notify<T: Summary>(item1: T, item2: T) {}

//通过+指定多个trait
//通过where 简化代码
//返回trait，但是只适用于返回单一类型的情况
fn returns_summarizable() -> impl Summary {}

//rust比C++ 安全简单很多，比C强大很多


