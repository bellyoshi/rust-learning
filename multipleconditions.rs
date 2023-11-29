fn main(){
    println!("Enter a number: ");
    let number: i32 = get_number();
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn get_number() -> i32 {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number"),
        }
    }
}