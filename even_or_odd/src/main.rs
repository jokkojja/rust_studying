use std::io;

fn is_even(number: u32) -> bool{
    if number % 2 == 0{
        return true
    }
    return false
}

fn main() {
    println!("Enter your number:");
    loop{
        let mut entered_number = String::new();
        io::stdin()
            .read_line(&mut entered_number)
            .expect("Failed to read line");
        let entered_number: u32 = match entered_number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        if is_even(entered_number){
            println!("Entered number is even")
        }
        else{
            println!("Entered number is odd")
        }
    }
}
