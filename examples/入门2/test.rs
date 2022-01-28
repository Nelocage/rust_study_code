fn fn main() {
    let tup:(i32,f64,u8)=(500,6.4,1);
    let (x,y,z)=tup;        //元组解构
    let five=tup.0; //也可以使用.进行访问，元组的第一个索引值为0
    let a:[i32,5]=[1,2,3,4,5];//定长数组，不可以扩大
    let first=a[0];
    println!(tup);
}
//rust 中没有函数声明，定义在前，在后都可以


fn if_test()
{
    let number =3;
    let condition=true;
    let number_1=if condition{5}else{6};    //中间不需要加分号
    if number<5{println!("true");}
    else {
        println!("false");
    }
    }

fn test1()
{
    let mut number =3;
    while number!=0
    {
        println!("{}",number);

        number=number-1;
    }

    let a=[10,20,30,40,50];
    let mut index=0;

    while index<5
    {
        println!("the value is :{}",a[index]);

        index=index+1;
    }

    for element in a.iter()  //使用for循环，比上一组增强了安全性
    {
        println!("the value is :{}",element);
    }

    //使用range

    for number in(1..4).rev()
    {
        println!("{}",number );
    }







}
