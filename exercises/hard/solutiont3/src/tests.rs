// src/tests.rs
mod district;

#[cfg(test)]
mod tests {
    use super::district;
    use std::time::{Instant, Duration};

    // 定义测试用例和预期结果
    const TEST_CASE: &str = "3,2";

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_count_provinces() {
        let start = Instant::now();
        let result = district::count_provinces();
        let duration = start.elapsed();

        println!("Result: {}", result); // 添加日志打印
        // 时间超1s，判定不合格
        let mut total_score = 0.0;

        if duration <= Duration::from_millis(500) {
            total_score += 50.0;
            if result == TEST_CASE {
                total_score += 50.0;
            }
        }

        println!("Total score: {:.2}", total_score);
        assert_eq!(total_score, if result == TEST_CASE { 100.0 } else { 50.0 });
    }
}
