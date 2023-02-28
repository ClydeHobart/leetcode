pub struct Solution;

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    mem::take,
    ops::{Index, Range},
};

/// A struct describing a node's neighbor, and the edge connecting the two.
struct NeighborData {
    /// A node neighboring the node that this `NeighborData` is associated with.
    neighbor: u16,

    /// The edge connecting the node that this `NeighborData` is associated with and the `neighbor`
    /// node.
    edge: u16,
}

/// An intermediate struct used to construct a `Neighbors` instance.
#[derive(Clone)]
struct DoubleNeighborData {
    /// The "from" neighbor node, used to determine the range for this node.
    neighbor_u: u16,

    /// The "to" neighbor node, used for the `neighbor` field of `NeighborData`.
    neighbor_v: u16,

    /// The edge index, relative to `edge_states`.
    edge: u16,
}

impl DoubleNeighborData {
    /// Construct a pair of `DoubleNeighborData` instances based on the same edge: one for each
    /// direction.
    ///
    /// # Arguments
    ///
    /// * `edge`: The source data describing an edge: `edges[i] == [u_i, v_i, cnt_i]`.
    /// * `edge_states`: The current vector of `EdgeState`, used to initialize the `edge` field.
    fn new_pair(edge: &[i32], edge_states: &Vec<EdgeState>) -> [Self; 2_usize] {
        let neighbor_u: u16 = edge[0_usize] as u16;
        let neighbor_v: u16 = edge[1_usize] as u16;
        let edge: u16 = edge_states.len() as u16;

        [
            Self {
                neighbor_u,
                neighbor_v,
                edge,
            },
            Self {
                neighbor_u: neighbor_v,
                neighbor_v: neighbor_u,
                edge,
            },
        ]
    }
}

impl From<DoubleNeighborData> for NeighborData {
    fn from(value: DoubleNeighborData) -> Self {
        Self {
            neighbor: value.neighbor_v,
            edge: value.edge,
        }
    }
}

/// A struct for querying and iterating over neighbors of nodes.
struct Neighbors {
    /// A list of `NeighborData` elements indexed by ranges from the `ranges` field.
    neighbors: Vec<NeighborData>,

    /// A map of original nodes to the range of `NeighborData` elements associated to that node.
    ranges: HashMap<u16, Range<u16>>,
}

impl Neighbors {
    /// Construct a new, empty `Neighbors` instance.
    ///
    /// # Arguments
    ///
    /// * `capacity`: The capacity to allocate. This should be the total number of edges.
    fn with_capacity(capacity: usize) -> Self {
        Self {
            // Since each edge appears twice (once for each direction), allocate double `capacity`
            neighbors: Vec::with_capacity(capacity * 2_usize),

            // It's hard to say how much we'll need to allocate without an idea of the branching
            // factor, so just use an empty one.
            ranges: HashMap::new(),
        }
    }
}

impl Index<u16> for Neighbors {
    type Output = [NeighborData];

    fn index(&self, index: u16) -> &Self::Output {
        self.ranges
            .get(&index)
            // Default to an empty range at the start of `neighbors` for neighborless nodes
            .map_or(&self.neighbors[..0_usize], |range| {
                &self.neighbors[range.start as usize..range.end as usize]
            })
    }
}

/// Runtime state information about an edge.
struct EdgeState {
    /// The nodes between node `u_i` to node `v_i`. This is the same as `cnt_i`. This is immutable
    /// during the lifetime of the object.
    cnt: u16,

    /// The new nodes starting from `u_i` that have been explored. When the whole edge has been
    /// explored, even if starting from `v_i`, this will be `cnt_i`.
    u: u16,

    /// The new nodes starting from `v_i` that have been explored. When the whole edge has been
    /// explored, even if starting from `u_i`, this will be `cnt_i`.
    v: u16,
}

/// A struct describing the results of calling `EdgeState::explore`
struct ExploreResult {
    /// The count of new nodes that were explored.
    new_nodes: i32,

    /// Whether or not the other node is reachable with the provided distance.
    reaches_other_node: bool,
}

impl EdgeState {
    /// Construct a new, unexplored `EdgeState` instance
    fn new(cnt: i32) -> Self {
        Self {
            cnt: cnt as u16,
            u: 0_u16,
            v: 0_u16,
        }
    }

