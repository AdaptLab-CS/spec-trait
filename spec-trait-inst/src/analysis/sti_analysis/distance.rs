use std::collections::HashMap;
use std::fmt::Debug;

/// A simple tree node structure.
#[derive(Debug, PartialEq, Eq)]
pub struct Node<T: Eq + Debug> {
    pub label: T,
    pub children: Vec<Node<T>>,
}

impl<T: Eq + Debug> Node<T> {
    pub fn size(&self) -> usize {
        1 + self.children.iter().map(|c| c.size()).sum::<usize>()
    }
}

impl<T: Eq + Debug> From<&Node<T>> for usize {
    fn from(t: &Node<T>) -> Self {
        t as *const _ as usize
    }
}

/// Post-order traversal and left-most indices.
struct PostOrder<'a, T: Eq + Debug> {
    nodes: Vec<&'a Node<T>>,
    leftmost: Vec<usize>,
}

type TreeDistanceTable = HashMap<(usize, usize), usize>;
type ForestDistanceTable = Vec<Vec<usize>>;

/// Computes the similarity between two trees based on their edit distance.
pub fn compute_similarity<T: Eq + Debug>(a: &Node<T>, b: &Node<T>) -> f64 {
    let dist = compute_tree_distance(a, b) as f64;
    let max_size = a.size().max(b.size()) as f64;
    1.0 - (dist / max_size)
}

/// Computes the zss edit distance between two trees.
pub fn compute_tree_distance<T: Eq + Debug>(a: &Node<T>, b: &Node<T>) -> usize {
    let post_a = post_order_and_leftmost(a);
    let post_b = post_order_and_leftmost(b);

    // handle empty trees quickly
    if post_a.nodes.len() == 0 {
        return post_b.nodes.len(); // all insertions
    }
    if post_b.nodes.len() == 0 {
        return post_a.nodes.len(); // all deletions
    }

    // compute key-roots: last index for each leftmost value
    let kr_a = get_key_roots(&post_a.leftmost);
    let kr_b = get_key_roots(&post_b.leftmost);

    // tree distances, indexed by post-order indices
    let mut treedist = TreeDistanceTable::new();

    // iteratively compute key-root distances
    for &ia in &kr_a {
        for &ib in &kr_b {
            compute_key_root_distance(ia, ib, &post_a, &post_b, &mut treedist);
        }
    }

    // the index of the root in post order is the last index of the list
    let root_idx_a = post_a.nodes.len() - 1;
    let root_idx_b = post_b.nodes.len() - 1;
    *treedist.get(&(root_idx_a, root_idx_b)).unwrap()
}

/// Collect post-order and left-most for a tree.
fn post_order_and_leftmost<'a, T: Eq + Debug>(root: &'a Node<T>) -> PostOrder<'a, T> {
    let mut nodes = Vec::new();
    collect_post_order(root, &mut nodes);

    let n = nodes.len();

    // index of the node in post order by its pointer
    let mut ptr_to_idx = HashMap::with_capacity(n);
    for (i, nd) in nodes.iter().enumerate() {
        ptr_to_idx.insert(usize::from(*nd), i);
    }

    let mut leftmost = vec![0usize; n];
    for (i, node) in nodes.iter().enumerate() {
        if node.children.is_empty() {
            leftmost[i] = i;
        } else {
            let first_child = &node.children[0];
            let child_post_idx = ptr_to_idx[&first_child.into()];
            leftmost[i] = leftmost[child_post_idx];
        }
    }

    PostOrder { nodes, leftmost }
}

/// Collect nodes in post order.
fn collect_post_order<'a, T: Eq + Debug>(n: &'a Node<T>, out: &mut Vec<&'a Node<T>>) {
    for c in &n.children {
        collect_post_order(c, out);
    }
    out.push(n);
}

/// Collect key-roots (last occurrence of each left-most).
fn get_key_roots(left_most: &[usize]) -> Vec<usize> {
    let mut last = HashMap::with_capacity(left_most.len());
    for (i, &lm) in left_most.iter().enumerate() {
        last.insert(lm, i);
    }
    let mut keyroots_a = last.values().copied().collect::<Vec<_>>();
    keyroots_a.sort_unstable();
    keyroots_a
}

