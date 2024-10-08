fn main() {
    println!("Hello, world!");

    test();

    println!("{}",hello("Viggo"));
}

fn test() {
    println!("test");
}

// can you make a function that takes a string and returns a string?
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}
