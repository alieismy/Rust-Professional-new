pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    if threshold < 1 {
        return 0;
    }
    
    let mut sum = 0;
    let mut prev = 0;
    let mut curr = 1;
    
    // 当前数小于阈值时继续计算
    while curr < threshold {
        // 如果当前数是奇数，加入总和
        if curr % 2 == 1 {
            sum += curr;
        }
        
        // 计算下一个斐波那契数
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    
    sum
}
