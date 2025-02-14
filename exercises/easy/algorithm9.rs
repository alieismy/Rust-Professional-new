/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 添加新元素到堆底
        self.items.push(value);
        self.count += 1;
        
        // 向上调整以维护堆的性质
        let mut idx = self.count;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            
            // 如果当前节点满足堆的性质，停止调整
            if (self.comparator)(&self.items[parent_idx], &self.items[idx]) {
                break;
            }
            
            // 否则交换当前节点和父节点
            self.items.swap(parent_idx, idx);
            idx = parent_idx;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // 如果没有子节点，返回当前索引
        if !self.children_present(idx) {
            return idx;
        }
        
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        
        // 如果只有左子节点
        if right_idx > self.count {
            return left_idx;
        }
        
        // 返回较小（或较大，取决于比较器）的子节点索引
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // 保存堆顶元素
        let result = std::mem::replace(&mut self.items[1], T::default());
        
        // 将最后一个元素移到堆顶
        if self.count > 1 {
            self.items[1] = self.items.pop().unwrap();
        } else {
            self.items.pop();
        }
        
        self.count -= 1;
        
        // 向下调整以维护堆的性质
        if !self.is_empty() {
            let mut current_idx = 1;
            while self.children_present(current_idx) {
                let child_idx = self.smallest_child_idx(current_idx);
                
                // 如果当前节点满足堆的性质，停止调整
                if (self.comparator)(&self.items[current_idx], &self.items[child_idx]) {
                    break;
                }
                
                // 否则交换当前节点和子节点
                self.items.swap(current_idx, child_idx);
                current_idx = child_idx;
            }
        }
        
        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}