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
    let mut nums = nums; // 获取数组的可变引用
    let mut result = Vec::new();
    
    // 使用数组元素的正负号来标记是否出现过
    for i in 0..nums.len() {
        // 获取当前数字的绝对值作为索引（减1是因为数组索引从0开始）
        let index = nums[i].abs() as usize - 1;
        
        // 如果对应位置的数字已经是负数，说明这个数字之前出现过
        if nums[index] < 0 {
            // 避免重复添加相同的数字
            if !result.contains(&(index as i32 + 1)) {
                result.push(index as i32 + 1);
            }
        } else {
            // 将对应位置的数字标记为负数，表示这个数字已经出现过
            nums[index] = -nums[index];
        }
    }
    
    // 对结果进行排序，保证输出顺序一致
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
