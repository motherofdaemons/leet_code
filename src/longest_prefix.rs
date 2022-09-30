struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
            acc
               .chars()
               .zip(x.chars())
               .take_while(|(x,y)| x == y)
               .map(|(x, _)| x)
               .collect()
       })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
    }
}
