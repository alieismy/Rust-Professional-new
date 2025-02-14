pub fn goldbach_conjecture() -> String {
    let mut counter = 0;
    let mut result = Vec::with_capacity(2);
    let mut num = 3;
    
    // 判断一个数是否为素数
    fn is_prime(n: u32) -> bool {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }
        
        let sqrt = (n as f64).sqrt() as u32;
        for i in (3..=sqrt).step_by(2) {
            if n % i == 0 { return false; }
        }
        true
    }
    
    // 检查一个数是否满足猜想
    fn check_conjecture(n: u32) -> bool {
        let sqrt_limit = ((n as f64) / 2.0).sqrt() as u32;
        
        for i in 1..=sqrt_limit {
            let square_double = 2 * i * i;
            if square_double >= n { break; }
            
            let prime_needed = n - square_double;
            if is_prime(prime_needed) {
                return true;
            }
        }
        false
    }
    
    // 主循环：寻找不满足猜想的奇合数
    while counter < 2 {
        if num % 2 == 1 { // 奇数
            if !check_conjecture(num) {
                result.push(num);
                counter += 1;
            }
        }
        num += 2;
    }
    
    // 返回结果字符串
    format!("{},{}", result[0], result[1])
}
