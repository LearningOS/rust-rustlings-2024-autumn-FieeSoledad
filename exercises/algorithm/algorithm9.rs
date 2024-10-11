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
    T: Default+Ord,
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

    //T had not implments Ord Trait! 
    pub fn add(&mut self, value: T) 
    where 
        T:Clone,
    {
        //TODO
        self.count+=1;
        self.items.push(value);
        //up-filter 
        let mut index = self.count;
        //when should be swapped?
        while index>1 && !(self.comparator)(&self.items[self.parent_idx(index)],&self.items[index]){
            let parent_idx = self.parent_idx(index);
            self.items.swap(parent_idx,index);
            index = self.parent_idx(index);
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
        //TODO
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right <= self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            right // 右子节点更小
        } else {
            left // 左子节点更小或只有左子节点
        }
    }

    fn down_filter(&mut self, index: usize) {
        //TODO
        let mut index = index;
        while self.left_child_idx(index) <= self.count {
            let smallest_child = self.smallest_child_idx(index);
            if !(self.comparator)(&self.items[index], &self.items[smallest_child]) {
                self.items.swap(index, smallest_child);
                index = smallest_child;
            } else {
                break;
            }
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
    T: Default+Clone+Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.is_empty(){
            return None;
        }
        let top = self.items[1].clone();
        self.items[1] = self.items[self.count].clone();
        self.count-=1;

        //down-filter
        self.down_filter(1);
        return Some(top);
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

    // #[test]
    // fn test_min_heap() {
    //     let mut heap = MinHeap::new();
    //     heap.add(4);
    //     heap.add(2);
    //     heap.add(9);
    //     heap.add(11);
    //     assert_eq!(heap.len(), 4);
    //     assert_eq!(heap.next(), Some(2));
    //     assert_eq!(heap.next(), Some(4));
    //     assert_eq!(heap.next(), Some(9));
    //     heap.add(1);
    //     assert_eq!(heap.next(), Some(1));
    // }

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