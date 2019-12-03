use std::io;
fn main() {
    println!("Calculator");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
    .expect("Enter correct opertors");

    let input : i32  = input.trim().parse().unwrap();
    println!("{}",input);

}
