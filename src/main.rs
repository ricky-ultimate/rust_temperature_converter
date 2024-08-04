use std::io::{self, Write};

fn main() {
    display();
}

pub fn f_to_c_and_k(option: u8){
    match option{
        1 => {
            println!();
            let f = get_input("Fahrenheit");
            let c = (f - 32.0) * 1.8;
            println!("{}F -> {}C", f, c);
        },
        2 => {
            println!();
            let f = get_input("Fahrenheit");
            let k: f64 = (5.0/9.0) * (f + 459.67);
            println!("{}F -> {}K", f, k);
        },
        _ => {}
    }
}

pub fn c_to_f_and_k(option: u8){
    match option {
        1 => {
            let c = get_input("Celcius");
            let f = (c * 1.8) + 32.0;
            println!("{}C -> {}F", c, f);
        },
        2 => {
            println!();
            let c: f64 = get_input("Celcius");
            let k = c + 273.15;
            println!("{}C -> {}K", c, k);
        },
        _ => {}
    }
}

pub fn k_to_f_and_c(option: u8){
    match option {
        1 => {
            println!();
            let k = get_input("Kelvin");
            let f = 1.8 * (k - 273.0) + 32.0;
            println!("{}K -> {}F", k, f);
        },
        2 => {
            println!();
            let k = get_input("Kelvin");
            let c = k - 273.15;
            println!("{}K -> {}C", k, c);
        },
        _ => {}
    }
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
        1 => f_to_c_and_k(1),
        2 => c_to_f_and_k(1),
        3 => f_to_c_and_k(2),
        4 => c_to_f_and_k(2),
        5 => k_to_f_and_c(1),
        6 => k_to_f_and_c(2),
        _ => {}
    };
}

pub fn get_user_choice() -> u8 {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error");

    let choice = choice.trim().parse().unwrap();
    return choice;
}
