/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut table = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = table.get(&(target - num)) {
                return vec![i as i32, j as i32];
            } else {
                table.insert(num, i);
            }
        }

        vec![-1, -1]
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let mut left = Solution::two_sum([3, 2, 4].into(), 6);
        left.sort_unstable();
        let right = vec![1, 2];
        assert_eq!(left, right);
    }
}
