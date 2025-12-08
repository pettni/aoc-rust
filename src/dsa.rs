/// Disjoint-set data structure with automatic path contraction
#[derive(Debug, Clone)]
pub struct Dsa {
    nodes: Vec<DsaNode>,
    len: usize,
}

#[derive(Debug, Clone)]
struct DsaNode {
    parent: Option<usize>,
    size: usize,
}

impl Dsa {
    /// Create new Dsa with given size. Initially all nodes make up a singleton cluster.
    pub fn new(size: usize) -> Self {
        Dsa {
            nodes: vec![
                DsaNode {
                    parent: None,
                    size: 1
                };
                size
            ],
            len: size,
        }
    }

    /// Number of clusters
    pub fn len(&self) -> usize {
        self.len
    }

    /// Always true
    pub fn is_empty(&self) -> bool {
        self.len != 0
    }

    /// Iterate over all clusters.
    pub fn sizes<'a>(&'a self) -> impl Iterator<Item=(usize, usize)> + use<'a> {
        self.nodes.iter().enumerate().filter(|(_, node)| node.parent.is_none()).map(|(i, node)| (i,  node.size))
    }

    /// Find root node (cluster index) of node idx
    pub fn find(&mut self, idx: usize) -> usize {
        if let Some(pidx) = self.nodes[idx].parent {
            let root_idx = self.find(pidx);
            self.nodes[idx].parent = Some(root_idx);
            return root_idx;
        }
        idx
    }

    // Merge clusters containing two nodes
    pub fn merge(&mut self, idx1: usize, idx2: usize) {
        let c1 = self.find(idx1);
        let c2 = self.find(idx2);
        if c1 != c2 {
            self.nodes[c2].parent = Some(c1);
            self.nodes[c1].size += self.nodes[c2].size;
            self.len -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        let mut dsa = Dsa::new(10);
        assert_eq!(dsa.len, 10);

        for i in 0..10 {
            assert_eq!(dsa.find(i), i);
        }

        assert_ne!(dsa.find(0), dsa.find(1));
        assert_ne!(dsa.find(1), dsa.find(2));
        assert_ne!(dsa.find(1), dsa.find(9));
        dsa.merge(0, 1);
        assert_eq!(dsa.len, 9);
        assert_eq!(dsa.find(0), dsa.find(1));
        assert_ne!(dsa.find(1), dsa.find(2));
        assert_ne!(dsa.find(0), dsa.find(9));
        dsa.merge(1, 9);
        assert_eq!(dsa.len, 8);
        assert_eq!(dsa.find(0), dsa.find(1));
        assert_ne!(dsa.find(1), dsa.find(2));
        assert_eq!(dsa.find(0), dsa.find(9));
    }
}
