use std::cmp::Ordering;
use std::usize::MAX;

#[derive(Debug)]
pub struct AVLTree<T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialOrd + Clone + 'static,
{
  buffer: Vec<AVLTreeNode<T>>,
  root: usize,
  first_index: usize,
  last_index: usize,
}

impl<T> Default for AVLTree<T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialOrd + Clone + 'static,
{
  fn default() -> Self {
    Self {
      buffer: Vec::new(),
      root: MAX,
      first_index: MAX,
      last_index: MAX,
    }
  }
}

// Public API
impl<T> AVLTree<T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialOrd + Clone + 'static,
{
  pub fn new(default_capacity: usize) -> Self {
    let buffer = Vec::with_capacity(default_capacity);
    Self {
      buffer,
      ..Self::default()
    }
  }

  // Inserts a new element into the AVLTree
  pub fn push(&mut self, elem: T) {
    match self.find_elem(&elem) {
      FindNodeResult::Eq(i) => {
        self.buffer[i].value = elem;
      }
      FindNodeResult::LeftOf(parent_i) => {
        let new_node_index = self.buffer.len();
        self.buffer.push(AVLTreeNode::new_with_parent(elem, parent_i));
        self.buffer[parent_i].left_child = new_node_index;
        if self.first_index == parent_i {
          self.first_index = new_node_index;
        }
      }
      FindNodeResult::RightOf(parent_i) => {
        let new_node_index = self.buffer.len();
        self.buffer.push(AVLTreeNode::new_with_parent(elem, parent_i));
        self.buffer[parent_i].right_child = new_node_index;
        if self.last_index == parent_i {
          self.last_index = new_node_index;
        }
      }
      FindNodeResult::Empty => {
        let node = AVLTreeNode::new(elem);
        self.buffer.push(node);
        self.root = 0usize;
        self.first_index = 0usize;
        self.last_index = 0usize;
      }
    }
  }

  // Find, remove, and return the element that equals `elem_matching`
  pub fn remove(&mut self, elem_matching: &T) -> Option<T> {
    let index_opt = match self.find_elem(elem_matching) {
      FindNodeResult::Eq(i) => Some(i),
      _ => None,
    };
    if let Some(index) = index_opt {
      // Found the element to remove. Need to swap it out of the buffer
      self.swap_nodes(index, self.len() - 1);
      // Need to safely clean up tree
      // First Clone some indices so I can mutate
      let (removing_parent, removing, removing_left, removing_right) = {
        let removing = &self.buffer[self.len() - 1];
        (
          removing.parent,
          self.len() - 1,
          removing.left_child,
          removing.right_child,
        )
      };
      let removing_leftof_parent = self.a_leftof_b(removing, removing_parent);
      if self.buffer[removing].has_one_child() {
        // IF only one child, I just promote that child up to its parent's prior home.
        let child = self.buffer[removing].get_child();
        if removing_leftof_parent {
          self.buffer[removing_parent].left_child = child;
          self.buffer[child].parent = removing_parent;
        } else {
          self.buffer[removing_parent].right_child = child;
          self.buffer[child].parent = removing_parent;
        }
      } else if !self.buffer[removing].is_leaf() {
        // Arbitrarily promote left child up.
        // Insert the right child by recursing down the entire tree.
        if removing_leftof_parent {
          self.buffer[removing_parent].left_child = removing_left;
        } else {
          self.buffer[removing_parent].right_child = removing_left;
        }
        self.buffer[removing_left].parent = removing_parent;
        self.insert_helper(removing_right);
      } else {
        // Removing a leaf. Just need to set its parent to MAX
        if removing_leftof_parent {
          self.buffer[removing_parent].left_child = MAX;
        } else {
          self.buffer[removing_parent].right_child = MAX;
        }
      }
      // Everything is reshuffled and I an safely trim the vector
      self.buffer.pop().map(|node| node.value)
    } else {
      None
    }
  }

  pub fn len(&self) -> usize {
    self.buffer.len()
  }

  pub fn empty(&self) -> bool {
    self.root == MAX
  }

  pub fn iter(&self) -> AVLTreeIterator<'_, T> {
    AVLTreeIterator::new(self)
  }

  pub fn drain(&mut self) {
    self.buffer = Vec::new();
    self.root = MAX;
    self.first_index = MAX;
    self.last_index = MAX;
  }
}

