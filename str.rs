fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main(){
    let s1 : String = String::from("Hello, World!");
    let s2 : &str = &s1;
    let s3 : String = s2.to_string();
    print_type_of(&s1);
    print_type_of(&s2);
    print_type_of(&s3);
}