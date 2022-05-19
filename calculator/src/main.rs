use std::env::{args, Args};
fn main() {
    let mut args: Args = args();

    let first_number: f32 = args.nth(1).unwrap().parse::<f32>().unwrap();
    let operator: String = args.nth(0).unwrap();
    let second_number: f32 = args.nth(0).unwrap().parse::<f32>().unwrap();

    let result: f32 = operate(first_number, second_number, operator.chars().next().unwrap());

    println!("{:?}", output(first_number, second_number, operator.chars().next().unwrap(), result));
}

fn operate(first_number: f32, second_number: f32, operator: char) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator passed in!"),
    }
}

fn output(first_number: f32, second_number: f32, operator: char, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}