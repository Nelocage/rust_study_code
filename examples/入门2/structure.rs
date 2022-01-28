 struct User {
     username: String,
     email: String,
     sign_in_count: u64,
     active: bool,
 }

let user1 = User {
    email:String::from("@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

//获取结构体中某个特定的值，可以使用点号

//更改结构体中的值,若可变则都可变，不允许部分可变
let mut user2=User
{
    email:String::from("@exap"),
    username:String::from("hello"),
    active:true,
    sign_in_count:1,
};

user2.email=String::from("helllll");
//结构体也是表达式，可以被函数返回

fn build_user(email:String,username:String)->User
{
    User
    {
        email:email,    //可简写成email,
        username:username,      //可简写成username，
        active:true,
        sign_in_count:1,
    }
}

//结构体更新语法
let user2=User{
    email,
    username,
    active:user1.active,//使用user1 的部分值
    ..user1 //剩下的和user1完全相同
}

//使用没有命名字段的元组结构体来创建不同的类型，即元组结构体，没有具体的字段名，只有字段的类型
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
//一个获取color类型参数的函数，不可以接收一个point类型的值，元组结构体也类似元组
//可以解构成单独的部分，也可以通过点号加索引来访问单独的值

//rust中多使用结构体,特别是某些具有关系的问题中

#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

fn main()
{
    let rect1=Rectangle{width:30,height:50};

    println!("is {}",area(&rect1));     //一定要和C++/C中的引用区分开
    println!(&rect1.area())
    //希望借用结构体而不是获取它的所有权，
    //这样 main 函数就可以保持 rect1 的所有权并继续使用它，所以这就是为什么在函数签名和调用的地方会有 &
}

fn area(rectangle:&Rectangle)->u32
{
    rectangle.width*rectangle.width     
}

//使用派生trait来增加实用功能,比如第56行的情况
//可以通过derive 注解来使用trait

//方法  impl 为implementation，这种相当于Java和C++ 类的方法
//注意仍然需要在 self 前面加上 &，就像 &Rectangle 一样。方法可以选择获取 self 的所有权，
//或者像我们这里一样不可变地借用 self，或者可变地借用 self，就跟其他参数一样。
//&在rust中的作用感觉和C++ 中正好相反，恰恰是借用，并不想获取所有权，只是希望读取
//结构体中数据，而不是写入
impl Rectangle {
    fn area(&self)->u32
    {
        self.width*self.height
    }

    fn can_hole(&self,other:&Rectangle)->bool  //获得一个不可变的引用
    {
        self.width>other.width && self.height>other.height
    }

    //关联函数，不以self为参数的函数，用于返回一个结构体新实例的构造函数，C++中的构造函数
    //调用语法为结构体名字::,表示这个方法位于结构体的命名空间中，::语法用于关联函数和模块创建的命名空间
    fn square(size:u32)->Rectangle
    {
        Rectangle{width:size,height:size}
    }

}

//每个结构体都允许拥有多个 impl 块