// Private helper functions API
impl<T> AVLTree<T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialOrd + Clone + 'static,
{
  fn find_elem(&self, elem: &T) -> FindNodeResult {
    if self.empty() {
      FindNodeResult::Empty
    } else {
      let mut rover = self.root;
      while rover != MAX && &self.buffer[rover].value != elem {
        if elem < &self.buffer[rover].value {
          if self.buffer[rover].has_left() {
            rover = self.buffer[rover].left_child;
          } else {
            return FindNodeResult::LeftOf(rover);
          }
        } else {
          if self.buffer[rover].has_right() {
            rover = self.buffer[rover].right_child
          } else {
            return FindNodeResult::RightOf(rover);
          }
        }
      }
      FindNodeResult::Eq(rover)
    }
  }

  fn insert_helper(&mut self, node_ind: usize) {
    //TODO: Clean this up to not duplicate code with 'insert'
    match self.find_elem(&self.buffer[node_ind].value) {
      FindNodeResult::Eq(_) => {
        panic!("Tried to insert a node that's already in the tree!");
      }
      FindNodeResult::LeftOf(parent_i) => {
        self.buffer[parent_i].left_child = node_ind;
        self.buffer[node_ind].parent = parent_i;
        if self.first_index == parent_i {
          self.first_index = node_ind;
        }
      }
      FindNodeResult::RightOf(parent_i) => {
        self.buffer[parent_i].right_child = node_ind;
        self.buffer[node_ind].parent = parent_i;
        if self.last_index == parent_i {
          self.last_index = node_ind;
        }
      }
      FindNodeResult::Empty => {
        panic!("Tried to allocate a node that shouldn't need allocated!")
      }
    }
  }

  fn a_leftof_b(&self, a: usize, b: usize) -> bool {
    self.buffer[a].value < self.buffer[b].value
  }

  fn swap_nodes(&mut self, i_node: usize, j_node: usize) {
    let (i_parent, i_left, i_right) = {
      let node = &self.buffer[i_node];
      (node.parent, node.left_child, node.right_child)
    };
    let (j_parent, j_left, j_right) = {
      let node = &self.buffer[j_node];
      (node.parent, node.left_child, node.right_child)
    };
    // Fix pointers to the i_node
    if i_parent != MAX {
      if self.buffer[i_parent].right_child == i_node {
        // i_node is to the right of its parent
        self.buffer[i_parent].right_child = j_node;
      } else {
        self.buffer[i_parent].left_child = j_node;
      }
    }

    // Fix pointers to the (new) j_node
    if j_parent != MAX {
      if self.buffer[j_parent].right_child == j_node {
        // i_node is to the right of its parent
        self.buffer[j_parent].right_child = i_node;
      } else {
        self.buffer[j_parent].left_child = i_node;
      }
    }

    // Clean up the pointers of my i,j nodes' children to their new parents
    if i_left != MAX {
      self.buffer[i_left].parent = j_node;
    }
    if i_right != MAX {
      self.buffer[i_right].parent = j_node;
    }
    if j_left != MAX {
      self.buffer[j_left].parent = i_node;
    }
    if j_right != MAX {
      self.buffer[j_right].parent = i_node;
    }

    // Swap Values
    self.buffer.swap(i_node, j_node);

    // Fix pointers into buffer, if necessary
    if self.root == i_node {
      self.root = j_node;
    } else if self.root == j_node {
      self.root = i_node
    }

    if self.first_index == i_node {
      self.first_index = j_node;
    } else if self.first_index == j_node {
      self.first_index = i_node;
    }

    if self.last_index == i_node {
      self.last_index = j_node;
    } else if self.last_index == j_node {
      self.last_index = i_node;
    }
  }
}

impl<T> AVLTree<T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialOrd + Clone + std::fmt::Display + 'static,
{
  fn print_helper(&self, index: usize, vector: &mut Vec<T>) {
    println!("{}", self.buffer[index]);
    if self.buffer[index].has_left() {
      self.print_helper(self.buffer[index].left_child, vector);
    }
    vector.push(self.buffer[index].value.clone());
    if self.buffer[index].has_right() {
      self.print_helper(self.buffer[index].right_child, vector);
    }
  }

