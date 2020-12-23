#![feature(min_const_generics)]

struct Node<T, const DIM: usize> {
    point: [T; DIM],
    left: Option<Box<Node<T, DIM>>>,
    right: Option<Box<Node<T, DIM>>>,
}

impl<T, const DIM: usize> Node<T, DIM> {
    fn new(point: [T; DIM]) -> Self {
        Self {
            point,
            left: None,
            right: None,
        }
    }
}

#[derive(Default)]
pub struct KdTree<T, const DIM: usize> {
    root: Option<Box<Node<T, DIM>>>,
}

impl<T, const DIM: usize> KdTree<T, DIM> {
    pub fn dim() -> usize {
        DIM
    }
}
impl<T: Ord, const DIM: usize> KdTree<T, DIM> {
    pub fn contains(&self, point: &[T; DIM]) -> bool {
        let mut depth = 0;
        let mut next = self.root.as_ref();
        while let Some(curr) = next {
            let curr_point = &curr.point;
            if curr_point == point {
                return true;
            }
            let d = depth % DIM;
            next = if point[d] <= curr_point[d] {
                curr.left.as_ref()
            } else {
                curr.right.as_ref()
            };
            depth += 1;
        }
        false
    }
    pub fn insert(&mut self, point: [T; DIM]) -> bool {
        let mut depth = 0;
        let mut next = &mut self.root;
        while let Some(curr) = next {
            let curr_point = &curr.point;
            if *curr_point == point {
                return false;
            }
            let d = depth % DIM;
            next = if point[d] <= curr_point[d] {
                &mut curr.left
            } else {
                &mut curr.right
            };
            depth += 1;
        }
        *next = Some(Box::new(Node::new(point)));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::KdTree;

    #[test]
    fn kdtree() {
        let mut kdt = KdTree::<i32, 3>::default();
        kdt.insert([1, 2, 3]);
        kdt.insert([5, 1, -6]);
        kdt.insert([5, 1, -4]);
        kdt.insert([3, 6, 8]);
        kdt.insert([2, 4, 6]);
        kdt.insert([8, 0, 1]);
        assert!(kdt.contains(&[1, 2, 3]));
        assert!(kdt.contains(&[5, 1, -6]));
        assert!(kdt.contains(&[5, 1, -4]));
        assert!(kdt.contains(&[3, 6, 8]));
        assert!(kdt.contains(&[2, 4, 6]));
        assert!(kdt.contains(&[8, 0, 1]));
        assert!(!kdt.contains(&[1, 2, 4]));
        assert!(!kdt.contains(&[0, 3, 1]));
    }
}
