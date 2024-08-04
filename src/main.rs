use std::io::{self, Write};

fn main() {
    display();
}

pub fn c_to_f() {
    let temp = get_input("Celcius");
    let fahrenheit = (temp * 1.8) + 32.0;
    println!("{}C -> {}F", temp, fahrenheit);
}

pub fn f_to_c() {
    println!();
    let temp = get_input("Fahrenheit");
    let celcius = (temp - 32.0) * 1.8;
    println!("{}F -> {}C", temp, celcius);
}

pub fn c_to_k() {
    println!();
    let temp = get_input("Celcius");
    let kelvin = temp + 273.15;
    println!("{}C -> {}K", temp, kelvin);
}

pub fn f_to_k() {
    println!();
    let temp = get_input("Fahrenheit");
    let kelvin: f64 = (5.0/9.0) * (temp + 459.67);
    println!("{}F -> {}K", temp, kelvin);
}

pub fn k_to_f(){
    println!();
    let temp = get_input("Kelvin");
    let fahrenheit = 1.8 * (temp - 273.0) + 32.0;
    println!("{}K -> {}F", temp, fahrenheit);
}

pub fn k_to_c(){
    println!();
    let temp = get_input("Kelvin");
    let celcius = temp - 273.15;
    println!("{}K -> {}C", temp, celcius);
}

pub fn get_input(name: &str) -> f64 {
    println!();
    print!("Enter Temperature in {} : ", name);
    io::stdout().flush().unwrap();

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Invalid Message!");

    let temp: f64 = temp.trim().parse().unwrap();

    return temp;
}

pub fn display() {
    println!("Temperture Converter");
    println!("--------------------");
    println!();
    println!("1. F -> C");
    println!("2. C -> F");
    println!("3. F -> K");
    println!("4. C -> K");
    println!("5. k -> F");
    println!("6. K -> C");
    println!();
    print!("Enter option (1/2): ");
    io::stdout().flush().unwrap();

    let choice = get_user_choice();

    match choice {
        1 => f_to_c(),
        2 => c_to_f(),
        3 => f_to_k(),
        4 => c_to_k(),
        5 => k_to_f(),
        6 => k_to_c(),
        _ => {}
    };
}

pub fn get_user_choice() -> u8 {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error");

    let choice = choice.trim().parse().unwrap();
    return choice;
}
