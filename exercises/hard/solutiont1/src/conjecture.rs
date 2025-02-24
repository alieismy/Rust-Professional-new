pub fn goldbach_conjecture() -> String {
    // 存储找到的反例
    let mut counter = 0;
    let mut result = Vec::new();
    
    // 从3开始检查奇数
    let mut n = 3;
    
    while counter < 2 {
        // 只检查奇合数
        if n % 2 == 1 {
            if !can_be_goldbach(n) {
                result.push(n);
                counter += 1;
            }
        }
        n += 2;  // 只检查奇数
    }
    
    // 返回结果字符串
    format!("{},{}", result[0], result[1])
}

// 检查一个数是否可以写成素数+2×平方数的形式
fn can_be_goldbach(n: i32) -> bool {
    // 对于每个可能的平方数
    for i in 0..((n as f64).sqrt() as i32 + 1) {
        let square_part = 2 * i * i;
        if square_part >= n {
            break;
        }
        
        // 检查剩余部分是否为素数
        let remaining = n - square_part;
        if is_prime(remaining) {
            return true;
        }
    }
    false
}

// 判断一个数是否为素数
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let sqrt = (n as f64).sqrt() as i32;
    for i in (3..=sqrt).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