    /// Explore a certain distance along the edge, marking nodes along the edge as explored.
    ///
    /// # Arguments
    ///
    /// * `dist`: The distance to explore along the edge.
    /// * `from_u`: Whether or not the exploration is starting from the `u_i` node.
    fn explore(&mut self, dist: i32, from_u: bool) -> ExploreResult {
        let cnt: u16 = self.cnt;
        let edge_dist: u16 = dist.try_into().unwrap_or(u16::MAX).min(cnt);
        let (fwd, rev): (&mut u16, &mut u16) = if from_u {
            (&mut self.u, &mut self.v)
        } else {
            (&mut self.v, &mut self.u)
        };
        let (fwd_val, rev_val): (u16, u16) = (*fwd, *rev);
        let new_nodes: i32 = if edge_dist <= fwd_val {
            // No new nodes were explored.
            0_i32
        } else if edge_dist + rev_val < cnt {
            // Record the edge distance in the forward direction.
            *fwd = edge_dist;

            // The count of new nodes that were explored is the extra distance along the forward
            // direction.
            (edge_dist - fwd_val) as i32
        } else {
            // This fully explores the edge, so record both directions as fully explored.
            *fwd = cnt;
            *rev = cnt;

            // The count of new nodes that were explored is whatever hadn't been explored so far.
            (cnt - fwd_val - rev_val) as i32
        };

        // `dist` reaches the other node if it's at least `total()` (using the local copy of
        // `self.cnt`).
        let reaches_other_node: bool = dist >= cnt as i32 + 1_i32;

        ExploreResult {
            new_nodes,
            reaches_other_node,
        }
    }

    /// Return the total distance between the two end nodes of the edge
    fn total(&self) -> i32 {
        self.cnt as i32 + 1_i32
    }
}

/// A struct containing information regarding a node.
///
/// # "Pseudo-Fields"
///
/// * `dist: i32`: The current distance of the node from node 0. This should be kept up to date with
///     the `dist` field on the corresponding `QueueData` instance.
/// * `in_q: bool`: Whether or not the node has been removed from the queue. When `true`, the node
///     is within the queue; when `false`, the node has been removed from the queue.
///
/// # Explanation
///
/// In `Solution::reachable_nodes`, `NodeData` is used in `dist: HashMap<u16, NodeData>`. `HashMap`
/// (as of `rustc` v1.67) stores it's key-value pairs in a dynamically sized slice of tuples,
/// `(u16, NodeData)` in this case. `NodeData` needs to store a `bool` and an integer value in the
/// range `0..=1_000_000_000_i32`. Consider the following table:
///
/// | `T` | `size_of::<T>()` | `align_of::<T>()` |
/// | --- | --- | --- |
/// | `(u16, (bool, i32))` | `12_usize` | `4_usize` |
/// | `(u16, (i32, bool))` | `12_usize` | `4_usize` |
/// | `(u16, bool, i32)` | `8_usize` | `4_usize` |
/// | `(u16, u32)` | `8_usize` | `4_usize` |
///
/// Storing both the `bool` and `i32` separately would be ideal for the sake of clarity, but with
/// potentially 3000 of these being stored in the hashmap, it's best to keep this as slim as
/// possible (especially considering the memory ranking of LeetCode problems). The third type has
/// the better size and the clarity of composition, but unfortunately it doesn't work with the type
/// system. Luckily, the integer value only needs 30 bits, we can store the `bool` in the last bit
/// and the integer in the remaining bits.
#[derive(Clone, Copy)]
struct NodeData(u32);

impl NodeData {
    /// The bit index for the `in_q` value.
    const IN_Q_BIT: u32 = u32::BITS - 1_u32;

    /// The mask of bits for the `dist` value.
    const DIST_MASK: u32 = i32::MAX as u32;

    /// Construct a new `NodeData` instance with the given `dist` and `true` for `in_q`.
    fn new(dist: i32) -> Self {
        Self(!Self::DIST_MASK | (dist as u32))
    }

    /// Get the `dist` value.
    fn get_dist(self) -> i32 {
        (self.0 & Self::DIST_MASK) as i32
    }

    /// Get the `in_q` value.
    fn get_in_q(self) -> bool {
        (self.0 & !Self::DIST_MASK) != 0_u32
    }

