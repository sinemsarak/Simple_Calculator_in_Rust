use std::io;
enum Operation {
    Add {x:f64, y:f64,},
    Subtract {x:f64, y:f64},
    Divide {x:f64, y:f64},
    Multiply {x:f64, y:f64},
}

fn calculate (calculation: Operation, x: f64, y:f64) ->  f64 {
    match calculation {

        Operation::Add {x, y} => x+y,
 
        Operation::Divide {x, y} => { 
            if y != 0.0{
                x/y
            } else {
                f64::NAN
            }
        }
        Operation::Multiply {x, y} => x*y,
        Operation::Subtract { x, y } => x-y,
    }       

}

fn main() {

    println!("Enter the first number");


    println!("Enter the operation");
    
    println!("Enter the second number");

}