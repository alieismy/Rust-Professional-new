/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord>(array: &mut [T]) {
	if array.len() <= 1 {
		return;
	}
	
	let pivot = partition(array);
	
	// 递归排序左右两部分
	sort(&mut array[0..pivot]);
	sort(&mut array[pivot + 1..]);
}

// 分区函数，返回基准元素的位置
fn partition<T: Ord>(array: &mut [T]) -> usize {
	let len = array.len();
	let pivot = len - 1; // 选择最后一个元素作为基准
	let mut i = 0;
	
	// 将小于基准的元素移到左边
	for j in 0..len - 1 {
		if array[j] <= array[pivot] {
			array.swap(i, j);
			i += 1;
		}
	}
	
	// 将基准元素放到正确的位置
	array.swap(i, pivot);
	i
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sort_1() {
		let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
		sort(&mut vec);
		assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
	}
	#[test]
	fn test_sort_2() {
		let mut vec = vec![1];
		sort(&mut vec);
		assert_eq!(vec, vec![1]);
	}
	#[test]
	fn test_sort_3() {
		let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
		sort(&mut vec);
		assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
	}
}