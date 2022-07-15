use std::io;

fn convert_f_to_c(degree: f32) -> f32 {
    (degree - 32.0) * 5.0 / 9.0
}

fn convert_c_to_f(degree: f32) -> f32 {
    (degree * 9.0 / 5.0) + 32.0
}

fn welcome_message() {
    println!("Welcome to the Fahrenheit/Celsius converter");
}

fn display_direction_conversion_message() {
    println!("Please select the direction of conversion:");
    println!("1) F -> C");
    println!("2) C -> F");
}

fn get_direction_conversion() -> u32 {
    display_direction_conversion_message();
    let mut direction_str = String::new();
    io::stdin()
        .read_line(&mut direction_str)
        .expect("Failed to read line");

    direction_str
        .trim()
        .parse()
        .expect("Invalid direction entered! Must 1 or 2")
}

fn main() {
    welcome_message();

    let direction = get_direction_conversion();
    let input_unity = if direction == 1 { "F" } else { "C" };
    let output_unity = if direction == 1 { "C" } else { "F" };
    println!("Enter value to convert (in {}):", input_unity);

    let mut input_degree_str = String::new();
    io::stdin()
        .read_line(&mut input_degree_str)
        .expect("Failed to read line");

    let input_degree: f32 = input_degree_str
        .trim()
        .parse()
        .expect("Degree entered was not a number");

    let output_degree = if direction == 1 {
        convert_f_to_c(input_degree)
    } else {
        convert_c_to_f(input_degree)
    };

    println!(
        "{:.2}{} = {:.2}{}",
        input_degree, input_unity, output_degree, output_unity
    );
}
