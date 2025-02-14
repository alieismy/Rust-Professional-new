pub fn dp_rec_mc(amount: u32) -> u32 {
    if amount == 0 {
        return 0;
    }
    
    // 可用的纸币面额
    const CASHES: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];
    
    // 创建 dp 数组，dp[i] 表示金额 i 需要的最少纸币数
    let mut dp = vec![u32::MAX; (amount + 1) as usize];
    dp[0] = 0;
    
    // 对每个金额进行计算
    for i in 1..=amount {
        // 尝试每种面额的纸币
        for &cash in CASHES.iter() {
            if cash <= i {
                // 如果当前金额减去这个面额后的结果有解
                if dp[(i - cash) as usize] != u32::MAX {
                    // 更新最小纸币数
                    dp[i as usize] = dp[i as usize].min(dp[(i - cash) as usize] + 1);
                }
            }
        }
    }
    
    // 如果无法找零，返回0（根据测试用例的要求）
    if dp[amount as usize] == u32::MAX {
        0
    } else {
        dp[amount as usize]
    }
}
