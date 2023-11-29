fn main(){
    let mut t = (1, "2");
    println!("{:?}", t);
    t.0 = 2;
    t.1 = "3";
    println!("{:?}", t);
}