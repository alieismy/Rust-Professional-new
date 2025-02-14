pub fn find_max_prime_factor(mut number: u128) -> u128 {
    let mut max_factor = 1;
    
    // 处理2这个特殊的素数
    while number % 2 == 0 {
        max_factor = 2;
        number /= 2;
    }
    
    // 从3开始检查奇数因子
    let mut i = 3;
    while i * i <= number {
        while number % i == 0 {
            max_factor = i;
            number /= i;
        }
        i += 2;
    }
    
    // 如果剩余的数大于1，它本身就是最大的素因子
    if number > 1 {
        max_factor = number;
    }
    
    max_factor
}
