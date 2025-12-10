fn main() {
    hello_world();
    tell_height(180);
    human_id("Alice", 30, 165.0);
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
