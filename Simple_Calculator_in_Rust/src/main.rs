use std::io;
#[derive(Debug)]

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

    let x = 1.0;
    let y = 2.0;
    let user_operation = Operation::Divide {x, y};
    println!("{:?}", user_operation);
    let result = calculate(user_operation);
    println!("{:?}", result);
}