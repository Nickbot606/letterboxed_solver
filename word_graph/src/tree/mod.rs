mod tests;
use std::collections::HashMap;

#[allow(unused)]
pub struct Tree<T> {
    head: Link<T>,
}

#[allow(unused)]
type Link<T> = Option<Box<Node<T>>>;

#[allow(unused)]
struct Node<T> {
    elem: T,
    next: HashMap<T, Box<Node<T>>>,
    tag: bool,
}

#[allow(unused)]
impl<T> Tree<T> 
where
    T: Eq + Clone + std::hash::Hash, 
{
    // New 
    pub fn new(root_val: T) -> Self {
        Tree {
            head: Some(Box::new(Node {
                elem: root_val,
                next: HashMap::new(),
                tag: false,
            }))
        }
    }

    // puts a branch onto the tree list at a time
    pub fn push_branch(&mut self, branch: &[T]) {
        if branch.is_empty() {
            return;
        }

        let mut current_node = self.head.as_mut().expect("Tree head is missing!");

        for (i, val) in branch.iter().enumerate() {
            let child_exists = current_node.next.contains_key(val);

            if child_exists {
                current_node = current_node.next.get_mut(val).unwrap();
                if i == branch.len() - 1 {
                    current_node.tag = true;
                }
            } else {
                let new_node = Box::new(Node {
                    elem: val.clone(),
                    next: HashMap::new(),
                    tag: i == branch.len() - 1
                });

                current_node.next.insert(val.clone(), new_node);
                current_node = current_node.next.get_mut(val).unwrap();
            }
        }
    }

    // verify if the branch actually exists
    pub fn check_branch(&mut self, branch: &[T]) -> bool {
        let mut current_node = self.head.as_mut().expect("Tree head is missing!");

        for (i, val) in branch.iter().enumerate() {
            let child_exists = current_node.next.contains_key(val);

            if child_exists {
                current_node = current_node.next.get_mut(val).unwrap();
                if i == branch.len() - 1 {
                    return current_node.tag;
                }
            } else {
                return false;
            }
        }
        return false;
    }

    // prints out all the leaves on the tree from a specified branch
    pub fn get_leaves(&mut self, branch: &[T]) -> Option<Vec<&T>> {
        let mut current_node = self.head.as_mut().expect("Tree head is missing!");

        for (i, val) in branch.iter().enumerate() {
            let child_exists = current_node.next.contains_key(val);

            if child_exists {
                current_node = current_node.next.get_mut(val).unwrap();
                if i == branch.len() - 1 {
                    return Some(current_node.next.keys().collect());
                }
            } else {
                return None;
            }
        }
        return None;
    }


}