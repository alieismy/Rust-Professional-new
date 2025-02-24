pub fn time_info(time: &str) -> String {
    let parts: Vec<&str> = time.split('-').collect();
    let year: i32 = parts[0].parse().unwrap();
    let month: u32 = parts[1].parse().unwrap();
    let day: u32 = parts[2].parse().unwrap();

    // 计算当天是本年的第几天
    let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut day_of_year = day;
    for m in 1..month {
        day_of_year += days_in_month[m as usize];
    }
    if month > 2 && is_leap_year(year) {
        day_of_year += 1; // 闰年2月多一天
    }

  // 计算当前是第几周
  let week_number = (((day_of_year as i32) + first_day_of_year(year) - 1 + 6) / 7) as u32; // 计算周数
    let week_day = (((day_of_year as i32) + first_day_of_year(year) - 2) % 7) + 1; // 1=周一, 2=周二, ..., 7=周日

    // 计算当年还剩多少天
    let total_days = if is_leap_year(year) { 366 } else { 365 };
    let remaining_days = total_days - day_of_year;

    // 计算距离春节还有多少天
    let spring_festival_day = 29; // 春节近似为1月29日
    let next_spring_festival = 365 + spring_festival_day;
    let days_until_spring: u32 = if day_of_year < spring_festival_day {
        spring_festival_day - day_of_year
    } else {
        (next_spring_festival - day_of_year) as u32
    };





  // 计算距离下次A股开盘还有多少天
  let days_until_next_open = if month == 5 && day == 1 { 4 } else if week_day == 6 { 2 } else if week_day == 7 { 1 } else { 0 };

    // 返回结果
    format!("{},{},{},{},{},{}", week_number, week_day, day_of_year, remaining_days, days_until_spring, days_until_next_open)
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn first_day_of_year(year: i32) -> i32 {
    let y = year - 1;
    let mut day =
        (y * 365 + y / 4 - y / 100 + y / 400 + 1) % 7;
    if day == 0 {
        day = 7;
    }
    day
}
