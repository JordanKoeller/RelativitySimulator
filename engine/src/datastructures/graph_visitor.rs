use std::collections::{HashMap, HashSet, VecDeque};
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
    // Node -> The subgraph it belongs to.
    /*
     * For each node:
     *   start a dfs.
     *   If the dfs encounters a node visited already, set the new
     *     node's subgraph_id to equal the already-visited node's subgraph_id.
     *
     * If at the end there is only one subgraph_id for all nodes, then the graph is not disjoint
     */

    let mut walker = GraphWalker::new(self.graph, |_| false);

    if let Some(origin) = self.graph.nodes().next() {
      walker.walk_bfs(origin);
    }

    walker.visited.len() != self.graph.len()
    // let mut subgraphs: HashMap<usize, usize> = HashMap::new();

    // for origin in self.graph.nodes() {
    //   let subgraph_id = self.graph.id(origin);
    //   if !subgraphs.contains_key(&subgraph_id) {
    //     // We've never visited this node, ever. So start a new subgraph entry.

    //     let mut walker = GraphWalker::new(self.graph, visitor);

    //     walker.walk_dfs(origin);
    //   }
    // }

    // let mut ids = None;

    // for subgraph_id in subgraphs.values() {
    //   if ids.is_none() {
    //     ids = Some(subgraph_id);
    //   } else {
    //     if ids.unwrap() != subgraph_id {
    //       return true;
    //     }
    //   }
    // }

    // false
  }
}

// Private helper definitions
pub struct GraphWalker<'a, T, F>
where
  T: Graph,
  F: FnMut(&T::Node) -> bool,
{
  visited: HashSet<usize>,
  visitor: F,
  graph: &'a T,
}

impl<'a, T, F> GraphWalker<'a, T, F>
where
  T: Graph,
  F: FnMut(&T::Node) -> bool,
{
  pub fn new(graph: &'a T, visitor: F) -> Self {
    Self {
      visited: HashSet::new(),
      visitor,
      graph,
    }
  }

  // Walk the graph, applying the visitor function to each node.
  pub fn walk_dfs(&mut self, start: T::Node) {
    if self.visited.contains(&self.graph.id(&start)) {
      return;
    }

    let mut stack = VecDeque::<T::Node>::new();

    stack.push_back(start);

    while !stack.is_empty() {
      let origin = stack.pop_back().unwrap();

      self.visited.insert(self.graph.id(&origin));
      if self.visit(&origin) {
        return;
      }

      for adjacent in self.graph.adjacent(&origin) {
        if !self.visited.contains(&self.graph.id(&adjacent)) {
          stack.push_back(adjacent);
        }
      }
    }
  }

  pub fn walk_bfs(&mut self, start: T::Node) {
    if self.visited.contains(&self.graph.id(&start)) {
      return;
    }

    let mut queue = VecDeque::<T::Node>::new();

    queue.push_back(start);

    while !queue.is_empty() {
      let origin = queue.pop_front().unwrap();

      self.visited.insert(self.graph.id(&origin));
      if self.visit(&origin) {
        return;
      }

      for adjacent in self.graph.adjacent(&origin) {
        if !self.visited.contains(&self.graph.id(&adjacent)) {
          queue.push_back(adjacent);
        }
      }
    }
  }

  fn visit(&mut self, node: &T::Node) -> bool {
    (self.visitor)(node)
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
    fn adjacent(&self, node: &Self::Node) -> Box<dyn Iterator<Item = Self::Node> + '_> {
      let mut nodes: Vec<usize> = self.edges[*node].clone();
      nodes.remove(0);
      Box::from(nodes.into_iter())
    }

    /** Return an iterator of all Nodes in the graph */
    fn nodes(&self) -> Box<dyn Iterator<Item = Self::Node> + '_> {
      let indices = self.edges.iter().map(|e| e[0]);
      Box::from(indices)
    }

    /** Returns the number of nodes in the graph */
    fn len(&self) -> usize {
      self.edges.len()
    }
  }

  #[test]
  fn disjoint_notDisjoint_success() {
    assert_disjoint(vec![vec![1, 3], vec![2, 3], vec![0, 3], vec![]], false);
  }

  #[test]
  fn disjoint_empty_returnsFalse() {
    assert_disjoint(vec![], false);
  }

  #[test]
  fn disjoint_isDisjoint_success() {
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
