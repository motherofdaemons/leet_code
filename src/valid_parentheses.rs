struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' if Some(c) != stack.pop() => return false,
                _ => ()
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod  tests {
    use super::*;
    #[test]
    fn example_1() {
        assert!(Solution::is_valid("()".to_string()))
    }

    #[test]
    fn example_2() {
        assert!(Solution::is_valid("()[]{}".to_string()))
    }

    #[test]
    fn example_3() {
        assert!(!Solution::is_valid("(]".to_string()))
    }

    #[test]
    fn example_4() {
        assert!(!Solution::is_valid("([)]".to_string()))
    }
}