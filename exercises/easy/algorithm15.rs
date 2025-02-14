/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters. 
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.
    
    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/

use std::fmt::{self, Display, Formatter};
use std::collections::HashMap;

pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut char_index = HashMap::new(); // 记录字符最后出现的位置
    let mut max_length = 0;
    let mut start = 0; // 滑动窗口的起始位置
    
    for (end, &ch) in chars.iter().enumerate() {
        // 如果字符已经在当前窗口中出现过，更新窗口起始位置
        if let Some(&prev_index) = char_index.get(&ch) {
            // 只有当前窗口包含重复字符时才更新起始位置
            if prev_index >= start {
                start = prev_index + 1;
            }
        }
        
        // 更新字符最后出现的位置
        char_index.insert(ch, end);
        
        // 更新最大长度
        max_length = max_length.max(end - start + 1);
    }
    
    max_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1);  // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0);  // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5);  // "abcde"
    }
}
