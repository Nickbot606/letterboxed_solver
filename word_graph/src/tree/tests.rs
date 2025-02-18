#[cfg(test)]
mod tests {
    use crate::Tree;

    #[test]
    fn test_push_branch() {
        let mut tree = Tree::new(' '); // Dummy root node

        // Add branches
        tree.push_branch(&['a', 'b', 'c']);
        tree.push_branch(&['a', 'b', 'd']);
        tree.push_branch(&['a', 'b']);

        // Verify the tree structure
        if let Some(root) = &tree.head {
            assert_eq!(root.next.len(), 1);


            if let Some(node_a) = root.next.get(&'a') {
                assert_eq!(node_a.elem, 'a');
                assert_eq!(node_a.next.len(), 1);

                if let Some(node_b) = node_a.next.get(&'b') {
                    assert_eq!(node_b.elem, 'b');
                    assert_eq!(node_b.next.len(), 2); 
                    assert!(node_b.tag);

                    if let Some(node_c) = node_b.next.get(&'c') {
                        assert_eq!(node_c.elem, 'c');
                        assert!(node_c.tag);
                    }

                    if let Some(node_d) = node_b.next.get(&'d') {
                        assert_eq!(node_d.elem, 'd');
                        assert!(node_d.tag);
                    }
                }
            }
        }
    }

    #[test]
    fn test_check_branch() {
        let mut tree = Tree::new(' ');

        tree.push_branch(&['a', 'b']);
        assert!(tree.check_branch(&['a', 'b']));
        assert!(!tree.check_branch(&['a', 'b', 'c']));
        assert!(!tree.check_branch(&['a', 'd']));
        assert!(!tree.check_branch(&['a']));
    }

    // #[test]
    // fn test_get_leaves() {
    //     let mut tree = Tree::new(' ');

    //     tree.push_branch(&['a', 'b']);
    //     assert!(tree.check_branch(&['a', 'b']));
    //     assert!(!tree.check_branch(&['a', 'b', 'c']));
    //     assert!(!tree.check_branch(&['a', 'd']));
    //     assert!(!tree.check_branch(&['a']));
        
    // }
}