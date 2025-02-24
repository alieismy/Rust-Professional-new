/*
    Find Duplicates in Array
    Given an array, find all the duplicate elements and return them. 
    You need to solve the problem with O(1) space complexity (i.e., without using extra arrays or hash tables).

    Implement the function `find_duplicates(nums: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the duplicate elements in the array.
    
    Hint: You can modify the input array in place to track duplicates.
*/

use std::fmt::{self, Display, Formatter};

pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut result = Vec::new();
    
    // 遍历数组中的每个数字
    for i in 0..nums.len() {
        let current = nums[i].abs();  // 获取当前数字的绝对值
        
        // 如果当前数字在数组范围内
        if current as usize <= nums.len() {
            let index = (current - 1) as usize;  // 转换为0-based索引
            
            // 如果该位置的数字大于0，将其变为负数
            if nums[index] > 0 {
                nums[index] = -nums[index];
            } else {
                // 如果已经是负数，说明这个数字是重复的
                if !result.contains(&current) {
                    result.push(current);
                }
            }
        } else {
            // 对于超出范围的数字，检查是否已经在结果中
            if nums.iter().filter(|&&x| x.abs() == current).count() > 1 
                && !result.contains(&current) {
                result.push(current);
            }
        }
    }
    
    // 排序结果以保证测试通过
    result.sort_unstable();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates_1() {
        let nums = vec![1, 2, 3, 4, 5, 6, 2, 3];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_find_duplicates_2() {
        let nums = vec![4, 5, 6, 7, 5, 4];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![4, 5]);
    }

    #[test]
    fn test_find_duplicates_3() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_find_duplicates_4() {
        let nums = vec![1, 1, 1, 1, 1];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_find_duplicates_5() {
        let nums = vec![10, 9, 8, 7, 6, 7, 8];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![7, 8]);
    }
}
