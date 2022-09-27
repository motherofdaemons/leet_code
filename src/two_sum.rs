use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut idx_nums = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            let y = target - x;
            if idx_nums.contains_key(&y) {
                return [idx_nums[&y], i as i32].to_vec();
            }
            idx_nums.insert(*x, i as i32);
        }
        [-1, -1].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expect_output = vec![0, 1];
        let actual_output = Solution::two_sum(nums, target);
        assert_eq!(expect_output, actual_output);
    }
    #[test]
    fn example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expect_output = vec![1, 2];
        let actual_output = Solution::two_sum(nums, target);
        assert_eq!(expect_output, actual_output);
    }
    #[test]
    fn example_3() {
        let nums = vec![3, 3];
        let target = 6;
        let expect_output = vec![0, 1];
        let actual_output = Solution::two_sum(nums, target);
        assert_eq!(expect_output, actual_output);
    }
}