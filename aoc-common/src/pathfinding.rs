use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap},
    fmt::Debug,
    hash::Hash,
};

use rustc_hash::FxHashMap;

type ParentsMemo<N> = FxHashMap<N, (i64, Vec<N>)>;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct DijkstraNode<N>
where
    N: Eq,
{
    cost: i64,
    node: N,
}

impl<N> PartialOrd for DijkstraNode<N>
where
    N: Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<N> Ord for DijkstraNode<N>
where
    N: Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub fn dijkstra<N, FN, IN, FS>(start: &N, successors: FN, success: FS) -> Option<i64>
where
    N: Eq + Clone + Hash,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = (N, i64)>,
    FS: Fn(&N) -> bool,
{
    dijkstra_internal(start, successors, success).map(|(parents, target)| parents[&target].0)
}

pub fn dijkstra_with_paths<N, FN, IN, FS>(
    start: &N,
    successors: FN,
    success: FS,
) -> Option<(i64, Vec<Vec<N>>)>
where
    N: Eq + Clone + Hash,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = (N, i64)>,
    FS: Fn(&N) -> bool,
{
    dijkstra_internal(start, successors, success)
        .map(|(parents, target)| (parents[&target].0, build_paths(&parents, start, &target)))
}

fn build_paths<N>(parents: &ParentsMemo<N>, start: &N, target: &N) -> Vec<Vec<N>>
where
    N: Eq + Clone + Hash,
{
    if start == target {
        return vec![vec![target.clone()]];
    }

    if let Some((_, parent_nodes)) = parents.get(target) {
        parent_nodes
            .iter()
            .flat_map(|x| build_paths(parents, start, x))
            .map(|mut p| {
                p.push(target.clone());
                p
            })
            .collect()
    } else {
        Vec::default()
    }
}

fn dijkstra_internal<N, FN, IN, FS>(
    start: &N,
    successors: FN,
    success: FS,
) -> Option<(ParentsMemo<N>, N)>
where
    N: Eq + Clone + Hash,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = (N, i64)>,
    FS: Fn(&N) -> bool,
{
    let mut parents = FxHashMap::default();
    let mut q = BinaryHeap::default();
    q.push(DijkstraNode {
        cost: 0,
        node: start.clone(),
    });
    parents.insert(start.clone(), (0, Vec::default()));

    while let Some(DijkstraNode { cost, node }) = q.pop() {
        if success(&node) {
            return Some((parents, node));
        }

        for (n, c) in successors(&node) {
            let next_cost = cost + c;
            match parents.entry(n.clone()) {
                Entry::Occupied(mut e) => {
                    let (existing_cost, parent_nodes) = e.get_mut();
                    match next_cost.cmp(existing_cost) {
                        Ordering::Less => {
                            parent_nodes.clear();
                            parent_nodes.push(node.clone());
                            *existing_cost = next_cost;
                            q.push(DijkstraNode {
                                cost: next_cost,
                                node: n,
                            });
                        }
                        Ordering::Equal => {
                            parent_nodes.push(node.clone());
                        }
                        _ => (),
                    }
                }
                Entry::Vacant(e) => {
                    e.insert((next_cost, vec![node.clone()]));
                    q.push(DijkstraNode {
                        cost: next_cost,
                        node: n,
                    });
                }
            }
        }
    }

    None
}
