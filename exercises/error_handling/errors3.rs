// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let number = match pretend_user_input.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };

    let cost = total_cost(number)?;

    
    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    Ok(())
}

pub fn total_cost(qty: i32) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    Ok(qty * cost_per_item + processing_fee)
}

/* 
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
        Ok(())
}
*/
