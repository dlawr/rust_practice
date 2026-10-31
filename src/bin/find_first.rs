fn main() {
    println!("{:?}", find_first(&[1, 3, 4, 6], |n| n % 2 == 0));
}

fn find_first<F>(nums: &[i32], predicate: F) -> Option<i32>
where
    F: Fn(i32) -> bool,
{
    for num in nums {
        if predicate(*num) {
            return Some(*num);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_even() {
        assert_eq!(find_first(&[1, 3, 4, 6], |n| n % 2 == 0), Some(4));
    }

    #[test]
    fn begining() {
        assert_eq!(find_first(&[1, 3, 4, 6], |n| n % 2 == 1), Some(1));
    }

    #[test]
    fn end() {
        assert_eq!(find_first(&[1, 3, 4, 6], |n| n > 4), Some(6));
    }

    #[test]
    fn no_match() {
        assert_eq!(find_first(&[1, 3, 4, 6], |n| n % 7 == 0), None);
    }

    #[test]
    fn empty() {
        assert_eq!(find_first(&[], |n| n % 2 == 0), None);
    }
}
