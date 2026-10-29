fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6];
    rotate_right(&mut nums, 2);
    println!("{:?}", nums);
    // println!("{}", euclid(10, 225));
}

fn rotate_right(nums: &mut [i32], k: usize) {
    if nums.is_empty() {
        return;
    }

    let lcd = euclid(nums.len(), k);
    println!("{}", lcd);
    let mut idx = 0;
    let mut current = nums[0];
    let mut swap;

    if lcd == 1 {
        for _i in 0..nums.len() {
            idx = (idx + k) % nums.len();
            swap = nums[idx];
            nums[idx] = current;
            current = swap;
            println!("{:?} {} {:?}", nums, idx, swap);
        }
    } else {
        let cycle = k % nums.len();
        for i in 0..k {
            println!("{}", i);
            let mut idx = i;
            let mut current = nums[i];
            let mut swap;
            for _j in 0..cycle + 1 {
                idx = (idx + k) % nums.len();
                swap = nums[idx];
                nums[idx] = current;
                current = swap;
                println!("{:?} {} {:?}", nums, idx, swap);
            }
        }
    }
}

fn euclid(a: usize, b: usize) -> usize {
    let mut top;
    let mut bottom;

    if a > b {
        top = a;
        bottom = b;
    } else {
        top = b;
        bottom = a;
    };

    let mut remainder = top % bottom;
    while remainder > 0 {
        top = bottom;
        bottom = remainder;
        remainder = top % bottom;
    }
    bottom
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
    #[test]
    fn six_elements_rotated_by_two() {
        let mut nums = vec![1, 2, 3, 4, 5, 6];
        rotate_right(&mut nums, 2);
        assert_eq!(nums, vec![5, 6, 1, 2, 3, 4]);
    }
}
