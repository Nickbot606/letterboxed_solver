// mod list; // Import the module
mod tree;

use tree::Tree;
// use list::List; // Bring List into scope

#[allow(unused)]
fn main() {
    // let mut test_list = List::new();
    let mut tree:Tree<char> = Tree::new(' ');//Head of the tree does nothing

}
