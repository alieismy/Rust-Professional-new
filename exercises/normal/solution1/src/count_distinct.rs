use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    // 将输入字符串按逗号分割，并收集到 HashSet 中以去重
    let unique_elements: HashSet<&str> = input_str.split(',').collect();
    
    // 返回 HashSet 的大小，即不重复元素的个数
    unique_elements.len()
}
