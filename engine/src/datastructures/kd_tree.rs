use std::cmp::{Ord, Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};

use cgmath::prelude::*;

use super::{HasPosition, SpatialIndex};
use crate::Vec3F;

pub struct KdTree<T: HasPosition> {
    data: Vec<T>,
    indices: Vec<usize>,
    partition_tree: Vec<KdTreeNode>,
    node_size: usize,
}

impl<T: HasPosition> Default for KdTree<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            indices: Vec::new(),
            partition_tree: Vec::new(),
            node_size: 8,
        }
    }
}

impl<T: HasPosition> KdTree<T> {
    pub fn new(data: Vec<T>, node_size: usize) -> Self {
        let mut ret = Self {
            data: Vec::new(),
            indices: Vec::new(),
            partition_tree: Vec::new(),
            node_size,
        };
        ret.set_data(data);
        ret
    }
}

impl<T: HasPosition> SpatialIndex<T> for KdTree<T> {
    // TODO: Implement push
    fn push(&mut self, _element: T) {}

    fn set_data(&mut self, data: Vec<T>) {
        self.clear();
        self.data = data;
        self.indices = (0..self.data.len()).collect();
        let mut splits_queue = VecDeque::new();
        splits_queue.push_back(KdTreeNode::new(0, self.data.len(), 0));
        while splits_queue.len() > 0 {
            let mut node = splits_queue.pop_front().unwrap();
            if node.end_index - node.start_index > self.node_size {
                let midpt = self.partition(node.start_index, node.end_index, node.dimension);
                node.partition_value = midpt;
                let dim = (node.dimension + 1) % 3;
                let mid_index = (node.end_index + node.start_index) / 2;
                splits_queue.push_back(KdTreeNode::new(node.start_index, mid_index, dim));
                splits_queue.push_back(KdTreeNode::new(mid_index, node.end_index, dim));
            }
            self.partition_tree.push(node);
        }
    }

    fn clear(&mut self) {
        self.data.clear();
        self.partition_tree.clear();
        self.indices.clear();
    }
    fn count(&self) -> usize {
        self.data.len()
    }
    fn query_near(&self, position: &Vec3F, radius: f64) -> Vec<usize> {
        let mut ret = Vec::new();
        let r32 = radius as f64;
        self.kd_reduce(position, &r32, |&i| ret.push(i));
        ret
    }
    fn query_near_count(&self, position: &Vec3F, radius: f64) -> usize {
        let mut ret = 0usize;
        let r32 = radius as f64;
        self.kd_reduce(position, &r32, |_| ret = ret + 1);
        ret
    }

    fn data(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T: HasPosition> KdTree<T> {
    fn partition(&mut self, start_i: usize, end_i: usize, d: usize) -> f64 {
        // Todo: Optimize this another time. I'm just going to heapsort
        let mut heap = BinaryHeap::new();
        let k = (start_i + end_i) / 2;
        for i in start_i..end_i {
            heap.push(Reverse(SortIndexArrayElem::new(&self.data, i, d)));
        }
        let mut ret = self.data[self.indices[start_i]].position()[d];
        for i in start_i..end_i {
            let elem = heap.pop().unwrap().0;
            self.indices[i] = elem.index;
            if elem.index == k {
                ret = *elem.value();
            }
        }
        ret
    }

    fn within(&self, left: &f64, right: &f64, value: &f64) -> bool {
        left <= value && value <= right
    }

    fn kd_reduce<R: FnMut(&usize) -> ()>(&self, position: &Vec3F, radius: &f64, mut reducer: R) {
        let mut nodes = vec![0];
        let r2 = radius * radius;
        while nodes.len() > 0 {
            let node_ind = nodes.pop().unwrap();
            let node = &self.partition_tree[node_ind];
            let left_child = node_ind * 2 + 1;
            let right_child = node_ind * 2 + 2;
            if left_child < nodes.len() && node.partition_value >= position[node.dimension] - radius {
                // Need to recurse down left
                nodes.push(left_child);
            }
            if right_child < nodes.len() && node.partition_value <= position[node.dimension] + radius {
                // Need to recurse down right
                nodes.push(right_child);
            }
            if nodes.len() < left_child {
                // We are at a leaf. So time to accumulate!
                for i in node.start_index..node.end_index {
                    let elem_pos = self.data[self.indices[i]].position();
                    let delta = elem_pos - position;
                    if delta.dot(delta) < r2 {
                        reducer(&self.indices[i]);
                    }
                }
            }
        }
    }
}

// Inclusive on the low end, exclusive on the high
struct KdTreeNode {
    pub start_index: usize,
    pub end_index: usize,
    pub partition_value: f64,
    pub dimension: usize,
}

impl KdTreeNode {
    pub fn new(s: usize, e: usize, dimension: usize) -> Self {
        Self {
            start_index: s,
            end_index: e,
            partition_value: 0f64,
            dimension,
        }
    }
}

struct SortIndexArrayElem<'a, T: HasPosition> {
    data: &'a Vec<T>,
    pub index: usize,
    dimension: usize,
}

impl<'a, T: HasPosition> SortIndexArrayElem<'a, T> {
    fn value(&self) -> &f64 {
        &self.data[self.index].position()[self.dimension]
    }

