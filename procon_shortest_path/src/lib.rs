extern crate __procon_graph as graph;
extern crate __procon_math_traits as math_traits;

use graph::Graph;
use math_traits::{BoundedAbove, Zero};

use std::{cmp::Reverse, collections::BinaryHeap, ops::Add};

pub struct Dijkstra<N, E> {
    graph: Graph<N, E>,
    pub dists: Vec<E>,
    backs: Vec<Option<usize>>,
}

impl<N, E> From<Graph<N, E>> for Dijkstra<N, E>
where
    N: Clone,
    E: Clone + BoundedAbove,
{
    fn from(graph: Graph<N, E>) -> Dijkstra<N, E> {
        let n = graph.len();
        let dists = vec![E::max(); n];
        let backs = vec![None; n];
        Dijkstra { graph, dists, backs }
    }
}

impl<N, E> Dijkstra<N, E>
where
    N: Clone,
    E: Copy + Eq + Ord + Zero + Add<Output = E>
{
    pub fn build(&mut self, start: usize) {
        let mut heap = BinaryHeap::new();
        self.dists[start] = E::zero();
        heap.push(Reverse((self.dists[start], start)));
        while !heap.is_empty() {
            let Reverse((d, v)) = heap.pop().unwrap();
            if self.dists[v] < d { continue; }
            for e in self.graph[v].iter() {
                if self.dists[e.to] > self.dists[v] + e.weight {
                    self.dists[e.to] = self.dists[v] + e.weight;
                    self.backs[e.to] = Some(v);
                    heap.push(Reverse((self.dists[e.to], e.to)))
                }
            }
        }
    }

    pub fn restore(&self, goal: usize) -> Vec<usize> {
        let mut path = vec![goal];
        let mut cur = self.backs[goal];
        if cur.is_none() { return vec![]; }
        while let Some(v) = cur {
            path.push(v);
            cur = self.backs[v];
        }
        path.reverse();
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::graph::SimpleGraph;

    #[test]
    fn test_dijkstra_1() {
        let mut g = SimpleGraph::new_directed(4);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 4);
        g.add_edge(1, 2, 2);
        g.add_edge(2, 3, 1);
        g.add_edge(1, 3, 5);
        let mut g: Dijkstra<_, _> = g.into();
        g.build(0);
        assert_eq!(g.dists, vec![0, 1, 3, 4]);
        assert_eq!(g.restore(3), vec![0, 1, 2, 3]);
        assert_eq!(g.restore(2), vec![0, 1, 2]);
    }

    #[test]
    fn test_dijkstra_2() {
        let mut g = SimpleGraph::new_directed(4);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 4);
        g.add_edge(2, 0, 1);
        g.add_edge(1, 2, 2);
        g.add_edge(3, 1, 1);
        g.add_edge(3, 2, 5);
        let mut g: Dijkstra<_, _> = g.into();
        g.build(1);
        assert_eq!(g.dists, vec![3, 0, 2, std::i32::MAX]);
        assert_eq!(g.restore(3), vec![]);
    }
}
