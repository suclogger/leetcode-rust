fn main() {
    println!("Hello, world!");
}

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
use std::rc::Rc;
use std::cell::RefCell;
// 输出二叉树中所有从根到叶子的路径，回溯法使用与否有什么区别？
// A：不需要回溯也可以返回所有组合，因为二叉树的性质决定了到每个根节点的路径是唯一的
pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, cur_path: String, path: &mut Vec<String>) {
        if let Some(mut node) = node {
            let v = node.borrow().val;
            let mut new_path: String = cur_path;
            new_path.push_str(&v.to_string());
            if node.borrow().left == None && node.borrow().right == None {
                path.push(new_path);
                return;
            }
            new_path.push_str("->");
            dfs(node.borrow_mut().left.take(), new_path.to_string(), path);
            dfs(node.borrow_mut().right.take(), new_path.to_string(), path);
        }
    }
    let mut path:Vec<String> = vec![];
    dfs(root, String::from(""), &mut path);
    path
}

