pub fn retire_time(birth_date: &str, worker_type: &str) -> String {
    let (birth_year, birth_month) = {
        let parts: Vec<&str> = birth_date.split('-').collect();
        (parts[0].parse::<i32>().unwrap(), parts[1].parse::<u32>().unwrap())
    };
    
    let (base_age, delay_months) = match worker_type {
        "原法定退休年龄55周岁女职工" => get_female_55_policy(birth_year, birth_month),
        "原法定退休年龄50周岁女职工" => get_female_50_policy(birth_year, birth_month),
        "男职工" => {
            // 男性退休政策实现
            if birth_year < 1963 || (birth_year == 1963 && birth_month <= 12) {
                (60, 0)
            } else {
                (63, 36)
            }
        },
        _ => panic!("Unknown worker type")
    };
    
    let (retire_year, retire_month, actual_age) = 
        calculate_retirement_date(birth_year, birth_month, base_age, delay_months);
    
    format!("{}-{:02},{:.2},{}", retire_year, retire_month, actual_age, delay_months)
}

// 解析日期
fn parse_date(date: &str) -> (i32, u32) {
    let parts: Vec<&str> = date.split('-').collect();
    let year = parts[0].parse::<i32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();
    (year, month)
}

// 获取退休政策
fn get_retirement_policy(birth_year: i32, birth_month: u32, tp: &str) -> (u32, u32) {
    match tp {
        "男职工" => get_male_policy(birth_year, birth_month),
        "原法定退休年龄55周岁女职工" => get_female_55_policy(birth_year, birth_month),
        "原法定退休年龄50周岁女职工" => get_female_50_policy(birth_year, birth_month),
        _ => panic!("Unknown type")
    }
}

// 男性退休政策
fn get_male_policy(birth_year: i32, birth_month: u32) -> (u32, u32) {
    if birth_year < 1963 || (birth_year == 1963 && birth_month <= 12) {
        (60, 0)
    } else if birth_year == 1964 {
        (60, 0)
    } else if birth_year == 1965 {
        if birth_month == 1 {
            (60, 1)
        } else {
            (60, 3)
        }
    } else {
        (63, 36) // 最终目标
    }
}

// 女性55岁退休政策
fn get_female_55_policy(birth_year: i32, birth_month: u32) -> (u32, u32) {
    if birth_year < 1963 || (birth_year == 1963 && birth_month <= 12) {
        (55, 0)
    } else if birth_year == 1971 && birth_month == 4 {
        (55, 4)
    } else if birth_year >= 2000 {
        (58, 36) // 最终目标
    } else {
        (55, 0) // 默认情况
    }
}

// 女性50岁退休政策
fn get_female_50_policy(birth_year: i32, birth_month: u32) -> (u32, u32) {
    if birth_year >= 1995 {
        (55, 60) // 最终目标
    } else {
        (50, 0) // 默认情况
    }
}

// 计算具体退休日期和年龄
fn calculate_retirement_date(
    birth_year: i32,
    birth_month: u32,
    base_age: u32,
    delay_months: u32
) -> (i32, u32, f64) {
    let total_months = birth_month as i32 + (base_age * 12) as i32 + delay_months as i32;
    let retire_year = birth_year + (total_months / 12);
    let retire_month = (total_months % 12) as u32;
    
    // 计算实际退休年龄
    let actual_age = base_age as f64 + (delay_months as f64 / 12.0);
    
    (retire_year, if retire_month == 0 { 12 } else { retire_month }, actual_age)
}