  fn inline_tree(&self) -> Vec<T> {
    println!("Inlining Tree");
    if self.root == MAX {
      Vec::new()
    } else {
      let mut data = Vec::new();
      self.print_helper(self.root, &mut data);
      data
    }
  }
}

enum FindNodeResult {
  LeftOf(usize),
  RightOf(usize),
  Eq(usize),
  Empty,
}

// Node struct for this tree.
#[derive(Debug)]
struct AVLTreeNode<T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialEq + 'static,
{
  pub value: T,
  pub parent: usize,
  pub height: usize,
  pub left_child: usize,
  pub right_child: usize,
  // TODO: add threading to the tree.
  //   prev_elem: usize,
  //   next_elem: usize,
}

// Helper functions for nodes
impl<T> AVLTreeNode<T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialEq + 'static,
{
  pub fn new(value: T) -> Self {
    Self {
      value,
      parent: MAX,
      height: MAX,
      left_child: MAX,
      right_child: MAX,
    }
  }

  pub fn new_with_parent(value: T, parent: usize) -> Self {
    Self {
      value,
      parent,
      height: MAX,
      left_child: MAX,
      right_child: MAX,
    }
  }

  pub fn is_leaf(&self) -> bool {
    self.left_child == MAX && self.right_child == MAX
  }

  pub fn has_left(&self) -> bool {
    self.left_child != MAX
  }

  pub fn has_right(&self) -> bool {
    self.right_child != MAX
  }

  pub fn has_one_child(&self) -> bool {
    (self.has_left() && !self.has_right()) || (self.has_right() && !self.has_left())
  }

  pub fn get_child(&self) -> usize {
    if self.has_left() {
      self.left_child
    } else {
      self.right_child
    }
  }
}

impl<T> std::fmt::Display for AVLTreeNode<T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialEq + std::fmt::Display + 'static,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Node {{value = {} left = {} parent = {} right = {}}}",
      self.value, self.left_child, self.parent, self.right_child
    )
  }
}

impl<T> PartialEq for AVLTreeNode<T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialEq + 'static,
{
  fn eq(&self, other: &Self) -> bool {
    self.value == other.value
  }
}

impl<T> Eq for AVLTreeNode<T> where T: Sized + Ord + PartialOrd + Eq + PartialEq + 'static {}

impl<T> PartialOrd for AVLTreeNode<T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialEq + 'static,
{
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.value.partial_cmp(&other.value)
  }
}

impl<T> Ord for AVLTreeNode<T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialEq + 'static,
{
  fn cmp(&self, other: &Self) -> Ordering {
    self.value.cmp(&other.value)
  }
}

pub struct AVLTreeIterator<'a, T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialEq + Clone + 'static,
{
  tree: &'a AVLTree<T>,
  index: usize,
}

impl<'a, T> Iterator for AVLTreeIterator<'a, T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialEq + Clone + 'static,
{
  type Item = &'a T;
  fn next(&mut self) -> Option<<Self as Iterator>::Item> {
    if self.index != MAX {
      let index_to_return = self.index;
      if self.curr_ref().has_right() {
        self.index = self.curr_ref().right_child;
        self.chase_left();
      } else {
        while self.index != MAX && &self.curr_ref().value <= &self.tree.buffer[index_to_return].value {
          self.index = self.curr_ref().parent;
        }
      }
      Some(&self.tree.buffer[index_to_return].value)
    } else {
      None
    }
  }
}

