fn main() {
    assert_eq!(get_modified_array(5, vec![vec![1,3,2],vec![2,4,3],vec![0,2,-2]]), vec![-2,0,3,5,3]);
}
/**
主要通过这道题熟悉下线段树的updateRange的操作，实际这道题通过差分数组来解决效率更高
**/

struct SegmentTreeNode {
    start: i32,
    end: i32,
    left:  Option<Box<SegmentTreeNode>>,
    right:  Option<Box<SegmentTreeNode>>,
    tag: i32,
    info: i32,
}

impl SegmentTreeNode {
    fn new(start: i32, end: i32) -> Self {
        SegmentTreeNode{
            start,
            end,
            left: Option::None,
            right: Option::None,
            tag: 0,
            info: 0,
        }
    }

    fn init(&mut self) {
        if self.start == self.end {
            return;
        }

        let mid = self.start + (self.end - self.start) / 2;
        self.left = Option::Some(Box::new(SegmentTreeNode::new(self.start, mid)));
        self.right = Option::Some(Box::new(SegmentTreeNode::new(mid + 1, self.end)));

        self.left.as_mut().unwrap().init();
        self.right.as_mut().unwrap().init();
    }

    fn update_range(&mut self, a: i32, b: i32, val: i32) {
        if a > self.end || b < self.start {
            return;
        }
        if a <= self.start && b >= self.end {
            self.tag = self.tag + val;
            self.info = self.info + val * (self.end - self.start + 1);
            return;
        }

        self.push_down();
        self.left.as_mut().unwrap().update_range(a, b, val);
        self.right.as_mut().unwrap().update_range(a, b, val);
    }

    fn push_down(&mut self) {
        if self.tag != 0 && self.start < self.end{
            let left_mut = self.left.as_mut().unwrap();
            let right_mut = self.right.as_mut().unwrap();
            left_mut.tag = left_mut.tag + self.tag;
            left_mut.info = left_mut.info + self.tag * (left_mut.end - left_mut.start + 1);
            right_mut.tag = right_mut.tag + self.tag;
            right_mut.info = right_mut.info + self.tag * (right_mut.end - right_mut.start + 1);
            self.tag = 0;
        }
    }
}

pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {

    let mut seg_tree_node = SegmentTreeNode::new(0, length - 1);
    seg_tree_node.init();
    for update in updates {
        seg_tree_node.update_range(update[0], update[1], update[2]);
    }

    fn dfs(node: &mut SegmentTreeNode, ans: &mut Vec<i32>) {
        if node.start == node.end {
            ans.push(node.info);
            return;
        }
        node.push_down();
        dfs(node.left.as_mut().unwrap(), ans);
        dfs(node.right.as_mut().unwrap(), ans);
    }
    let mut ans = Vec::new();
    dfs(&mut seg_tree_node, &mut ans);
    ans
}