    fn new(data: &'a Vec<T>, index: usize, dimension: usize) -> Self {
        Self { data, index, dimension }
    }
}

impl<'a, T: HasPosition> Ord for SortIndexArrayElem<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.value();
        let rhs = other.value();
        if lhs < rhs {
            Ordering::Less
        } else if lhs == rhs {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl<'a, T: HasPosition> PartialEq for SortIndexArrayElem<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl<'a, T: HasPosition> PartialOrd for SortIndexArrayElem<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl<'a, T: HasPosition> Eq for SortIndexArrayElem<'a, T> {}

#[cfg(test)]
mod test {
    use super::*;

    struct TesterPoint(Vec3F);

    impl HasPosition for TesterPoint {
        fn position(&self) -> &Vec3F {
            &self.0
        }
    }

    #[test]
    fn test_tree_under_node_size_elems_does_not_recurse() {
        let verts = generate_voxel_points(2);
        let tree = KdTree::new(verts, 8);
        assert_eq!(tree.partition_tree.len(), 1);
    }

    #[test]
    fn test_tree_splits_after_tree_size() {
        let verts = generate_voxel_points(3);
        assert_eq!(verts.len(), 27);
        let tree = KdTree::new(verts, 8);
        assert_eq!(tree.partition_tree.len(), 7);
    }

    #[test]
    fn test_tree_grows_large() {
        let verts = generate_voxel_points(9); // 729
        assert_eq!(verts.len(), 729);
        let tree = KdTree::new(verts, 8);
        assert_eq!(tree.partition_tree.len(), 255);
    }

    #[test]
    fn test_can_query_tree_count() {
        let verts = generate_voxel_points(9); // 729
        let tree = KdTree::new(verts, 8);
        let count = tree.query_near_count(&Vec3F::new(4.0, 4.0, 4.0), 0.5);
        assert_eq!(count, 1);
        let count = tree.query_near_count(&Vec3F::new(4.0, 4.0, 4.0), 1.00001);
        assert_eq!(count, 7);
        let count = tree.query_near_count(&Vec3F::new(4.0, 4.0, 4.0), 3f64.sqrt() + 0.0001);
        assert_eq!(count, 27);
    }

    #[test]
    fn test_can_query_indices() {
        let verts = generate_voxel_points(9); // 729
        let tree = KdTree::new(verts, 8);
        let indices = tree.query_near(&Vec3F::new(4.0, 4.0, 4.0), 3f64.sqrt() + 0.0001);
        indices.iter().for_each(|pt| {
            let pos = tree.data()[*pt].position();
            assert_eq!(pos.x == 3.0 || pos.x == 4.0 || pos.x == 5.0, true);
            assert_eq!(pos.y == 3.0 || pos.y == 4.0 || pos.y == 5.0, true);
            assert_eq!(pos.z == 3.0 || pos.z == 4.0 || pos.z == 5.0, true);
        });
    }

    fn generate_voxel_points(dims: usize) -> Vec<TesterPoint> {
        let mut verts = Vec::new();
        for i in 0..dims {
            for j in 0..dims {
                for k in 0..dims {
                    verts.push(TesterPoint(Vec3F::new(i as f64, j as f64, k as f64)));
                }
            }
        }
        verts
    }
}
