extern crate __procon_graph as graph;
extern crate __procon_math_traits as math_traits;

use graph::{Graph, MatrixGraph};
use math_traits::{BoundedAbove, Zero};

use std::{
    cmp::{min, Reverse},
    collections::BinaryHeap,
    ops::{Add, Index, IndexMut},
};

pub struct Dijkstra<N, E> {
    graph: Graph<N, E>,
    pub dists: Vec<E>,
    pub backs: Vec<Option<usize>>,
}

impl<N, E> From<Graph<N, E>> for Dijkstra<N, E>
where
    N: Clone,
    E: Clone + BoundedAbove,
{
    fn from(graph: Graph<N, E>) -> Dijkstra<N, E> {
        let n = graph.len();
        let dists = vec![E::maximum(); n];
        let backs = vec![None; n];
        Dijkstra {
            graph,
            dists,
            backs,
        }
    }
}

impl<N, E> Dijkstra<N, E>
where
    N: Clone,
    E: Copy + Eq + Ord + Zero + Add<Output = E>,
{
    pub fn build(&mut self, start: usize) {
        let mut heap = BinaryHeap::new();
        self.dists[start] = E::zero();
        heap.push(Reverse((self.dists[start], start)));
        while !heap.is_empty() {
            let Reverse((d, v)) = heap.pop().unwrap();
            if self.dists[v] < d {
                continue;
            }
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
        if cur.is_none() {
            return vec![];
        }
        while let Some(v) = cur {
            path.push(v);
            cur = self.backs[v];
        }
        path.reverse();
        path
    }
}

pub struct BellmanFord<N, E> {
    graph: Graph<N, E>,
    pub dists: Vec<E>,
    backs: Vec<Option<usize>>,
}

impl<N, E> From<Graph<N, E>> for BellmanFord<N, E>
where
    N: Clone,
    E: Clone + BoundedAbove,
{
    fn from(graph: Graph<N, E>) -> BellmanFord<N, E> {
        let n = graph.len();
        let dists = vec![E::maximum(); n];
        let backs = vec![None; n];
        BellmanFord {
            graph,
            dists,
            backs,
        }
    }
}

impl<N, E> BellmanFord<N, E>
where
    N: Clone,
    E: Copy + Eq + Ord + Zero + Add<Output = E> + BoundedAbove,
{
    pub fn build(&mut self, start: usize) -> bool {
        self.dists[start] = E::zero();
        let n = self.dists.len();
        for i in 0..n {
            let mut updated = false;
            for from in 0..n {
                let edges = &self.graph[from];
                for edge in edges.iter() {
                    if self.dists[from] != E::maximum()
                        && self.dists[edge.to] > self.dists[from] + edge.weight
                    {
                        self.dists[edge.to] = self.dists[from] + edge.weight;
                        self.backs[edge.to] = Some(from);
                        updated = true;
                    }
                }
            }
            if !updated {
                break;
            }
            if i == n - 1 {
                return false;
            }
        }
        true
    }

    pub fn restore(&self, goal: usize) -> Vec<usize> {
        let mut path = vec![goal];
        let mut cur = self.backs[goal];
        if cur.is_none() {
            return vec![];
        }
        while let Some(v) = cur {
            path.push(v);
            cur = self.backs[v];
        }
        path.reverse();
        path
    }
}

#[derive(Debug, Clone)]
pub struct WarshallFloyd<E>(MatrixGraph<E>);

impl<E> From<MatrixGraph<E>> for WarshallFloyd<E> {
    fn from(g: MatrixGraph<E>) -> Self {
        WarshallFloyd(g)
    }
}

impl<E> Index<usize> for WarshallFloyd<E> {
    type Output = Vec<E>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<E> IndexMut<usize> for WarshallFloyd<E> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<E> WarshallFloyd<E>
where
    E: Copy + Eq + Ord + BoundedAbove + Add<Output = E> + Zero,
{
    pub fn build(&mut self) {
        let n = self.0.len();
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if self[i][k] != E::maximum() && self[k][j] != E::maximum() {
                        self[i][j] = min(self[i][j], self[i][k] + self[k][j]);
                    }
                }
            }
        }
    }

    pub fn has_negative_cycle(&self) -> bool {
        let n = self.0.len();
        (0..n).any(|i| self[i][i] < E::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::graph::SimpleGraph;
    use super::*;

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

    #[test]
    fn test_bellmanford_1() {
        let mut g = SimpleGraph::new_directed(4);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 4);
        g.add_edge(1, 2, 2);
        g.add_edge(2, 3, 1);
        g.add_edge(1, 3, 5);
        let mut g = BellmanFord::from(g);
        g.build(0);
        assert_eq!(g.dists, vec![0, 1, 3, 4]);
        assert_eq!(g.restore(3), vec![0, 1, 2, 3]);
        assert_eq!(g.restore(2), vec![0, 1, 2]);
    }

    #[test]
    fn test_bellmanford_2() {
        let mut g = SimpleGraph::new_directed(4);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 4);
        g.add_edge(2, 0, 1);
        g.add_edge(1, 2, 2);
        g.add_edge(3, 1, 1);
        g.add_edge(3, 2, 5);
        let mut g = BellmanFord::from(g);
        g.build(1);
        assert_eq!(g.dists, vec![3, 0, 2, std::i32::MAX]);
        assert_eq!(g.restore(3), vec![]);
    }

    #[test]
    fn test_bellmanford_with_negative_edge() {
        let mut g = SimpleGraph::new_directed(6);
        g.add_edge(0, 1, 2);
        g.add_edge(1, 2, 3);
        g.add_edge(0, 3, 4);
        g.add_edge(2, 3, -2);
        g.add_edge(2, 5, 2);
        g.add_edge(3, 5, 4);
        g.add_edge(3, 4, 2);
        g.add_edge(4, 5, 1);
        let mut g = BellmanFord::from(g);
        assert!(g.build(0));
        assert_eq!(g.dists, vec![0, 2, 5, 3, 5, 6]);
        assert_eq!(g.restore(5), vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bellmanford_with_negative_circuit() {
        let mut g = SimpleGraph::new_directed(6);
        g.add_edge(0, 1, 2);
        g.add_edge(1, 2, 3);
        g.add_edge(0, 3, 4);
        g.add_edge(2, 3, -2);
        g.add_edge(2, 5, 2);
        g.add_edge(3, 5, 4);
        g.add_edge(3, 4, 2);
        g.add_edge(4, 5, 1);
        g.add_edge(3, 1, -2);
        let mut g = BellmanFord::from(g);
        assert!(!g.build(0));
    }

    #[test]
    fn test_warshall_floyd() {
        let mut g = MatrixGraph::new(4);
        g[0][1] = 1;
        g[0][2] = 5;
        g[1][2] = 2;
        g[1][3] = 4;
        g[2][3] = 1;
        g[3][2] = 7;
        let mut g = WarshallFloyd::from(g);
        g.build();
        let inf = std::i32::MAX;
        assert_eq!(g[0], vec![0, 1, 3, 4]);
        assert_eq!(g[1], vec![inf, 0, 2, 3]);
        assert_eq!(g[2], vec![inf, inf, 0, 1]);
        assert_eq!(g[3], vec![inf, inf, 7, 0]);
    }

    #[test]
    fn test_warshall_floyd_with_negative_cycle() {
        let mut g = MatrixGraph::new(4);
        g[0][1] = 1;
        g[0][2] = 5;
        g[1][2] = 2;
        g[1][3] = 4;
        g[2][3] = 1;
        g[3][2] = -7;
        let mut g = WarshallFloyd::from(g);
        g.build();
        assert!(g.has_negative_cycle());
    }
}
