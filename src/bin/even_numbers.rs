fn print_evens(nums: &[i32]) {
    for num in nums {
        if num % 2 == 0 {
            println!("{num}");
        }
    }
}

fn count_evens(nums: &[i32]) -> usize {
    let mut count: usize = 0;

    for num in nums {
        if num % 2 == 0 {
            count += 1;
        }
    }

    count
}

fn sum_even(nums: &[i32]) -> i32 {
    let mut sum: i32 = 0;

    for num in nums {
        if num % 2 == 0 {
            sum += num;
        }
    }

    sum
}

fn main() {
    let nums = [1, 2, 3, 4, 5, 6];

    print_evens(&nums);
    println!("Count: {}", count_evens(&nums));
    println!("Sum: {}", sum_even(&nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_even_numbers() {
        assert_eq!(sum_even(&[1, 2, 3, 4, 5, 6]), 12);
    }

    #[test]
    fn handles_negative_numbers() {
        assert_eq!(sum_even(&[-4, -3, -2, 1]), -6);
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(sum_even(&[]), 0);
    }

    #[test]
    fn counts_even_numbers() {
        assert_eq!(count_evens(&[1, 2, 3, 4, 5, 6]), 3);
    }
}
