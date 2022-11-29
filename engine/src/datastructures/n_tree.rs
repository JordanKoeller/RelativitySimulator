use std::{
  borrow::{Borrow, BorrowMut},
  cell::{RefCell, Ref, RefMut},
  rc::Rc,
  ops::{Deref, DerefMut},
};

use crate::utils::{GetMutRef, MutRef, Swap};

pub struct NTree<T> {
  children: Vec<Self>,
  node_type: NodeType,
  value: Swap<T>,
}

impl<T> NTree<T> {
  pub fn new_root(value: T) -> Self {
    Self {
      children: Vec::new(),
      node_type: NodeType::Root,
      value: Swap::new(value),
    }
  }

  fn new_child(value: T) -> Self {
    Self {
      children: Vec::new(),
      node_type: NodeType::Leaf,
      value: Swap::new(value)
    }
  }

  pub fn spawn_from(&mut self, child_value: T) -> &mut Self {
    let child_node = Self::new_child(child_value);
    self.children.push(child_node);
    if self.node_type == NodeType::Leaf {
      self.node_type = NodeType::Branch;
    }
    let i = self.children.len() - 1;
    &mut self.children[i]
  }

  pub fn and<F: FnOnce(&mut T) -> &mut T>(mut self, func: F) -> Self {
    func(self.value.deref_mut());
    self
  }

  pub fn map<F: FnOnce(T) -> T>(mut self, func: F) -> Self {
    self.value.swap_with(|v| func(v));
    self
  }

}

impl<T: NTreeNode> NTree<T> {
  pub fn spawn_child(&mut self) -> &mut Self {
    self.spawn_from(self.value.spawn_child())
  }
}

impl<T: ReducableTree> NTree<T> {
  pub fn consume(self) -> T::Output {
    let children = self.children.into_iter().map(|child| child.consume());
    self.value.unwrap().reduce(children)
  }
}

impl<T> Deref for NTree<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
      &self.value
  }
}

impl<T> DerefMut for NTree<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.value
  }
}

impl<T: NTreeNode> From<T> for NTree<T> {
  fn from(value: T) -> Self {
      NTree::new_root(value)
  }
}

pub trait NTreeNode {
  fn spawn_child(&self) -> Self;
}

pub trait ReducableTree {
  type Output;

  fn reduce<I: IntoIterator<Item = Self::Output>>(self, children: I) -> Self::Output;
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum NodeType {
  Root,
  Branch,
  Leaf,
}
