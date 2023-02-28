use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

mod solution {
    use super::TreeNode;

    pub struct Solution;

    // Definition for a binary tree node.
    // #[derive(Debug, PartialEq, Eq)]
    // pub struct TreeNode {
    //   pub val: i32,
    //   pub left: Option<Rc<RefCell<TreeNode>>>,
    //   pub right: Option<Rc<RefCell<TreeNode>>>,
    // }
    //
    // impl TreeNode {
    //   #[inline]
    //   pub fn new(val: i32) -> Self {
    //     TreeNode {
    //       val,
    //       left: None,
    //       right: None
    //     }
    //   }
    // }
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    pub type TreeNodeCell = Rc<RefCell<TreeNode>>;

    #[derive(Clone, Copy)]
    struct Indices {
        inorder: u16,
        postorder: u16,
    }

    struct TreeNodeData {
        cell: TreeNodeCell,
        indices: Indices,
    }

    impl TreeNodeData {
        fn new(val: i32, inorder: u16, postorder: u16) -> Self {
            Self {
                cell: Rc::new(RefCell::new(TreeNode::new(val))),
                indices: Indices { inorder, postorder },
            }
        }
    }

    impl Solution {
        pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<TreeNodeCell> {
            if inorder.is_empty() {
                None
            } else {
                let mut nodes: HashMap<i32, TreeNodeData> = inorder
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(inorder, val)| (val, TreeNodeData::new(val, inorder as u16, u16::MAX)))
                    .collect();

                for (postorder, val) in postorder.iter().copied().enumerate() {
                    nodes
                        .get_mut(&val)
                        .expect("Constraint 5 broken")
                        .indices
                        .postorder = postorder as u16;
                }

                postorder.last().copied().map(|val| {
                    let root: TreeNodeCell = nodes[&val].cell.clone();

                    Self::build_node(&inorder, &postorder, val, 0_usize, &mut nodes);

                    root
                })
            }
        }

        /// Builds out the children of a node
        ///
        /// # Arguments
        ///
        /// * `inorder_slice`: The slice of `inorder` corresponding to all descendent nodes of `val`
        /// * `postorder_slice`: A slice of `postorder` whose last element is `val`
        /// * `val`: The value of the node to build the children of
        /// * `inorder_offset`: The offset relative to `inorder` where `inorder_slice` starts
        /// * `nodes`: The map of value to node data to populate with descendent information
        ///
        /// # Returns
        ///
        /// The index of the minimum `postorder_slice` index that has been used. This is necessary
        /// to know where the left child starts relative to the parent/right child
        fn build_node(
            inorder_slice: &[i32],
            postorder_slice: &[i32],
            val: i32,
            inorder_offset: usize,
            nodes: &mut HashMap<i32, TreeNodeData>,
        ) -> usize {
            let val_indices: Indices = nodes[&val].indices;
            let val_inorder_index: usize = val_indices.inorder as usize - inorder_offset;
            let val_postorder_index: usize = val_indices.postorder as usize;

            let mut min_postorder_index: usize = val_postorder_index;

            if val_inorder_index + 1_usize < inorder_slice.len() {
                // There's a right node, so initialize it
                let right_inorder_slice_start: usize = val_inorder_index + 1_usize;
                let right_postorder_slice: &[i32] = &postorder_slice[..val_postorder_index];
                let right_val: i32 = *right_postorder_slice.last().unwrap();
                let right: TreeNodeCell = nodes[&right_val].cell.clone();

                nodes.get_mut(&val).unwrap().cell.borrow_mut().right = Some(right);
                min_postorder_index = Self::build_node(
                    &inorder_slice[right_inorder_slice_start..],
                    right_postorder_slice,
                    right_val,
                    inorder_offset + right_inorder_slice_start,
                    nodes,
                );
            }

            if val_inorder_index != 0_usize {
                // There's a left node, so initialize it
                let left_postorder_slice: &[i32] = &postorder_slice[..min_postorder_index];
                let left_val: i32 = *left_postorder_slice.last().unwrap();
                let left: TreeNodeCell = nodes[&left_val].cell.clone();

                nodes.get_mut(&val).unwrap().cell.borrow_mut().left = Some(left);
                min_postorder_index = Self::build_node(
                    &inorder_slice[..val_inorder_index],
                    left_postorder_slice,
                    left_val,
                    inorder_offset,
                    nodes,
                );
            }

            min_postorder_index
        }
    }
}

mod tests {
    use super::{solution::*, *};

    macro_rules! tree_node {
        ($val:expr $(, l: $left:expr)? $(, r: $right:expr)?) => {
            Rc::new(RefCell::new({
                #![allow(unused_mut)]

                let mut tree_node: TreeNode = TreeNode::new($val);

                $(
                    tree_node.left = Some($left);
                )?

                $(
                    tree_node.right = Some($right);
                )?

                tree_node
            }))
        };
    }

    #[test]
    fn example_1() {
        let inorder: Vec<i32> = vec![9, 3, 15, 20, 7];
        let postorder: Vec<i32> = vec![9, 15, 7, 20, 3];
        let output: TreeNodeCell = tree_node!(
            3,
            l: tree_node!(9),
            r: tree_node!(20, l: tree_node!(15), r: tree_node!(7))
        );

        assert_eq!(Solution::build_tree(inorder, postorder), Some(output));
    }

    #[test]
    fn example_2() {
        let inorder: Vec<i32> = vec![-1];
        let postorder: Vec<i32> = vec![-1];
        let output: TreeNodeCell = tree_node!(-1);

        assert_eq!(Solution::build_tree(inorder, postorder), Some(output));
    }

    #[test]
    fn example_3() {
        let inorder: Vec<i32> = vec![];
        let postorder: Vec<i32> = vec![];

        assert_eq!(Solution::build_tree(inorder, postorder), None);
    }

    #[test]
    fn example_4() {
        let inorder: Vec<i32> = vec![4, 3, 2, 1, 0];
        let postorder: Vec<i32> = vec![4, 3, 2, 1, 0];
        let output: TreeNodeCell = tree_node!(
            0,
            l: tree_node!(1, l: tree_node!(2, l: tree_node!(3, l: tree_node!(4))))
        );

        assert_eq!(Solution::build_tree(inorder, postorder), Some(output));
    }

    #[test]
    fn example_5() {
        let inorder: Vec<i32> = vec![0, 1, 2, 3, 4];
        let postorder: Vec<i32> = vec![4, 3, 2, 1, 0];
        let output: TreeNodeCell = tree_node!(
            0,
            r: tree_node!(1, r: tree_node!(2, r: tree_node!(3, r: tree_node!(4))))
        );

        assert_eq!(Solution::build_tree(inorder, postorder), Some(output));
    }

    #[test]
    fn example_6() {
        let inorder: Vec<i32> = vec![1, 3, 4, 2, 0];
        let postorder: Vec<i32> = vec![4, 3, 2, 1, 0];
        let output: TreeNodeCell = tree_node!(
            0,
            l: tree_node!(1, r: tree_node!(2, l: tree_node!(3, r: tree_node!(4))))
        );

        assert_eq!(Solution::build_tree(inorder, postorder), Some(output));
    }

    #[test]
    fn example_8() {
        let inorder: Vec<i32> = vec![0, 2, 4, 3, 1];
        let postorder: Vec<i32> = vec![4, 3, 2, 1, 0];
        let output: TreeNodeCell = tree_node!(
            0,
            r: tree_node!(1, l: tree_node!(2, r: tree_node!(3, l: tree_node!(4))))
        );

        assert_eq!(Solution::build_tree(inorder, postorder), Some(output));
    }
}
