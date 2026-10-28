fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    rotate_right(&mut nums, 2);
    println!("{:?}", nums);
}

fn rotate_right(nums: &mut [i32], k: usize) {
    if nums.is_empty() {
        return;
    }

    let mut idx = 0;
    let mut current = nums[0];
    let mut swap;
    for _i in 0..nums.len() {
        idx = (idx + k) % nums.len();
        swap = nums[idx];
        nums[idx] = current;
        current = swap;
        println!("{:?} {} {:?}", nums, idx, swap);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let mut nums = vec![1, 2, 3, 4, 5];
        rotate_right(&mut nums, 2);
        assert_eq!(nums, vec![4, 5, 1, 2, 3]);
    }
    #[test]
    fn two() {
        let mut nums = vec![1, 2, 3];
        rotate_right(&mut nums, 1);
        assert_eq!(nums, vec![3, 1, 2]);
    }
    #[test]
    fn three() {
        let mut nums = vec![1, 2, 3];
        rotate_right(&mut nums, 3);
        assert_eq!(nums, vec![1, 2, 3]);
    }
    #[test]
    fn four() {
        let mut nums = vec![];
        rotate_right(&mut nums, 4);
        assert_eq!(nums, vec![]);
    }
    #[test]
    fn five() {
        let mut nums = vec![1, 2, 3];
        rotate_right(&mut nums, 7);
        assert_eq!(nums, vec![3, 1, 2]);
    }
}
