use std::{cmp::Ordering, ops::{Index, IndexMut}};

pub type SimpleGraph<E> = Graph<(), E>;

#[derive(Debug, Clone)]
pub enum Graph<N, E> {
    Directed(DirectedGraph<N, E>),
    Undirected(UndirectedGraph<N, E>),
}

#[derive(Debug, Clone)]
pub struct DirectedGraph<N, E> {
    nodes: Vec<Node<N>>,
    edges: Vec<Vec<Edge<E>>>,
    inv: Vec<Vec<Edge<E>>>,
}

#[derive(Debug, Clone)]
pub struct UndirectedGraph<N, E> {
    nodes: Vec<Node<N>>,
    edges: Vec<Vec<Edge<E>>>,
}

#[derive(Debug, Clone)]
pub struct Node<N> {
    pub weight: N,
}

#[derive(Debug, Clone)]
pub struct Edge<E> {
    pub to: usize,
    pub weight: E,
}

impl<N, E> Graph<N, E>
where
    N: Clone + Default,
    E: Clone,
{
    pub fn new_undirected(size: usize) -> Graph<N, E> {
        let g = UndirectedGraph::new(size);
        Graph::Undirected(g)
    }

    pub fn new_directed(size: usize) -> Graph<N, E> {
        let g = DirectedGraph::new(size);
        Graph::Directed(g)
    }
}

impl<N, E> Graph<N, E>
where
    N: Clone,
    E: Clone,
{
    pub fn is_directed(&self) -> bool {
        use Graph::*;
        match self {
            Undirected(_) => false,
            Directed(_) => true,
        }
    }

    pub fn is_undirected(&self) -> bool {
        use Graph::*;
        match self {
            Undirected(_) => true,
            Directed(_) => false,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: E) {
        use Graph::*;
        match self {
            Undirected(g) => g.add_edge(from, to, weight),
            Directed(g) => g.add_edge(from, to, weight),
        }
    }

    pub fn node_weight(&self, index: usize) -> Option<&N> {
        use Graph::*;
        match self {
            Undirected(g) => g.node_weight(index),
            Directed(g) => g.node_weight(index),
        }
    }

    pub fn node_weight_mut(&mut self, index: usize) -> Option<&mut N> {
        use Graph::*;
        match self {
            Undirected(g) => g.node_weight_mut(index),
            Directed(g) => g.node_weight_mut(index),
        }
    }

    pub fn len(&self) -> usize {
        use Graph::*;
        match self {
            Undirected(g) => g.len(),
            Directed(g) => g.len(),
        }
    }
}

impl<N, E> Index<usize> for Graph<N, E> {
    type Output = Vec<Edge<E>>;
    fn index(&self, index: usize) -> &Self::Output {
        use Graph::*;
        match self {
            Undirected(g) => g.index(index),
            Directed(g) => g.index(index),
        }
    }
}

impl<N, E> IndexMut<usize> for Graph<N, E>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        use Graph::*;
        match self {
            Undirected(g) => g.index_mut(index),
            Directed(g) => g.index_mut(index),
        }
    }
}

impl<N, E> UndirectedGraph<N, E>
where
    N: Clone + Default,
    E: Clone,
{
    pub fn new(size: usize) -> UndirectedGraph<N, E> {
        let nodes = vec![Default::default(); size];
        let edges = vec![vec![]; size];
        Self { nodes, edges }
    }
}

impl<N, E> From<Vec<Node<N>>> for UndirectedGraph<N, E>
where
    N: Clone,
    E: Clone,
{
    fn from(nodes: Vec<Node<N>>) -> Self {
        let size = nodes.len();
        let edges = vec![vec![]; size];
        Self { nodes, edges }
    }
}

impl<N, E> UndirectedGraph<N, E>
where
    N: Clone,
    E: Clone,
{
    pub fn add_edge(&mut self, from: usize, to: usize, weight: E) {
        let edge = Edge::new(to, weight.clone());
        self[from].push(edge);
        let edge = Edge::new(from, weight);
        self[to].push(edge);
    }

    pub fn node_weight(&self, index: usize) -> Option<&N> {
        self.nodes.get(index).map(|n| &n.weight)
    }

    pub fn node_weight_mut(&mut self, index: usize) -> Option<&mut N> {
        self.nodes.get_mut(index).map(|n| &mut n.weight)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl<N, E> Index<usize> for UndirectedGraph<N, E> {
    type Output = Vec<Edge<E>>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}

impl<N, E> IndexMut<usize> for UndirectedGraph<N, E>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.edges[index]
    }
}

impl<N, E> DirectedGraph<N, E>
where
    N: Clone + Default,
    E: Clone,
{
    pub fn new(size: usize) -> DirectedGraph<N, E> {
        let nodes = vec![Default::default(); size];
        let edges = vec![vec![]; size];
        let inv = vec![vec![]; size];
        Self { nodes, edges, inv }
    }
}

impl<N, E> From<Vec<Node<N>>> for DirectedGraph<N, E>
where
    N: Clone,
    E: Clone,
{
    fn from(nodes: Vec<Node<N>>) -> Self {
        let size = nodes.len();
        let edges = vec![vec![]; size];
        let inv = vec![vec![]; size];
        Self { nodes, edges, inv }
    }
}

impl<N, E> DirectedGraph<N, E>
where
    N: Clone,
    E: Clone,
{
    pub fn add_edge(&mut self, from: usize, to: usize, weight: E) {
        let edge = Edge::new(to, weight.clone());
        self[from].push(edge);
        let edge = Edge::new(from, weight);
        self.inv[to].push(edge);
    }

    pub fn node_weight(&self, index: usize) -> Option<&N> {
        self.nodes.get(index).map(|n| &n.weight)
    }

    pub fn node_weight_mut(&mut self, index: usize) -> Option<&mut N> {
        self.nodes.get_mut(index).map(|n| &mut n.weight)
    }
    
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl<N, E> Index<usize> for DirectedGraph<N, E> {
    type Output = Vec<Edge<E>>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}

impl<N, E> IndexMut<usize> for DirectedGraph<N, E>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.edges[index]
    }
}

impl<N> Node<N> {
    fn new(weight: N) -> Node<N> {
        Node { weight }
    }
}

impl<N: Default> Default for Node<N> {
    fn default() -> Node<N> {
        Node::new(N::default())
    }
}

impl<E> Edge<E> {
    pub fn new(to: usize, weight: E) -> Edge<E> {
        Edge { to, weight }
    }
}

impl<E: PartialEq> PartialEq for Edge<E> {
    fn eq(&self, other: &Self) -> bool {
        self.weight.eq(&other.weight)
    }
}
impl<E: PartialOrd> PartialOrd for Edge<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<E: Eq> Eq for Edge<E> {}
impl<E: Ord> Ord for Edge<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}