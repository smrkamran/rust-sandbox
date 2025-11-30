fn main() {
    let x: i32 = -42;
    let y: u64 = 42;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);


    let e: i32 = 2_147_483_647; // Maximum value for i32
    let f: u32 = 4_294_967_295; // Maximum value for u32
    println!("Max i32: {}", e);
    println!("Max u32: {}", f);

    let pi: f64 = 3.141592653589793;
    println!("Floating-point number: {}", pi);


    let is_active: bool = true;
    println!("Boolean value: {}", is_active);

    let letter: char = 'R';
    println!("Character value: {}", letter);
}
