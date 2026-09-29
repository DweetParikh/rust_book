// fn is used for defining a function

// Data Types 
// Numbers u32, u64, i32, i64
fn sum(a: u32, b: u32) -> u32 {     
    return a + b;
}

// Boolean -> bool = true or false
fn is_even(a: u32) -> bool {
    return a % 2 == 0;
}

fn main() {
    let ans = sum(1, 2);
    println!("{}", ans);
    println!("{}", is_even(9));
}
