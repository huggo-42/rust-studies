use std::io;

fn to_fahrenheit(temperature: f64) -> f64 {
    (temperature * 1.8) + 32.0
}

fn to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) / 1.8
}

fn main() {
    let mut temperature = String::new();
    let mut convertion_type = String::new();

    println!("Select how you wish to convert from:\n[0] °C to °F\n[1] °F to °C");

    io::stdin()
        .read_line(&mut convertion_type)
        .expect("Failed to read line");

    println!("Ok. Now enter the temperature:");
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f64 = temperature.trim().parse().expect("Please type a number!");

    match convertion_type.trim() {
        "0" => {
            let fahrenheit = to_fahrenheit(temperature.clone());
            println!("{}°C = {}°F", temperature, fahrenheit);
        }
        "1" => {
            let celsius = to_celsius(temperature.clone());
            println!("{}°F = {}°C", temperature, celsius);
        }
        _ => panic!("Invalid selection"),
    }
}
