fn main() {
    hello_world();
    tell_height(180);
    human_id("Alice", 30, 165.0);

    let x = {
        let price = 5;
        let quantity = 10;
        price * quantity
    };

    println!("Total cost is: {}", x);
    println!("Sum is: {}", add(10, 20));

    let weight = 70.0;
    let height = 1.75;

    let result = calculate_bmi(weight, height);
    println!("BMI is: {:.2}", result);
}

fn hello_world() {
    println!("Hello Rust");
}

fn tell_height(height: u32) {
    println!("Height is: {} cm", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My Name is: {}, and Age is: {}, and Height is: {} cm",
        name, age, height
    );
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// BMI = weight(kg) / height(m) ^2

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
