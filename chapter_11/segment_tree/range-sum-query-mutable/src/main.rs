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
    fn new(start: i32, end: i32) -> Self {
        SegTree {
            info: 0,
            start,
            end,
            left: Option::None,
            right: Option::None,
        }
    }

    fn init(&mut self, nums: &Vec<i32>) {
        if self.start == self.end {
            self.info = nums[self.start as usize];
            return;
        }
        let mid = (self.start + self.end ) / 2;
        if let None = self.left {
            let l = SegTree::new(self.start, mid);
            let r = SegTree::new(mid + 1, self.end);
            self.left = Option::Some(Box::new(l));
            self.right = Option::Some(Box::new(r));
        }

        self.left.as_mut().unwrap().init(nums);
        self.right.as_mut().unwrap().init(nums);

        self.info = self.left.as_ref().unwrap().info + self.right.as_ref().unwrap().info;
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
        let mut na = NumArray {
            seg_tree : SegTree::new(0, nums.len() as i32 - 1),
        };
        na.seg_tree.init(&nums);
        na
    }

    fn update(&mut self, index: i32, val: i32) {
        self.seg_tree.update_single(index, val);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.seg_tree.query_range(left, right)
    }
}