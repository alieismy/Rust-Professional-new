/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place. 
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    if n <= 1 {
        return;
    }
    
    // 如果是非方阵，需要先调整矩阵大小
    let m = matrix[0].len();
    if n != m {
        // 创建新的矩阵大小
        let mut new_matrix = vec![vec![0; n]; m];
        
        // 旋转填充新矩阵
        for i in 0..n {
            for j in 0..m {
                new_matrix[j][n-1-i] = matrix[i][j];
            }
        }
        
        // 替换原矩阵
        *matrix = new_matrix;
        return;
    }
    
    // 对于方阵，按层处理旋转
    for layer in 0..n/2 {
        let last = n - 1 - layer;
        for i in layer..last {
            // 保存顶部元素
            let top = matrix[layer][i];
            
            // 左边 -> 顶部
            matrix[layer][i] = matrix[n-1-i][layer];
            
            // 底部 -> 左边
            matrix[n-1-i][layer] = matrix[last][n-1-i];
            
            // 右边 -> 底部
            matrix[last][n-1-i] = matrix[i][last];
            
            // 顶部 -> 右边
            matrix[i][last] = top;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![3, 1],
            vec![4, 2],
        ]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![
            vec![1],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![1],
        ]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![5, 3, 1],
            vec![6, 4, 2],
        ]);
    }
}
