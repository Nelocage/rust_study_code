// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

use std::fmt;
// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);      //因为matrix本身就没有mut,所以也不可以创建一个可变引用
//这块不需要多考虑，matrix已经限定只有四个值

//display 只能返回result，注意第二行没有分号，表示一个返回值
impl fmt::Display for Matrix{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        writeln!(f,"({},{})",self.0,self.1)?;
        write!(f,"({},{})",self.2,self.3)
    }
}

//需不需要引用，需不需要可变
fn transpose(m: &Matrix)->Matrix{


//初始化的方式要会
let m1=Matrix(m.0,m.2,m.1,m.3);
m1


/*
错误的方式
let  mut m1:Matrix;
m1.0=m.0;
m1.2=m.1;
m1.1=m.2;
m1.3=m.3;
m1
会提示m1,没有初始化
*/
    
}
fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    
    // But long Tuples cannot be printed
     //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
     //println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}",matrix);
    println!("{}",transpose(&matrix));

}

use必须带分号，和C和Python不同，理解为rust中的一条语句，
类型注释不可以丢
