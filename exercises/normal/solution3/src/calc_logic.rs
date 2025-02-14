pub fn new_birthday_probability(n: u32) -> f64 {
    if n < 2 {
        return 0.0;
    }
    
    // 计算至少两个人生日相同的概率
    // = 1 - 所有人生日都不相同的概率
    
    // 使用对数计算避免大数相乘导致的精度问题
    let mut log_probability = 0.0;
    
    // 计算所有人生日都不相同的概率的对数
    for i in 0..n {
        log_probability += ((365 - i) as f64 / 365.0).ln();
    }
    
    // 转换回概率并返回互补概率
    let probability = 1.0 - log_probability.exp();
    
    // 处理精度问题，确保结果在 [0,1] 范围内
    probability.max(0.0).min(1.0)
}
