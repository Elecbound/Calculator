mod calculator;

use calculator::{Calculator, Error};

fn main() -> Result<(), Error>{
    loop {
        println!("Give me numbers and operators! */+- (number)");
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
        let tokens = Calculator::parse(input);
        if tokens.is_err() {
            println!("{:?}", tokens.err().unwrap());
            continue;
        }
        let expr = Calculator::expression(tokens?);
        if let Some(v) = Calculator::evaluate(expr) {
            println!("{}", v);
        }
            },

            Err(error) => println!("error: {}", error)
        }
    }
}
fn example() {
    let tokens = calculator::Calculator::parse("2 * 2 + 48 / 4");
    println!("{:?}", tokens);
    let expr = calculator::Calculator::expression(tokens.unwrap());
    println!("{:?}", expr);
    let value = calculator::Calculator::evaluate(expr);
    println!("{:?}", value.unwrap());
}
