fn main() {
    println!("Hello, world!");
}


fn to_fahrenheit(temp: f64) -> f64{
    let fahrenheit = (temp * 1.8) + 32.0;
    return fahrenheit
}