impl<'a, T> AVLTreeIterator<'a, T>
where
  T: Sized + Ord + PartialOrd + Eq + PartialEq + Clone + 'static,
{
  pub fn peek(&self) -> Option<<Self as Iterator>::Item> {
    if self.index != MAX {
      Some(&self.tree.buffer[self.index].value)
    } else {
      None
    }
  }

  pub fn empty(&self) -> bool {
    self.index == MAX
  }

  fn new(tree: &'a AVLTree<T>) -> Self {
    let mut ret = Self { tree, index: 0 };
    ret.chase_left();
    ret
  }

  fn chase_left(&mut self) {
    while self.tree.buffer[self.index].has_left() {
      self.index = self.tree.buffer[self.index].left_child;
    }
  }

  fn curr_ref(&self) -> &AVLTreeNode<T> {
    &self.tree.buffer[self.index]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_swap_nodes() {
    let mut tree = AVLTree::<usize>::default();
    tree.push(4);
    tree.push(2);
    tree.push(6);
    tree.push(1);
    tree.push(3);
    tree.push(5);
    tree.push(7);
    tree.swap_nodes(1, 2);
    let left = &tree.buffer[2];
    let righ = &tree.buffer[1];
    assert_eq!(left.parent, 0);
    assert_eq!(righ.parent, 0);
    assert_eq!(left.left_child, 3);
    assert_eq!(left.right_child, 4);
    assert_eq!(righ.left_child, 5);
    assert_eq!(righ.right_child, 6);
    assert_eq!(tree.buffer[3].parent, 2);
    assert_eq!(tree.buffer[4].parent, 2);
    assert_eq!(tree.buffer[5].parent, 1);
    assert_eq!(tree.buffer[6].parent, 1);
    assert_eq!(tree.buffer[0].left_child, 2);
    assert_eq!(tree.buffer[0].right_child, 1);
  }

  #[test]
  fn test_insert_tree() {
    let mut tree = AVLTree::<usize>::default();
    tree.push(5);
    assert_eq!(tree.len(), 1);
    tree.push(4);
    assert_eq!(tree.len(), 2);
    tree.push(6);
    tree.push(3);
    tree.push(2);
    tree.push(1);
    tree.push(9);
    tree.push(8);
    tree.push(7);
    assert_eq!(tree.len(), 9);
    let inline = tree.inline_tree();
    print!("Tree = ");
    for i in 0..9 {
      print!("{} ", inline[i]);
      assert_eq!(inline[i], i + 1);
    }
  }

  #[test]
  fn test_remove_tree() {
    let mut tree = AVLTree::<usize>::default();
    tree.push(5);
    assert_eq!(tree.len(), 1);
    tree.push(4);
    assert_eq!(tree.len(), 2);
    assert_inline(&tree);
    tree.push(6);
    assert_inline(&tree);
    tree.push(3);
    assert_inline(&tree);
    tree.push(2);
    assert_inline(&tree);
    tree.push(1);
    assert_inline(&tree);
    tree.push(9);
    assert_inline(&tree);
    tree.push(8);
    assert_inline(&tree);
    tree.push(7);
    assert_inline(&tree);
    assert_eq!(tree.len(), 9);
    assert_eq!(tree.remove(&4), Some(4));
    assert_inline(&tree);
    assert_eq!(tree.remove(&6), Some(6));
    assert_inline(&tree);
    assert_eq!(tree.remove(&1), Some(1));
    let inline = tree.inline_tree();
    let expected = vec![2, 3, 5, 7, 8, 9];
    for (i, j) in expected.iter().zip(inline.iter()) {
      assert_eq!(i, j);
    }
  }

  #[test]
  fn test_traverse_tree() {
    let mut tree = AVLTree::<usize>::default();
    tree.push(5);
    assert_iterator(&tree);
    tree.push(4);
    assert_iterator(&tree);
    tree.push(6);
    assert_iterator(&tree);
    tree.push(3);
    assert_iterator(&tree);
    tree.push(2);
    assert_iterator(&tree);
    tree.push(1);
    assert_iterator(&tree);
    tree.push(9);
    assert_iterator(&tree);
    tree.push(8);
    assert_iterator(&tree);
    tree.push(7);
    assert_iterator(&tree);
    assert_eq!(tree.len(), 9);
    println!("About to start removing");
    assert_eq!(tree.remove(&4), Some(4));
    assert_iterator(&tree);
    assert_eq!(tree.remove(&6), Some(6));
    assert_iterator(&tree);
    assert_eq!(tree.remove(&1), Some(1));
    assert_iterator(&tree);
  }

  fn assert_inline(tree: &AVLTree<usize>) {
    let inline = tree.inline_tree();
    assert_eq!(inline.len(), tree.len());
    for i in 1..inline.len() {
      assert_eq!(inline[i - 1] < inline[i], true);
    }
  }

  fn assert_iterator(tree: &AVLTree<usize>) {
    let inline_tree = tree.inline_tree();
    for (i, j) in tree.iter().zip(inline_tree.iter()) {
      assert_eq!(i, j);
    }
  }
}
