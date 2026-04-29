// const variable defined globaly with a const time expression
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 + 3;

fn main() {
    // mut is necessary to have the variable mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 8;
    println!("The value of x is: {x}");

    // testing constants
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    {
        let x = x * 2;
        println!("The value of x after scoped shadow is: {x}");
        // using format! to have a formatted string
        let x = format!("x string changed from {x}");
        println!("The value of x is now the string:\"{x}\"");
    }

    println!("The value of x outside of the inner scope is still: {x}");
}
