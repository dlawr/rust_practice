fn print_message(message: &str) {
    println!("{message}");
}

fn main() {
    let message = String::from("Hello, Rust!");

    print_message(&message);
    println!("{message}");
}
