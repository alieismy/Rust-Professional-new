mod calc_time;

fn main() {
    let time = "2025-01-18";
    let result = calc_time::time_info(time);
    println!("{}", result);
}
