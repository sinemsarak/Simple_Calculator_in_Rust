use std::io;

enum Operation {
    Add {x:f64, y:f64,},
    Subtract {x:f64, y:f64},
    Divide {x:f64, y:f64},
    Multiply {x:f64, y:f64},
}

fn calculate (calculation: Operation) ->  f64 {
    match calculation {

        Operation::Add {x, y} => {
        return x+y;
        }
        Operation::Divide {x, y} => { 
            if y != 0.0{
                return x/y;
            } else {
                return f64::NAN;
            }
        }
        Operation::Multiply {x, y} => {
            return x*y;
        }
        Operation::Subtract { x, y } => {
            return x-y;
        }       

    }
}

fn main() {
    println!("Enter an operation to calculate (e.g: 3 + 5): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");

    
    let mut index = 0;

    while &input[index..index+1] != " " {
        index=index+1;
    }
    let x_str = &input[0..index];
    let user_operation = &input[index+1..index+2];
    let y_str = &input[index+3..input.len()];

    let x:f64 = x_str.trim().parse().expect("Invalid input. Please check the example and try again.");
    let y:f64 = y_str.trim().parse().expect("Invalid input. Please check the example and try again.");
    let answer:f64;
    match user_operation {
        "+" => {
            answer = calculate(Operation::Add { x, y });
        }
        "-" => {
            answer = calculate(Operation::Subtract { x, y });
        }
        "*" => {
            answer = calculate(Operation::Multiply { x, y });
        }
        "/" => {
            
            if y == 0.0 {
                println!("You can not divide a number by zero");
                return;
            
            } else {
                answer = calculate(Operation::Divide { x, y });
            }    
        }
        _ => {
            println!("Invalid input. Please check the example and try again.");
            return;
        }
    }
    println!("Here is the result: {}", answer);
}    