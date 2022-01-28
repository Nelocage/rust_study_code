fn main() {
    let number_list=vec![34,50,25,100,65];

    let mut largest=number_list[0];
    for number in number_list
    {
        if number>largest
        {
            largest=number;
        }
    }

    println!("the largest number is {}",largest);
}


//一般泛型
fn largest(list :[i32])->i32
{
    let mut largest=list[0];

    for &item in list.iter()
    {
        if item>largest
        {
            largest=item;
        }
           }

           largest  //作为返回值，返回最后一个数
}

//更一般的泛型,这个会报错，因为没有实现比较机制，无法使用大于,加上泛型就好
fn largest<T：PartialOrd+Copy> (list:&[T])->T
{
    let mut largest=list[0];

    for &item in list.iter()
    {
        if item>largest
        {
            largest=item;
        }
    }

    largest
}
//结构体重的泛型
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

//永远不要碰C++ ，只用rust进行编程
//定义多个泛型（这也可以，C++ 好像只支持一种泛型，辣鸡C++）
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

//现在所有这些 Point 实例都是被允许的了！你可以在定义中使用任意多的泛型类型参数，不过太多的话代码将难以阅读和理解。
//当你的代码中需要许多泛型类型时，它可能表明你的代码需要重组为更小的部分。

//枚举定义中的泛型
enum Option<T> {
    Some(T),
    None,
}

//如你所见 Option<T> 是一个拥有泛型 T 的枚举，它有两个成员：Some，它存放了一个类型 T 的值，和不存在任何值的None。
//通过 Option<T> 枚举可以表达有一个可能的值的抽象概念，同时因为 Option<T> 是泛型的，无论这个可能的值是什么类型都可以使用这个抽象。

//枚举也可以有多个泛型类型，如result枚举
enum Result<T, E> {
    Ok(T),
    Err(E),
}

//当代码中有多个只有存放的值的类型有所不同的结构体或者枚举定义时，应该引入泛型来减少代码
//方法中的泛型
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

//注意必须在 impl 后面声明 T，这样就可以在 Point<T> 上实现的方法中使用它了。在 impl 之后声明泛型 T ，
//这样 Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型。

//方法使用了与结构体定义中不同类型的泛型,新建项目， 命令行，cargo new
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


//千万别看编译原理，坑太深，别陷进去， rust的编译器已经足够好，编写C++需要理解编译原理，和
//编译器斗智斗勇的时代已经过去了

//Rust 实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失
//我们可以使用泛型来编写不重复的代码，而 Rust 将会为每一个实例编译其特定类型的代码。这意味着在使用泛型时没有运行时开销；当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样。这个单态化过程正是 Rust 泛型在运行时极其高效的原因。