/*
 * @lc app=leetcode.cn id=412 lang=rust
 *
 * [412] Fizz Buzz
 */

// @lc code=start
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|i| match (i % 3 == 0, i % 5 == 0) {
                (true, true) => "FizzBuzz".to_string(),
                (true, false) => "Fizz".to_string(),
                (false, true) => "Buzz".to_string(),
                _ => i.to_string(),
            })
            .collect()
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_3() {
        assert!(Solution::fizz_buzz(3) == vec!["1", "2", "Fizz"]);
    }

    #[test]
    fn test_5() {
        assert!(Solution::fizz_buzz(5) == vec!["1", "2", "Fizz", "4", "Buzz"])
    }

    #[test]
    fn test_15() {
        assert!(
            Solution::fizz_buzz(15)
                == vec![
                    "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                    "13", "14", "FizzBuzz"
                ]
        )
    }
}
