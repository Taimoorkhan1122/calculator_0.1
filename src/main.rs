use std::io;

fn main() {
    println!("Calculator");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
    .expect("Enter correct opertors");
    
    let result = calculator(input);

}

fn calculator(x : String) { 
    let mut vec = Vec::new();
    for i in x.as_str().split_whitespace(){
    vec.push(i)
    }
    // let index = vec.len();
    // if vec.len() <  {
    //     panic!("Try adding space between elements");
    // }
    //else {
    let x = vec[0];
    let y = vec[2];
    
    let mut x : f32 = x.trim().parse().unwrap();
    let mut y : f32 = y.trim().parse().unwrap();
    let mut power : f32 = 0.0;
    let mut new = x; 
    match vec[1] {
        "+" => println!("{} + {} = {}",x, y, (x + y)),
        "-" => println!("{} - {} = {}",x, y, (x - y)),
        "/" => println!("{} / {} = {}",x, y, (x / y)),
        "*" => println!("{} * {} = {}",x, y, (x * y)),
        "^" =>  for inn in 1..y as i32 {
            power = x*new;
            x = power;
        },
        _ => (),
    };
    println!("{} ^ {} = {}",new, y, power)
  
}
