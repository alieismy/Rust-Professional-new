// src/tests.rs
mod district;

#[cfg(test)]
mod tests {
    use super::district::count_provinces;
    use serde_json::{Map, Value};
    use std::time::{Instant, Duration};
    use std::fs::File;
    use std::io::Read;
    
    // 定义测试用例和预期结果
    const TEST_CASE: &str = "3,2,2,2,1";
    
    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_count_provinces() {
        // 准备JSON数据
        let mut file = File::open("district.json").expect("无法打开测试数据文件district.json");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("无法读取测试数据文件district.json");
        let data: Value = serde_json::from_str(&contents).expect("解析JSON过程失败");
    
        // 计时与处理每个批次
        let start = Instant::now();
        let mut results = Vec::new();
        let batches = data.as_object().expect("测试数据格式非法");
    
        for batch in batches.values() {
            if let Some(cities) = batch.as_object() {
                results.push(count_provinces(cities).to_string());
            }
        }
    
        // 验证处理时间
        let duration = start.elapsed();
        if duration > Duration::from_millis(500) {
            println!("性能测试失败！时间超出限制： {:.2?}", duration);
        } else {
            println!("性能测试通过！处理时间： {:.2?}", duration);
        }
    
        // 打印实际与预期结果
        let result_str = results.join(",");
        println!("期待的结果: {}\n计算的结果: {}", TEST_CASE, result_str);
    
        // 总评分逻辑
        let mut total_score = 0.0;
        if duration <= Duration::from_millis(500) && result_str == TEST_CASE {
            total_score += 100.0;
        }
    
        println!("总得分: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
