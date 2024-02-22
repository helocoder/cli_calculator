use std::env::{args, Args};
fn main() {


    /*      For refernce how iterators work in args case

    let mut args:Args = args(); // ["path", "1", "+", "2"]
    let first_operand = args.nth(0).unwrap(); // [ (path), "1", "+", "2"] 
    let operator = args.nth(1).unwrap();// [ "1", (+), "2"]
    let second_operand = args.nth(2).unwrap();
    println!("{:?} {:?} {:?}",first_operand,operator,second_operand);
    // println!("{:?} {:?}",first_operand,operator);

     */
    let mut args:Args = args();
    let first_operand = args.nth(1).unwrap(); 
    let operator = args.nth(0).unwrap();
    let second_operand = args.nth(0).unwrap();
    
    //Typecasting
     let opera =  operator.parse::<char>().unwrap();
     let first_number :f32= first_operand.parse::<f32>().unwrap(); 
     let second_number:f32 = second_operand.parse::<f32>().unwrap();


     match operate(opera,first_number,second_number)
     {
         Some(result) => println!("Ans is: {}",result),
         None => println!("Some error occur")
     }




}

//using match to match with i/p operator and do the required operation accordingly
fn operate(operator:char, first_number:f32,second_number:f32) -> Option<f32>
{
    match operator{

        '+' => Some(first_number+second_number),
        '-' => Some(first_number-second_number),
        '*' => Some(first_number*second_number),
        '/' => { 
            if second_number != 0.0
            {
             return Some(first_number/second_number);
            }
            else {
            return None;
            }

        },
        _ => None //default case
    }
}

