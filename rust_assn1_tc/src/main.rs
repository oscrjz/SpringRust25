const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT
}

fn main() {
    let mut temp = 32.0;
    let celsius = fahrenheit_to_celsius(temp);
    println!("{}°F is {:.2}°C", temp, celsius);

    for i in 1..=5 {
        let next_temp = temp + i as f64;
        let next_celsius = fahrenheit_to_celsius(next_temp);
        println!("{}°F is {:.2}°C", next_temp, next_celsius);
    }
}
