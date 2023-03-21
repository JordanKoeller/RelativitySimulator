use std::{
  cell::RefCell,
  collections::{HashMap, HashSet, VecDeque},
};
/**
 * Visitor pattern for an arbitrary graph-like datastructure.
 *
 * This visitor makes no assumptions on the shape of the graph.
 * Graphs may be cyclical, disjoint, directed or undirected, etc.
 *
 */
pub trait Graph {
  type Node;

  /** Unique identifier for a particular node. */
  fn id(&self, node: &Self::Node) -> usize;

  /** Given a particular node n, return an iterator of all nodes accessible from the n. */
  fn adjacent(&self, node: &Self::Node) -> Box<dyn Iterator<Item = Self::Node> + '_>;

  /** Return an iterator of all Nodes in the graph */
  fn nodes(&self) -> Box<dyn Iterator<Item = Self::Node> + '_>;

  /** Returns the number of nodes in the graph */
  fn len(&self) -> usize;
}

pub struct GraphVisitor<'a, T: Graph> {
  graph: &'a T,
}

impl<'a, T: Graph> GraphVisitor<'a, T> {
  pub fn new(graph: &'a T) -> Self {
    Self { graph }
  }

  pub fn is_disjoint(&self) -> bool {
    if let Some(origin) = self.graph.nodes().next() {
      let mut walker = GraphIterator::new(self.graph, origin, GraphTraversalOrdering::BreadthFirst);
      walker.walk();
      walker.visited.len() != self.graph.len()
    } else {
      false
    }
  }

  pub fn disjoint_subgraphs(&self) -> impl Iterator<Item = Vec<T::Node>> + '_ {
    let mut all_seen_nodes: HashSet<usize> = HashSet::new();

    self.graph.nodes().filter_map(move |origin| {
      if all_seen_nodes.contains(&self.graph.id(&origin)) {
        None
      } else {
        Some(
          GraphIterator::new(self.graph, origin, GraphTraversalOrdering::BreadthFirst)
            .map(|node| {
              all_seen_nodes.insert(self.graph.id(&node));
              node
            })
            .collect(),
        )
      }
    })
  }

  pub fn breadth_first_traverse(&self) -> impl Iterator<Item = T::Node> + '_ {
    let mut all_seen_nodes: HashSet<usize> = HashSet::default();
    self
      .graph
      .nodes()
      .filter_map(move |origin| {
        if all_seen_nodes.contains(&self.graph.id(&origin)) {
          None
        } else {
          let nodes: Vec<T::Node> = GraphIterator::new(self.graph, origin, GraphTraversalOrdering::BreadthFirst)
            .map(|node| {
              all_seen_nodes.insert(self.graph.id(&node));
              node
            })
            .collect();
          Some(nodes.into_iter())
        }
      })
      .flatten()
  }
}

// Private helper definitions
enum GraphTraversalOrdering {
  BreadthFirst,
  DepthFirst,
}

struct GraphIterator<'a, T: Graph> {
  graph: &'a T,
  deque: VecDeque<T::Node>,
  visited: HashSet<usize>,
  ordering: GraphTraversalOrdering,
}

impl<'a, T: Graph> GraphIterator<'a, T> {
  fn new(graph: &'a T, start: T::Node, ordering: GraphTraversalOrdering) -> Self {
    let mut ret = Self {
      graph,
      deque: VecDeque::new(),
      visited: HashSet::default(),
      ordering,
    };
    ret.visited.insert(ret.graph.id(&start));
    ret.deque.push_back(start);
    ret
  }

  fn next_elem(&mut self) -> Option<T::Node> {
    match self.ordering {
      GraphTraversalOrdering::BreadthFirst => self.deque.pop_front(),
      GraphTraversalOrdering::DepthFirst => self.deque.pop_back(),
    }
  }

  fn walk(&mut self) {
    while let Some(_) = self.next() {}
  }
}

impl<'a, T: Graph> Iterator for GraphIterator<'a, T> {
  type Item = T::Node;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(top) = self.next_elem() {
      self.visited.insert(self.graph.id(&top));
      for adjacent in self.graph.adjacent(&top) {
        if !self.visited.contains(&self.graph.id(&adjacent)) {
          self.deque.push_back(adjacent);
        }
      }
      Some(top)
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct GraphMatrix {
    edges: Vec<Vec<usize>>,
  }

  impl GraphMatrix {
    fn new(mut data: Vec<Vec<usize>>) -> Self {
      for i in 0..data.len() {
        data[i].insert(0, i);
      }
      Self { edges: data }
    }
  }

  impl Graph for GraphMatrix {
    type Node = usize;

    /** Unique identifier for a particular node. */
    fn id(&self, node: &Self::Node) -> usize {
      *node
    }

    /** Given a particular node n, return an iterator of all nodes accessible from the n. */
    fn adjacent(&self, node: &Self::Node) -> Box<dyn Iterator<Item = Self::Node>> {
      let mut nodes: Vec<usize> = self.edges[*node].clone();
      nodes.remove(0);
      Box::from(nodes.into_iter())
    }

    /** Return an iterator of all Nodes in the graph */
    fn nodes(&self) -> Box<dyn Iterator<Item = Self::Node>> {
      let indices: Vec<usize> = self.edges.iter().map(|e| e[0]).collect();
      Box::from(indices.into_iter())
    }

    /** Returns the number of nodes in the graph */
    fn len(&self) -> usize {
      self.edges.len()
    }
  }

  #[test]
  fn disjoint_not_disjoint_success() {
    assert_disjoint(vec![vec![1, 3], vec![2, 3], vec![0, 3], vec![]], false);
  }

  #[test]
  fn disjoint_empty_returns_false() {
    assert_disjoint(vec![], false);
  }

  #[test]
  fn disjoint_is_disjoint_success() {
    assert_disjoint(vec![vec![2], vec![], vec![0]], true)
  }

  // #[test]
  // fn disjoint_bidirectional_success() {
  //   assert_disjoint(vec![
  //     vec![2],
  //     vec![0, 2],
  //     vec![0],
  //   ], false);
  // }

  #[test]
  fn walker_self_loop_terminates() {
    assert_disjoint(vec![vec![0]], false);
  }

  fn assert_disjoint(data: Vec<Vec<usize>>, expected: bool) {
    let graph = GraphMatrix::new(data);

    let visitor = GraphVisitor::new(&graph);

    assert_eq!(visitor.is_disjoint(), expected);
  }
}
