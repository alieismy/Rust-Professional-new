/*
    stack
	This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>, // 直接使用 Vec<T> 存储数据，size 可以通过 data.len() 获取
}

impl<T> Stack<T> {
	fn new() -> Self {
        Self { data: Vec::new() }
		}
	fn is_empty(&self) -> bool {
        self.data.is_empty() // 使用 Vec 的 is_empty() 方法
	}

	fn len(&self) -> usize {
        self.data.len() // 使用 Vec 的 len() 方法
	}

	fn clear(&mut self) {
        self.data.clear(); // 使用 Vec 的 clear() 方法
	}

	fn push(&mut self, val: T) {
        self.data.push(val); // 使用 Vec 的 push() 方法
	}

	fn pop(&mut self) -> Option<T> {
        self.data.pop() // 使用 Vec 的 pop() 方法
    }
	fn peek(&self) -> Option<&T> {
        self.data.last() // 使用 Vec 的 last() 方法
		}
	fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut() // 使用 Vec 的 last_mut() 方法
		}
    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.data.into_iter() // 直接使用 Vec 的 into_iter()
	}
    fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter() // 使用 Vec 的 iter() 方法
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data.iter_mut() // 使用 Vec 的 iter_mut() 方法
    }
}
fn bracket_match(bracket: &str) -> bool {
	let mut stack = Stack::new();
	
	for ch in bracket.chars() {
		match ch {
			// 遇到左括号，压入栈中
			'(' | '[' | '{' => stack.push(ch),
			
			// 遇到右括号，检查是否匹配
			')' | ']' | '}' => {
                // 使用 if let 简化代码
                if let Some(top) = stack.pop() {
				// 检查括号是否匹配
                    match (top, ch) {
                        ('(', ')') | ('[', ']') | ('{', '}') => {} // 匹配，继续
                        _ => return false,                       // 不匹配，返回 false
                    }
                } else {
                    return false; // 栈为空，没有对应的左括号，返回 false
                }
            }
			// 忽略其他字符
            _ => continue,
		}
	}
	
	// 最后检查栈是否为空（是否所有左括号都有匹配的右括号）
	stack.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
    fn bracket_matching_1() {
		let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
	}

	#[test]
    fn bracket_matching_2() {
		let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
	}

	#[test]
    fn bracket_matching_3() {
		let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
	}

	#[test]
    fn bracket_matching_4() {
		let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
	}

	#[test]
    fn bracket_matching_5() {
		let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
	}

	#[test]
    fn bracket_matching_6() {
		let s = "";
        assert_eq!(bracket_match(s), true);
	}
}