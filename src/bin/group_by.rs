fn main() {
    let result = group_by(&[1, 2, 3, 4, 5, 6], |n| n % 2 == 0);
    println!("{:?}", result);
}

fn group_by<F>(nums: &[i32], predicate: F) -> (Vec<i32>, Vec<i32>)
where
    F: Fn(i32) -> bool,
{
    // let mut v1: Vec<i32> = Vec::new();
    // let mut v2: Vec<i32> = Vec::new();
    // for i in 0..nums.len() {
    //     if predicate(nums[i]) {
    //         v1.push(nums[i]);
    //     } else {
    //         v2.push(nums[i]);
    //     }
    // }
    let v1: Vec<i32> = nums.iter().copied().filter(|n| predicate(*n)).collect();
    let v2: Vec<i32> = nums.iter().copied().filter(|n| !predicate(*n)).collect();
    return (v1, v2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separates_even_and_odd() {
        let result = group_by(&[1, 2, 3, 4, 5, 6], |n| n % 2 == 0);
        assert_eq!(result, (vec![2, 4, 6], vec![1, 3, 5]));
    }

    #[test]
    fn all_match() {
        let result = group_by(&[2, 4, 6], |n| n % 2 == 0);
        assert_eq!(result, (vec![2, 4, 6], vec![]));
    }

    #[test]
    fn none_match() {
        let result = group_by(&[1, 3, 5], |n| n % 2 == 0);
        assert_eq!(result, (vec![], vec![1, 3, 5]));
    }

    #[test]
    fn handles_empty_input() {
        let result = group_by(&[], |n| n > 0);
        assert_eq!(result, (vec![], vec![]));
    }

    #[test]
    fn preserves_order() {
        let result = group_by(&[5, 2, 7, 4, 1, 6], |n| n > 3);
        assert_eq!(result, (vec![5, 7, 4, 6], vec![2, 1]));
    }
}
