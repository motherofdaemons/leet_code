struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        //no negative is a palindrome
        if x < 0 {
            return false;
        }
        //how many digits
        let digit_count = 1 + (x as f32).log10() as u32;
        //find half so we only check as much as needed
        let half = (digit_count / 2) as usize;
        //get an iterator of each digit
        let digits = (0..digit_count).map(|exp| x / 10_i32.pow(exp) % 10);
        //take front half of the digits and combine them with the back half and make sure the match each other
        digits
            .clone()
            .take(half)
            .zip(digits.rev().take(half))
            .all(|(lhs, rhs)| lhs == rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(Solution::is_palindrome(121));
    }
    #[test]
    fn example_2() {
        assert!(!Solution::is_palindrome(123));
    }
    #[test]
    fn example_3() {
        assert!(!Solution::is_palindrome(10));
    }
    #[test]
    fn example_4() {
        assert!(!Solution::is_palindrome(-121));
    }
}