    /// Set the `in_q` value.
    fn set_in_q(&mut self, in_q: bool) {
        self.0 = ((in_q as u32) << Self::IN_Q_BIT) | self.0 & Self::DIST_MASK;
    }
}

/// A struct containing data to be stored in the priority queue.
struct QueueData {
    /// The current distance of the node from node 0. This should be kept up to date with the `dist`
    /// "pseudo-field" of the corresponding `NodeData` instance.
    dist: i32,

    /// The node this `QueueData` instance refers to.
    node: u16,

    /// When this `QueueData` is in `updates`, this indicates whether or not the node this update is
    /// for is currently in the queue and data map. Thankfully, this doesn't increase the size of
    /// the struct.
    in_dist: bool,
}

impl PartialEq for QueueData {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl PartialOrd for QueueData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for QueueData {}

impl Ord for QueueData {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order so that cost is minimized when popping from the heap
        other.dist.cmp(&self.dist)
    }
}

impl Solution {
    /// Find the number of reachable nodes in the finished map
    ///
    /// # Constraints
    ///
    /// * `0_usize <= edges.len() && edges.len() as i32 <= min(n * (n - 1_i32) / 2_i32, 10_000_i32)`
    /// * `edges[i].len() == 3_usize`
    /// * `0_i32 <= u_i && u_i < v_i && v_i < n`
    /// * There are **no multiple edges** in the graph
    /// * `0_i32 <= cnt_i && cnt_i <= 10_000_i32`
    /// * `0_i32 <= max_moves && max_moves <= 1_000_000_000_i32`
    /// * `1 <= n && n <= 3_000_i32`
    ///
    /// # Acknowledgements
    ///
    /// This is a modification of the pseudocode on the [Wikipedia aritcle for Dijkstra's
    /// algorithm][article]
    ///
    /// [article]: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, _n: i32) -> i32 {
        let (neighbors, mut edge_states): (Neighbors, Vec<EdgeState>) =
            Self::build_neighbors_and_edge_states(edges);
        let mut dist: HashMap<u16, NodeData> = HashMap::new();
        let mut q: BinaryHeap<QueueData> = BinaryHeap::new();
        let mut reachable_nodes: i32 = 0_i32;
        let mut updates: Vec<QueueData> = Vec::new();

        dist.insert(0_u16, NodeData::new(0_i32));
        q.push(QueueData {
            dist: 0_i32,
            node: 0_u16,
            in_dist: true,
        });

        while let Some(u_queue_data) = q.pop() {
            let u: u16 = u_queue_data.node;
            let u_dist: i32 = dist
                .get_mut(&u)
                .map(|u_node_data| {
                    u_node_data.set_in_q(false);
                    u_node_data.get_dist()
                })
                .unwrap();

            // Increment `reachable_nodes` to account for `u`
            reachable_nodes += 1_i32;

            let dist_remaining: i32 = max_moves - u_dist;

            if dist_remaining == 0_i32 {
                // There's no further distance to explore from this node
                continue;
            }

            let u_neighbors: &[NeighborData] = &neighbors[u];

            updates.reserve(u_neighbors.len());

            for neighbor_data in u_neighbors.iter() {
                let v: u16 = neighbor_data.neighbor;
                let edge_state: &mut EdgeState = &mut edge_states[neighbor_data.edge as usize];
                let ExploreResult {
                    new_nodes,
                    reaches_other_node,
                } = edge_state.explore(dist_remaining, u < v);

                // Add the number of new nodes explored from this edge.
                reachable_nodes += new_nodes;

                // Only proceed if `v` is reached by the remaining distance.
                if reaches_other_node {
                    let (in_dist, v_node_data): (bool, NodeData) = dist
                        .get(&v)
                        .map_or((false, NodeData::new(0_i32)), |node_data| {
                            (true, *node_data)
                        });

                    // Only proceed if `v` is in `q` but hasn't been removed, or hasn't been in `q`
                    // yet (`NodeData::new` initializes the `in_q` bit to `true`).
                    if v_node_data.get_in_q() {
                        let alt_v_dist: i32 = u_dist + edge_state.total();

                        // If `v` isn't in `dist` yet or `alt_v_dist` is shorter
                        if !in_dist || alt_v_dist < v_node_data.get_dist() {
                            updates.push(QueueData {
                                dist: alt_v_dist,
                                node: v,
                                in_dist,
                            });
                        }
                    }
                }
            }

            // Add any updates from `u`'s neighbors.
            if !updates.is_empty() {
                let old_q_len: usize = q.len();

                // Take `q`'s data to manipulate it, then return it when done.
                let mut q_elements: Vec<QueueData> = take(&mut q).into_vec();

                for update in updates.drain(..) {
                    dist.insert(update.node, NodeData::new(update.dist));

                    if update.in_dist {
                        // If the update is in `dist`, it also was in `q`, and thus is in the first
                        // `old_q_len` elements of `q_elements`.
                        q_elements[..old_q_len]
                            .iter_mut()
                            .find(|queue_data| queue_data.node == update.node)
                            .unwrap()
                            .dist = update.dist;
                    } else {
                        q_elements.push(update);
                    }
                }

                // Re-establish `q` from `q_elements`
                q = q_elements.into();
            }
        }

