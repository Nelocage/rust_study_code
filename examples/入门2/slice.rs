//slice允许引用集合中的一段连续的元素序列，而不用引用整个集合

fn first_word(some_string : &String)->usize  //由于不需要使用所有权，所以这个可以
{
   let bytes=some_string.as_bytes(); //将String 转化为字符数组

   for(i,&item) in bytes.iter().enumerate() //iter创建一个迭代器，enumerate包装iter的结果并返回一个
   //元组，其中每一个元素是元组中的一部分，enumerate返回元组中的第一个元素是索引，第二个元素是集合中元素的引用
   {
       if item==b' '{       //模式解构
           return i;
       }
   }

 some_string.len()  //vscode 如果没有合适的提醒，一定是有一定的错误，iter方法可以返回集合中的每一个元素

}
//建立一个rust项目命令行使用cargo new 才能创建一个完整的

//上一个程序，usize,是一个与string无关的变量，必须时刻进行同步

//slice 是对一部分值的引用
let s=String::from("hello,world");
let hello=&s[0..5];     //返回一个引用，默认是左闭右开，如果想变成闭区间
let hello1=&s[0..=5];   //闭区间，这种写法则表示包含最后一个元素
let hello2=&s[..5]; //如果想从第一个索引开始，则可以不写，如果到最后，则最后一个元素也可以省略
let hello3=&s[..];  //从头到尾

//更好的写法
fn first_word1(s:String)->&str      //字符串字面值的不可变的引用，
//fn first_word1(s:&str)->&str  更好的函数声明，可以对String和str使用相同的函数,使接口具有通用性
{
    let bytes=s.as_bytes();

    for (i ,&item) in bytes.iter().enumerate()
    {
        if item==b' '
        {
            return &s[0..i];
        }
    }
    &s[..]
}

//字符串字面值就是slice,s的类型是&str，&str是一个不可变的引用
let s="hello ,world";

