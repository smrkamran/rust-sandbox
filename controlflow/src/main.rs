fn main() {
    let age = 18;
    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are a minor.");
    }

    let condition = false;
    let number = if condition { 5 } else { 10 };
    println!("The number is: {number}");
}
