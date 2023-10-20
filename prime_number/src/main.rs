use std::io;

fn is_prime(number: u32) -> bool{
    if number < 1{
        return false;
    }
    let count_of_numbers: u32 = ((number as f32 / 2.0).ceil() + 1.0) as u32; //looks like bullshit
    for i in 2..count_of_numbers{
        if number % i == 0{
            return false;
        }
    }
    return true
}

fn main() {
    loop{
        println!("Enter your number:");
        let mut number = String::new();

        io::stdin().read_line(&mut number).expect("Failed to read line");

        let number: u32 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        if is_prime(number){
            println!("{number} is prime\n");
        }
        else{
            println!("{number} is not prime\n");
        }
    }
}
