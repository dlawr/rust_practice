// fn print_message(message: &str) {
//     println!("{message}");
// }
//
// fn main() {
//     let message = String::from("Hello, Rust!");
//
//     print_message(&message);
//     println!("{message}");
// }

fn add_exclamation(message: &mut String) {
    message.push('!');
}

fn main() {
    let mut message = String::from("Hello");
    add_exclamation(&mut message);
    println!("{message}");
}
