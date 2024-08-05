use std::io::{self, Write};

struct Fahrenheit {
    f: f64,
}

struct Celcius {
    c: f64,
}

struct Kelvin {
    k: f64,
}

impl Fahrenheit {
    fn to_celcius(&self) {
        let c: f64 = (self.f - 32.0) * (5.0 / 9.0);
        println!("{}F -> {}C", self.f, c);
    }

    fn to_kelvin(&self) {
        let k: f64 = (5.0 / 9.0) * (self.f + 459.67);
        println!("{}F -> {}K", self.f, k);
    }
}

impl Celcius {
    fn to_fahrenheit(&self) {
        let f = (self.c * 1.8) + 32.0;
        println!("{}C -> {}F", self.c, f);
    }

    fn to_kelvin(&self) {
        let k = self.c + 273.15;
        println!("{}C -> {}K", self.c, k);
    }
}

impl Kelvin {
    fn to_fahrenheit(&self) {
        let f = 1.8 * (self.k - 273.0) + 32.0;
        println!("{}K -> {}F", self.k, f);
    }

    fn to_celcius(&self) {
        let c = self.k - 273.15;
        println!("{}K -> {}C", self.k, c);
    }
}

fn main() {
    manage_user_choice();
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

pub fn option_display(){
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
}

pub fn manage_user_choice() {
    
    option_display();
    print!("Enter option (1/2/3/4/5/6): ");
    io::stdout().flush().unwrap();

    let choice = get_user_choice();

    match choice {
        1 => {
            let f = Fahrenheit {
                f: get_input("Fahrenheit"),
            };
            f.to_celcius();
        }
        2 => {
            let c = Celcius {
                c: get_input("Celcius"),
            };
            c.to_fahrenheit();
        }
        3 => {
            let f = Fahrenheit {
                f: get_input("Fahrenheit"),
            };
            f.to_kelvin();
        }
        4 => {
            let c = Celcius {
                c: get_input("Celcius"),
            };
            c.to_kelvin();
        }
        5 => {
            let k = Kelvin {
                k: get_input("Kelvin"),
            };
            k.to_fahrenheit();
        }
        6 => {
            let k = Kelvin {
                k: get_input("Kelvin"),
            };
            k.to_celcius();
        }
        _ => {}
    };
}

pub fn get_user_choice() -> u8 {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error");

    let choice = choice.trim().parse().unwrap();
    return choice;
}