/// Computes the tree distance for the subtrees rooted at kr_a and kr_b.
fn compute_key_root_distance<T: Eq + Debug>(
    kr_a: usize,
    kr_b: usize,
    post_a: &PostOrder<T>,
    post_b: &PostOrder<T>,
    td: &mut TreeDistanceTable,
) {
    let kr_a_leftmost = post_a.leftmost[kr_a];
    let kr_b_leftmost = post_b.leftmost[kr_b];

    let n = kr_a - kr_a_leftmost + 1; // size of forest in A
    let m = kr_b - kr_b_leftmost + 1; // size of forest in B

    let mut fd = init_forest_distance(n, m);

    for i in 1..=n {
        for j in 1..=m {
            let post_a_i = kr_a_leftmost + i - 1; // index in post-order in A
            let post_b_i = kr_b_leftmost + j - 1; // index in post-order in B

            let td_key = (post_a_i, post_b_i); // key for tree distance table

            let left_a_i = post_a.leftmost[post_a_i]; // leftmost of current index in A
            let left_b_i = post_b.leftmost[post_b_i]; // leftmost of current index in B

            // both node_a and node_b are roots of their respective (sub)trees in this forest
            if left_a_i == kr_a_leftmost && left_b_i == kr_b_leftmost {
                let node_a = post_a.nodes[post_a_i]; // node at current index in A
                let node_b = post_b.nodes[post_b_i]; // node at current index in B

                let remove = remove_cost(&fd, i, j);
                let insert = insert_cost(&fd, i, j);
                let relabel = relabel_cost(node_a, node_b);
                let match_ = fd[i - 1][j - 1] + relabel;
                let val = remove.min(insert).min(match_);

                fd[i][j] = val;
                td.insert(td_key, val);
            } else {
                let pre_a = left_a_i - kr_a_leftmost; // nodes before current subtree in A
                let pre_b = left_b_i - kr_b_leftmost; // nodes before current subtree in B

                let remove = remove_cost(&fd, i, j);
                let insert = insert_cost(&fd, i, j);
                let sub_forest_dist = *td.get(&td_key).unwrap();
                let match_ = fd[pre_a][pre_b] + sub_forest_dist;

                fd[i][j] = remove.min(insert).min(match_);
            }
        }
    }
}

/// Initializes the forest distance table with base cases.
fn init_forest_distance(n: usize, m: usize) -> ForestDistanceTable {
    let mut fd = vec![vec![0; m + 1]; n + 1];
    fd.iter_mut().enumerate().for_each(|(i, row)| row[0] = i); // deletion
    fd[0].iter_mut().enumerate().for_each(|(j, cost)| *cost = j); // insertion
    fd
}

/// Computes the cost of deleting a node.
fn remove_cost(fd: &ForestDistanceTable, i: usize, j: usize) -> usize {
    let prev = fd[i - 1][j];
    let cost = 1;
    prev + cost
}

/// Computes the cost of inserting a node.
fn insert_cost(fd: &ForestDistanceTable, i: usize, j: usize) -> usize {
    let prev = fd[i][j - 1];
    let cost = 1;
    prev + cost
}

