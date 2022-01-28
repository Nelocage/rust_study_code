pub fn bubble_sort<T:Ord>(arr:&mut [T]){
    for i in 0..arr.len(){
        for j in 0..arr.len()-i-1{
            if arr[j]>arr[j+1]{
                arr.swap(j, j+1);
            }
        }
    }
}






#[cfg(test)]
mod  test{
    use super::*;
}

fn main(){
    let mut ve1=vec![6,7,4,2,6,77,10];
    bubble_sort(&mut ve1);
    println!("{:?}",ve1);

}