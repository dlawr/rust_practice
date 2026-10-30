fn main() {
    println!("{:?}", transform(&[1, 2, 3], |n| n * 2));
}

fn transform<F>(nums: &[i32], operation: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // nums.iter().copied().map(|num| operation(num)).collect()
    nums.iter().copied().map(operation).collect()
}
