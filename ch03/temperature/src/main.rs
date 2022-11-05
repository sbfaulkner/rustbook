use std::env;

const INPUT_REGEX: &str = r"\A\s*(?P<temperature>-?\d+)\s*(?P<scale>[CF])\s*\z";

fn main() {
    let input_regex = regex::Regex::new(INPUT_REGEX).unwrap();

    let input = env::args().nth(1).expect("No argument provided");

    match input_regex.captures(input.trim()) {
        Some(m) => {
            let temperature = m.name("temperature").unwrap().as_str().parse::<i32>().unwrap();
            let scale = m.name("scale").unwrap().as_str();

            print!("{}{} is ", temperature, scale);

            match scale {
                "C" => println!("{}F", celsius_to_fahrenheit(temperature)),
                "F" => println!("{}C", fahrenheit_to_celsius(temperature)),
                _ => unreachable!(),
            }
        },
        None => println!("Invalid input"),
    }
}

fn celsius_to_fahrenheit(temperature: i32) -> i32 {
    temperature * 9 / 5 + 32
}

fn fahrenheit_to_celsius(temperature: i32) -> i32 {
    (temperature - 32) * 5 / 9
}
