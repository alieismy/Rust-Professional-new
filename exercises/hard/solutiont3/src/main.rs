use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde_json::{Value, Map};

mod district;

fn main() {
    let result = district::count_provinces_from_file();
    println!("{}", result);
}

// 测试运行模式
fn run_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("启动测试模式...");
    Ok(())
}