/// Computes the cost of relabeling a node.
fn relabel_cost<T: Eq + Debug>(a: &Node<T>, b: &Node<T>) -> usize {
    let needs_relabel = a.label != b.label;
    let cost = 1;
    if needs_relabel {
        cost
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Eq, Debug)]
    enum Label {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
    }

    #[test]
    fn compute_distance_equal() {
        // Tree 1 structure:
        //      A
        //    /   \
        //   B     C
        let tree1 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::B,
                    children: vec![],
                },
                Node {
                    label: Label::C,
                    children: vec![],
                },
            ],
        };

        // Tree 2 structure:
        //      A
        //    /   \
        //   B     C
        let tree2 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::B,
                    children: vec![],
                },
                Node {
                    label: Label::C,
                    children: vec![],
                },
            ],
        };

        let distance = compute_tree_distance(&tree1, &tree2);
        let similarity = compute_similarity(&tree1, &tree2);
        assert_eq!(distance, 0);
        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn compute_distance_insertion() {
        // Tree 1 structure:
        //      A
        //    /   \
        //   B     C
        let tree1 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::B,
                    children: vec![],
                },
                Node {
                    label: Label::C,
                    children: vec![],
                },
            ],
        };

        // Tree 2 structure:
        //      A
        //    / | \
        //   B  C  D
        let tree2 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::B,
                    children: vec![],
                },
                Node {
                    label: Label::C,
                    children: vec![],
                },
                Node {
                    label: Label::D,
                    children: vec![],
                },
            ],
        };

        let distance = compute_tree_distance(&tree1, &tree2);
        let similarity = compute_similarity(&tree1, &tree2);
        assert_eq!(distance, 1);
        assert_eq!(similarity, 0.75);
    }

    #[test]
    fn compute_distance_insertion_with_sub_tree() {
        // Tree 1 structure:
        //      A
        //      |
        //      C
        let tree1 = Node {
            label: Label::A,
            children: vec![Node {
                label: Label::C,
                children: vec![],
            }],
        };

        // Tree 2 structure:
        //      A
        //    /   \
        //   B     C
        //   |
        //   D
        let tree2 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::B,
                    children: vec![Node {
                        label: Label::D,
                        children: vec![],
                    }],
                },
                Node {
                    label: Label::C,
                    children: vec![],
                },
            ],
        };

        let distance = compute_tree_distance(&tree1, &tree2);
        let similarity = compute_similarity(&tree1, &tree2);
        assert_eq!(distance, 2);
        assert_eq!(similarity, 0.5);
    }

    #[test]
    fn compute_distance_deletion() {
        // Tree 1 structure:
        //      A
        //    / | \
        //   B  C  D
        let tree1 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::B,
                    children: vec![],
                },
                Node {
                    label: Label::C,
                    children: vec![],
                },
                Node {
                    label: Label::D,
                    children: vec![],
                },
            ],
        };

        // Tree 2 structure:
        //      A
        //    /   \
        //   B     C
        let tree2 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::B,
                    children: vec![],
                },
                Node {
                    label: Label::C,
                    children: vec![],
                },
            ],
        };

        let distance = compute_tree_distance(&tree1, &tree2);
        let similarity = compute_similarity(&tree1, &tree2);
        assert_eq!(distance, 1);
        assert_eq!(similarity, 0.75);
    }

    #[test]
    fn compute_distance_deletion_root() {
        // Tree 1 structure:
        //      A
        //      |
        //      B
        //      |
        //      C
        let tree1 = Node {
            label: Label::A,
            children: vec![Node {
                label: Label::B,
                children: vec![Node {
                    label: Label::C,
                    children: vec![],
                }],
            }],
        };

        // Tree 2 structure:
        //      B
        //      |
        //      C
        let tree2 = Node {
            label: Label::B,
            children: vec![Node {
                label: Label::C,
                children: vec![],
            }],
        };

        let distance = compute_tree_distance(&tree1, &tree2);
        let similarity = compute_similarity(&tree1, &tree2);
        assert_eq!(distance, 1);
        assert!((similarity - 0.667).abs() < 0.001);
    }

    #[test]
    fn compute_distance_deletion_with_sub_tree() {
        // Tree 1 structure:
        //      A
        //    /   \
        //   B     C
        //   |
        //   D
        let tree1 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::B,
                    children: vec![Node {
                        label: Label::D,
                        children: vec![],
                    }],
                },
                Node {
                    label: Label::C,
                    children: vec![],
                },
            ],
        };

        // Tree 2 structure:
        //      A
        //    /   \
        //   D     C
        let tree2 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::D,
                    children: vec![],
                },
                Node {
                    label: Label::C,
                    children: vec![],
                },
            ],
        };

        let distance = compute_tree_distance(&tree1, &tree2);
        let similarity = compute_similarity(&tree1, &tree2);
        assert_eq!(distance, 1);
        assert_eq!(similarity, 0.75);
    }

    #[test]
    fn compute_distance_relabel() {
        // Tree 1 structure:
        //      A
        //    /   \
        //   B     C
        let tree1 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::B,
                    children: vec![],
                },
                Node {
                    label: Label::C,
                    children: vec![],
                },
            ],
        };

        // Tree 2 structure:
        //      A
        //    /   \
        //   B     D
        let tree2 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::B,
                    children: vec![],
                },
                Node {
                    label: Label::D,
                    children: vec![],
                },
            ],
        };

        let distance = compute_tree_distance(&tree1, &tree2);
        let similarity = compute_similarity(&tree1, &tree2);
        assert_eq!(distance, 1);
        assert!((similarity - 0.667).abs() < 0.001);
    }

    #[test]
    fn compute_distance_relabel_with_sub_tree() {
        // Tree 1 structure:
        //      A
        //    /   \
        //   B     C
        let tree1 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::B,
                    children: vec![],
                },
                Node {
                    label: Label::C,
                    children: vec![],
                },
            ],
        };

        // Tree 2 structure:
        //      D
        //    /   \
        //   E     F
        let tree2 = Node {
            label: Label::D,
            children: vec![
                Node {
                    label: Label::E,
                    children: vec![],
                },
                Node {
                    label: Label::F,
                    children: vec![],
                },
            ],
        };

        let distance = compute_tree_distance(&tree1, &tree2);
        let similarity = compute_similarity(&tree1, &tree2);
        assert_eq!(distance, 3);
        assert_eq!(similarity, 0.0);
    }

    #[test]
    fn compute_distance_deep() {
        // Tree 1 structure:
        //      A
        //    /   \
        //   B     C
        //   |     |
        //   D     E
        //  / \    |
        // H   F   I
        let tree1 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::B,
                    children: vec![Node {
                        label: Label::D,
                        children: vec![
                            Node {
                                label: Label::H,
                                children: vec![],
                            },
                            Node {
                                label: Label::F,
                                children: vec![],
                            },
                        ],
                    }],
                },
                Node {
                    label: Label::C,
                    children: vec![Node {
                        label: Label::E,
                        children: vec![Node {
                            label: Label::I,
                            children: vec![],
                        }],
                    }],
                },
            ],
        };

        // Tree 2 structure:
        //      A
        //    /   \
        //   B     E
        //   |     |
        //   G     I
        //  / \
        // H   C
        let tree2 = Node {
            label: Label::A,
            children: vec![
                Node {
                    label: Label::B,
                    children: vec![Node {
                        label: Label::G,
                        children: vec![
                            Node {
                                label: Label::H,
                                children: vec![],
                            },
                            Node {
                                label: Label::C,
                                children: vec![],
                            },
                        ],
                    }],
                },
                Node {
                    label: Label::E,
                    children: vec![Node {
                        label: Label::I,
                        children: vec![],
                    }],
                },
            ],
        };

        let distance = compute_tree_distance(&tree1, &tree2);
        let similarity = compute_similarity(&tree1, &tree2);
        assert_eq!(distance, 3);
        assert!((similarity - 0.625).abs() < 0.001);
    }
}
