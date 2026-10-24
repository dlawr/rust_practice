fn print_message(message: &String) {
    println!("{}", message);
}

fn main() {
    let message = String::from("Hello, Rust!");

    print_message(&message);

    println!("{}", message);
}
