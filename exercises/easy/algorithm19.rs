/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number. 
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.
    
    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

pub fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    
    // 定义基础矩阵 [[1,1],[1,0]]
    let mut base = vec![vec![1, 1], vec![1, 0]];
    let mut result = vec![vec![1, 0], vec![0, 1]]; // 单位矩阵
    let mut power = n - 1;
    
    // 快速幂计算矩阵的 n-1 次方
    while power > 0 {
        if power & 1 == 1 {
            result = multiply_matrix(&result, &base);
        }
        base = multiply_matrix(&base, &base);
        power >>= 1;
    }
    
    // 结果矩阵的 [0][0] 元素就是第 n 个斐波那契数
    result[0][0]
}

// 辅助函数：2x2矩阵乘法
fn multiply_matrix(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    vec![
        vec![
            a[0][0] * b[0][0] + a[0][1] * b[1][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1]
        ],
        vec![
            a[1][0] * b[0][0] + a[1][1] * b[1][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1]
        ]
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
