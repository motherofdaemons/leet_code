struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans = 0;
        let mut prev = 0;
        for x in s.chars().rev() {
            let x = match x {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!(),
            };
            if x < prev {
                ans -= x;
            } else {
                ans += x;
            }
            prev = x;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }
    #[test]
    fn example_2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }
    #[test]
    fn example_3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
    #[test]
    fn example_4() {
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    }
}
