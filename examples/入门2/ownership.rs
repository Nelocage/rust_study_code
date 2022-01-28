//所有权的实验代码，通过所有权系统管理程序的内存，内存在离开作用域后就自动被释放

let s="hello";
{
let p=String::from("hello"); //动态请求内存
}


//深拷贝和浅拷贝，rust中只有移动，没有深拷贝，浅拷贝之分
//深拷贝的功能由clone实现

let s1=String::from("helllo");
let s2=s1.clone();  //执行深赋值，复制堆中的程序



fn main() {
    
    let s1=gives_ownership();       //将返回值移交给s1
    let s2=String::from("hello");   //s2进入作用域
    let s3=takes_and_gives_back(s2);    //s2被移动到takes_and 函数中，它也将返回值移 给s3
    takes_owership(s);

    let x=5;
    makes_copy(x);
}

fn gives_ownership()->String
{
    let some_string=String::from("hello");
    some_string
    
    }

fn takes_and_gives_back(a_string:String)->String
{
    a_string        //返回a_string 
}

fn takes_owership(some_string:String)       //变量要指明类型
{
    println!("{}",some_string);
}

fn makes_copy(some_interge:i32)
{
    println!("{}",some_interge);
}

fn main()
{
    let s1=String::from("hello");

    let(s2,len)=calculate_lenght(s1);

    println!("the length of '{}'is {}",s2,len);
}

fn calculate_lenght(a_string:String) ->(String,usize)
{
    let length=s.len();
    (s,length)
}

//rust中的引用
fn references()
{
    let s1=String::from("hello");

    let len=calculate_lenght_references(&s1);       //&即代表C++ 中的引用，允许使用值，但不获取所有权
    //创建一个指向s1的引用，但是并不拥有它，因为不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃
    //当引用离开作用域后并不丢弃他指向的数据，因为没有所有权，，当函数使用引用而不是实际值作为参数，
    //无需返回值来交还所有权，因为就不曾拥有过所有权


}

fn calculate_lenght_references(s:&String )->usize
{
    s.len()
}

//将获取引用作为函数参数称为借用，borrowing,也没法修改借用的变量
//正如变量默认是不可变的一样，引用也一样，（默认）不允许修改引用的值，注意这和C++ 完全不同

//可变引用
//原始变量是mut,并且还要创建一个Mut 的引用，还要接受一个可变引用，
fn references_mut()
{
    let mut s=String::from("hello");

    change(&mut s);
}

fn change(some_string :&mut String)//注意这里的写法
{
    some_string.push_str(",world");
}

//可变引用有 一个极大的限制，就是同一个作用域内，有且只有一个可变引用，不可有多个
//这个限制允许有一定的可变性，但是是一种受限制的方式允许的，这个限制有什么好处
//可以在编译的时候就避免数据竟态，有点像操作系统中的锁一样

//数据竟态会导致未定义的行为，难以在运行时追踪，并且难以诊断和修复，rust避免了这种情况，因为rust根本就不会编译存在数据竟态的代码
//但是可以不同时拥有

let mut s=String::from("hello");


{
    let r1=&mut s;
}       //r1在这里离开了作用裕，所以可以创建一个新的引用

let r2=&mut s;

//我们也不能在拥有不可变引用的同时拥有可变的引用，然而多个不可变引用是可以的

//垂直引用 dangling references

fn mian()
{
    let reference_to_nothing =dangle();
}

fn dangle()->&String //返回一个字符串的引用
{
    let s=String::from("hello");    //s是一个新的字符串

    &s  //返回字符串s的引用，s已经被释放却返回一个对他的引用，正确的代码是直接返回一个字符串，这样所有权被移动出去，所以没有值被释放

}

//引用的规则
//1.在任意给定的时间，要么只能有一个可变引用，要么只能有多个不可变的引用
//2.引用必须总是有效的