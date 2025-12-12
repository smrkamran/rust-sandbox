// fn main() {
//     let age = 18;
//     if age >= 18 {
//         println!("You are an adult.");
//     } else {
//         println!("You are a minor.");
//     }

//     let condition = false;
//     let number = if condition { 5 } else { 10 };
//     println!("The number is: {number}");
// }

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("Hello World! The result is: {result}");
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("coount = {count}");
//         let mut remaining = 10;
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
// }

fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [1, 2, 3, 4, 5, 6];
    for element in a {
        println!("the value is: {element}");
    }
}
