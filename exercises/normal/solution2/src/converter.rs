pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 解析输入字符串，提取数字和原始进制
    let (num, from_base) = parse_input(num_str);
    
    // 先将数字转换为10进制
    let decimal = to_decimal(&num, from_base);
    
    // 再将10进制转换为目标进制
    from_decimal(decimal, to_base)
}

// 解析输入字符串，返回数字部分和原始进制
fn parse_input(input: &str) -> (String, u32) {
    let parts: Vec<&str> = input.trim_end_matches(')').split('(').collect();
    let num = parts[0].to_string();
    let base = parts[1].parse::<u32>().unwrap();
    (num, base)
}

// 将任意进制转换为10进制
fn to_decimal(num: &str, from_base: u32) -> u32 {
    let mut result = 0;
    for c in num.chars() {
        let digit = match c {
            '0'..='9' => c as u32 - '0' as u32,
            'a'..='f' => c as u32 - 'a' as u32 + 10,
            'A'..='F' => c as u32 - 'A' as u32 + 10,
            _ => panic!("Invalid digit"),
        };
        result = result * from_base + digit;
    }
    result
}

// 将10进制转换为目标进制
fn from_decimal(mut num: u32, to_base: u32) -> String {
    if num == 0 {
        return "0".to_string();
    }
    
    let mut result = String::new();
    while num > 0 {
        let digit = num % to_base;
        let char = match digit {
            0..=9 => (digit as u8 + b'0') as char,
            10..=15 => (digit as u8 - 10 + b'a') as char,
            _ => panic!("Invalid digit"),
        };
        result.insert(0, char);
        num /= to_base;
    }
    result
}
