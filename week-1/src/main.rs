fn main() {
    println!("Hello, world!");
}


pub fn print_name() {
    println!("Jay Dee");
}

pub fn return_str() -> &'static str {
    let s: &str = "Random String";
    return s
}

pub fn return_num()  -> u64 {
    let n: u64 = 100;
    n
}

pub fn return_bool() -> bool {
    let b: bool = true;
    b
}
