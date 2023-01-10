use std::cell::RefCell;
use std::rc::Rc;
use rand::{random, Rng};
use crate::datastructures::graph::edge::Edge;
use crate::print_edges;

pub struct QuickSelect();

/// Returns median of the given vector in O(n) time.
impl QuickSelect {
    pub fn find_median_edges(array: &mut [Rc<Edge>]) -> Rc<Edge> {
        let mut left = 0;
        let mut right = array.len() - 1;
        let median = (left + right) / 2;
        while left < right {
            let pivot_index = QuickSelect::partition_edges(array, left, right);
            match pivot_index {
                i if i < median => left = i + 1,
                i if i > median => right = i - 1,
                _ => return Rc::clone(&array[median]),
            }
        }
        Rc::clone(&array[median])
    }
    pub fn find_median_edges_mut(array: &mut [Rc<RefCell<Edge>>]) -> Rc<RefCell<Edge>> {
        let mut left = 0;
        let mut right = array.len() - 1;
        let median = (left + right) / 2;
        while left < right {
            let pivot_index = QuickSelect::partition_edges_mut(array, left, right);
            match pivot_index {
                i if i < median => left = i + 1,
                i if i > median => right = i - 1,
                _ => return Rc::clone(&array[median]),
            }
        }
        Rc::clone(&array[median])
    }
    fn partition_edges_mut(array: &mut [Rc<RefCell<Edge>>], left: usize, right: usize) -> usize {
        let mut pivot_index = rand::thread_rng().gen_range(left..=right);
        array.swap(pivot_index, right);
        pivot_index = right;
        let mut i = left;
        for j in left..right {
            if array[j].borrow().get_weight() < array[pivot_index].borrow().get_weight() {
                array.swap(i, j);
                i += 1;
            }
        }
        array.swap(i, pivot_index);
        i
    }
    pub fn find_median_f64(array: &mut [f64]) -> f64 {
        let mut left = 0;
        let mut right = array.len() - 1;
        let median = (left + right) / 2;
        while left < right {
            let pivot_index = QuickSelect::partition_f64(array, left, right);
            match pivot_index {
                i if i < median => left = i + 1,
                i if i > median => right = i - 1,
                _ => return array[median],
            }
        }
        array[median]
    }
    fn partition_f64(array: &mut [f64], left: usize, right: usize) -> usize {
        //let pivot_index = rand::thread_rng().gen_range(left..=right);
        let pivot_index = right;
        let mut i = left;
        for j in left..=right {
            if array[j] < array[pivot_index] {
                array.swap(i, j);
                i += 1;
            }
        }
        array.swap(i, pivot_index);
        i
    }
    fn partition_edges(array: &mut [Rc<Edge>], left: usize, right: usize) -> usize {
        //pivot_index random within range left and right
        let mut pivot_index: usize = rand::thread_rng().gen_range(left..=right);
        array.swap(pivot_index, right);
        pivot_index = right;
        //let pivot_index = right;
        let mut i = left;
        for j in left..=right {
            if array[j].as_ref() < array[pivot_index].as_ref() {
                array.swap(i, j);
                i += 1;
            }
        }
        array.swap(i, pivot_index);
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_select() {
        let mut array = vec![Rc::new(Edge::new(0, 1).weight(2.0)),
                             Rc::new(Edge::new(0, 2).weight(1.0)),
                             Rc::new(Edge::new(0, 3).weight(5.0)),
                             Rc::new(Edge::new(0, 4).weight(4.0)),
                             Rc::new(Edge::new(0, 5).weight(3.0))];
        assert_eq!(QuickSelect::find_median_edges(&mut array).get_weight(), 3.0);
    }

    #[test]
    fn test2_quick_select() {
        let mut array = vec![Rc::new(Edge::new(0, 1).weight(1.0)),
                             Rc::new(Edge::new(0, 2).weight(2.0)),
                             Rc::new(Edge::new(0, 3).weight(3.0)),
                             Rc::new(Edge::new(0, 4).weight(4.0)),
                             Rc::new(Edge::new(0, 5).weight(5.0)),
                             Rc::new(Edge::new(0, 5).weight(5.0))];
        assert_eq!(QuickSelect::find_median_edges(&mut array).get_weight(), 3.0);
    }
}