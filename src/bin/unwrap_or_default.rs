fn main() {
    println!(
        "{:?} {:?}",
        unwrap_or_default(Some(42)),
        unwrap_or_default(None)
    )
}

fn unwrap_or_default(value: Option<i32>) -> i32 {
    match value {
        Some(val) => val,
        None => 0,
    }
    // value.unwrap_or(0)
}