        reachable_nodes
    }

    fn build_neighbors_and_edge_states(edges: Vec<Vec<i32>>) -> (Neighbors, Vec<EdgeState>) {
        let mut result: (Neighbors, Vec<EdgeState>) = (
            Neighbors::with_capacity(edges.len()),
            Vec::with_capacity(edges.len()),
        );
        let mut double_neighbors: Vec<DoubleNeighborData> =
            Vec::with_capacity(2_usize * edges.len());

        let (neighbors, edge_states) = &mut result;

        for edge in edges {
            double_neighbors.extend(DoubleNeighborData::new_pair(edge.as_slice(), edge_states));
            edge_states.push(EdgeState::new(edge[2_usize]));
        }

        double_neighbors.sort_by_key(|dnd| dnd.neighbor_u);

        let mut dnd_index: usize = 0_usize;

        while dnd_index < double_neighbors.len() {
            let neighbor_a: u16 = double_neighbors[dnd_index].neighbor_u;
            let next_dnd_index: usize = double_neighbors[dnd_index..]
                .iter()
                .position(|dnd| dnd.neighbor_u != neighbor_a)
                .map_or(double_neighbors.len(), |position| position + dnd_index);

            neighbors
                .ranges
                .insert(neighbor_a, dnd_index as u16..next_dnd_index as u16);
            neighbors.neighbors.extend(
                double_neighbors[dnd_index..next_dnd_index]
                    .iter()
                    .cloned()
                    .map(Into::into),
            );
            dnd_index = next_dnd_index;
        }

        result
    }
}

mod tests {
    use super::Solution;

    macro_rules! edges {
        [ $( [ $u_i:expr, $v_i:expr, $cnt_i:expr ] ),* ] => {
            vec![ $(
                vec![$u_i, $v_i, $cnt_i],
            )* ]
        };
    }

    /// Example 1 from the problem description
    #[test]
    fn example_1() {
        let edges: Vec<Vec<i32>> = edges![[0, 1, 10], [0, 2, 1], [1, 2, 2]];
        let max_moves: i32 = 6_i32;
        let n: i32 = 3_i32;

        assert_eq!(Solution::reachable_nodes(edges, max_moves, n), 13_i32);
    }

    /// Example 2 from the problem description
    #[test]
    fn example_2() {
        let edges: Vec<Vec<i32>> = edges![[0, 1, 4], [1, 2, 6], [0, 2, 8], [1, 3, 1]];
        let max_moves: i32 = 10_i32;
        let n: i32 = 4_i32;

        assert_eq!(Solution::reachable_nodes(edges, max_moves, n), 23_i32);
    }

    /// Example 3 from the problem description
    #[test]
    fn example_3() {
        let edges: Vec<Vec<i32>> = edges![[1, 2, 4], [1, 4, 5], [1, 3, 1], [2, 3, 4], [3, 4, 5]];
        let max_moves: i32 = 17_i32;
        let n: i32 = 5_i32;

        assert_eq!(Solution::reachable_nodes(edges, max_moves, n), 1_i32);
    }

    /// Example to make sure sub-nodes along edges aren't double counted
    #[test]
    fn example_4() {
        let edges: Vec<Vec<i32>> = edges![[0, 1, 3], [0, 2, 2], [1, 2, 4]];
        let max_moves: i32 = 6_i32;
        let n: i32 = 3_i32;

        assert_eq!(Solution::reachable_nodes(edges, max_moves, n), 12_i32);
    }
}
