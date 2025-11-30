fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);

    // let mix = [1, 2.5, "three", true];
    // println!("Mixed Array: {:?}", mix);

    let fruits: [&str; 3] = ["Apple", "Banana", "Cherry"];
    println!("Fruits: {:?}", fruits);
    println!("Fruits: 1 {}", fruits[0]);
    println!("Fruits: 2 {}", fruits[1]);

    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1,2,4,5]);
    println!("My Mixed Tuple: {:?}", my_mix_tuple);

    let number_slices: &[i32] = &[1,2,3,4,5,6];
    println!("Number Slices: {:?}", number_slices);

    let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"Lord of the Rings".to_string()];
    println!("Book Slices: {:?}", book_slices);

    let book_slices_2: &[&str] = &["IT", "Harry Potter", "Lord of the Rings"];
    println!("Book Slices: {:?}", book_slices_2);


    let mut stone_cold : String = String::from("Hell, ");
    println!("Stone Cold says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold says: {}", stone_cold);

    let string: String = String::from("Hello, World!");
    let slice:&str = &string[0..5];
    println!("String Slice: {}", slice);
}