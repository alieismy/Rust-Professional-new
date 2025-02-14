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
    
    // 处理非方阵的情况：调整矩阵大小为最大边长的方阵
    let max_dim = n.max(matrix[0].len());
    
    // 调整矩阵行数
    while matrix.len() < max_dim {
        matrix.push(vec![0; matrix[0].len()]);
    }
    
    // 调整矩阵列数
    for row in matrix.iter_mut() {
        while row.len() < max_dim {
            row.push(0);
        }
    }
    
    // 从外到内，一层一层旋转
    for layer in 0..max_dim/2 {
        let last = max_dim - 1 - layer;
        
        for i in layer..last {
            // 保存左上角的值
            let temp = matrix[layer][i];
            
            // 左边的值移到上边
            matrix[layer][i] = matrix[last - (i - layer)][layer];
            
            // 下边的值移到左边
            matrix[last - (i - layer)][layer] = matrix[last][last - (i - layer)];
            
            // 右边的值移到下边
            matrix[last][last - (i - layer)] = matrix[i][last];
            
            // 上边的值（之前保存的）移到右边
            matrix[i][last] = temp;
        }
    }
    
    // 如果原矩阵不是方阵，需要调整结果矩阵的大小
    if n != matrix[0].len() {
        // 调整行数
        while matrix.len() > matrix[0].len() {
            matrix.pop();
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
