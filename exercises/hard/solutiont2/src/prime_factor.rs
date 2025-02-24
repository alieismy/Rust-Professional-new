pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut n = number;
    let mut max_prime = 0;
    
    // 处理2这个特殊的素数
    while n % 2 == 0 {
        max_prime = 2;
        n /= 2;
    }
    
    // 处理3这个特殊的素数
    while n % 3 == 0 {
        max_prime = 3;
        n /= 3;
    }
    
    // 使用6k±1优化的试除法
    let mut i = 5;
    while i * i <= n {
        // 检查6k-1的形式
        while n % i == 0 {
            max_prime = i;
            n /= i;
        }
        // 检查6k+1的形式
        while n % (i + 2) == 0 {
            max_prime = i + 2;
            n /= (i + 2);
        }
        i += 6;
    }
    
    // 如果n大于1，说明n本身是素数
    if n > 1 {
        max_prime = n;
    }
    
    max_prime
}
