fn main() {
    println!("Hello, world!");
}


fn to_fahrenheit(temp: f64) -> f64{
    let fahrenheit = (temp * 1.8) + 32.0;
    return fahrenheit
}

fn to_celcius(temp: f64) -> f64{
    let celcius = (temp - 32.0) * 1.8;
    return celcius;
}