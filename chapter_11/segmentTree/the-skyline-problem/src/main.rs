fn main() {
    assert_eq!(get_skyline(vec![vec![0,2,3],vec![2,5,3]]), vec![vec![0,3],vec![5,0]]);
}
/**
线段树的终极hard+题目
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
        if self.start == self.end {
            self.info = std::cmp::max(self.info, val);
            return;
        }
        if a <= self.start && b >= self.end  && val > self.info{
            self.tag = 1;
            self.info = val;
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
            left_mut.tag = 1;
            left_mut.info = std::cmp::max(self.info, left_mut.info);
            right_mut.tag = 1;
            right_mut.info = std::cmp::max(self.info, right_mut.info);
            self.tag = 0;
        }
    }
}


pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    // 先做离散化，把离散的坐标映射为有限的节点
    let mut set = std::collections::BTreeSet::new();
    for build in buildings.to_vec() {
        set.insert(build[0]);
        set.insert(build[1]);
    }
    let mut pos2id = std::collections::HashMap::new();
    let mut id2pos = std::collections::HashMap::new();
    for (id, pos) in set.into_iter().enumerate() {
        pos2id.entry(pos).or_insert(id);
        id2pos.entry(id).or_insert(pos);
    }

    let mut root = SegmentTreeNode::new(0, pos2id.len() as i32 - 1);
    root.init();
    for build in buildings.to_vec() {
        root.update_range(*pos2id.get(&build[0]).unwrap() as i32,
                          *pos2id.get(&build[1]).unwrap() as i32 - 1, build[2]);
    }

    fn dfs(node: &mut SegmentTreeNode, heights: &mut Vec<(i32, i32)>) {
        if node.start == node.end {
            heights.push((node.start, node.info));
            return;
        }
        node.push_down();
        dfs(node.left.as_mut().unwrap(), heights);
        dfs(node.right.as_mut().unwrap(), heights);
    }
    let mut heights = Vec::new();
    dfs(&mut root, &mut heights);

    let mut ans = Vec::new();
    let mut i = 0;
    while i < heights.len() {
        ans.push(vec![*id2pos.get(&(heights[i].0 as usize)).unwrap(), heights[i].1]);
        while i < heights.len() -1 && heights[i + 1].1 == heights[i].1 {
            i = i + 1;
        }
        i = i + 1;
    }
    ans
}