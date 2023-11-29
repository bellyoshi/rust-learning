fn main(){
    let mut a : [i32;5] = [1, 2, 3, 4, 5];
    let b : [i32;5] = [10; 5];
    a[1] = b[1];
    println!("{:?}", a);
    println!("{:?}", b);
}