use std::io;
fn convert(x: i32) -> i32{
    ((x - 32) * 5)/9
}

fn main() {
    let fahrenheit = loop{
        println!("Please input amount of Fahrenheit that u want to convert into Celsius");
        let mut fahnr = String::new();

        io::stdin()
        .read_line(&mut fahnr)
        .expect("Failed to read line");

        let fahnr: i32 = match fahnr.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Enter a integer number please");
                continue
            }
        };
        break fahnr;
    };
    
    let celsius = convert(fahrenheit);
    println!("{} F is {} Celsius ",fahrenheit,celsius);
}
