pub fn retire_time(time: &str, tp: &str) -> String {
    // 解析出生年月
    let parts: Vec<&str> = time.split('-').collect();
    let birth_year: i32 = parts[0].parse().unwrap();
    let birth_month: i32 = parts[1].parse().unwrap();

    // 计算基础退休年龄和延迟月数
    let (base_age, delay_months) = match tp {
        "原法定退休年龄55周岁女职工" => calculate_female_55(birth_year),
        "原法定退休年龄50周岁女职工" => calculate_female_50(birth_year),
        "男职工" => calculate_male(birth_year),
        _ => panic!("Unknown worker type"),
    };

    // 计算实际退休年月和年龄
    let total_months = (birth_month + base_age * 12 + delay_months) as i32;
    let retire_year = birth_year + total_months / 12;
    let retire_month = total_months % 12;
    let retire_month = if retire_month == 0 { 12 } else { retire_month };
    let retire_year = if retire_month == 12 { retire_year - 1 } else { retire_year };

    // 计算精确退休年龄
    let retire_age = base_age as f64 + (delay_months as f64) / 12.0;
    let retire_age = (retire_age * 100.0).round() / 100.0;

    // 格式化输出
    format!("{:04}-{:02},{:.2},{}", 
            retire_year, 
            retire_month, 
            retire_age, 
            delay_months)
}

fn calculate_female_55(birth_year: i32) -> (i32, i32) {
    match birth_year {
        ..=1963 => (55, 0),
        1964..=1965 => (55, 4),
        1966..=1970 => (56, 0),
        1971..=1975 => (55, 4),
        1976..=1980 => (56, 12),
        1981..=1990 => (57, 24),
        1991..=1995 => (57, 36),
        1996..=2000 => (58, 36),
        _ => (58, 36),
    }
}

fn calculate_female_50(birth_year: i32) -> (i32, i32) {
    match birth_year {
        ..=1963 => (50, 0),
        1964..=1965 => (50, 4),
        1966..=1970 => (51, 0),
        1971..=1975 => (51, 4),
        1976..=1980 => (52, 12),
        1981..=1990 => (53, 24),
        1991..=1995 => (55, 60),
        1996..=2000 => (55, 60),
        _ => (55, 60),
    }
}

fn calculate_male(birth_year: i32) -> (i32, i32) {
    match birth_year {
        ..=1963 => (60, 0),
        1964..=1965 => (60, 4),
        1966..=1970 => (61, 0),
        1971..=1975 => (61, 4),
        1976..=1980 => (62, 12),
        1981..=1990 => (62, 24),
        1991..=1995 => (63, 36),
        1996..=2000 => (63, 36),
        _ => (63, 36),
    }
}
