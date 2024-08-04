use std::io::{self, Write};

fn main() {
    let far = to_fahrenheit();
    print!("{}", far)
}


pub fn to_fahrenheit() -> f64{
    println!("Enter Temperature in Celcius: ");
    io::stdout().flush().unwrap();

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Error");

    let temp:f64 = temp.trim().parse().unwrap();

    let fahrenheit:f64 = (temp * 1.8) + 32.0;
    return fahrenheit
}

pub fn to_celcius(temp: f64) -> f64{
    let celcius = (temp - 32.0) * 1.8;
    return celcius;
}