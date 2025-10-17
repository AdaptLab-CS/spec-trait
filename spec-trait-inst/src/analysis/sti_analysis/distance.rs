use std::collections::HashMap;

/// A simple tree node structure.
pub struct Node<T: Eq> {
    pub label: T,
    pub children: Vec<Node<T>>,
}

impl<T: Eq> Node<T> {
    pub fn size(&self) -> usize {
        1 + self.children.iter().map(|c| c.size()).sum::<usize>()
    }
}

impl<T: Eq> From<&Node<T>> for usize {
    fn from(t: &Node<T>) -> Self {
        t as *const _ as usize
    }
}

/// A keyroot in a tree, consisting of a node and its sub-forest.
struct KeyRoot<'a, T: Eq> {
    node: &'a Node<T>,
    sub_forest: Vec<&'a Node<T>>,
}

type TreeDistanceTable = HashMap<(usize, usize), usize>;
type ForestDistanceTable = Vec<Vec<usize>>;

/// Computes the similarity between two trees based on their edit distance.
pub fn compute_similarity<T: Eq>(a: &Node<T>, b: &Node<T>) -> f64 {
    let dist = compute_distance(a, b) as f64;
    let max_size = a.size().max(b.size()) as f64;
    1.0 - (dist / max_size)
}

/// Computes the zss edit distance between two trees.
pub fn compute_distance<T: Eq>(a: &Node<T>, b: &Node<T>) -> usize {
    let mut td = TreeDistanceTable::new();
    tree_distance(a, b, &mut td)
}

/// Computes the tree distance between two trees.
fn tree_distance<T: Eq>(a: &Node<T>, b: &Node<T>, td: &mut TreeDistanceTable) -> usize {
    let key = (a.into(), b.into());

    // check if distance already computed
    if let Some(&dist) = td.get(&key) {
        return dist;
    }

    // compute distance using keyroots
    let kr_a = get_key_roots(a);
    let kr_b = get_key_roots(b);

    let dist = forest_distance_keyroots(&kr_a, &kr_b, td);
    td.insert(key, dist);

    dist
}

/// Extracts keyroots from a tree.
fn get_key_roots<'a, T: Eq>(node: &'a Node<T>) -> Vec<KeyRoot<'a, T>> {
    get_key_roots_recursive(node)
}

/// Recursively collects keyroots from the tree.
fn get_key_roots_recursive<'a, T: Eq>(node: &'a Node<T>) -> Vec<KeyRoot<'a, T>> {
    let mut key_roots = vec![];

    // traverse children first (post-order traversal)
    for child in &node.children {
        key_roots.extend(get_key_roots_recursive(child));
    }
    key_roots.push(KeyRoot {
        node,
        sub_forest: vec![],
    });

    key_roots
}

/// Computes the distance between two forests using their keyroots.
fn forest_distance_keyroots<T: Eq>(
    kr1: &[KeyRoot<T>],
    kr2: &[KeyRoot<T>],
    td: &mut TreeDistanceTable,
) -> usize {
    compute_forest_distance(kr1.len(), kr2.len(), |i, j| {
        let kr_1 = &kr1[i];
        let kr_2 = &kr2[j];

        let relabel = relabel_cost(kr_1.node, kr_2.node);
        let sub_forest_dist = forest_distance(&kr_1.sub_forest, &kr_2.sub_forest, td);

        relabel + sub_forest_dist
    })
}

/// Computes the distance between two forests.
fn forest_distance<T: Eq>(f1: &[&Node<T>], f2: &[&Node<T>], td: &mut TreeDistanceTable) -> usize {
    compute_forest_distance(f1.len(), f2.len(), |i, j| tree_distance(f1[i], f2[j], td))
}

/// Unified DP computation for forest distance.
fn compute_forest_distance<F>(n: usize, m: usize, mut match_cost_fn: F) -> usize
where
    F: FnMut(usize, usize) -> usize,
{
    let mut fd = init_forest_distance(n, m);

    for i in 1..=n {
        for j in 1..=m {
            let remove = remove_cost(&fd, i, j);
            let insert = insert_cost(&fd, i, j);
            let match_ = fd[i - 1][j - 1] + match_cost_fn(i - 1, j - 1);
            fd[i][j] = remove.min(insert).min(match_);
        }
    }

    fd[n][m]
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
fn relabel_cost<T: Eq>(a: &Node<T>, b: &Node<T>) -> usize {
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

    #[derive(PartialEq, Eq)]
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

        let distance = compute_distance(&tree1, &tree2);
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

        let distance = compute_distance(&tree1, &tree2);
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

        let distance = compute_distance(&tree1, &tree2);
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

        let distance = compute_distance(&tree1, &tree2);
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

        let distance = compute_distance(&tree1, &tree2);
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

        let distance = compute_distance(&tree1, &tree2);
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

        let distance = compute_distance(&tree1, &tree2);
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

        let distance = compute_distance(&tree1, &tree2);
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

        let distance = compute_distance(&tree1, &tree2);
        let similarity = compute_similarity(&tree1, &tree2);
        assert_eq!(distance, 3);
        assert!((similarity - 0.625).abs() < 0.001);
    }
}
