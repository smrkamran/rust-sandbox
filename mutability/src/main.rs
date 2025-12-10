// fn main() {
//     println!("Hello, world!");

//     let mut _a = 5;
//     println!("The value of a is: {}", _a);
//     _a = 10;
//     println!("The value of a is: {}", _a);
// }
// //

fn main() {
    const Y: i32 = 5;
    println!("The value of y is: {}", Y);

    println!("The value of PI is: {}", PI);

    println!(
        "The value of three_hours_in_seconds is: {}",
        THREE_HOURS_IN_SECONDS
    );
}

const PI: f64 = 3.14159;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
