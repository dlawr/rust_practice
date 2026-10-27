fn main() {
    let mut nums = vec![1, 1, 2, 2, 3];
    dedup_adjacent(&mut nums);
}

fn dedup_adjacent(nums: &mut Vec<i32>) {
    // for (i, num) in nums.iter_mut().enumerate() {
    let mut i = 0;
    while i < nums.len() {
        let mut last_num = false;
        while !last_num {
            if i + 1 < nums.len() {
                println!("{} {:?} {}", nums[i], nums, nums[i + 1]);
                if nums[i] == nums[i + 1] {
                    nums.remove(i + 1);
                } else {
                    last_num = true;
                }
            } else {
                last_num = true;
            }
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_adjacent_duplicates() {
        let mut nums = vec![1, 1, 2, 2, 3];
        dedup_adjacent(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn preserves_non_adjacent_duplicates() {
        let mut nums = vec![1, 2, 1];
        dedup_adjacent(&mut nums);
        assert_eq!(nums, vec![1, 2, 1]);
    }

    #[test]
    fn handles_all_equal_values() {
        let mut nums = vec![5, 5, 5, 5];
        dedup_adjacent(&mut nums);
        assert_eq!(nums, vec![5]);
    }

    #[test]
    fn handles_empty_vector() {
        let mut nums: Vec<i32> = vec![];
        dedup_adjacent(&mut nums);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn handles_duplicates_in_multiple_runs() {
        let mut nums = vec![1, 2, 2, 3, 3, 1];
        dedup_adjacent(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 1]);
    }
}
