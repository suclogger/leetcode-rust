fn main() {
    let na = NumArray::new(vec![9,-8]);
    assert_eq!(na.sum_range(1, 1), -8);
}

pub struct SegTree {
    pub info: i32,
    pub start: i32,
    pub end: i32,
    pub left: Option<Box<SegTree>>,
    pub right: Option<Box<SegTree>>,
}

impl SegTree {
    // init while create
    fn new(start: i32, end: i32, nums: &Vec<i32>) -> Self {
        if start == end  {
            return SegTree {
                info: nums[start as usize],
                start,
                end,
                left: Option::None,
                right: Option::None,
            }
        }
        let mut node = SegTree {
            info: 0,
            start,
            end,
            left: Option::None,
            right: Option::None,
        };
        let mid = (start + end ) / 2;
        let l = SegTree::new(start, mid, nums);
        let r = SegTree::new(mid + 1, end, nums);
        node.info = l.info + r.info;
        node.left = Option::Some(Box::new(l));
        node.right = Option::Some(Box::new(r));
        node
    }

    fn update_single(&mut self, idx: i32, val: i32) {
        if idx < self.start || idx > self.end {
            return;
        }

        if self.start == idx && self.end == idx {
            self.info = val;
            return;
        }

        self.left.as_mut().unwrap().update_single(idx, val);
        self.right.as_mut().unwrap().update_single(idx, val);

        self.info = self.left.as_ref().unwrap().info + self.right.as_ref().unwrap().as_ref().info;
    }

    fn query_range(&self, a: i32, b: i32) -> i32 {
        if a > self.end || b < self.start {
            return 0;
        }

        if a <= self.start && b >= self.end {
            return self.info;
        }

        return self.left.as_ref().unwrap().query_range(a, b) + self.right.as_ref().unwrap().query_range(a, b);
    }

}

struct NumArray {
    pub seg_tree: SegTree,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        NumArray {
            seg_tree : SegTree::new(0, nums.len() as i32 - 1, &nums),
        }
    }

    fn update(&mut self, index: i32, val: i32) {
        self.seg_tree.update_single(index, val);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.seg_tree.query_range(left, right)
    }
}